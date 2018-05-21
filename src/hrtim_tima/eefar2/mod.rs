#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEFAR2 {
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
#[doc = "Possible values of the field `EE10FLTR`"]
pub type EE10FLTRR = super::eefar1::EE5FLTRR;
#[doc = r" Value of the field"]
pub struct EE10LTCHR {
    bits: bool,
}
impl EE10LTCHR {
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
#[doc = "Possible values of the field `EE9FLTR`"]
pub type EE9FLTRR = super::eefar1::EE5FLTRR;
#[doc = r" Value of the field"]
pub struct EE9LTCHR {
    bits: bool,
}
impl EE9LTCHR {
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
#[doc = "Possible values of the field `EE8FLTR`"]
pub type EE8FLTRR = super::eefar1::EE5FLTRR;
#[doc = r" Value of the field"]
pub struct EE8LTCHR {
    bits: bool,
}
impl EE8LTCHR {
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
#[doc = "Possible values of the field `EE7FLTR`"]
pub type EE7FLTRR = super::eefar1::EE5FLTRR;
#[doc = r" Value of the field"]
pub struct EE7LTCHR {
    bits: bool,
}
impl EE7LTCHR {
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
#[doc = "Possible values of the field `EE6FLTR`"]
pub type EE6FLTRR = super::eefar1::EE5FLTRR;
#[doc = r" Value of the field"]
pub struct EE6LTCHR {
    bits: bool,
}
impl EE6LTCHR {
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
#[doc = "Values that can be written to the field `EE10FLTR`"]
pub type EE10FLTRW = super::eefar1::EE5FLTRW;
#[doc = r" Proxy"]
pub struct _EE10FLTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EE10FLTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EE10FLTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filtering"]
    #[inline]
    pub fn no_filtering(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::NO_FILTERING)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn blank_compare_1(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn blank_compare_2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn blank_compare_3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn blank_compare_4(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR1)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR3)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR4)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR5)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR6)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR7)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR8)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn window_compare_2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_COMPARE_2)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn window_compare_3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_COMPARE_3)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_TIMWIN)
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
pub struct _EE10LTCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EE10LTCHW<'a> {
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
#[doc = "Values that can be written to the field `EE9FLTR`"]
pub type EE9FLTRW = super::eefar1::EE5FLTRW;
#[doc = r" Proxy"]
pub struct _EE9FLTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EE9FLTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EE9FLTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filtering"]
    #[inline]
    pub fn no_filtering(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::NO_FILTERING)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn blank_compare_1(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn blank_compare_2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn blank_compare_3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn blank_compare_4(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR1)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR3)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR4)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR5)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR6)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR7)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR8)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn window_compare_2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_COMPARE_2)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn window_compare_3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_COMPARE_3)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_TIMWIN)
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
pub struct _EE9LTCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EE9LTCHW<'a> {
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
#[doc = "Values that can be written to the field `EE8FLTR`"]
pub type EE8FLTRW = super::eefar1::EE5FLTRW;
#[doc = r" Proxy"]
pub struct _EE8FLTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EE8FLTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EE8FLTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filtering"]
    #[inline]
    pub fn no_filtering(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::NO_FILTERING)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn blank_compare_1(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn blank_compare_2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn blank_compare_3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn blank_compare_4(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR1)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR3)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR4)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR5)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR6)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR7)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR8)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn window_compare_2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_COMPARE_2)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn window_compare_3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_COMPARE_3)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_TIMWIN)
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
pub struct _EE8LTCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EE8LTCHW<'a> {
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
#[doc = "Values that can be written to the field `EE7FLTR`"]
pub type EE7FLTRW = super::eefar1::EE5FLTRW;
#[doc = r" Proxy"]
pub struct _EE7FLTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EE7FLTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EE7FLTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filtering"]
    #[inline]
    pub fn no_filtering(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::NO_FILTERING)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn blank_compare_1(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn blank_compare_2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn blank_compare_3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn blank_compare_4(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR1)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR3)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR4)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR5)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR6)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR7)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR8)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn window_compare_2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_COMPARE_2)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn window_compare_3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_COMPARE_3)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_TIMWIN)
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
pub struct _EE7LTCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EE7LTCHW<'a> {
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
#[doc = "Values that can be written to the field `EE6FLTR`"]
pub type EE6FLTRW = super::eefar1::EE5FLTRW;
#[doc = r" Proxy"]
pub struct _EE6FLTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EE6FLTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EE6FLTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filtering"]
    #[inline]
    pub fn no_filtering(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::NO_FILTERING)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn blank_compare_1(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn blank_compare_2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn blank_compare_3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn blank_compare_4(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_COMPARE_4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR1)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR3)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR4)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR5)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR6)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR7)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::BLANK_TIMFLTR8)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn window_compare_2(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_COMPARE_2)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn window_compare_3(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_COMPARE_3)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(super::eefar1::EE5FLTRW::WINDOW_TIMWIN)
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
pub struct _EE6LTCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EE6LTCHW<'a> {
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
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline]
    pub fn ee10fltr(&self) -> EE10FLTRR {
        EE10FLTRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline]
    pub fn ee10ltch(&self) -> EE10LTCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE10LTCHR { bits }
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline]
    pub fn ee9fltr(&self) -> EE9FLTRR {
        EE9FLTRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline]
    pub fn ee9ltch(&self) -> EE9LTCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE9LTCHR { bits }
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline]
    pub fn ee8fltr(&self) -> EE8FLTRR {
        EE8FLTRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline]
    pub fn ee8ltch(&self) -> EE8LTCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE8LTCHR { bits }
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline]
    pub fn ee7fltr(&self) -> EE7FLTRR {
        EE7FLTRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline]
    pub fn ee7ltch(&self) -> EE7LTCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE7LTCHR { bits }
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline]
    pub fn ee6fltr(&self) -> EE6FLTRR {
        EE6FLTRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline]
    pub fn ee6ltch(&self) -> EE6LTCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE6LTCHR { bits }
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
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline]
    pub fn ee10fltr(&mut self) -> _EE10FLTRW {
        _EE10FLTRW { w: self }
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline]
    pub fn ee10ltch(&mut self) -> _EE10LTCHW {
        _EE10LTCHW { w: self }
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline]
    pub fn ee9fltr(&mut self) -> _EE9FLTRW {
        _EE9FLTRW { w: self }
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline]
    pub fn ee9ltch(&mut self) -> _EE9LTCHW {
        _EE9LTCHW { w: self }
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline]
    pub fn ee8fltr(&mut self) -> _EE8FLTRW {
        _EE8FLTRW { w: self }
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline]
    pub fn ee8ltch(&mut self) -> _EE8LTCHW {
        _EE8LTCHW { w: self }
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline]
    pub fn ee7fltr(&mut self) -> _EE7FLTRW {
        _EE7FLTRW { w: self }
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline]
    pub fn ee7ltch(&mut self) -> _EE7LTCHW {
        _EE7LTCHW { w: self }
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline]
    pub fn ee6fltr(&mut self) -> _EE6FLTRW {
        _EE6FLTRW { w: self }
    }
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline]
    pub fn ee6ltch(&mut self) -> _EE6LTCHW {
        _EE6LTCHW { w: self }
    }
}
