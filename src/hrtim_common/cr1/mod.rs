#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
#[doc = "Possible values of the field `AD4USRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD4USRCR {
    #[doc = "Master Timer"]
    MASTER,
    #[doc = "Timer A"]
    TIMER_A,
    #[doc = "Timer B"]
    TIMER_B,
    #[doc = "Timer C"]
    TIMER_C,
    #[doc = "Timer D"]
    TIMER_D,
    #[doc = "Timer E"]
    TIMER_E,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AD4USRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AD4USRCR::MASTER => 0,
            AD4USRCR::TIMER_A => 1,
            AD4USRCR::TIMER_B => 2,
            AD4USRCR::TIMER_C => 3,
            AD4USRCR::TIMER_D => 4,
            AD4USRCR::TIMER_E => 5,
            AD4USRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AD4USRCR {
        match value {
            0 => AD4USRCR::MASTER,
            1 => AD4USRCR::TIMER_A,
            2 => AD4USRCR::TIMER_B,
            3 => AD4USRCR::TIMER_C,
            4 => AD4USRCR::TIMER_D,
            5 => AD4USRCR::TIMER_E,
            i => AD4USRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == AD4USRCR::MASTER
    }
    #[doc = "Checks if the value of the field is `TIMER_A`"]
    #[inline]
    pub fn is_timer_a(&self) -> bool {
        *self == AD4USRCR::TIMER_A
    }
    #[doc = "Checks if the value of the field is `TIMER_B`"]
    #[inline]
    pub fn is_timer_b(&self) -> bool {
        *self == AD4USRCR::TIMER_B
    }
    #[doc = "Checks if the value of the field is `TIMER_C`"]
    #[inline]
    pub fn is_timer_c(&self) -> bool {
        *self == AD4USRCR::TIMER_C
    }
    #[doc = "Checks if the value of the field is `TIMER_D`"]
    #[inline]
    pub fn is_timer_d(&self) -> bool {
        *self == AD4USRCR::TIMER_D
    }
    #[doc = "Checks if the value of the field is `TIMER_E`"]
    #[inline]
    pub fn is_timer_e(&self) -> bool {
        *self == AD4USRCR::TIMER_E
    }
}
#[doc = "Possible values of the field `AD3USRC`"]
pub type AD3USRCR = AD4USRCR;
#[doc = "Possible values of the field `AD2USRC`"]
pub type AD2USRCR = AD4USRCR;
#[doc = "Possible values of the field `AD1USRC`"]
pub type AD1USRCR = AD4USRCR;
#[doc = r" Value of the field"]
pub struct TEUDISR {
    bits: bool,
}
impl TEUDISR {
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
#[doc = r" Value of the field"]
pub struct TDUDISR {
    bits: bool,
}
impl TDUDISR {
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
#[doc = r" Value of the field"]
pub struct TCUDISR {
    bits: bool,
}
impl TCUDISR {
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
#[doc = r" Value of the field"]
pub struct TBUDISR {
    bits: bool,
}
impl TBUDISR {
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
#[doc = r" Value of the field"]
pub struct TAUDISR {
    bits: bool,
}
impl TAUDISR {
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
#[doc = r" Value of the field"]
pub struct MUDISR {
    bits: bool,
}
impl MUDISR {
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
#[doc = "Values that can be written to the field `AD4USRC`"]
pub enum AD4USRCW {
    #[doc = "Master Timer"]
    MASTER,
    #[doc = "Timer A"]
    TIMER_A,
    #[doc = "Timer B"]
    TIMER_B,
    #[doc = "Timer C"]
    TIMER_C,
    #[doc = "Timer D"]
    TIMER_D,
    #[doc = "Timer E"]
    TIMER_E,
}
impl AD4USRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AD4USRCW::MASTER => 0,
            AD4USRCW::TIMER_A => 1,
            AD4USRCW::TIMER_B => 2,
            AD4USRCW::TIMER_C => 3,
            AD4USRCW::TIMER_D => 4,
            AD4USRCW::TIMER_E => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AD4USRCW<'a> {
    w: &'a mut W,
}
impl<'a> _AD4USRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AD4USRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Master Timer"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(AD4USRCW::MASTER)
    }
    #[doc = "Timer A"]
    #[inline]
    pub fn timer_a(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_A)
    }
    #[doc = "Timer B"]
    #[inline]
    pub fn timer_b(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_B)
    }
    #[doc = "Timer C"]
    #[inline]
    pub fn timer_c(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_C)
    }
    #[doc = "Timer D"]
    #[inline]
    pub fn timer_d(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_D)
    }
    #[doc = "Timer E"]
    #[inline]
    pub fn timer_e(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_E)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AD3USRC`"]
pub type AD3USRCW = AD4USRCW;
#[doc = r" Proxy"]
pub struct _AD3USRCW<'a> {
    w: &'a mut W,
}
impl<'a> _AD3USRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AD3USRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Master Timer"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(AD4USRCW::MASTER)
    }
    #[doc = "Timer A"]
    #[inline]
    pub fn timer_a(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_A)
    }
    #[doc = "Timer B"]
    #[inline]
    pub fn timer_b(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_B)
    }
    #[doc = "Timer C"]
    #[inline]
    pub fn timer_c(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_C)
    }
    #[doc = "Timer D"]
    #[inline]
    pub fn timer_d(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_D)
    }
    #[doc = "Timer E"]
    #[inline]
    pub fn timer_e(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_E)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AD2USRC`"]
