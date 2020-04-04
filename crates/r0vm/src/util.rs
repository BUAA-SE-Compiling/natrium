pub trait IntoBytes {
    fn into_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_into_bytes_num {
    ($ty:ty) => {
        impl IntoBytes for $ty {
            fn into_bytes(&self) -> Vec<u8> {
                self.to_ne_bytes().to_vec()
            }
        }
    };
}

impl_into_bytes_num!(u8);
impl_into_bytes_num!(u16);
impl_into_bytes_num!(u32);
impl_into_bytes_num!(u64);
impl_into_bytes_num!(u128);
impl_into_bytes_num!(i8);
impl_into_bytes_num!(i16);
impl_into_bytes_num!(i32);
impl_into_bytes_num!(i64);
impl_into_bytes_num!(i128);
impl_into_bytes_num!(f32);
impl_into_bytes_num!(f64);

impl IntoBytes for [u8] {
    fn into_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl IntoBytes for str {
    fn into_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}
