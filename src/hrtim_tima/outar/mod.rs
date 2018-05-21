#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTAR {
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
pub struct DIDL2R {
    bits: bool,
}
impl DIDL2R {
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
pub struct CHP2R {
    bits: bool,
}
impl CHP2R {
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
#[doc = "Possible values of the field `FAULT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT2R {
    #[doc = "No action: the output is not affected by the fault input and stays in run mode."]
    NO_ACTION,
    #[doc = "Active"]
    ACTIVE,
    #[doc = "Inactive"]
    INACTIVE,
    #[doc = "High-Z"]
    HIGH_Z,
}
impl FAULT2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FAULT2R::NO_ACTION => 0,
            FAULT2R::ACTIVE => 1,
            FAULT2R::INACTIVE => 2,
            FAULT2R::HIGH_Z => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FAULT2R {
        match value {
            0 => FAULT2R::NO_ACTION,
            1 => FAULT2R::ACTIVE,
            2 => FAULT2R::INACTIVE,
            3 => FAULT2R::HIGH_Z,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACTION`"]
    #[inline]
    pub fn is_no_action(&self) -> bool {
        *self == FAULT2R::NO_ACTION
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == FAULT2R::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == FAULT2R::INACTIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_Z`"]
    #[inline]
    pub fn is_high_z(&self) -> bool {
        *self == FAULT2R::HIGH_Z
    }
}
#[doc = r" Value of the field"]
pub struct IDLES2R {
    bits: bool,
}
impl IDLES2R {
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
pub struct IDLEM2R {
    bits: bool,
}
impl IDLEM2R {
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
pub struct POL2R {
    bits: bool,
}
impl POL2R {
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
pub struct DLYPRTR {
    bits: u8,
}
impl DLYPRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYPRTENR {
    bits: bool,
}
impl DLYPRTENR {
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
pub struct DTENR {
    bits: bool,
}
impl DTENR {
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
pub struct DIDL1R {
    bits: bool,
}
impl DIDL1R {
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
pub struct CHP1R {
    bits: bool,
}
impl CHP1R {
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
#[doc = "Possible values of the field `FAULT1`"]
pub type FAULT1R = FAULT2R;
#[doc = "Possible values of the field `IDLES1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLES1R {
    #[doc = "Inactive"]
    INACTIVE,
    #[doc = "Active"]
    ACTIVE,
}
impl IDLES1R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            IDLES1R::INACTIVE => false,
            IDLES1R::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLES1R {
        match value {
            false => IDLES1R::INACTIVE,
            true => IDLES1R::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == IDLES1R::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == IDLES1R::ACTIVE
    }
}
#[doc = r" Value of the field"]
pub struct IDLEM1R {
    bits: bool,
}
impl IDLEM1R {
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
#[doc = "Possible values of the field `POL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL1R {
    #[doc = "positive polarity (output active high)"]
    ACTIVE_HIGH,
    #[doc = "negative polarity (output active low)"]
    ACTIVE_LOW,
}
impl POL1R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            POL1R::ACTIVE_HIGH => false,
            POL1R::ACTIVE_LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL1R {
        match value {
            false => POL1R::ACTIVE_HIGH,
            true => POL1R::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline]
    pub fn is_active_high(&self) -> bool {
        *self == POL1R::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline]
    pub fn is_active_low(&self) -> bool {
        *self == POL1R::ACTIVE_LOW
    }
}
#[doc = r" Proxy"]
pub struct _DIDL2W<'a> {
    w: &'a mut W,
}
impl<'a> _DIDL2W<'a> {
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
pub struct _CHP2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHP2W<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FAULT2`"]
pub enum FAULT2W {
    #[doc = "No action: the output is not affected by the fault input and stays in run mode."]
    NO_ACTION,
    #[doc = "Active"]
    ACTIVE,
    #[doc = "Inactive"]
    INACTIVE,
    #[doc = "High-Z"]
    HIGH_Z,
}
impl FAULT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FAULT2W::NO_ACTION => 0,
            FAULT2W::ACTIVE => 1,
            FAULT2W::INACTIVE => 2,
            FAULT2W::HIGH_Z => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULT2W<'a> {
    w: &'a mut W,
}
impl<'a> _FAULT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULT2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action: the output is not affected by the fault input and stays in run mode."]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(FAULT2W::NO_ACTION)
    }
    #[doc = "Active"]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(FAULT2W::ACTIVE)
    }
    #[doc = "Inactive"]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(FAULT2W::INACTIVE)
    }
    #[doc = "High-Z"]
    #[inline]
    pub fn high_z(self) -> &'a mut W {
        self.variant(FAULT2W::HIGH_Z)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDLES2W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLES2W<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDLEM2W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLEM2W<'a> {
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
#[doc = r" Proxy"]
pub struct _POL2W<'a> {
    w: &'a mut W,
}
impl<'a> _POL2W<'a> {
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
pub struct _DLYPRTW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYPRTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYPRTENW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYPRTENW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTENW<'a> {
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
pub struct _DIDL1W<'a> {
    w: &'a mut W,
}
impl<'a> _DIDL1W<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHP1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHP1W<'a> {
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
#[doc = "Values that can be written to the field `FAULT1`"]
pub type FAULT1W = FAULT2W;
#[doc = r" Proxy"]
pub struct _FAULT1W<'a> {
    w: &'a mut W,
}
impl<'a> _FAULT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULT1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action: the output is not affected by the fault input and stays in run mode."]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(FAULT2W::NO_ACTION)
    }
    #[doc = "Active"]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(FAULT2W::ACTIVE)
    }
    #[doc = "Inactive"]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(FAULT2W::INACTIVE)
    }
    #[doc = "High-Z"]
    #[inline]
    pub fn high_z(self) -> &'a mut W {
        self.variant(FAULT2W::HIGH_Z)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDLES1`"]
pub enum IDLES1W {
    #[doc = "Inactive"]
    INACTIVE,
    #[doc = "Active"]
    ACTIVE,
}
impl IDLES1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLES1W::INACTIVE => false,
            IDLES1W::ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLES1W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLES1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLES1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inactive"]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(IDLES1W::INACTIVE)
    }
    #[doc = "Active"]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(IDLES1W::ACTIVE)
    }
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
pub struct _IDLEM1W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLEM1W<'a> {
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
#[doc = "Values that can be written to the field `POL1`"]
pub enum POL1W {
    #[doc = "positive polarity (output active high)"]
    ACTIVE_HIGH,
    #[doc = "negative polarity (output active low)"]
    ACTIVE_LOW,
}
impl POL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL1W::ACTIVE_HIGH => false,
            POL1W::ACTIVE_LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL1W<'a> {
    w: &'a mut W,
}
impl<'a> _POL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "positive polarity (output active high)"]
    #[inline]
    pub fn active_high(self) -> &'a mut W {
        self.variant(POL1W::ACTIVE_HIGH)
    }
    #[doc = "negative polarity (output active low)"]
    #[inline]
    pub fn active_low(self) -> &'a mut W {
        self.variant(POL1W::ACTIVE_LOW)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline]
    pub fn didl2(&self) -> DIDL2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIDL2R { bits }
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline]
    pub fn chp2(&self) -> CHP2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHP2R { bits }
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline]
    pub fn fault2(&self) -> FAULT2R {
        FAULT2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline]
    pub fn idles2(&self) -> IDLES2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDLES2R { bits }
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline]
    pub fn idlem2(&self) -> IDLEM2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDLEM2R { bits }
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline]
    pub fn pol2(&self) -> POL2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POL2R { bits }
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline]
    pub fn dlyprt(&self) -> DLYPRTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYPRTR { bits }
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline]
    pub fn dlyprten(&self) -> DLYPRTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLYPRTENR { bits }
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline]
    pub fn dten(&self) -> DTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTENR { bits }
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline]
    pub fn didl1(&self) -> DIDL1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIDL1R { bits }
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline]
    pub fn chp1(&self) -> CHP1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHP1R { bits }
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline]
    pub fn fault1(&self) -> FAULT1R {
        FAULT1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline]
    pub fn idles1(&self) -> IDLES1R {
        IDLES1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline]
    pub fn idlem1(&self) -> IDLEM1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDLEM1R { bits }
    }
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline]
    pub fn pol1(&self) -> POL1R {
        POL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline]
    pub fn didl2(&mut self) -> _DIDL2W {
        _DIDL2W { w: self }
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline]
    pub fn chp2(&mut self) -> _CHP2W {
        _CHP2W { w: self }
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline]
    pub fn fault2(&mut self) -> _FAULT2W {
        _FAULT2W { w: self }
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline]
    pub fn idles2(&mut self) -> _IDLES2W {
        _IDLES2W { w: self }
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline]
    pub fn idlem2(&mut self) -> _IDLEM2W {
        _IDLEM2W { w: self }
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline]
    pub fn pol2(&mut self) -> _POL2W {
        _POL2W { w: self }
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline]
    pub fn dlyprt(&mut self) -> _DLYPRTW {
        _DLYPRTW { w: self }
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline]
    pub fn dlyprten(&mut self) -> _DLYPRTENW {
        _DLYPRTENW { w: self }
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline]
    pub fn dten(&mut self) -> _DTENW {
        _DTENW { w: self }
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline]
    pub fn didl1(&mut self) -> _DIDL1W {
        _DIDL1W { w: self }
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline]
    pub fn chp1(&mut self) -> _CHP1W {
        _CHP1W { w: self }
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline]
    pub fn fault1(&mut self) -> _FAULT1W {
        _FAULT1W { w: self }
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline]
    pub fn idles1(&mut self) -> _IDLES1W {
        _IDLES1W { w: self }
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline]
    pub fn idlem1(&mut self) -> _IDLEM1W {
        _IDLEM1W { w: self }
    }
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline]
    pub fn pol1(&mut self) -> _POL1W {
        _POL1W { w: self }
    }
}
