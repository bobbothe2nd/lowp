use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use float4::F4E2M1;
use float8::F8E4M3;
use lowp::{f4, hf8fn};
use std::hint::black_box;

const OPS: usize = 4096;

fn bench(c: &mut Criterion) {
    let mut xs = [0u8; OPS];
    let mut ys = [0u8; OPS];

    let mut s = 0x12345678u32;

    for i in 0..OPS {
        s ^= s << 13;
        s ^= s >> 17;
        s ^= s << 5;

        xs[i] = s as u8;
        ys[i] = (s >> 8) as u8;
    }

    {
        let mut group = c.benchmark_group("f8e4m3");

        group.throughput(Throughput::Elements(OPS as u64));

        group.bench_function("float8 f8e4m3 add", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = F8E4M3::from_bits(black_box(xs[i]));
                    let b = F8E4M3::from_bits(black_box(ys[i]));

                    out ^= (a + b).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("lowp hf8fn add", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = hf8fn::from_bits(black_box(xs[i]));
                    let b = hf8fn::from_bits(black_box(ys[i]));

                    out ^= (a + b).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("float8 f8e4m3 mul", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = F8E4M3::from_bits(black_box(xs[i]));
                    let b = F8E4M3::from_bits(black_box(ys[i]));

                    out ^= (a * b).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("lowp hf8fn mul", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = hf8fn::from_bits(black_box(xs[i]));
                    let b = hf8fn::from_bits(black_box(ys[i]));

                    out ^= (a * b).to_bits();
                }

                black_box(out)
            });
        });
    }

    {
        let mut group = c.benchmark_group("f4e2m1");

        group.throughput(Throughput::Elements(OPS as u64));

        group.bench_function("float4 f4e2m1 add", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = F4E2M1::from_bits(black_box(xs[i]));
                    let b = F4E2M1::from_bits(black_box(ys[i]));

                    out ^= F4E2M1::from_f64(a.to_f64() + b.to_f64()).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("lowp f4 add", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = f4::from_bits(black_box(xs[i]));
                    let b = f4::from_bits(black_box(ys[i]));

                    out ^= (a + b).to_bits();
                }

                black_box(out)
            });
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
