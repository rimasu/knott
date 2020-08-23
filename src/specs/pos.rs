use std::num::NonZeroU16;
use std::convert::TryFrom;
use std::fmt;
use crate::lookup::Indexed;

#[derive(Copy, Clone, PartialEq)]
pub struct Pos(NonZeroU16);

static MIN_POS: u32 = 1;
static MAX_POS: u32 = 9999;

#[derive(Debug, PartialEq)]
pub struct InvalidPos(pub u32);

impl TryFrom<u32> for Pos {
    type Error = InvalidPos;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > MAX_POS {
            Err(InvalidPos(value))
        } else {
            // as cast is safe because of MAX_POS check above.
            NonZeroU16::new(value as u16)
                .ok_or(InvalidPos(value))
                .map(|inner| Pos(inner))
        }
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:03}", self.0.get())
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.get())
    }
}

impl Indexed for Pos {
    fn as_usize(&self) -> usize {
        self.0.get() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn can_convert_numbers_to_pos() {
        assert_eq!(Ok(Pos(NonZeroU16::new(MIN_POS as u16).unwrap())), MIN_POS.try_into());
        assert_eq!(Ok(Pos(NonZeroU16::new(MAX_POS as u16).unwrap())), MAX_POS.try_into());
    }

    #[test]
    fn can_convert_zero_to_pos() {
        let r: Result<Pos, InvalidPos> = 0.try_into();
        assert_eq!(Err(InvalidPos(0)), r);
    }

    #[test]
    fn can_convert_over_max_to_pos() {
        let r: Result<Pos, InvalidPos> = (MAX_POS + 1).try_into();
        assert_eq!(Err(InvalidPos(10000)), r);
    }

    #[test]
    fn check_max_pos_is_a_valid_u16() {
        assert!(std::u16::MAX as u32 > MAX_POS);
    }

    #[test]
    fn debug_is_three_digit_zero_padded_number() {
        let pos: Pos = 45.try_into().unwrap();
        assert_eq!("045", format!("{:?}", pos));
    }

    #[test]
    fn debug_is_number() {
        let pos: Pos = 45.try_into().unwrap();
        assert_eq!("45", format!("{}", pos));
    }
}