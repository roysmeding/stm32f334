#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEFAR1 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `EE5FLTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EE5FLTRR {
    #[doc = "No filtering"]
    NO_FILTERING,
    #[doc = "undocumented"]
    BLANK_COMPARE_1,
    #[doc = "undocumented"]
    BLANK_COMPARE_2,
    #[doc = "undocumented"]
    BLANK_COMPARE_3,
    #[doc = "undocumented"]
    BLANK_COMPARE_4,
    #[doc = "undocumented"]
    BLANK_TIMFLTR1,
    #[doc = "undocumented"]
    BLANK_TIMFLTR2,
    #[doc = "undocumented"]
    BLANK_TIMFLTR3,
    #[doc = "undocumented"]
    BLANK_TIMFLTR4,
    #[doc = "undocumented"]
    BLANK_TIMFLTR5,
    #[doc = "undocumented"]
    BLANK_TIMFLTR6,
    #[doc = "undocumented"]
    BLANK_TIMFLTR7,
    #[doc = "undocumented"]
    BLANK_TIMFLTR8,
    #[doc = "undocumented"]
    WINDOW_COMPARE_2,
    #[doc = "undocumented"]
    WINDOW_COMPARE_3,
    #[doc = "undocumented"]
    WINDOW_TIMWIN,
}
impl EE5FLTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EE5FLTRR::NO_FILTERING => 0,
            EE5FLTRR::BLANK_COMPARE_1 => 1,
            EE5FLTRR::BLANK_COMPARE_2 => 2,
            EE5FLTRR::BLANK_COMPARE_3 => 3,
            EE5FLTRR::BLANK_COMPARE_4 => 4,
            EE5FLTRR::BLANK_TIMFLTR1 => 5,
            EE5FLTRR::BLANK_TIMFLTR2 => 6,
            EE5FLTRR::BLANK_TIMFLTR3 => 7,
            EE5FLTRR::BLANK_TIMFLTR4 => 8,
            EE5FLTRR::BLANK_TIMFLTR5 => 9,
            EE5FLTRR::BLANK_TIMFLTR6 => 10,
            EE5FLTRR::BLANK_TIMFLTR7 => 11,
            EE5FLTRR::BLANK_TIMFLTR8 => 12,
            EE5FLTRR::WINDOW_COMPARE_2 => 13,
            EE5FLTRR::WINDOW_COMPARE_3 => 14,
            EE5FLTRR::WINDOW_TIMWIN => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EE5FLTRR {
        match value {
            0 => EE5FLTRR::NO_FILTERING,
            1 => EE5FLTRR::BLANK_COMPARE_1,
            2 => EE5FLTRR::BLANK_COMPARE_2,
            3 => EE5FLTRR::BLANK_COMPARE_3,
            4 => EE5FLTRR::BLANK_COMPARE_4,
            5 => EE5FLTRR::BLANK_TIMFLTR1,
            6 => EE5FLTRR::BLANK_TIMFLTR2,
            7 => EE5FLTRR::BLANK_TIMFLTR3,
            8 => EE5FLTRR::BLANK_TIMFLTR4,
            9 => EE5FLTRR::BLANK_TIMFLTR5,
            10 => EE5FLTRR::BLANK_TIMFLTR6,
            11 => EE5FLTRR::BLANK_TIMFLTR7,
            12 => EE5FLTRR::BLANK_TIMFLTR8,
            13 => EE5FLTRR::WINDOW_COMPARE_2,
            14 => EE5FLTRR::WINDOW_COMPARE_3,
            15 => EE5FLTRR::WINDOW_TIMWIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_FILTERING`"]
    #[inline]
    pub fn is_no_filtering(&self) -> bool {
        *self == EE5FLTRR::NO_FILTERING
    }
    #[doc = "Checks if the value of the field is `BLANK_COMPARE_1`"]
    #[inline]
    pub fn is_blank_compare_1(&self) -> bool {
        *self == EE5FLTRR::BLANK_COMPARE_1
    }
    #[doc = "Checks if the value of the field is `BLANK_COMPARE_2`"]
    #[inline]
    pub fn is_blank_compare_2(&self) -> bool {
        *self == EE5FLTRR::BLANK_COMPARE_2
    }
    #[doc = "Checks if the value of the field is `BLANK_COMPARE_3`"]
    #[inline]
    pub fn is_blank_compare_3(&self) -> bool {
        *self == EE5FLTRR::BLANK_COMPARE_3
    }
    #[doc = "Checks if the value of the field is `BLANK_COMPARE_4`"]
    #[inline]
    pub fn is_blank_compare_4(&self) -> bool {
        *self == EE5FLTRR::BLANK_COMPARE_4
    }
    #[doc = "Checks if the value of the field is `BLANK_TIMFLTR1`"]
    #[inline]
    pub fn is_blank_timfltr1(&self) -> bool {
        *self == EE5FLTRR::BLANK_TIMFLTR1
    }
    #[doc = "Checks if the value of the field is `BLANK_TIMFLTR2`"]
    #[inline]
    pub fn is_blank_timfltr2(&self) -> bool {
        *self == EE5FLTRR::BLANK_TIMFLTR2
    }
    #[doc = "Checks if the value of the field is `BLANK_TIMFLTR3`"]
    #[inline]
    pub fn is_blank_timfltr3(&self) -> bool {
        *self == EE5FLTRR::BLANK_TIMFLTR3
    }
    #[doc = "Checks if the value of the field is `BLANK_TIMFLTR4`"]
    #[inline]
    pub fn is_blank_timfltr4(&self) -> bool {
        *self == EE5FLTRR::BLANK_TIMFLTR4
    }
    #[doc = "Checks if the value of the field is `BLANK_TIMFLTR5`"]
    #[inline]
    pub fn is_blank_timfltr5(&self) -> bool {
        *self == EE5FLTRR::BLANK_TIMFLTR5
    }
    #[doc = "Checks if the value of the field is `BLANK_TIMFLTR6`"]
    #[inline]
    pub fn is_blank_timfltr6(&self) -> bool {
        *self == EE5FLTRR::BLANK_TIMFLTR6
    }
    #[doc = "Checks if the value of the field is `BLANK_TIMFLTR7`"]
    #[inline]
    pub fn is_blank_timfltr7(&self) -> bool {
        *self == EE5FLTRR::BLANK_TIMFLTR7
    }
    #[doc = "Checks if the value of the field is `BLANK_TIMFLTR8`"]
    #[inline]
    pub fn is_blank_timfltr8(&self) -> bool {
        *self == EE5FLTRR::BLANK_TIMFLTR8
    }
    #[doc = "Checks if the value of the field is `WINDOW_COMPARE_2`"]
    #[inline]
    pub fn is_window_compare_2(&self) -> bool {
        *self == EE5FLTRR::WINDOW_COMPARE_2
    }
    #[doc = "Checks if the value of the field is `WINDOW_COMPARE_3`"]
    #[inline]
    pub fn is_window_compare_3(&self) -> bool {
        *self == EE5FLTRR::WINDOW_COMPARE_3
    }
    #[doc = "Checks if the value of the field is `WINDOW_TIMWIN`"]
    #[inline]
    pub fn is_window_timwin(&self) -> bool {
        *self == EE5FLTRR::WINDOW_TIMWIN
    }
}
#[doc = r" Value of the field"]
pub struct EE5LTCHR {
    bits: bool,
}
impl EE5LTCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `EE4FLTR`"]
pub type EE4FLTRR = EE5FLTRR;
#[doc = r" Value of the field"]
pub struct EE4LTCHR {
    bits: bool,
}
impl EE4LTCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `EE3FLTR`"]
pub type EE3FLTRR = EE5FLTRR;
#[doc = r" Value of the field"]
pub struct EE3LTCHR {
    bits: bool,
}
impl EE3LTCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `EE2FLTR`"]
pub type EE2FLTRR = EE5FLTRR;
#[doc = r" Value of the field"]
pub struct EE2LTCHR {
    bits: bool,
}
impl EE2LTCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `EE1FLTR`"]
pub type EE1FLTRR = EE5FLTRR;
#[doc = r" Value of the field"]
pub struct EE1LTCHR {
    bits: bool,
}
impl EE1LTCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `EE5FLTR`"]
pub enum EE5FLTRW {
    #[doc = "No filtering"]
    NO_FILTERING,
    #[doc = "`1`"]
    BLANK_COMPARE_1,
    #[doc = "`10`"]
    BLANK_COMPARE_2,
    #[doc = "`11`"]
    BLANK_COMPARE_3,
    #[doc = "`100`"]
    BLANK_COMPARE_4,
    #[doc = "`101`"]
    BLANK_TIMFLTR1,
    #[doc = "`110`"]
    BLANK_TIMFLTR2,
    #[doc = "`111`"]
    BLANK_TIMFLTR3,
    #[doc = "`1000`"]
    BLANK_TIMFLTR4,
    #[doc = "`1001`"]
    BLANK_TIMFLTR5,
    #[doc = "`1010`"]
    BLANK_TIMFLTR6,
    #[doc = "`1011`"]
    BLANK_TIMFLTR7,
    #[doc = "`1100`"]
    BLANK_TIMFLTR8,
    #[doc = "`1101`"]
    WINDOW_COMPARE_2,
    #[doc = "`1110`"]
    WINDOW_COMPARE_3,
    #[doc = "`1111`"]
    WINDOW_TIMWIN,
}
impl EE5FLTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EE5FLTRW::NO_FILTERING => 0,
            EE5FLTRW::BLANK_COMPARE_1 => 1,
            EE5FLTRW::BLANK_COMPARE_2 => 2,
            EE5FLTRW::BLANK_COMPARE_3 => 3,
            EE5FLTRW::BLANK_COMPARE_4 => 4,
            EE5FLTRW::BLANK_TIMFLTR1 => 5,
            EE5FLTRW::BLANK_TIMFLTR2 => 6,
            EE5FLTRW::BLANK_TIMFLTR3 => 7,
            EE5FLTRW::BLANK_TIMFLTR4 => 8,
            EE5FLTRW::BLANK_TIMFLTR5 => 9,
            EE5FLTRW::BLANK_TIMFLTR6 => 10,
            EE5FLTRW::BLANK_TIMFLTR7 => 11,
            EE5FLTRW::BLANK_TIMFLTR8 => 12,
            EE5FLTRW::WINDOW_COMPARE_2 => 13,
            EE5FLTRW::WINDOW_COMPARE_3 => 14,
            EE5FLTRW::WINDOW_TIMWIN => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EE5FLTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EE5FLTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EE5FLTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filtering"]
    #[inline]
    pub fn no_filtering(self) -> &'a mut W {
        self.variant(EE5FLTRW::NO_FILTERING)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn blank_compare_1(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn blank_compare_2(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn blank_compare_3(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn blank_compare_4(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR1)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR3)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR4)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR5)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR6)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR7)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR8)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn window_compare_2(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_COMPARE_2)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn window_compare_3(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_COMPARE_3)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_TIMWIN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE5LTCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EE5LTCHW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EE4FLTR`"]
pub type EE4FLTRW = EE5FLTRW;
#[doc = r" Proxy"]
pub struct _EE4FLTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EE4FLTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EE4FLTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filtering"]
    #[inline]
    pub fn no_filtering(self) -> &'a mut W {
        self.variant(EE5FLTRW::NO_FILTERING)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn blank_compare_1(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn blank_compare_2(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn blank_compare_3(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn blank_compare_4(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR1)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR3)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR4)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR5)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR6)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR7)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR8)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn window_compare_2(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_COMPARE_2)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn window_compare_3(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_COMPARE_3)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_TIMWIN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE4LTCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EE4LTCHW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EE3FLTR`"]
pub type EE3FLTRW = EE5FLTRW;
#[doc = r" Proxy"]
pub struct _EE3FLTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EE3FLTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EE3FLTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filtering"]
    #[inline]
    pub fn no_filtering(self) -> &'a mut W {
        self.variant(EE5FLTRW::NO_FILTERING)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn blank_compare_1(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn blank_compare_2(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn blank_compare_3(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn blank_compare_4(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR1)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR3)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR4)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR5)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR6)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR7)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR8)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn window_compare_2(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_COMPARE_2)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn window_compare_3(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_COMPARE_3)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_TIMWIN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE3LTCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EE3LTCHW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EE2FLTR`"]
