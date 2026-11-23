# **Low P**recision (`lowp`)

Provides an extensive set of small float formats:

- `hf8`: 8-bit float format E4M3
- `hf8fn`: 8-bit float format E4M3FN
- `hf8uz`: 8-bit float format E4M3FNUZ
- `bf8`: 8-bit float format E5M2
- `bf8uz`: 8-bit float format E5M2FNUZ
- `f4`: 4-bit float format E2M1
- `f4x2`: 2 packed `f4` into 1 byte

## Crate Features

- `alloc`: Enable features that depend on the Rust `alloc` library
- `std`: Enable features that depend on the Rust standard library (and implement the `num_traits::Float` trait in `no_std`)
- `libm`: Enable the `libm` feature for `num-traits` to implement the `Float` trait in `no_std`
- `serde`: Add support for the `serde` crate with `Serialize` (and `Deserialize` eventually) traits
- `num-traits`: Implement traits from `num-traits` such as `ToPrimitive`, `FromPrimitive`, `AsPrimitive`, `Num`, `Float`, `FloatCore`, and `Bounded`.
- `bytemuck`: Implement traits from `bytemuck` including `Zeroable` and `Pod`
- `zerocopy`: Implement traits from `zerocopy` including `AsBytes` and `FromBytes`
- `stable_auto`: Implement trait from `stable_auto` including `Auto`
- `lut-f4`: Use highly optimized LUT for `f4` operations at the cost of 4 KiB of static storage

## Conversion

All `lowp` float types can be converted to/from all other `lowp` float types and both `core` and `half` float types.

## Example

Where possible, these float types match the standard library as closely as possible.

```rust
use lowp::*;

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
```

which outputs:

```text
bf6:   1.5
hf6:   2.25
bf8:   3.5
bf8uz: 4
hf8:   5.5
hf8fn: 6.5
hf8uz: 7.5
f4:    0.5
bf8 bits: 0x3F
add:     5
sub:     -1
mul:     6
div:     0.6875
rem:     2
mul_add: 10
is_nan:    false
is_finite: true
is_sign_positive: true
is_sign_negative: false
bf6x4: bf6x4[0.0, 0.0625, 0.125, 0.1875]
hf6x4: hf6x4[0.0, 0.125, 0.25, 0.375]
f4x2:  f4x2[0.0, 0.5]
```

## Performance

`lowp` delivers significant performance improvements over standard floating-point implementations for low-precision arithmetic.

### Benchmark configuration

All benchmarks were run with the following configuration:

- CPU: Intel Core i7-1250U
- Benchmark size: 4096 operations
- Compiler settings: Default Rust compiler settings (no `target-cpu=native` or additional CPU-specific optimizations)
- `lut-f4` and `lut-f6` enabled

### Benchmark results

The following table compares the median execution time and throughput of `lowp` against the corresponding standard floating-point implementations.

| Format | Operation    | Crate  | Time       | Throughput      |
|--------|--------------|--------|-----------:|----------------:|
| F8E4M3 | Add          | float8 | 39.727 µs  | 103.10 Melem/s  |
| F8E4M3 | Add          | lowp   | 19.311 µs  | 212.11 Melem/s  |
| F8E4M3 | Multiply     | float8 | 38.648 µs  | 105.98 Melem/s  |
| F8E4M3 | Multiply     | lowp   | 18.098 µs  | 226.32 Melem/s  |
| F8E4M3 | Subtract     | float8 | 42.585 µs  |  96.18 Melem/s  |
| F8E4M3 | Subtract     | lowp   | 20.993 µs  | 195.11 Melem/s  |
| F8E4M3 | Divide       | float8 | 39.879 µs  | 102.71 Melem/s  |
| F8E4M3 | Divide       | lowp   | 22.533 µs  | 181.78 Melem/s  |
| F8E4M3 | Remainder    | float8 | 75.803 µs  |  54.04 Melem/s  |
| F8E4M3 | Remainder    | lowp   | 42.242 µs  |  96.97 Melem/s  |
| F8E4M3 | Multiply-add | float8 | 56.918 µs  |  71.96 Melem/s  |
| F8E4M3 | Multiply-add | lowp   | 31.102 µs  | 131.70 Melem/s  |
| F4E2M1 | Add          | float4 | 85.824 µs  |  47.73 Melem/s  |
| F4E2M1 | Add          | lowp   | 2.1204 µs  |   1.932 Gelem/s |
| F4E2M1 | Multiply     | float4 | 83.775 µs  |  48.89 Melem/s  |
| F4E2M1 | Multiply     | lowp   | 2.2978 µs  |   1.783 Gelem/s |
| F4E2M1 | Subtract     | float4 | 85.701 µs  |  47.79 Melem/s  |
| F4E2M1 | Subtract     | lowp   | 2.1001 µs  |   1.950 Gelem/s |
| F4E2M1 | Divide       | float4 | 92.667 µs  |  44.20 Melem/s  |
| F4E2M1 | Divide       | lowp   | 2.1258 µs  |   1.927 Gelem/s |
| F4E2M1 | Remainder    | float4 | 126.78 µs  |  32.31 Melem/s  |
| F4E2M1 | Remainder    | lowp   | 2.1265 µs  |   1.926 Gelem/s |
| F4E2M1 | Multiply-add | float4 | 145.69 µs  |  28.11 Melem/s  |
| F4E2M1 | Multiply-add | lowp   | 3.2515 µs  |   1.260 Gelem/s |

### Performance improvements

Comparing median throughput, `lowp` achieves the following speedups:

| Format | Operation    | Speedup |
|--------|--------------|--------:|
| F8E4M3 | Add          |   2.06x |
| F8E4M3 | Multiply     |   2.14x |
| F8E4M3 | Subtract     |   2.03x |
| F8E4M3 | Divide       |   1.77x |
| F8E4M3 | Remainder    |   1.79x |
| F8E4M3 | Multiply-add |   1.83x |
| F4E2M1 | Add          |  40.47x |
| F4E2M1 | Multiply     |  36.46x |
| F4E2M1 | Subtract     |  40.81x |
| F4E2M1 | Divide       |  43.59x |
| F4E2M1 | Remainder    |  59.62x |
| F4E2M1 | Multiply-add |  44.81x |

### Takeaways

- `lowp` achieves between 1.77x and 2.14x the throughput of `float8` across all tested operations.
- F8E4M3 multiplication achieves 2.14x the throughput of the standard implementation, while addition achieves 2.06x.
- `lowp` achieves between 36x and 60x the throughput of `float4` across all tested operations.
- F4E2M1 remainder achieves approximately 59.62x the throughput of the standard implementation.

### How does `lowp` achieve this performance?

The optimizations differ depending on the floating-point format.

> **F8E4M3 (8-bit & 6-bit floating point)**
>
> While `float8` converts to/from larger floats as intermediate representations, `lowp` implements each operation as software bit manipulation. This has proven to improve performance up to 2x in many cases. This is likely to get faster with new updates.

> **F4E2M1 (4-bit floating point)**
> 
> Since `f4` has just 16 possible values, a lookup table for each operation has reasonable size. This is exactly how `lowp` achieves almost 60x the throughput of `float4`. This comes at a cost though; 4KiB of static data. In cases where this tradeoff is acceptable, `lowp` offers exceptional performance. If it isn't acceptable, just disable the `lut-f4` feature to use bit-manipulation-based arithmetic.

Regardless, the implementation NEVER converts to/from other floats implicitly.
