use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use crate::layers::{
    core::{Bytes, Direction, Value},
    encryptors::KeyBytes,
    Compressible, Correctable, Encryptable, Serializable,
};
use std::collections::HashMap;

// Benchmark data structures
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct SmallRecord {
    id: u64,
    name: String,
    value: u32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct MediumRecord {
    id: u64,
    metadata: HashMap<String, String>,
    tags: Vec<String>,
    data: Vec<u8>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct LargeRecord {
    id: u64,
    document: String,
    binary_data: Vec<u8>,
    metadata: HashMap<String, String>,
}

// Trait implementations for benchmark data
macro_rules! impl_layer_traits {
    ($type:ty, $compress_level:expr, $ecc_level:expr) => {
        #[cfg(all(
            feature = "serde-safety",
            any(
                feature = "serializer-bincode-serde",
                feature = "serializer-bitcode-serde", 
                feature = "serializer-messagepack",
                feature = "serializer-postcard-serde"
            )
        ))]
        unsafe impl crate::layers::serializers::SafeForSerde for $type {}

        impl Serializable for $type {
            const DIRECTION: Direction = Direction::Both;
        }

        impl Compressible for $type {
            const DIRECTION: Direction = Direction::Both;
            const LEVEL: crate::layers::compressors::Level = $compress_level;
        }

        impl Encryptable for $type {
            const DIRECTION: Direction = Direction::Both;
        }

        impl Correctable for $type {
            const DIRECTION: Direction = Direction::Both;
            const LEVEL: crate::layers::correctors::Level = $ecc_level;
        }
    };
}

impl_layer_traits!(SmallRecord, crate::layers::compressors::Level::Fast, crate::layers::correctors::Level::Fast);
impl_layer_traits!(MediumRecord, crate::layers::compressors::Level::Balanced, crate::layers::correctors::Level::Balanced);
impl_layer_traits!(LargeRecord, crate::layers::compressors::Level::Maximum, crate::layers::correctors::Level::Maximum);

// Test data generators
fn create_small_record() -> SmallRecord {
    SmallRecord {
        id: 12345,
        name: "Test Record".to_string(),
        value: 42,
    }
}

fn create_medium_record() -> MediumRecord {
    let mut metadata = HashMap::new();
    metadata.insert("type".to_string(), "test".to_string());
    metadata.insert("version".to_string(), "1.0".to_string());
    metadata.insert("author".to_string(), "benchmark".to_string());

    MediumRecord {
        id: 67890,
        metadata,
        tags: vec!["rust".to_string(), "database".to_string(), "benchmark".to_string()],
        data: (0..1024).map(|i| (i % 256) as u8).collect(), // 1KB of data
    }
}

fn create_large_record(size_kb: usize) -> LargeRecord {
    let mut metadata = HashMap::new();
    for i in 0..50 {
        metadata.insert(format!("key_{}", i), format!("value_{}", i));
    }

    LargeRecord {
        id: 999999,
        document: "Lorem ipsum ".repeat(100), // ~1.1KB of text
        binary_data: (0..255).cycle().take(size_kb * 1024).collect(),
        metadata,
    }
}

fn benchmark_key() -> KeyBytes<'static> {
    b"BENCHMARK_KEY_32_BYTES__________".into()
}

// Individual layer benchmarks
fn bench_serialization_only(c: &mut Criterion) {
    let small = create_small_record();
    let medium = create_medium_record();
    let large = create_large_record(10); // 10KB

    let mut group = c.benchmark_group("serialization_only");
    
    group.bench_function("small_record", |b| {
        b.iter(|| {
            let serialized = serde_json::to_vec(&small).unwrap();
            black_box(serialized);
        });
    });

    group.bench_function("medium_record", |b| {
        b.iter(|| {
            let serialized = serde_json::to_vec(&medium).unwrap();
            black_box(serialized);
        });
    });

    group.bench_function("large_record", |b| {
        b.iter(|| {
            let serialized = serde_json::to_vec(&large).unwrap();
            black_box(serialized);
        });
    });

    group.finish();
}

// Full pipeline benchmarks
fn bench_full_pipeline_write(c: &mut Criterion) {
    let key = benchmark_key();
    let small = create_small_record();
    let medium = create_medium_record();

    let mut group = c.benchmark_group("pipeline_write");
    
    group.bench_function("small_record", |b| {
        b.iter(|| {
            let result = Bytes::apply_write_layers(
                black_box(&small), 
                (*key).into(), 
                None, 
                None
            ).unwrap();
            black_box(result);
        });
    });

    group.bench_function("medium_record", |b| {
        b.iter(|| {
            let result = Bytes::apply_write_layers(
                black_box(&medium), 
                (*key).into(), 
                None, 
                None
            ).unwrap();
            black_box(result);
        });
    });

    // Test various large record sizes
    for size_kb in [1, 10, 100, 1000] {
        let large = create_large_record(size_kb);
        group.throughput(Throughput::Bytes((size_kb * 1024) as u64));
        group.bench_with_input(
            BenchmarkId::new("large_record", format!("{}KB", size_kb)),
            &large,
            |b, data| {
                b.iter(|| {
                    let result = Bytes::apply_write_layers(
                        black_box(data), 
                        (*key).into(), 
                        None, 
                        None
                    ).unwrap();
                    black_box(result);
                });
            }
        );
    }

    group.finish();
}

