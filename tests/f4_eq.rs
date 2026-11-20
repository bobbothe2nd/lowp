use float4::F4E2M1;
use lowp::f4;

fn reference_add(a: u8, b: u8) -> u8 {
    let x = F4E2M1::from_bits(a);
    let y = F4E2M1::from_bits(b);

    F4E2M1::from_f64(x.to_f64() + y.to_f64()).to_bits()
}

fn reference_sub(a: u8, b: u8) -> u8 {
    let x = F4E2M1::from_bits(a);
    let y = F4E2M1::from_bits(b);

    F4E2M1::from_f64(x.to_f64() - y.to_f64()).to_bits()
}

fn reference_mul(a: u8, b: u8) -> u8 {
    let x = F4E2M1::from_bits(a);
    let y = F4E2M1::from_bits(b);

    F4E2M1::from_f64(x.to_f64() * y.to_f64()).to_bits()
}

fn reference_div(a: u8, b: u8) -> u8 {
    let x = F4E2M1::from_bits(a);
    let y = F4E2M1::from_bits(b);

    F4E2M1::from_f64(x.to_f64() / y.to_f64()).to_bits()
}

fn reference_rem(a: u8, b: u8) -> u8 {
    let x = F4E2M1::from_bits(a);
    let y = F4E2M1::from_bits(b);

    F4E2M1::from_f64(x.to_f64() % y.to_f64()).to_bits()
}

fn reference_fma(a: u8, b: u8, c: u8) -> u8 {
    let x = F4E2M1::from_bits(a);
    let y = F4E2M1::from_bits(b);
    let z = F4E2M1::from_bits(c);

    F4E2M1::from_f64(x.to_f64().mul_add(y.to_f64(), z.to_f64())).to_bits()
}

fn check_binary_operation(name: &str, operation: fn(u8, u8) -> u8, reference: fn(u8, u8) -> u8) {
    let mut failures = 0;

    for a in 0u16..=15 {
        for b in 0u16..=15 {
            let a = a as u8;
            let b = b as u8;

            let ref_result = reference(a, b);
            let soft_result = operation(a, b);

            if ref_result != soft_result {
                if failures < 20 {
                    println!(
                        "mismatch: {name}({a:02x}, {b:02x}): \
                         soft={soft_result:02x}, ref={ref_result:02x}"
                    );
                }

                failures += 1;
            }
        }
    }

    println!("{name} mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_f4_add() {
    check_binary_operation(
        "add",
        |a, b| {
            let x = f4::from_bits(a);
            let y = f4::from_bits(b);
            (x + y).to_bits()
        },
        reference_add,
    );
}

#[test]
fn check_f4_sub() {
    check_binary_operation(
        "sub",
        |a, b| {
            let x = f4::from_bits(a);
            let y = f4::from_bits(b);
            (x - y).to_bits()
        },
        reference_sub,
    );
}

#[test]
fn check_f4_mul() {
    check_binary_operation(
        "mul",
        |a, b| {
            let x = f4::from_bits(a);
            let y = f4::from_bits(b);
            (x * y).to_bits()
        },
        reference_mul,
    );
}

#[test]
fn check_f4_div() {
    check_binary_operation(
        "div",
        |a, b| {
            let x = f4::from_bits(a);
            let y = f4::from_bits(b);
            (x / y).to_bits()
        },
        reference_div,
    );
}

#[test]
fn check_f4_rem() {
    check_binary_operation(
        "rem",
        |a, b| {
            let x = f4::from_bits(a);
            let y = f4::from_bits(b);
            (x % y).to_bits()
        },
        reference_rem,
    );
}

#[test]
fn check_f4_fma() {
    let mut failures = 0;

    for a in 0u16..=15 {
        for b in 0u16..=15 {
            for c in 0u16..=15 {
                let a = a as u8;
                let b = b as u8;
                let c = c as u8;

                let ref_result = reference_fma(a, b, c);

                let soft_result = {
                    let x = f4::from_bits(a);
                    let y = f4::from_bits(b);
                    let z = f4::from_bits(c);

                    x.mul_add(y, z).to_bits()
                };

                if ref_result != soft_result {
                    if failures < 20 {
                        println!(
                            "mismatch: fma({a:02x}, {b:02x}, {c:02x}): \
                             soft={soft_result:02x}, ref={ref_result:02x}"
                        );
                    }

                    failures += 1;
                }
            }
        }
    }

    println!("fma mismatches: {failures}");
    assert_eq!(failures, 0);
}

#[test]
fn check_f4_to_f32() {
    let mut failures = 0;

    for bits in 0u16..=15 {
        let bits = bits as u8;

        let reference = F4E2M1::from_bits(bits).to_f64() as f32;
        let soft = f4::from_bits(bits).to_f32();

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
fn check_f4_to_f64() {
    let mut failures = 0;

    for bits in 0u16..=15 {
        let bits = bits as u8;

        let reference = F4E2M1::from_bits(bits).to_f64();
        let soft = f4::from_bits(bits).to_f64();

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
fn check_f4_from_f32() {
    let mut failures = 0;

    for bits in 0u16..=15 {
        let bits = bits as u8;

        let reference = F4E2M1::from_bits(bits).to_f64() as f32;

        let ref_bits = F4E2M1::from_f64(reference as f64).to_bits();
        let soft_bits = f4::from_f32(reference).to_bits();

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
fn check_f4_from_f64() {
    let mut failures = 0;

    for bits in 0u16..=15 {
        let bits = bits as u8;

        let reference = F4E2M1::from_bits(bits).to_f64();

        let ref_bits = F4E2M1::from_f64(reference).to_bits();
        let soft_bits = f4::from_f64(reference).to_bits();

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
fn check_f4_rounding_boundaries() {
    let cases: &[(f64, u8)] = &[
        (0.0, 0x0),
        (0.125, 0x0),
        (0.25, 0x0),
        (0.375, 0x1),
        (0.5, 0x1),
        (0.625, 0x1),
        (0.75, 0x2),
        (1.0, 0x2),
        (1.25, 0x2),
        (1.5, 0x3),
        (2.0, 0x4),
        (3.0, 0x5),
        (4.0, 0x6),
        (6.0, 0x7),
        (-0.0, 0x8),
        (-0.125, 0x8),
        (-0.25, 0x8),
        (-0.375, 0x9),
        (-0.5, 0x9),
        (-0.625, 0x9),
        (-0.75, 0xA),
        (-1.0, 0xA),
        (-1.25, 0xA),
        (-1.5, 0xB),
        (-2.0, 0xC),
        (-3.0, 0xD),
        (-4.0, 0xE),
        (-6.0, 0xF),
    ];

    for &(input, expected) in cases {
        let soft_f64 = f4::from_f64(input).to_bits();
        let soft_f32 = f4::from_f32(input as f32).to_bits();

        let reference = F4E2M1::from_f64(input).to_bits();

        assert_eq!(
            soft_f64, expected,
            "from_f64({input}): got {soft_f64:02x}, expected {expected:02x}"
        );

        assert_eq!(
            soft_f32, expected,
            "from_f32({input}): got {soft_f32:02x}, expected {expected:02x}"
        );

        assert_eq!(
            reference, expected,
            "reference({input}): got {reference:02x}, expected {expected:02x}"
        );
    }
}
