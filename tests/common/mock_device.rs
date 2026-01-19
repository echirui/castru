use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};
use tokio_rustls::TlsAcceptor;
use rcgen::generate_simple_self_signed;
use tokio::io::AsyncReadExt;

pub struct MockDevice {
    port: u16,
    _join_handle: tokio::task::JoinHandle<()>,
}

impl MockDevice {
    pub async fn start() -> Self {
        let certified_key = generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let cert_der = certified_key.cert.der().to_vec();
        let key_der = certified_key.signing_key.serialize_der();
        
        let certs = vec![Certificate(cert_der)];
        let key = PrivateKey(key_der);

        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .unwrap();
        let acceptor = TlsAcceptor::from(Arc::new(config));

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        let handle = tokio::spawn(async move {
            loop {
                if let Ok((stream, _)) = listener.accept().await {
                    let acceptor = acceptor.clone();
                    tokio::spawn(async move {
                        if let Ok(mut stream) = acceptor.accept(stream).await {
                             let mut buf = [0u8; 1024];
                             while let Ok(n) = stream.read(&mut buf).await {
                                 if n == 0 { break; }
                             }
                        }
                    });
                }
            }
        });

        Self {
            port,
            _join_handle: handle,
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
