pub trait ValidIndex {}

pub struct Vec6x4Idx<const IDX: u8>;

impl ValidIndex for Vec6x4Idx<0> {}
impl ValidIndex for Vec6x4Idx<1> {}
impl ValidIndex for Vec6x4Idx<2> {}
impl ValidIndex for Vec6x4Idx<3> {}
