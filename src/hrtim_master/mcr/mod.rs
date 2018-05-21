#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = "Possible values of the field `BRSTDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRSTDMAR {
    #[doc = "Update done independently from the DMA burst transfer completion"]
    INDEPENDENT,
    #[doc = "Update done when the DMA burst transfer is completed"]
    COMPLETION,
    #[doc = "Update done on master timer roll-over following a DMA burst transfer completion. This mode only works in continuous mode."]
    ROLLOVER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BRSTDMAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BRSTDMAR::INDEPENDENT => 0,
            BRSTDMAR::COMPLETION => 1,
            BRSTDMAR::ROLLOVER => 2,
            BRSTDMAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BRSTDMAR {
        match value {
            0 => BRSTDMAR::INDEPENDENT,
            1 => BRSTDMAR::COMPLETION,
            2 => BRSTDMAR::ROLLOVER,
            i => BRSTDMAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == BRSTDMAR::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `COMPLETION`"]
    #[inline]
    pub fn is_completion(&self) -> bool {
        *self == BRSTDMAR::COMPLETION
    }
    #[doc = "Checks if the value of the field is `ROLLOVER`"]
    #[inline]
    pub fn is_rollover(&self) -> bool {
        *self == BRSTDMAR::ROLLOVER
    }
}
#[doc = r" Value of the field"]
pub struct MREPUR {
    bits: bool,
}
impl MREPUR {
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
pub struct PREENR {
    bits: bool,
}
impl PREENR {
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
#[doc = "Possible values of the field `DACSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACSYNCR {
    #[doc = "No update generated"]
    NONE,
    #[doc = "Update generated on DACtrigOut1"]
    DACTRIGOUT1,
    #[doc = "Update generated on DACtrigOut2"]
    DACTRIGOUT2,
    #[doc = "Update generated on DACtrigOut3"]
    DACTRIGOUT3,
}
impl DACSYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DACSYNCR::NONE => 0,
            DACSYNCR::DACTRIGOUT1 => 1,
            DACSYNCR::DACTRIGOUT2 => 2,
            DACSYNCR::DACTRIGOUT3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DACSYNCR {
        match value {
            0 => DACSYNCR::NONE,
            1 => DACSYNCR::DACTRIGOUT1,
            2 => DACSYNCR::DACTRIGOUT2,
            3 => DACSYNCR::DACTRIGOUT3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == DACSYNCR::NONE
    }
    #[doc = "Checks if the value of the field is `DACTRIGOUT1`"]
    #[inline]
    pub fn is_dactrig_out1(&self) -> bool {
        *self == DACSYNCR::DACTRIGOUT1
    }
    #[doc = "Checks if the value of the field is `DACTRIGOUT2`"]
    #[inline]
    pub fn is_dactrig_out2(&self) -> bool {
        *self == DACSYNCR::DACTRIGOUT2
    }
    #[doc = "Checks if the value of the field is `DACTRIGOUT3`"]
    #[inline]
    pub fn is_dactrig_out3(&self) -> bool {
        *self == DACSYNCR::DACTRIGOUT3
    }
}
#[doc = r" Value of the field"]
pub struct TECENR {
    bits: bool,
}
impl TECENR {
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
pub struct TDCENR {
    bits: bool,
}
impl TDCENR {
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
pub struct TCCENR {
    bits: bool,
}
impl TCCENR {
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
pub struct TBCENR {
    bits: bool,
}
impl TBCENR {
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
pub struct TACENR {
    bits: bool,
}
impl TACENR {
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
pub struct MCENR {
    bits: bool,
}
impl MCENR {
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
#[doc = "Possible values of the field `SYNC_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_SRCR {
    #[doc = "Master timer Start"]
    MASTER_START,
    #[doc = "Master timer Compare 1 event"]
    MASTER_COMPARE1,
    #[doc = "Timer A start/reset"]
    TIMERA_START,
    #[doc = "Timer A Compare 1 event"]
    TIMERA_COMPARE1,
}
impl SYNC_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNC_SRCR::MASTER_START => 0,
            SYNC_SRCR::MASTER_COMPARE1 => 1,
            SYNC_SRCR::TIMERA_START => 2,
            SYNC_SRCR::TIMERA_COMPARE1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNC_SRCR {
        match value {
            0 => SYNC_SRCR::MASTER_START,
            1 => SYNC_SRCR::MASTER_COMPARE1,
            2 => SYNC_SRCR::TIMERA_START,
            3 => SYNC_SRCR::TIMERA_COMPARE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_START`"]
    #[inline]
    pub fn is_master_start(&self) -> bool {
        *self == SYNC_SRCR::MASTER_START
    }
    #[doc = "Checks if the value of the field is `MASTER_COMPARE1`"]
    #[inline]
    pub fn is_master_compare1(&self) -> bool {
        *self == SYNC_SRCR::MASTER_COMPARE1
    }
    #[doc = "Checks if the value of the field is `TIMERA_START`"]
    #[inline]
    pub fn is_timer_a_start(&self) -> bool {
        *self == SYNC_SRCR::TIMERA_START
    }
    #[doc = "Checks if the value of the field is `TIMERA_COMPARE1`"]
    #[inline]
    pub fn is_timer_a_compare1(&self) -> bool {
        *self == SYNC_SRCR::TIMERA_COMPARE1
    }
}
#[doc = "Possible values of the field `SYNCOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCOUTR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Positive pulse on HRTIM_SCOUT output (16x f_HRTIM clock cycles)"]
    SCOUT_POSITIVE,
    #[doc = "Negative pulse on HRTIM_SCOUT output (16x f_HRTIM clock cycles)"]
    SCOUT_NEGATIVE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYNCOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCOUTR::DISABLED => 0,
            SYNCOUTR::SCOUT_POSITIVE => 2,
            SYNCOUTR::SCOUT_NEGATIVE => 3,
            SYNCOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCOUTR {
        match value {
            0 => SYNCOUTR::DISABLED,
            2 => SYNCOUTR::SCOUT_POSITIVE,
            3 => SYNCOUTR::SCOUT_NEGATIVE,
            i => SYNCOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCOUTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `SCOUT_POSITIVE`"]
    #[inline]
    pub fn is_scout_positive(&self) -> bool {
        *self == SYNCOUTR::SCOUT_POSITIVE
    }
    #[doc = "Checks if the value of the field is `SCOUT_NEGATIVE`"]
    #[inline]
    pub fn is_scout_negative(&self) -> bool {
        *self == SYNCOUTR::SCOUT_NEGATIVE
    }
}
#[doc = r" Value of the field"]
pub struct SYNCSTRTMR {
    bits: bool,
}
impl SYNCSTRTMR {
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
pub struct SYNCRSTMR {
    bits: bool,
}
impl SYNCRSTMR {
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
#[doc = "Possible values of the field `SYNCIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCINR {
    #[doc = "disabled. HRTIM is not synchronized and runs in standalone mode."]
    DISABLED,
    #[doc = "Internal event: the HRTIM is synchronized with the on-chip timer (see Synchronization input)."]
    INTERNAL_EVENT,
    #[doc = "External event (input pin). A positive pulse on HRTIM_SCIN input triggers the HRTIM."]
    EXTERNAL_EVENT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYNCINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCINR::DISABLED => 0,
            SYNCINR::INTERNAL_EVENT => 2,
            SYNCINR::EXTERNAL_EVENT => 3,
            SYNCINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCINR {
        match value {
            0 => SYNCINR::DISABLED,
            2 => SYNCINR::INTERNAL_EVENT,
            3 => SYNCINR::EXTERNAL_EVENT,
            i => SYNCINR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCINR::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERNAL_EVENT`"]
    #[inline]
    pub fn is_internal_event(&self) -> bool {
        *self == SYNCINR::INTERNAL_EVENT
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_EVENT`"]
    #[inline]
    pub fn is_external_event(&self) -> bool {
        *self == SYNCINR::EXTERNAL_EVENT
    }
}
#[doc = r" Value of the field"]
pub struct HALFR {
    bits: bool,
}
impl HALFR {
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
pub struct RETRIGR {
    bits: bool,
}
impl RETRIGR {
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
pub struct CONTR {
    bits: bool,
}
impl CONTR {
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
pub struct CKPSCR {
    bits: u8,
}
impl CKPSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `BRSTDMA`"]
pub enum BRSTDMAW {
    #[doc = "Update done independently from the DMA burst transfer completion"]
    INDEPENDENT,
    #[doc = "Update done when the DMA burst transfer is completed"]
    COMPLETION,
    #[doc = "Update done on master timer roll-over following a DMA burst transfer completion. This mode only works in continuous mode."]
    ROLLOVER,
}
impl BRSTDMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BRSTDMAW::INDEPENDENT => 0,
            BRSTDMAW::COMPLETION => 1,
            BRSTDMAW::ROLLOVER => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRSTDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _BRSTDMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRSTDMAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Update done independently from the DMA burst transfer completion"]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(BRSTDMAW::INDEPENDENT)
    }
    #[doc = "Update done when the DMA burst transfer is completed"]
    #[inline]
    pub fn completion(self) -> &'a mut W {
        self.variant(BRSTDMAW::COMPLETION)
    }
    #[doc = "Update done on master timer roll-over following a DMA burst transfer completion. This mode only works in continuous mode."]
    #[inline]
    pub fn rollover(self) -> &'a mut W {
        self.variant(BRSTDMAW::ROLLOVER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MREPUW<'a> {
    w: &'a mut W,
}
impl<'a> _MREPUW<'a> {
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
pub struct _PREENW<'a> {
    w: &'a mut W,
}
impl<'a> _PREENW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DACSYNC`"]
pub enum DACSYNCW {
    #[doc = "No update generated"]
    NONE,
    #[doc = "Update generated on DACtrigOut1"]
    DACTRIGOUT1,
    #[doc = "Update generated on DACtrigOut2"]
    DACTRIGOUT2,
    #[doc = "Update generated on DACtrigOut3"]
    DACTRIGOUT3,
}
impl DACSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DACSYNCW::NONE => 0,
            DACSYNCW::DACTRIGOUT1 => 1,
            DACSYNCW::DACTRIGOUT2 => 2,
            DACSYNCW::DACTRIGOUT3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _DACSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACSYNCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No update generated"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(DACSYNCW::NONE)
    }
    #[doc = "Update generated on DACtrigOut1"]
    #[inline]
    pub fn dactrig_out1(self) -> &'a mut W {
        self.variant(DACSYNCW::DACTRIGOUT1)
    }
    #[doc = "Update generated on DACtrigOut2"]
    #[inline]
    pub fn dactrig_out2(self) -> &'a mut W {
        self.variant(DACSYNCW::DACTRIGOUT2)
    }
    #[doc = "Update generated on DACtrigOut3"]
    #[inline]
    pub fn dactrig_out3(self) -> &'a mut W {
        self.variant(DACSYNCW::DACTRIGOUT3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TECENW<'a> {
    w: &'a mut W,
}
impl<'a> _TECENW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TDCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TDCENW<'a> {
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
pub struct _TCCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCCENW<'a> {
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
pub struct _TBCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TBCENW<'a> {
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
pub struct _TACENW<'a> {
    w: &'a mut W,
}
impl<'a> _TACENW<'a> {
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
pub struct _MCENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCENW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNC_SRC`"]
pub enum SYNC_SRCW {
    #[doc = "Master timer Start"]
    MASTER_START,
    #[doc = "Master timer Compare 1 event"]
    MASTER_COMPARE1,
    #[doc = "Timer A start/reset"]
    TIMERA_START,
    #[doc = "Timer A Compare 1 event"]
    TIMERA_COMPARE1,
}
impl SYNC_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNC_SRCW::MASTER_START => 0,
            SYNC_SRCW::MASTER_COMPARE1 => 1,
            SYNC_SRCW::TIMERA_START => 2,
            SYNC_SRCW::TIMERA_COMPARE1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC_SRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Master timer Start"]
    #[inline]
    pub fn master_start(self) -> &'a mut W {
        self.variant(SYNC_SRCW::MASTER_START)
    }
    #[doc = "Master timer Compare 1 event"]
    #[inline]
    pub fn master_compare1(self) -> &'a mut W {
        self.variant(SYNC_SRCW::MASTER_COMPARE1)
    }
    #[doc = "Timer A start/reset"]
    #[inline]
    pub fn timer_a_start(self) -> &'a mut W {
        self.variant(SYNC_SRCW::TIMERA_START)
    }
    #[doc = "Timer A Compare 1 event"]
    #[inline]
    pub fn timer_a_compare1(self) -> &'a mut W {
        self.variant(SYNC_SRCW::TIMERA_COMPARE1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCOUT`"]
pub enum SYNCOUTW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Positive pulse on HRTIM_SCOUT output (16x f_HRTIM clock cycles)"]
    SCOUT_POSITIVE,
    #[doc = "Negative pulse on HRTIM_SCOUT output (16x f_HRTIM clock cycles)"]
    SCOUT_NEGATIVE,
}
impl SYNCOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCOUTW::DISABLED => 0,
            SYNCOUTW::SCOUT_POSITIVE => 2,
            SYNCOUTW::SCOUT_NEGATIVE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCOUTW::DISABLED)
    }
    #[doc = "Positive pulse on HRTIM_SCOUT output (16x f_HRTIM clock cycles)"]
    #[inline]
    pub fn scout_positive(self) -> &'a mut W {
        self.variant(SYNCOUTW::SCOUT_POSITIVE)
    }
    #[doc = "Negative pulse on HRTIM_SCOUT output (16x f_HRTIM clock cycles)"]
    #[inline]
    pub fn scout_negative(self) -> &'a mut W {
        self.variant(SYNCOUTW::SCOUT_NEGATIVE)
    }
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
pub struct _SYNCSTRTMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCSTRTMW<'a> {
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
pub struct _SYNCRSTMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCRSTMW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCIN`"]
pub enum SYNCINW {
    #[doc = "disabled. HRTIM is not synchronized and runs in standalone mode."]
    DISABLED,
    #[doc = "Internal event: the HRTIM is synchronized with the on-chip timer (see Synchronization input)."]
    INTERNAL_EVENT,
    #[doc = "External event (input pin). A positive pulse on HRTIM_SCIN input triggers the HRTIM."]
    EXTERNAL_EVENT,
}
impl SYNCINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCINW::DISABLED => 0,
            SYNCINW::INTERNAL_EVENT => 2,
            SYNCINW::EXTERNAL_EVENT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCINW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCINW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "disabled. HRTIM is not synchronized and runs in standalone mode."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCINW::DISABLED)
    }
    #[doc = "Internal event: the HRTIM is synchronized with the on-chip timer (see Synchronization input)."]
    #[inline]
    pub fn internal_event(self) -> &'a mut W {
        self.variant(SYNCINW::INTERNAL_EVENT)
    }
    #[doc = "External event (input pin). A positive pulse on HRTIM_SCIN input triggers the HRTIM."]
    #[inline]
    pub fn external_event(self) -> &'a mut W {
        self.variant(SYNCINW::EXTERNAL_EVENT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HALFW<'a> {
    w: &'a mut W,
}
impl<'a> _HALFW<'a> {
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
pub struct _RETRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _RETRIGW<'a> {
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
pub struct _CONTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTW<'a> {
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
pub struct _CKPSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CKPSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline]
    pub fn brstdma(&self) -> BRSTDMAR {
        BRSTDMAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline]
    pub fn mrepu(&self) -> MREPUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MREPUR { bits }
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline]
    pub fn preen(&self) -> PREENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PREENR { bits }
    }
    #[doc = "Bits 25:26 - DAC Synchronization"]
    #[inline]
    pub fn dacsync(&self) -> DACSYNCR {
        DACSYNCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline]
    pub fn tecen(&self) -> TECENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TECENR { bits }
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline]
    pub fn tdcen(&self) -> TDCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TDCENR { bits }
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline]
    pub fn tccen(&self) -> TCCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCCENR { bits }
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline]
    pub fn tbcen(&self) -> TBCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBCENR { bits }
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline]
    pub fn tacen(&self) -> TACENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TACENR { bits }
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline]
    pub fn mcen(&self) -> MCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MCENR { bits }
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline]
    pub fn sync_src(&self) -> SYNC_SRCR {
        SYNC_SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline]
    pub fn syncout(&self) -> SYNCOUTR {
        SYNCOUTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline]
    pub fn syncstrtm(&self) -> SYNCSTRTMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNCSTRTMR { bits }
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline]
    pub fn syncrstm(&self) -> SYNCRSTMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNCRSTMR { bits }
    }
    #[doc = "Bits 8:9 - ynchronization input"]
    #[inline]
    pub fn syncin(&self) -> SYNCINR {
        SYNCINR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline]
    pub fn half(&self) -> HALFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HALFR { bits }
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline]
    pub fn retrig(&self) -> RETRIGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RETRIGR { bits }
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline]
    pub fn cont(&self) -> CONTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONTR { bits }
    }
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline]
    pub fn ckpsc(&self) -> CKPSCR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CKPSCR { bits }
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
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline]
    pub fn brstdma(&mut self) -> _BRSTDMAW {
        _BRSTDMAW { w: self }
    }
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline]
    pub fn mrepu(&mut self) -> _MREPUW {
        _MREPUW { w: self }
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline]
    pub fn preen(&mut self) -> _PREENW {
        _PREENW { w: self }
    }
    #[doc = "Bits 25:26 - DAC Synchronization"]
    #[inline]
    pub fn dacsync(&mut self) -> _DACSYNCW {
        _DACSYNCW { w: self }
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline]
    pub fn tecen(&mut self) -> _TECENW {
        _TECENW { w: self }
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline]
    pub fn tdcen(&mut self) -> _TDCENW {
        _TDCENW { w: self }
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline]
    pub fn tccen(&mut self) -> _TCCENW {
        _TCCENW { w: self }
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline]
    pub fn tbcen(&mut self) -> _TBCENW {
        _TBCENW { w: self }
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline]
    pub fn tacen(&mut self) -> _TACENW {
        _TACENW { w: self }
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline]
    pub fn mcen(&mut self) -> _MCENW {
        _MCENW { w: self }
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline]
    pub fn sync_src(&mut self) -> _SYNC_SRCW {
        _SYNC_SRCW { w: self }
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline]
    pub fn syncout(&mut self) -> _SYNCOUTW {
        _SYNCOUTW { w: self }
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline]
    pub fn syncstrtm(&mut self) -> _SYNCSTRTMW {
        _SYNCSTRTMW { w: self }
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline]
    pub fn syncrstm(&mut self) -> _SYNCRSTMW {
        _SYNCRSTMW { w: self }
    }
    #[doc = "Bits 8:9 - ynchronization input"]
    #[inline]
    pub fn syncin(&mut self) -> _SYNCINW {
        _SYNCINW { w: self }
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline]
    pub fn half(&mut self) -> _HALFW {
        _HALFW { w: self }
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline]
    pub fn retrig(&mut self) -> _RETRIGW {
        _RETRIGW { w: self }
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline]
    pub fn cont(&mut self) -> _CONTW {
        _CONTW { w: self }
    }
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline]
    pub fn ckpsc(&mut self) -> _CKPSCW {
        _CKPSCW { w: self }
    }
}
