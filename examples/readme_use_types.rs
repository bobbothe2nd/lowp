use lowp::{
    bf6, hf6,
    bf8, bf8uz,
    hf8, hf8fn, hf8uz,
    f4,
    bf6x4, hf6x4, f4x2,
};

fn main() {
    // scalar construction
    let a = bf6::from_f32(1.5);
    let b = hf6::from_f32(2.25);

    let c = bf8::from_f32(3.5);
    let d = bf8uz::from_f32(4.5);

    let e = hf8::from_f32(5.5);
    let f = hf8fn::from_f32(6.5);
    let g = hf8uz::from_f32(7.5);

    let h = f4::from_f32(0.5);

    println!("bf6:   {a}");
    println!("hf6:   {b}");
    println!("bf8:   {c}");
    println!("bf8uz: {d}");
    println!("hf8:   {e}");
    println!("hf8fn: {f}");
    println!("hf8uz: {g}");
    println!("f4:    {h}");

    // raw bit representations
    let raw = bf8::from_bits(0x3F);

    println!("bf8 bits: {:#04X}", raw.to_bits());

    // scalar arithmetic
    let x = hf8fn::from_f32(2.0);
    let y = hf8fn::from_f32(3.0);
    let z = hf8fn::from_f32(4.0);

    println!("add:     {}", x + y);
    println!("sub:     {}", x - y);
    println!("mul:     {}", x * y);
    println!("div:     {}", x / y);
    println!("rem:     {}", x % y);
    println!("mul_add: {}", x.mul_add(y, z));

    // classification
    println!("is_nan:    {}", x.is_nan());
    println!("is_finite: {}", x.is_finite());
    println!("is_sign_positive: {}", x.is_sign_positive());
    println!("is_sign_negative: {}", x.is_sign_negative());

    // vector construction
    let bf6_vec = bf6x4::from_array([
        bf6::from_bits(0x00),
        bf6::from_bits(0x01),
        bf6::from_bits(0x02),
        bf6::from_bits(0x03),
    ]);
    let hf6_vec = hf6x4::from_array([
        hf6::from_bits(0x00),
        hf6::from_bits(0x01),
        hf6::from_bits(0x02),
        hf6::from_bits(0x03),
    ]);
    let f4_vec = f4x2::new(f4::from_bits(0x0), f4::from_bits(0x1));

    println!("bf6x4: {bf6_vec:?}");
    println!("hf6x4: {hf6_vec:?}");
    println!("f4x2:  {f4_vec:?}");
}
