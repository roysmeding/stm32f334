#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EECR2 {
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
#[doc = r" Value of the field"]
pub struct EE10SNSR {
    bits: u8,
}
impl EE10SNSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE10POLR {
    bits: bool,
}
impl EE10POLR {
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
pub struct EE10SRCR {
    bits: u8,
}
impl EE10SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE9SNSR {
    bits: u8,
}
impl EE9SNSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE9POLR {
    bits: bool,
}
impl EE9POLR {
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
pub struct EE9SRCR {
    bits: u8,
}
impl EE9SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE8SNSR {
    bits: u8,
}
impl EE8SNSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE8POLR {
    bits: bool,
}
impl EE8POLR {
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
pub struct EE8SRCR {
    bits: u8,
}
impl EE8SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE7SNSR {
    bits: u8,
}
impl EE7SNSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE7POLR {
    bits: bool,
}
impl EE7POLR {
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
pub struct EE7SRCR {
    bits: u8,
}
impl EE7SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE6SNSR {
    bits: u8,
}
impl EE6SNSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE6POLR {
    bits: bool,
}
impl EE6POLR {
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
pub struct EE6SRCR {
    bits: u8,
}
impl EE6SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _EE10SNSW<'a> {
    w: &'a mut W,
}
impl<'a> _EE10SNSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE10POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EE10POLW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE10SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EE10SRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE9SNSW<'a> {
    w: &'a mut W,
}
impl<'a> _EE9SNSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE9POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EE9POLW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE9SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EE9SRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE8SNSW<'a> {
    w: &'a mut W,
}
impl<'a> _EE8SNSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE8POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EE8POLW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE8SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EE8SRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE7SNSW<'a> {
    w: &'a mut W,
}
impl<'a> _EE7SNSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE7POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EE7POLW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE7SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EE7SRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE6SNSW<'a> {
    w: &'a mut W,
}
impl<'a> _EE6SNSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE6POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EE6POLW<'a> {
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
pub struct _EE6SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EE6SRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 27:28 - External Event 10 Sensitivity"]
    #[inline]
    pub fn ee10sns(&self) -> EE10SNSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE10SNSR { bits }
    }
    #[doc = "Bit 26 - External Event 10 Polarity"]
    #[inline]
    pub fn ee10pol(&self) -> EE10POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE10POLR { bits }
    }
    #[doc = "Bits 24:25 - External Event 10 Source"]
    #[inline]
    pub fn ee10src(&self) -> EE10SRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE10SRCR { bits }
    }
    #[doc = "Bits 21:22 - External Event 9 Sensitivity"]
    #[inline]
    pub fn ee9sns(&self) -> EE9SNSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE9SNSR { bits }
    }
    #[doc = "Bit 20 - External Event 9 Polarity"]
    #[inline]
    pub fn ee9pol(&self) -> EE9POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE9POLR { bits }
    }
    #[doc = "Bits 18:19 - External Event 9 Source"]
    #[inline]
    pub fn ee9src(&self) -> EE9SRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE9SRCR { bits }
    }
    #[doc = "Bits 15:16 - External Event 8 Sensitivity"]
    #[inline]
    pub fn ee8sns(&self) -> EE8SNSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE8SNSR { bits }
    }
    #[doc = "Bit 14 - External Event 8 Polarity"]
    #[inline]
    pub fn ee8pol(&self) -> EE8POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE8POLR { bits }
    }
    #[doc = "Bits 12:13 - External Event 8 Source"]
    #[inline]
    pub fn ee8src(&self) -> EE8SRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE8SRCR { bits }
    }
    #[doc = "Bits 9:10 - External Event 7 Sensitivity"]
    #[inline]
    pub fn ee7sns(&self) -> EE7SNSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE7SNSR { bits }
    }
    #[doc = "Bit 8 - External Event 7 Polarity"]
    #[inline]
    pub fn ee7pol(&self) -> EE7POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE7POLR { bits }
    }
    #[doc = "Bits 6:7 - External Event 7 Source"]
    #[inline]
    pub fn ee7src(&self) -> EE7SRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE7SRCR { bits }
    }
    #[doc = "Bits 3:4 - External Event 6 Sensitivity"]
    #[inline]
    pub fn ee6sns(&self) -> EE6SNSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE6SNSR { bits }
    }
    #[doc = "Bit 2 - External Event 6 Polarity"]
    #[inline]
    pub fn ee6pol(&self) -> EE6POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE6POLR { bits }
    }
    #[doc = "Bits 0:1 - External Event 6 Source"]
    #[inline]
    pub fn ee6src(&self) -> EE6SRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE6SRCR { bits }
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
    #[doc = "Bits 27:28 - External Event 10 Sensitivity"]
    #[inline]
    pub fn ee10sns(&mut self) -> _EE10SNSW {
        _EE10SNSW { w: self }
    }
    #[doc = "Bit 26 - External Event 10 Polarity"]
    #[inline]
    pub fn ee10pol(&mut self) -> _EE10POLW {
        _EE10POLW { w: self }
    }
    #[doc = "Bits 24:25 - External Event 10 Source"]
    #[inline]
    pub fn ee10src(&mut self) -> _EE10SRCW {
        _EE10SRCW { w: self }
    }
    #[doc = "Bits 21:22 - External Event 9 Sensitivity"]
    #[inline]
    pub fn ee9sns(&mut self) -> _EE9SNSW {
        _EE9SNSW { w: self }
    }
    #[doc = "Bit 20 - External Event 9 Polarity"]
    #[inline]
    pub fn ee9pol(&mut self) -> _EE9POLW {
        _EE9POLW { w: self }
    }
    #[doc = "Bits 18:19 - External Event 9 Source"]
    #[inline]
    pub fn ee9src(&mut self) -> _EE9SRCW {
        _EE9SRCW { w: self }
    }
    #[doc = "Bits 15:16 - External Event 8 Sensitivity"]
    #[inline]
    pub fn ee8sns(&mut self) -> _EE8SNSW {
        _EE8SNSW { w: self }
    }
    #[doc = "Bit 14 - External Event 8 Polarity"]
    #[inline]
    pub fn ee8pol(&mut self) -> _EE8POLW {
        _EE8POLW { w: self }
    }
    #[doc = "Bits 12:13 - External Event 8 Source"]
    #[inline]
    pub fn ee8src(&mut self) -> _EE8SRCW {
        _EE8SRCW { w: self }
    }
    #[doc = "Bits 9:10 - External Event 7 Sensitivity"]
    #[inline]
    pub fn ee7sns(&mut self) -> _EE7SNSW {
        _EE7SNSW { w: self }
    }
    #[doc = "Bit 8 - External Event 7 Polarity"]
    #[inline]
    pub fn ee7pol(&mut self) -> _EE7POLW {
        _EE7POLW { w: self }
    }
    #[doc = "Bits 6:7 - External Event 7 Source"]
    #[inline]
    pub fn ee7src(&mut self) -> _EE7SRCW {
        _EE7SRCW { w: self }
    }
    #[doc = "Bits 3:4 - External Event 6 Sensitivity"]
    #[inline]
    pub fn ee6sns(&mut self) -> _EE6SNSW {
        _EE6SNSW { w: self }
    }
    #[doc = "Bit 2 - External Event 6 Polarity"]
    #[inline]
    pub fn ee6pol(&mut self) -> _EE6POLW {
        _EE6POLW { w: self }
    }
    #[doc = "Bits 0:1 - External Event 6 Source"]
    #[inline]
    pub fn ee6src(&mut self) -> _EE6SRCW {
        _EE6SRCW { w: self }
    }
}
