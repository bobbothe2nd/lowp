use float8::F8E4M3;
use lowp::hf8fn;

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
fn check_hf8_to_f32() {
    let mut failures = 0;

    for bits in 0u16..=255 {
        let bits = bits as u8;

        let reference = F8E4M3::from_bits(bits).to_f32();
        let soft = hf8fn::from_bits(bits).to_f32();

        if reference.to_bits() != soft.to_bits()
            && !(reference.is_nan() && soft.is_nan())
        {
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

        if reference.to_bits() != soft.to_bits()
            && !(reference.is_nan() && soft.is_nan())
        {
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
