use float8::F8E4M3;
use lowp::hf8fn;

#[cfg(feature = "half")]
use half::bf16;

#[test]
fn check_hf8_add() {
    let mut failures = 0;

    for a in 0u16..=255 {
        for b in 0u16..=255 {
            let a = a as u8;
            let b = b as u8;

            let ref_add = {
                let x = F8E4M3::from_bits(a);
                let y = F8E4M3::from_bits(b);

                (x + y).to_bits()
            };

            let soft_add = {
                let x = hf8fn::from_bits(a);
                let y = hf8fn::from_bits(b);

                (x + y).to_bits()
            };

            if ref_add != soft_add {
                if failures < 20 {
                    println!("mismatch: {a:02x} + {b:02x}: soft={soft_add:02x}, ref={ref_add:02x}");
                }
                failures += 1;
            }
        }
    }

    println!("mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_hf8_sub() {
    let mut failures = 0;

    for a in 0u16..=255 {
        for b in 0u16..=255 {
            let a = a as u8;
            let b = b as u8;

            let ref_sub = {
                let x = F8E4M3::from_bits(a);
                let y = F8E4M3::from_bits(b);

                (x - y).to_bits()
            };

            let soft_sub = {
                let x = hf8fn::from_bits(a);
                let y = hf8fn::from_bits(b);

                (x - y).to_bits()
            };

            if ref_sub != soft_sub {
                if failures < 20 {
                    println!("mismatch: {a:02x} - {b:02x}: soft={soft_sub:02x}, ref={ref_sub:02x}");
                }
                failures += 1;
            }
        }
    }

    println!("mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_hf8_mul() {
    let mut failures = 0;

    for a in 0u16..=255 {
        for b in 0u16..=255 {
            let a = a as u8;
            let b = b as u8;

            let ref_mul = {
                let x = F8E4M3::from_bits(a);
                let y = F8E4M3::from_bits(b);

                (x * y).to_bits()
            };

            let soft_mul = {
                let x = hf8fn::from_bits(a);
                let y = hf8fn::from_bits(b);

                (x * y).to_bits()
            };

            if ref_mul != soft_mul {
                if failures < 20 {
                    println!("mismatch: {a:02x} * {b:02x}: soft={soft_mul:02x}, ref={ref_mul:02x}");
                }
                failures += 1;
            }
        }
    }

    println!("mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_hf8_div() {
    let mut failures = 0;

    for a in 0u16..=255 {
        for b in 0u16..=255 {
            let a = a as u8;
            let b = b as u8;

            let ref_div = {
                let x = F8E4M3::from_bits(a);
                let y = F8E4M3::from_bits(b);

                (x / y).to_bits()
            };

            let soft_div = {
                let x = hf8fn::from_bits(a);
                let y = hf8fn::from_bits(b);

                (x / y).to_bits()
            };

            if ref_div != soft_div {
                if failures < 20 {
                    println!("mismatch: {a:02x} / {b:02x}: soft={soft_div:02x}, ref={ref_div:02x}");
                }
                failures += 1;
            }
        }
    }

    println!("mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_hf8_to_f32() {
    let mut failures = 0;

    for bits in 0u16..=255 {
        let bits = bits as u8;

        let reference = F8E4M3::from_bits(bits).to_f32();
        let soft = hf8fn::from_bits(bits).to_f32();

        if reference.to_bits() != soft.to_bits() && !(reference.is_nan() && soft.is_nan()) {
            if failures < 20 {
                println!(
                    "to_f32 mismatch: {bits:02x}: \
                     soft={:08x}, ref={:08x}",
                    soft.to_bits(),
                    reference.to_bits(),
                );
            }
            failures += 1;
        }
    }

    println!("to_f32 mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_hf8_to_f64() {
    let mut failures = 0;

    for bits in 0u16..=255 {
        let bits = bits as u8;

        let reference = F8E4M3::from_bits(bits).to_f64();
        let soft = hf8fn::from_bits(bits).to_f64();

        if reference.to_bits() != soft.to_bits() && !(reference.is_nan() && soft.is_nan()) {
            if failures < 20 {
                println!(
                    "to_f64 mismatch: {bits:02x}: \
                     soft={:016x}, ref={:016x}",
                    soft.to_bits(),
                    reference.to_bits(),
                );
            }
            failures += 1;
        }
    }

    println!("to_f64 mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_hf8_from_f32() {
    let mut failures = 0;

    for bits in 0u16..=255 {
        let bits = bits as u8;
        let reference = F8E4M3::from_bits(bits).to_f32();

        let ref_bits = F8E4M3::from_f32(reference).to_bits();
        let soft_bits = hf8fn::from_f32(reference).to_bits();

        if ref_bits != soft_bits {
            if failures < 20 {
                println!(
                    "from_f32 mismatch: input={:08x}: \
                     soft={soft_bits:02x}, ref={ref_bits:02x}",
                    reference.to_bits(),
                );
            }
            failures += 1;
        }
    }

    println!("from_f32 mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_hf8_from_f64() {
    let mut failures = 0;

    for bits in 0u16..=255 {
        let bits = bits as u8;
        let reference = F8E4M3::from_bits(bits).to_f64();

        let ref_bits = F8E4M3::from_f64(reference).to_bits();
        let soft_bits = hf8fn::from_f64(reference).to_bits();

        if ref_bits != soft_bits {
            if failures < 20 {
                println!(
                    "from_f64 mismatch: input={:016x}: \
                     soft={soft_bits:02x}, ref={ref_bits:02x}",
                    reference.to_bits(),
                );
            }
            failures += 1;
        }
    }

    println!("from_f64 mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_hf8_to_f16() {
    let mut failures = 0;

    for bits in 0u16..=255 {
        let bits = bits as u8;

        let reference = F8E4M3::from_bits(bits).to_f16();
        let soft = hf8fn::from_bits(bits).to_f16();

        if reference.to_bits() != soft.to_bits() {
            if failures < 20 {
                println!(
                    "to_f16 mismatch: {bits:02x}: \
                     soft={:04x}, ref={:04x}",
                    soft.to_bits(),
                    reference.to_bits(),
                );
            }
            failures += 1;
        }
    }

    println!("to_f16 mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_hf8_to_bf16() {
    let mut failures = 0;

    for bits in 0u16..=255 {
        let bits = bits as u8;

        let reference = bf16::from_f32(F8E4M3::from_bits(bits).to_f32());
        let soft = hf8fn::from_bits(bits).to_bf16();

        if reference.to_bits() != soft.to_bits() {
            if failures < 20 {
                println!(
                    "to_bf16 mismatch: {bits:02x}: \
                     soft={:04x}, ref={:04x}",
                    soft.to_bits(),
                    reference.to_bits(),
                );
            }
            failures += 1;
        }
    }

    println!("to_bf16 mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
#[cfg(feature = "half")]
fn check_hf8_from_f16() {
    let mut failures = 0;

    for bits in 0u16..=255 {
        let bits = bits as u8;
        let reference = F8E4M3::from_bits(bits).to_f16();

        let ref_bits = F8E4M3::from_f32(reference.to_f32()).to_bits();
        let soft_bits = hf8fn::from_f16(reference).to_bits();

        if ref_bits != soft_bits {
            if failures < 20 {
                println!(
                    "from_f16 mismatch: input={:04x}: \
                     soft={soft_bits:02x}, ref={ref_bits:02x}",
                    reference.to_bits(),
                );
            }
            failures += 1;
        }
    }

    println!("from_f16 mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_hf8_from_bf16() {
    let mut failures = 0;

    for bits in 0u16..=255 {
        let bits = bits as u8;
        let reference = bf16::from_f32(F8E4M3::from_bits(bits).to_f32());

        let ref_bits = F8E4M3::from_f32(reference.to_f32()).to_bits();
        let soft_bits = hf8fn::from_bf16(reference).to_bits();

        if ref_bits != soft_bits {
            if failures < 20 {
                println!(
                    "from_bf16 mismatch: input={:04x}: \
                     soft={soft_bits:02x}, ref={ref_bits:02x}",
                    reference.to_bits(),
                );
            }
            failures += 1;
        }
    }

    println!("from_bf16 mismatches: {failures}");
    assert_eq!(failures, 0);
}