pub type EE2FLTRW = EE5FLTRW;
#[doc = r" Proxy"]
pub struct _EE2FLTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EE2FLTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EE2FLTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filtering"]
    #[inline]
    pub fn no_filtering(self) -> &'a mut W {
        self.variant(EE5FLTRW::NO_FILTERING)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn blank_compare_1(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn blank_compare_2(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn blank_compare_3(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn blank_compare_4(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR1)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR3)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR4)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR5)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR6)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR7)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR8)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn window_compare_2(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_COMPARE_2)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn window_compare_3(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_COMPARE_3)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_TIMWIN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE2LTCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EE2LTCHW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EE1FLTR`"]
pub type EE1FLTRW = EE5FLTRW;
#[doc = r" Proxy"]
pub struct _EE1FLTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EE1FLTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EE1FLTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filtering"]
    #[inline]
    pub fn no_filtering(self) -> &'a mut W {
        self.variant(EE5FLTRW::NO_FILTERING)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn blank_compare_1(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn blank_compare_2(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn blank_compare_3(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn blank_compare_4(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_COMPARE_4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR1)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR3)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR4)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR5)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR6)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR7)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE5FLTRW::BLANK_TIMFLTR8)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn window_compare_2(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_COMPARE_2)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn window_compare_3(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_COMPARE_3)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE5FLTRW::WINDOW_TIMWIN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE1LTCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EE1LTCHW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline]
    pub fn ee5fltr(&self) -> EE5FLTRR {
        EE5FLTRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline]
    pub fn ee5ltch(&self) -> EE5LTCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE5LTCHR { bits }
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline]
    pub fn ee4fltr(&self) -> EE4FLTRR {
        EE4FLTRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline]
    pub fn ee4ltch(&self) -> EE4LTCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE4LTCHR { bits }
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline]
    pub fn ee3fltr(&self) -> EE3FLTRR {
        EE3FLTRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline]
    pub fn ee3ltch(&self) -> EE3LTCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE3LTCHR { bits }
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline]
    pub fn ee2fltr(&self) -> EE2FLTRR {
        EE2FLTRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline]
    pub fn ee2ltch(&self) -> EE2LTCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE2LTCHR { bits }
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline]
    pub fn ee1fltr(&self) -> EE1FLTRR {
        EE1FLTRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline]
    pub fn ee1ltch(&self) -> EE1LTCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE1LTCHR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline]
    pub fn ee5fltr(&mut self) -> _EE5FLTRW {
        _EE5FLTRW { w: self }
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline]
    pub fn ee5ltch(&mut self) -> _EE5LTCHW {
        _EE5LTCHW { w: self }
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline]
    pub fn ee4fltr(&mut self) -> _EE4FLTRW {
        _EE4FLTRW { w: self }
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline]
    pub fn ee4ltch(&mut self) -> _EE4LTCHW {
        _EE4LTCHW { w: self }
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline]
    pub fn ee3fltr(&mut self) -> _EE3FLTRW {
        _EE3FLTRW { w: self }
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline]
    pub fn ee3ltch(&mut self) -> _EE3LTCHW {
        _EE3LTCHW { w: self }
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline]
    pub fn ee2fltr(&mut self) -> _EE2FLTRW {
        _EE2FLTRW { w: self }
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline]
    pub fn ee2ltch(&mut self) -> _EE2LTCHW {
        _EE2LTCHW { w: self }
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline]
    pub fn ee1fltr(&mut self) -> _EE1FLTRW {
        _EE1FLTRW { w: self }
    }
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline]
    pub fn ee1ltch(&mut self) -> _EE1LTCHW {
        _EE1LTCHW { w: self }
    }
}