pub type AD2USRCW = AD4USRCW;
#[doc = r" Proxy"]
pub struct _AD2USRCW<'a> {
    w: &'a mut W,
}
impl<'a> _AD2USRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AD2USRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Master Timer"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(AD4USRCW::MASTER)
    }
    #[doc = "Timer A"]
    #[inline]
    pub fn timer_a(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_A)
    }
    #[doc = "Timer B"]
    #[inline]
    pub fn timer_b(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_B)
    }
    #[doc = "Timer C"]
    #[inline]
    pub fn timer_c(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_C)
    }
    #[doc = "Timer D"]
    #[inline]
    pub fn timer_d(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_D)
    }
    #[doc = "Timer E"]
    #[inline]
    pub fn timer_e(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_E)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AD1USRC`"]
pub type AD1USRCW = AD4USRCW;
#[doc = r" Proxy"]
pub struct _AD1USRCW<'a> {
    w: &'a mut W,
}
impl<'a> _AD1USRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AD1USRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Master Timer"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(AD4USRCW::MASTER)
    }
    #[doc = "Timer A"]
    #[inline]
    pub fn timer_a(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_A)
    }
    #[doc = "Timer B"]
    #[inline]
    pub fn timer_b(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_B)
    }
    #[doc = "Timer C"]
    #[inline]
    pub fn timer_c(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_C)
    }
    #[doc = "Timer D"]
    #[inline]
    pub fn timer_d(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_D)
    }
    #[doc = "Timer E"]
    #[inline]
    pub fn timer_e(self) -> &'a mut W {
        self.variant(AD4USRCW::TIMER_E)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TEUDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TEUDISW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TDUDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TDUDISW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCUDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TCUDISW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TBUDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TBUDISW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAUDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TAUDISW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MUDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MUDISW<'a> {
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
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline]
    pub fn ad4usrc(&self) -> AD4USRCR {
        AD4USRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline]
    pub fn ad3usrc(&self) -> AD3USRCR {
        AD3USRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline]
    pub fn ad2usrc(&self) -> AD2USRCR {
        AD2USRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline]
    pub fn ad1usrc(&self) -> AD1USRCR {
        AD1USRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline]
    pub fn teudis(&self) -> TEUDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEUDISR { bits }
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline]
    pub fn tdudis(&self) -> TDUDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TDUDISR { bits }
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline]
    pub fn tcudis(&self) -> TCUDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCUDISR { bits }
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline]
    pub fn tbudis(&self) -> TBUDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBUDISR { bits }
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline]
    pub fn taudis(&self) -> TAUDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAUDISR { bits }
    }
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline]
    pub fn mudis(&self) -> MUDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MUDISR { bits }
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
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline]
    pub fn ad4usrc(&mut self) -> _AD4USRCW {
        _AD4USRCW { w: self }
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline]
    pub fn ad3usrc(&mut self) -> _AD3USRCW {
        _AD3USRCW { w: self }
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline]
    pub fn ad2usrc(&mut self) -> _AD2USRCW {
        _AD2USRCW { w: self }
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline]
    pub fn ad1usrc(&mut self) -> _AD1USRCW {
        _AD1USRCW { w: self }
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline]
    pub fn teudis(&mut self) -> _TEUDISW {
        _TEUDISW { w: self }
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline]
    pub fn tdudis(&mut self) -> _TDUDISW {
        _TDUDISW { w: self }
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline]
    pub fn tcudis(&mut self) -> _TCUDISW {
        _TCUDISW { w: self }
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline]
    pub fn tbudis(&mut self) -> _TBUDISW {
        _TBUDISW { w: self }
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline]
    pub fn taudis(&mut self) -> _TAUDISW {
        _TAUDISW { w: self }
    }
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline]
    pub fn mudis(&mut self) -> _MUDISW {
        _MUDISW { w: self }
    }
}
