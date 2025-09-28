pub trait BitMap<T> {
    /// Gets the bit at position `index` from `&self`.
    fn get_bit(&self, index: u8) -> T;
    /// Sets the bit at position `index` in `&self`.
    fn set_bit(&mut self, index: u8, value: T);
    /// Gets the bits at positions `indices.start..indices.end` from `&self`.
    fn get_bits(&self, indices: ::core::ops::Range<u8>) -> T;
    /// Sets the bits at positions `indices.start..indices.end` in `&self`.
    fn set_bits(&mut self, indices: ::core::ops::Range<u8>, value: T);
}

macro_rules! impl_bitmap {
    ($ty:ident) => {
        impl BitMap<$ty> for $ty {
            fn get_bit(&self, index: u8) -> $ty {
                *self >> index & 0b1
            }

            fn set_bit(&mut self, index: u8, value: $ty) {
                *self = (*self & !(1 << index)) | ((value & 1) << index);
            }

            fn set_bits(&mut self, indices: ::core::ops::Range<u8>, value: $ty) {
                let width = indices.end - indices.start;
                let bit_count = ::core::mem::size_of::<$ty>() * 8;

                let mask = if width as usize >= bit_count { $ty::MAX } else { (1 << width) - 1 };

                *self = (*self & !(mask << indices.start)) | ((value & mask) << indices.start);
            }

            fn get_bits(&self, indices: ::core::ops::Range<u8>) -> $ty {
                let width = indices.end - indices.start;
                let bit_count = ::core::mem::size_of::<$ty>() * 8;

                let mask = if width as usize >= bit_count { $ty::MAX } else { (1 << width) - 1 };

                (*self >> indices.start) & mask
            }
        }
    };
}

impl_bitmap!(u8);
impl_bitmap!(u16);
impl_bitmap!(u32);
impl_bitmap!(u64);
impl_bitmap!(u128);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_tests {
        ($ty:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $ty _single_bit>]() {
                    let mut x: $ty = 0;
                    let bit_width = ::core::mem::size_of::<$ty>() * 8;

                    for bit in 0..bit_width {
                        x.set_bit(bit as u8, 1);
                        assert_eq!(x.get_bit(bit as u8), 1);
                        assert_eq!(x, 1 << bit);

                        x.set_bit(bit as u8, 0);
                        assert_eq!(x.get_bit(bit as u8), 0);
                        assert_eq!(x, 0);
                    }
                }

                #[test]
                fn [<test_ $ty _bit_range>]() {
                    let mut x: $ty = 0;
                    let bit_width = ::core::mem::size_of::<$ty>() * 8;

                    for start in (0..bit_width).step_by(8) {
                        for width in 1..=8.min(bit_width - start) {
                            let end = start + width;
                            if end > bit_width {
                                break;
                            }
                            let max_val = if width >= bit_width {
                                $ty::MAX
                            } else {
                                (1 << width) - 1
                            };

                            x.set_bits(start as u8..end as u8, max_val);
                            assert_eq!(x.get_bits(start as u8..end as u8), max_val, "Failed range test: {start}..{end} with value {max_val}");

                            x = 0;
                        }
                    }
                }
            }
        };
    }

    generate_tests!(u8);
    generate_tests!(u16);
    generate_tests!(u32);
    generate_tests!(u64);
    generate_tests!(u128);
}
