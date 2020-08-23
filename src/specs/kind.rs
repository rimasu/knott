use std::num::NonZeroU16;
use std::convert::TryFrom;
use std::fmt;
use crate::lookup::Indexed;

#[derive(Copy, Clone, PartialEq)]
pub struct Kind(NonZeroU16);

static MIN_KIND: u32 = 1;
static MAX_KIND: u32 = 9999;

#[derive(Debug, PartialEq)]
pub struct InvalidKind(pub u32);

impl TryFrom<u32> for Kind {
    type Error = InvalidKind;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > MAX_KIND {
            Err(InvalidKind(value))
        } else {
            // as cast is safe because of MAX_KIND check above.
            NonZeroU16::new(value as u16)
                .ok_or(InvalidKind(value))
                .map(|inner| Kind(inner))
        }
    }
}

impl fmt::Debug for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:03}", self.0.get())
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.get())
    }
}

impl Indexed for Kind {
    fn as_usize(&self) -> usize {
        self.0.get() as usize
    }
}

impl Kind {
    pub fn as_u32(&self) -> u32 {
        self.0.get() as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn can_convert_numbers_to_kind() {
        assert_eq!(Ok(Kind(NonZeroU16::new(MIN_KIND as u16).unwrap())), MIN_KIND.try_into());
        assert_eq!(Ok(Kind(NonZeroU16::new(MAX_KIND as u16).unwrap())), MAX_KIND.try_into());
    }

    #[test]
    fn can_convert_kind_to_u32() {
        let t: Kind = 56.try_into().unwrap();
        assert_eq!(56, t.as_u32());
    }

    #[test]
    fn can_convert_kind_to_usize() {
        let t: Kind = 56.try_into().unwrap();
        assert_eq!(56, t.as_usize());
    }

    #[test]
    fn can_convert_zero_to_kind() {
        let r: Result<Kind, InvalidKind> = 0.try_into();
        assert_eq!(Err(InvalidKind(0)), r);
    }

    #[test]
    fn can_convert_over_max_to_kind() {
        let r: Result<Kind, InvalidKind> = (MAX_KIND + 1).try_into();
        assert_eq!(Err(InvalidKind(10000)), r);
    }

    #[test]
    fn check_max_kind_is_a_valid_u16() {
        assert!(std::u16::MAX as u32 > MAX_KIND);
    }

    #[test]
    fn debug_is_three_digit_zero_padded_number() {
        let kind: Kind = 45.try_into().unwrap();
        assert_eq!("045", format!("{:?}", kind));
    }

    #[test]
    fn debug_is_number() {
        let kind: Kind = 45.try_into().unwrap();
        assert_eq!("45", format!("{}", kind));
    }
}