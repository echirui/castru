use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tokio::runtime::Runtime;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn benchmark_buffer_copy(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let size = 10 * 1024 * 1024; // 10MB
    let data = vec![0u8; size];

    let mut group = c.benchmark_group("buffer_copy");
    group.throughput(Throughput::Bytes(size as u64));

    group.bench_function("copy_64k", |b| {
        b.to_async(&rt).iter(|| async {
            let mut reader = &data[..];
            let mut writer = Vec::with_capacity(size);
            let mut buf = [0u8; 65536]; // 64KB
            loop {
                let n = reader.read(&mut buf).await.unwrap();
                if n == 0 { break; }
                writer.write_all(&buf[..n]).await.unwrap();
            }
        })
    });

    group.bench_function("copy_1m", |b| {
        b.to_async(&rt).iter(|| async {
            let mut reader = &data[..];
            let mut writer = Vec::with_capacity(size);
            let mut buf = [0u8; 1024 * 1024]; // 1MB
            loop {
                let n = reader.read(&mut buf).await.unwrap();
                if n == 0 { break; }
                writer.write_all(&buf[..n]).await.unwrap();
            }
        })
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_buffer_copy);
criterion_main!(benches);
