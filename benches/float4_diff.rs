use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use float4::F4E2M1;
use lowp::f4;
use std::hint::black_box;

const OPS: usize = 4096;

fn bench(c: &mut Criterion) {
    let mut xs = [0u8; OPS];
    let mut ys = [0u8; OPS];
    let mut zs = [0u8; OPS];

    let mut s = 0x12345678u32;

    for i in 0..OPS {
        s ^= s << 13;
        s ^= s >> 17;
        s ^= s << 5;

        xs[i] = s as u8;
        ys[i] = (s >> 8) as u8;
        zs[i] = (s >> 16) as u8;
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

        group.bench_function("float4 f4e2m1 mul", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = F4E2M1::from_bits(black_box(xs[i]));
                    let b = F4E2M1::from_bits(black_box(ys[i]));

                    out ^= F4E2M1::from_f64(a.to_f64() * b.to_f64()).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("lowp f4 mul", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = f4::from_bits(black_box(xs[i]));
                    let b = f4::from_bits(black_box(ys[i]));

                    out ^= (a * b).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("float4 f4e2m1 sub", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = F4E2M1::from_bits(black_box(xs[i]));
                    let b = F4E2M1::from_bits(black_box(ys[i]));

                    out ^= F4E2M1::from_f64(a.to_f64() - b.to_f64()).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("lowp f4 sub", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = f4::from_bits(black_box(xs[i]));
                    let b = f4::from_bits(black_box(ys[i]));

                    out ^= (a - b).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("float4 f4e2m1 div", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = F4E2M1::from_bits(black_box(xs[i]));
                    let b = F4E2M1::from_bits(black_box(ys[i]));

                    out ^= F4E2M1::from_f64(a.to_f64() / b.to_f64()).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("lowp f4 div", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = f4::from_bits(black_box(xs[i]));
                    let b = f4::from_bits(black_box(ys[i]));

                    out ^= (a / b).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("float4 f4e2m1 rem", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = F4E2M1::from_bits(black_box(xs[i]));
                    let b = F4E2M1::from_bits(black_box(ys[i]));

                    out ^= F4E2M1::from_f64(a.to_f64() % b.to_f64()).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("lowp f4 rem", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = f4::from_bits(black_box(xs[i]));
                    let b = f4::from_bits(black_box(ys[i]));

                    out ^= (a % b).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("float4 f4e2m1 mul_add", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = F4E2M1::from_bits(black_box(xs[i]));
                    let b = F4E2M1::from_bits(black_box(ys[i]));
                    let c = F4E2M1::from_bits(black_box(zs[i]));

                    out ^= F4E2M1::from_f64(a.to_f64().mul_add(b.to_f64(), c.to_f64())).to_bits();
                }

                black_box(out)
            });
        });

        group.bench_function("lowp f4 mul_add", |bencher| {
            bencher.iter(|| {
                let mut out = 0u8;

                for i in 0..OPS {
                    let a = f4::from_bits(black_box(xs[i]));
                    let b = f4::from_bits(black_box(ys[i]));
                    let c = f4::from_bits(black_box(zs[i]));

                    out ^= a.mul_add(b, c).to_bits();
                }

                black_box(out)
            });
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
