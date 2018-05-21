#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EECR1 {
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
pub struct EE5FASTR {
    bits: bool,
}
impl EE5FASTR {
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
pub struct EE5SNSR {
    bits: u8,
}
impl EE5SNSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE5POLR {
    bits: bool,
}
impl EE5POLR {
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
pub struct EE5SRCR {
    bits: u8,
}
impl EE5SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE4FASTR {
    bits: bool,
}
impl EE4FASTR {
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
pub struct EE4SNSR {
    bits: u8,
}
impl EE4SNSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE4POLR {
    bits: bool,
}
impl EE4POLR {
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
pub struct EE4SRCR {
    bits: u8,
}
impl EE4SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE3FASTR {
    bits: bool,
}
impl EE3FASTR {
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
pub struct EE3SNSR {
    bits: u8,
}
impl EE3SNSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE3POLR {
    bits: bool,
}
impl EE3POLR {
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
pub struct EE3SRCR {
    bits: u8,
}
impl EE3SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE2FASTR {
    bits: bool,
}
impl EE2FASTR {
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
pub struct EE2SNSR {
    bits: u8,
}
impl EE2SNSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE2POLR {
    bits: bool,
}
impl EE2POLR {
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
pub struct EE2SRCR {
    bits: u8,
}
impl EE2SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE1FASTR {
    bits: bool,
}
impl EE1FASTR {
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
pub struct EE1SNSR {
    bits: u8,
}
impl EE1SNSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE1POLR {
    bits: bool,
}
impl EE1POLR {
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
pub struct EE1SRCR {
    bits: u8,
}
impl EE1SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _EE5FASTW<'a> {
    w: &'a mut W,
}
impl<'a> _EE5FASTW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE5SNSW<'a> {
    w: &'a mut W,
}
impl<'a> _EE5SNSW<'a> {
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
pub struct _EE5POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EE5POLW<'a> {
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
pub struct _EE5SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EE5SRCW<'a> {
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
pub struct _EE4FASTW<'a> {
    w: &'a mut W,
}
impl<'a> _EE4FASTW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE4SNSW<'a> {
    w: &'a mut W,
}
impl<'a> _EE4SNSW<'a> {
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
pub struct _EE4POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EE4POLW<'a> {
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
pub struct _EE4SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EE4SRCW<'a> {
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
pub struct _EE3FASTW<'a> {
    w: &'a mut W,
}
impl<'a> _EE3FASTW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE3SNSW<'a> {
    w: &'a mut W,
}
impl<'a> _EE3SNSW<'a> {
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
pub struct _EE3POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EE3POLW<'a> {
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
pub struct _EE3SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EE3SRCW<'a> {
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
pub struct _EE2FASTW<'a> {
    w: &'a mut W,
}
impl<'a> _EE2FASTW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EE2SNSW<'a> {
    w: &'a mut W,
}
impl<'a> _EE2SNSW<'a> {
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
pub struct _EE2POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EE2POLW<'a> {
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
pub struct _EE2SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EE2SRCW<'a> {
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
pub struct _EE1FASTW<'a> {
    w: &'a mut W,
}
impl<'a> _EE1FASTW<'a> {
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
pub struct _EE1SNSW<'a> {
    w: &'a mut W,
}
impl<'a> _EE1SNSW<'a> {
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
pub struct _EE1POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EE1POLW<'a> {
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
pub struct _EE1SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EE1SRCW<'a> {
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
    #[doc = "Bit 29 - External Event 5 Fast mode"]
    #[inline]
    pub fn ee5fast(&self) -> EE5FASTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE5FASTR { bits }
    }
    #[doc = "Bits 27:28 - External Event 5 Sensitivity"]
    #[inline]
    pub fn ee5sns(&self) -> EE5SNSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE5SNSR { bits }
    }
    #[doc = "Bit 26 - External Event 5 Polarity"]
    #[inline]
    pub fn ee5pol(&self) -> EE5POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE5POLR { bits }
    }
    #[doc = "Bits 24:25 - External Event 5 Source"]
    #[inline]
    pub fn ee5src(&self) -> EE5SRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE5SRCR { bits }
    }
    #[doc = "Bit 23 - External Event 4 Fast mode"]
    #[inline]
    pub fn ee4fast(&self) -> EE4FASTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE4FASTR { bits }
    }
    #[doc = "Bits 21:22 - External Event 4 Sensitivity"]
    #[inline]
    pub fn ee4sns(&self) -> EE4SNSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE4SNSR { bits }
    }
    #[doc = "Bit 20 - External Event 4 Polarity"]
    #[inline]
    pub fn ee4pol(&self) -> EE4POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE4POLR { bits }
    }
    #[doc = "Bits 18:19 - External Event 4 Source"]
    #[inline]
    pub fn ee4src(&self) -> EE4SRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE4SRCR { bits }
    }
    #[doc = "Bit 17 - External Event 3 Fast mode"]
    #[inline]
    pub fn ee3fast(&self) -> EE3FASTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE3FASTR { bits }
    }
    #[doc = "Bits 15:16 - External Event 3 Sensitivity"]
    #[inline]
    pub fn ee3sns(&self) -> EE3SNSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE3SNSR { bits }
    }
    #[doc = "Bit 14 - External Event 3 Polarity"]
    #[inline]
    pub fn ee3pol(&self) -> EE3POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE3POLR { bits }
    }
    #[doc = "Bits 12:13 - External Event 3 Source"]
    #[inline]
    pub fn ee3src(&self) -> EE3SRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE3SRCR { bits }
    }
    #[doc = "Bit 11 - External Event 2 Fast mode"]
    #[inline]
    pub fn ee2fast(&self) -> EE2FASTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE2FASTR { bits }
    }
    #[doc = "Bits 9:10 - External Event 2 Sensitivity"]
    #[inline]
    pub fn ee2sns(&self) -> EE2SNSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE2SNSR { bits }
    }
    #[doc = "Bit 8 - External Event 2 Polarity"]
    #[inline]
    pub fn ee2pol(&self) -> EE2POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE2POLR { bits }
    }
    #[doc = "Bits 6:7 - External Event 2 Source"]
    #[inline]
    pub fn ee2src(&self) -> EE2SRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE2SRCR { bits }
    }
    #[doc = "Bit 5 - External Event 1 Fast mode"]
    #[inline]
    pub fn ee1fast(&self) -> EE1FASTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE1FASTR { bits }
    }
    #[doc = "Bits 3:4 - External Event 1 Sensitivity"]
    #[inline]
    pub fn ee1sns(&self) -> EE1SNSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE1SNSR { bits }
    }
    #[doc = "Bit 2 - External Event 1 Polarity"]
    #[inline]
    pub fn ee1pol(&self) -> EE1POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE1POLR { bits }
    }
    #[doc = "Bits 0:1 - External Event 1 Source"]
    #[inline]
    pub fn ee1src(&self) -> EE1SRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE1SRCR { bits }
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
    #[doc = "Bit 29 - External Event 5 Fast mode"]
    #[inline]
    pub fn ee5fast(&mut self) -> _EE5FASTW {
        _EE5FASTW { w: self }
    }
    #[doc = "Bits 27:28 - External Event 5 Sensitivity"]
    #[inline]
    pub fn ee5sns(&mut self) -> _EE5SNSW {
        _EE5SNSW { w: self }
    }
    #[doc = "Bit 26 - External Event 5 Polarity"]
    #[inline]
    pub fn ee5pol(&mut self) -> _EE5POLW {
        _EE5POLW { w: self }
    }
    #[doc = "Bits 24:25 - External Event 5 Source"]
    #[inline]
    pub fn ee5src(&mut self) -> _EE5SRCW {
        _EE5SRCW { w: self }
    }
    #[doc = "Bit 23 - External Event 4 Fast mode"]
    #[inline]
    pub fn ee4fast(&mut self) -> _EE4FASTW {
        _EE4FASTW { w: self }
    }
    #[doc = "Bits 21:22 - External Event 4 Sensitivity"]
    #[inline]
    pub fn ee4sns(&mut self) -> _EE4SNSW {
        _EE4SNSW { w: self }
    }
    #[doc = "Bit 20 - External Event 4 Polarity"]
    #[inline]
    pub fn ee4pol(&mut self) -> _EE4POLW {
        _EE4POLW { w: self }
    }
    #[doc = "Bits 18:19 - External Event 4 Source"]
    #[inline]
    pub fn ee4src(&mut self) -> _EE4SRCW {
        _EE4SRCW { w: self }
    }
    #[doc = "Bit 17 - External Event 3 Fast mode"]
    #[inline]
    pub fn ee3fast(&mut self) -> _EE3FASTW {
        _EE3FASTW { w: self }
    }
    #[doc = "Bits 15:16 - External Event 3 Sensitivity"]
    #[inline]
    pub fn ee3sns(&mut self) -> _EE3SNSW {
        _EE3SNSW { w: self }
    }
    #[doc = "Bit 14 - External Event 3 Polarity"]
    #[inline]
    pub fn ee3pol(&mut self) -> _EE3POLW {
        _EE3POLW { w: self }
    }
    #[doc = "Bits 12:13 - External Event 3 Source"]
    #[inline]
    pub fn ee3src(&mut self) -> _EE3SRCW {
        _EE3SRCW { w: self }
    }
    #[doc = "Bit 11 - External Event 2 Fast mode"]
    #[inline]
    pub fn ee2fast(&mut self) -> _EE2FASTW {
        _EE2FASTW { w: self }
    }
    #[doc = "Bits 9:10 - External Event 2 Sensitivity"]
    #[inline]
    pub fn ee2sns(&mut self) -> _EE2SNSW {
        _EE2SNSW { w: self }
    }
    #[doc = "Bit 8 - External Event 2 Polarity"]
    #[inline]
    pub fn ee2pol(&mut self) -> _EE2POLW {
        _EE2POLW { w: self }
    }
    #[doc = "Bits 6:7 - External Event 2 Source"]
    #[inline]
    pub fn ee2src(&mut self) -> _EE2SRCW {
        _EE2SRCW { w: self }
    }
    #[doc = "Bit 5 - External Event 1 Fast mode"]
    #[inline]
    pub fn ee1fast(&mut self) -> _EE1FASTW {
        _EE1FASTW { w: self }
    }
    #[doc = "Bits 3:4 - External Event 1 Sensitivity"]
    #[inline]
    pub fn ee1sns(&mut self) -> _EE1SNSW {
        _EE1SNSW { w: self }
    }
    #[doc = "Bit 2 - External Event 1 Polarity"]
    #[inline]
    pub fn ee1pol(&mut self) -> _EE1POLW {
        _EE1POLW { w: self }
    }
    #[doc = "Bits 0:1 - External Event 1 Source"]
    #[inline]
    pub fn ee1src(&mut self) -> _EE1SRCW {
        _EE1SRCW { w: self }
    }
}
