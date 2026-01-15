use crate::error::CastError;
use crate::proto::CastMessage;
use bytes::{Buf, BufMut, BytesMut};
use prost::Message;

pub struct CastCodec;

impl CastCodec {
    pub fn encode(item: &CastMessage, dst: &mut BytesMut) -> Result<(), CastError> {
        let len = item.encoded_len();
        dst.reserve(4 + len);
        dst.put_u32(len as u32);
        item.encode(dst).map_err(CastError::from)
    }

    pub fn decode(src: &mut BytesMut) -> Result<Option<CastMessage>, CastError> {
        if src.len() < 4 {
            return Ok(None);
        }

        // Peek length without advancing
        let mut len_bytes = [0u8; 4];
        len_bytes.copy_from_slice(&src[..4]);
        let len = u32::from_be_bytes(len_bytes) as usize;

        if src.len() < 4 + len {
            // Not enough data yet
            dst_reserve(src, 4 + len - src.len());
            return Ok(None);
        }

        // We have the full message
        src.advance(4); // Consume header
        let data = src.split_to(len); // Consume payload
        let msg = CastMessage::decode(data).map_err(CastError::from)?;
        Ok(Some(msg))
    }
}

fn dst_reserve(dst: &mut BytesMut, additional: usize) {
    dst.reserve(additional);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::CastMessage;
    use bytes::BufMut;

    #[test]
    fn test_encode_decode() {
        let mut msg = CastMessage::default();
        msg.protocol_version = 0; // CASTV2_1_0
        msg.source_id = "sender-0".to_string();
        msg.destination_id = "receiver-0".to_string();
        msg.namespace = "urn:x-cast:com.google.cast.tp.heartbeat".to_string();
        msg.payload_type = 0; // STRING
        msg.payload_utf8 = Some("PING".to_string());

        let mut buf = BytesMut::new();
        CastCodec::encode(&msg, &mut buf).unwrap();

        // Check length
        assert!(buf.len() > 4);
        let len = u32::from_be_bytes(buf[..4].try_into().unwrap());
        assert_eq!(len as usize, buf.len() - 4);

        // Decode
        let decoded = CastCodec::decode(&mut buf).unwrap().unwrap();
        assert_eq!(decoded.source_id, "sender-0");
        assert_eq!(decoded.payload_utf8, Some("PING".to_string()));
    }

    #[test]
    fn test_decode_partial() {
        let mut buf = BytesMut::new();
        // Add length 10
        buf.put_u32(10);
        // Add 5 bytes of data
        buf.put_slice(&[0u8; 5]);

        let res = CastCodec::decode(&mut buf).unwrap();
        assert!(res.is_none());
        assert_eq!(buf.len(), 9); // 4 + 5 (not consumed)
    }
}
