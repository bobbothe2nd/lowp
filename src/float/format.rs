pub struct FloatFormat8 {
    /// exponent bits
    pub exp: u8,

    /// fraction bits
    pub frac: u8,

    /// exponent bias.
    pub bias: i8,

    /// largest finite encoding, unsigned
    pub max: u8,

    /// canonical NaN encoding
    pub nan: u8,

    /// infinite encodding
    pub inf: Option<u8>,
}

pub const E4M3: FloatFormat8 = FloatFormat8 {
    exp: 4,
    frac: 3,
    bias: 7,
    max: 0x7e,
    nan: 0x7f,
    inf: Some(0x78),
};

pub const E4M3FN: FloatFormat8 = FloatFormat8 {
    exp: 4,
    frac: 3,
    bias: 7,
    max: 0x7e,
    nan: 0x7f,
    inf: None,
};

pub const E4M3FNUZ: FloatFormat8 = FloatFormat8 {
    exp: 4,
    frac: 3,
    bias: 8,
    max: 0x7f,
    nan: 0x80,
    inf: None,
};

pub const E5M2: FloatFormat8 = FloatFormat8 {
    exp: 5,
    frac: 2,
    bias: 15,
    max: 0x7b,
    nan: 0x7f,
    inf: Some(0x7c),
};

pub const E5M2FN: FloatFormat8 = FloatFormat8 {
    exp: 5,
    frac: 2,
    bias: 15,
    max: 0x7f,
    nan: 0x7f,
    inf: None,
};

pub const E5M2FNUZ: FloatFormat8 = FloatFormat8 {
    exp: 5,
    frac: 2,
    bias: 16,
    max: 0x7f,
    nan: 0x80,
    inf: None,
};