fn bench_full_pipeline_read(c: &mut Criterion) {
    let key = benchmark_key();
    
    // Pre-serialize test data
    let small = create_small_record();
    let medium = create_medium_record();
    
    let small_bytes = Bytes::apply_write_layers(&small, (*key).into(), None, None).unwrap();
    let medium_bytes = Bytes::apply_write_layers(&medium, (*key).into(), None, None).unwrap();

    let mut group = c.benchmark_group("pipeline_read");
    
    group.bench_function("small_record", |b| {
        b.iter(|| {
            let result: crate::layers::core::ValueOrBytes<SmallRecord> = 
                Bytes::apply_read_layers(black_box(small_bytes.clone()), key, None).unwrap();
            black_box(result);
        });
    });

    group.bench_function("medium_record", |b| {
        b.iter(|| {
            let result: crate::layers::core::ValueOrBytes<MediumRecord> = 
                Bytes::apply_read_layers(black_box(medium_bytes.clone()), key, None).unwrap();
            black_box(result);
        });
    });

    // Test various large record sizes
    for size_kb in [1, 10, 100, 1000] {
        let large = create_large_record(size_kb);
        let large_bytes = Bytes::apply_write_layers(&large, (*key).into(), None, None).unwrap();
        
        group.throughput(Throughput::Bytes((size_kb * 1024) as u64));
        group.bench_with_input(
            BenchmarkId::new("large_record", format!("{}KB", size_kb)),
            &large_bytes,
            |b, data| {
                b.iter(|| {
                    let result: crate::layers::core::ValueOrBytes<LargeRecord> = 
                        Bytes::apply_read_layers(black_box(data.clone()), key, None).unwrap();
                    black_box(result);
                });
            }
        );
    }

    group.finish();
}

fn bench_round_trip(c: &mut Criterion) {
    let key = benchmark_key();
    let small = create_small_record();
    let medium = create_medium_record();

    let mut group = c.benchmark_group("round_trip");
    
    group.bench_function("small_record", |b| {
        b.iter(|| {
            let bytes = Bytes::apply_write_layers(black_box(&small), (*key).into(), None, None).unwrap();
            let result: crate::layers::core::ValueOrBytes<SmallRecord> = 
                Bytes::apply_read_layers(bytes, key, None).unwrap();
            black_box(result);
        });
    });

    group.bench_function("medium_record", |b| {
        b.iter(|| {
            let bytes = Bytes::apply_write_layers(black_box(&medium), (*key).into(), None, None).unwrap();
            let result: crate::layers::core::ValueOrBytes<MediumRecord> = 
                Bytes::apply_read_layers(bytes, key, None).unwrap();
            black_box(result);
        });
    });

    // Test compression effectiveness with repetitive data
    let repetitive_large = LargeRecord {
        id: 1,
        document: "A".repeat(50000), // Very compressible
        binary_data: vec![0xFF; 50000], // Also very compressible
        metadata: HashMap::new(),
    };

    group.bench_function("compressible_large", |b| {
        b.iter(|| {
            let bytes = Bytes::apply_write_layers(black_box(&repetitive_large), (*key).into(), None, None).unwrap();
            let result: crate::layers::core::ValueOrBytes<LargeRecord> = 
                Bytes::apply_read_layers(bytes, key, None).unwrap();
            black_box(result);
        });
    });

    group.finish();
}

// Compression ratio analysis
fn bench_compression_ratios(c: &mut Criterion) {
    let key = benchmark_key();
    
    let mut group = c.benchmark_group("compression_analysis");
    group.sample_size(10); // Fewer samples since we're measuring ratios
    
    // Test different data types for compression effectiveness
    let test_cases = vec![
        ("random_data", create_large_record(100)), // Random data - poor compression
        ("repetitive_text", LargeRecord {
            id: 1,
            document: "Hello world! ".repeat(8333), // ~100KB repetitive text
            binary_data: vec![],
            metadata: HashMap::new(),
        }),
        ("binary_pattern", LargeRecord {
            id: 2,
            document: String::new(),
            binary_data: (0..255).cycle().take(100 * 1024).collect(), // Patterned binary
            metadata: HashMap::new(),
        }),
    ];

    for (name, data) in test_cases {
        group.bench_function(name, |b| {
            b.iter(|| {
                let original_size = serde_json::to_vec(&data).unwrap().len();
                let compressed = Bytes::apply_write_layers(black_box(&data), (*key).into(), None, None).unwrap();
                let compressed_size = compressed.len();
                
                // Log compression ratio (in real benchmark, you'd collect this data)
                let ratio = original_size as f64 / compressed_size as f64;
                eprintln!("{}: {:.2}x compression ({} -> {} bytes)", 
                         name, ratio, original_size, compressed_size);
                
                black_box(compressed);
            });
        });
    }

    group.finish();
}

// Error correction overhead benchmark
fn bench_ecc_overhead(c: &mut Criterion) {
    // This would require implementing layer-specific benchmarks
    // comparing with and without ECC enabled
    let mut group = c.benchmark_group("ecc_overhead");
    
    // TODO: Implement ECC vs no-ECC comparison
    // This would require conditional compilation or runtime configuration
    
    group.finish();
}

criterion_group!(
    benches,
    bench_serialization_only,
    bench_full_pipeline_write,
    bench_full_pipeline_read,
    bench_round_trip,
    bench_compression_ratios,
    bench_ecc_overhead
);

criterion_main!(benches);