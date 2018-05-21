#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMACR {
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
#[doc = "Possible values of the field `UPDGAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDGATR {
    #[doc = "the update occurs independently from the DMA burst transfer"]
    INDEPENDENT,
    #[doc = "the update occurs when the DMA burst transfer is completed"]
    DMA_COMPLETED,
    #[doc = "the update occurs on the update event following the DMA burst transfer completion"]
    DMA_UPDATE_EVENT,
    #[doc = "the update occurs on a rising edge of HRTIM update enable input 1"]
    ENABLE_1,
    #[doc = "the update occurs on a rising edge of HRTIM update enable input 2"]
    ENABLE_2,
    #[doc = "the update occurs on a rising edge of HRTIM update enable input 3"]
    ENABLE_3,
    #[doc = "the update occurs on the update event following a rising edge of HRTIM update enable input 1"]
    ENABLE_1_UPDATE_EVENT,
    #[doc = "the update occurs on the update event following a rising edge of HRTIM update enable input 2"]
    ENABLE_2_UPDATE_EVENT,
    #[doc = "the update occurs on the update event following a rising edge of HRTIM update enable input 3"]
    ENABLE_3_UPDATE_EVENT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UPDGATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UPDGATR::INDEPENDENT => 0,
            UPDGATR::DMA_COMPLETED => 1,
            UPDGATR::DMA_UPDATE_EVENT => 2,
            UPDGATR::ENABLE_1 => 3,
            UPDGATR::ENABLE_2 => 4,
            UPDGATR::ENABLE_3 => 5,
            UPDGATR::ENABLE_1_UPDATE_EVENT => 6,
            UPDGATR::ENABLE_2_UPDATE_EVENT => 7,
            UPDGATR::ENABLE_3_UPDATE_EVENT => 8,
            UPDGATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UPDGATR {
        match value {
            0 => UPDGATR::INDEPENDENT,
            1 => UPDGATR::DMA_COMPLETED,
            2 => UPDGATR::DMA_UPDATE_EVENT,
            3 => UPDGATR::ENABLE_1,
            4 => UPDGATR::ENABLE_2,
            5 => UPDGATR::ENABLE_3,
            6 => UPDGATR::ENABLE_1_UPDATE_EVENT,
            7 => UPDGATR::ENABLE_2_UPDATE_EVENT,
            8 => UPDGATR::ENABLE_3_UPDATE_EVENT,
            i => UPDGATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == UPDGATR::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `DMA_COMPLETED`"]
    #[inline]
    pub fn is_dma_completed(&self) -> bool {
        *self == UPDGATR::DMA_COMPLETED
    }
    #[doc = "Checks if the value of the field is `DMA_UPDATE_EVENT`"]
    #[inline]
    pub fn is_dma_update_event(&self) -> bool {
        *self == UPDGATR::DMA_UPDATE_EVENT
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline]
    pub fn is_enable_1(&self) -> bool {
        *self == UPDGATR::ENABLE_1
    }
    #[doc = "Checks if the value of the field is `ENABLE_2`"]
    #[inline]
    pub fn is_enable_2(&self) -> bool {
        *self == UPDGATR::ENABLE_2
    }
    #[doc = "Checks if the value of the field is `ENABLE_3`"]
    #[inline]
    pub fn is_enable_3(&self) -> bool {
        *self == UPDGATR::ENABLE_3
    }
    #[doc = "Checks if the value of the field is `ENABLE_1_UPDATE_EVENT`"]
    #[inline]
    pub fn is_enable_1_update_event(&self) -> bool {
        *self == UPDGATR::ENABLE_1_UPDATE_EVENT
    }
    #[doc = "Checks if the value of the field is `ENABLE_2_UPDATE_EVENT`"]
    #[inline]
    pub fn is_enable_2_update_event(&self) -> bool {
        *self == UPDGATR::ENABLE_2_UPDATE_EVENT
    }
    #[doc = "Checks if the value of the field is `ENABLE_3_UPDATE_EVENT`"]
    #[inline]
    pub fn is_enable_3_update_event(&self) -> bool {
        *self == UPDGATR::ENABLE_3_UPDATE_EVENT
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
pub type DACSYNCR = ::hrtim_master::mcr::DACSYNCR;
#[doc = r" Value of the field"]
pub struct MSTUR {
    bits: bool,
}
impl MSTUR {
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
pub struct TEUR {
    bits: bool,
}
impl TEUR {
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
pub struct TDUR {
    bits: bool,
}
impl TDUR {
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
pub struct TCUR {
    bits: bool,
}
impl TCUR {
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
pub struct TBUR {
    bits: bool,
}
impl TBUR {
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
pub struct RSTUR {
    bits: bool,
}
impl RSTUR {
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
pub struct REPUR {
    bits: bool,
}
impl REPUR {
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
#[doc = "Possible values of the field `DELCMP4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DELCMP4R {
    #[doc = "CMP4 register is always active (standard compare mode)"]
    ALWAYS_ACTIVE,
    #[doc = "CMP4 value is recomputed and is active following a capture 2 event"]
    CAPTURE_2,
    #[doc = "CMP4 value is recomputed and is active following a capture 2 event, or is recomputed and active after Compare 1 match (timeout function if capture 2 event is missing)"]
    CAPTURE_2_OR_COMPARE_1,
    #[doc = "CMP4 value is recomputed and is active following a capture event, or is recomputed and active after Compare 3 match (timeout function if capture event is missing)"]
    CAPTURE_2_OR_COMPARE_3,
}
impl DELCMP4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DELCMP4R::ALWAYS_ACTIVE => 0,
            DELCMP4R::CAPTURE_2 => 1,
            DELCMP4R::CAPTURE_2_OR_COMPARE_1 => 2,
            DELCMP4R::CAPTURE_2_OR_COMPARE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DELCMP4R {
        match value {
            0 => DELCMP4R::ALWAYS_ACTIVE,
            1 => DELCMP4R::CAPTURE_2,
            2 => DELCMP4R::CAPTURE_2_OR_COMPARE_1,
            3 => DELCMP4R::CAPTURE_2_OR_COMPARE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline]
    pub fn is_always_active(&self) -> bool {
        *self == DELCMP4R::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `CAPTURE_2`"]
    #[inline]
    pub fn is_capture_2(&self) -> bool {
        *self == DELCMP4R::CAPTURE_2
    }
    #[doc = "Checks if the value of the field is `CAPTURE_2_OR_COMPARE_1`"]
    #[inline]
    pub fn is_capture_2_or_compare_1(&self) -> bool {
        *self == DELCMP4R::CAPTURE_2_OR_COMPARE_1
    }
    #[doc = "Checks if the value of the field is `CAPTURE_2_OR_COMPARE_3`"]
    #[inline]
    pub fn is_capture_2_or_compare_3(&self) -> bool {
        *self == DELCMP4R::CAPTURE_2_OR_COMPARE_3
    }
}
#[doc = "Possible values of the field `DELCMP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DELCMP2R {
    #[doc = "CMP4 register is always active (standard compare mode)"]
    ALWAYS_ACTIVE,
    #[doc = "CMP2 value is recomputed and is active following a capture 1 event"]
    CAPTURE_1,
    #[doc = "CMP2 value is recomputed and is active following a capture 1 event, or is recomputed and active after Compare 1 match (timeout function if capture event is missing)"]
    CAPTURE_1_OR_COMPARE_1,
    #[doc = "CMP2 value is recomputed and is active following a capture 1 event, or is recomputed and active after Compare 3 match (timeout function if capture event is missing)"]
    CAPTURE_1_OR_COMPARE_3,
}
impl DELCMP2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DELCMP2R::ALWAYS_ACTIVE => 0,
            DELCMP2R::CAPTURE_1 => 1,
            DELCMP2R::CAPTURE_1_OR_COMPARE_1 => 2,
            DELCMP2R::CAPTURE_1_OR_COMPARE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DELCMP2R {
        match value {
            0 => DELCMP2R::ALWAYS_ACTIVE,
            1 => DELCMP2R::CAPTURE_1,
            2 => DELCMP2R::CAPTURE_1_OR_COMPARE_1,
            3 => DELCMP2R::CAPTURE_1_OR_COMPARE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline]
    pub fn is_always_active(&self) -> bool {
        *self == DELCMP2R::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `CAPTURE_1`"]
    #[inline]
    pub fn is_capture_1(&self) -> bool {
        *self == DELCMP2R::CAPTURE_1
    }
    #[doc = "Checks if the value of the field is `CAPTURE_1_OR_COMPARE_1`"]
    #[inline]
    pub fn is_capture_1_or_compare_1(&self) -> bool {
        *self == DELCMP2R::CAPTURE_1_OR_COMPARE_1
    }
    #[doc = "Checks if the value of the field is `CAPTURE_1_OR_COMPARE_3`"]
    #[inline]
    pub fn is_capture_1_or_compare_3(&self) -> bool {
        *self == DELCMP2R::CAPTURE_1_OR_COMPARE_3
    }
}
#[doc = r" Value of the field"]
pub struct SYNCSTRTR {
    bits: bool,
}
impl SYNCSTRTR {
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
pub struct SYNCRSTXR {
    bits: bool,
}
impl SYNCRSTXR {
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
pub struct PSHPLLR {
    bits: bool,
}
impl PSHPLLR {
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
#[doc = "Values that can be written to the field `UPDGAT`"]
pub enum UPDGATW {
    #[doc = "the update occurs independently from the DMA burst transfer"]
    INDEPENDENT,
    #[doc = "the update occurs when the DMA burst transfer is completed"]
    DMA_COMPLETED,
    #[doc = "the update occurs on the update event following the DMA burst transfer completion"]
    DMA_UPDATE_EVENT,
    #[doc = "the update occurs on a rising edge of HRTIM update enable input 1"]
    ENABLE_1,
    #[doc = "the update occurs on a rising edge of HRTIM update enable input 2"]
    ENABLE_2,
    #[doc = "the update occurs on a rising edge of HRTIM update enable input 3"]
    ENABLE_3,
    #[doc = "the update occurs on the update event following a rising edge of HRTIM update enable input 1"]
    ENABLE_1_UPDATE_EVENT,
    #[doc = "the update occurs on the update event following a rising edge of HRTIM update enable input 2"]
    ENABLE_2_UPDATE_EVENT,
    #[doc = "the update occurs on the update event following a rising edge of HRTIM update enable input 3"]
    ENABLE_3_UPDATE_EVENT,
}
impl UPDGATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UPDGATW::INDEPENDENT => 0,
            UPDGATW::DMA_COMPLETED => 1,
            UPDGATW::DMA_UPDATE_EVENT => 2,
            UPDGATW::ENABLE_1 => 3,
            UPDGATW::ENABLE_2 => 4,
            UPDGATW::ENABLE_3 => 5,
            UPDGATW::ENABLE_1_UPDATE_EVENT => 6,
            UPDGATW::ENABLE_2_UPDATE_EVENT => 7,
            UPDGATW::ENABLE_3_UPDATE_EVENT => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPDGATW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDGATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPDGATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "the update occurs independently from the DMA burst transfer"]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(UPDGATW::INDEPENDENT)
    }
    #[doc = "the update occurs when the DMA burst transfer is completed"]
    #[inline]
    pub fn dma_completed(self) -> &'a mut W {
        self.variant(UPDGATW::DMA_COMPLETED)
    }
    #[doc = "the update occurs on the update event following the DMA burst transfer completion"]
    #[inline]
    pub fn dma_update_event(self) -> &'a mut W {
        self.variant(UPDGATW::DMA_UPDATE_EVENT)
    }
    #[doc = "the update occurs on a rising edge of HRTIM update enable input 1"]
    #[inline]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(UPDGATW::ENABLE_1)
    }
    #[doc = "the update occurs on a rising edge of HRTIM update enable input 2"]
    #[inline]
    pub fn enable_2(self) -> &'a mut W {
        self.variant(UPDGATW::ENABLE_2)
    }
    #[doc = "the update occurs on a rising edge of HRTIM update enable input 3"]
    #[inline]
    pub fn enable_3(self) -> &'a mut W {
        self.variant(UPDGATW::ENABLE_3)
    }
    #[doc = "the update occurs on the update event following a rising edge of HRTIM update enable input 1"]
    #[inline]
    pub fn enable_1_update_event(self) -> &'a mut W {
        self.variant(UPDGATW::ENABLE_1_UPDATE_EVENT)
    }
    #[doc = "the update occurs on the update event following a rising edge of HRTIM update enable input 2"]
    #[inline]
    pub fn enable_2_update_event(self) -> &'a mut W {
        self.variant(UPDGATW::ENABLE_2_UPDATE_EVENT)
    }
    #[doc = "the update occurs on the update event following a rising edge of HRTIM update enable input 3"]
    #[inline]
    pub fn enable_3_update_event(self) -> &'a mut W {
        self.variant(UPDGATW::ENABLE_3_UPDATE_EVENT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
pub type DACSYNCW = ::hrtim_master::mcr::DACSYNCW;
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
        self.variant(::hrtim_master::mcr::DACSYNCW::NONE)
    }
    #[doc = "Update generated on DACtrigOut1"]
    #[inline]
    pub fn dactrig_out1(self) -> &'a mut W {
        self.variant(::hrtim_master::mcr::DACSYNCW::DACTRIGOUT1)
    }
    #[doc = "Update generated on DACtrigOut2"]
    #[inline]
    pub fn dactrig_out2(self) -> &'a mut W {
        self.variant(::hrtim_master::mcr::DACSYNCW::DACTRIGOUT2)
    }
    #[doc = "Update generated on DACtrigOut3"]
    #[inline]
    pub fn dactrig_out3(self) -> &'a mut W {
        self.variant(::hrtim_master::mcr::DACSYNCW::DACTRIGOUT3)
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
pub struct _MSTUW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTUW<'a> {
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
#[doc = r" Proxy"]
pub struct _TEUW<'a> {
    w: &'a mut W,
}
impl<'a> _TEUW<'a> {
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
pub struct _TDUW<'a> {
    w: &'a mut W,
}
impl<'a> _TDUW<'a> {
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
#[doc = r" Proxy"]
pub struct _TCUW<'a> {
    w: &'a mut W,
}
impl<'a> _TCUW<'a> {
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
pub struct _TBUW<'a> {
    w: &'a mut W,
}
impl<'a> _TBUW<'a> {
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
pub struct _RSTUW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTUW<'a> {
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
pub struct _REPUW<'a> {
    w: &'a mut W,
}
impl<'a> _REPUW<'a> {
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
#[doc = "Values that can be written to the field `DELCMP4`"]
pub enum DELCMP4W {
    #[doc = "CMP4 register is always active (standard compare mode)"]
    ALWAYS_ACTIVE,
    #[doc = "CMP4 value is recomputed and is active following a capture 2 event"]
    CAPTURE_2,
    #[doc = "CMP4 value is recomputed and is active following a capture 2 event, or is recomputed and active after Compare 1 match (timeout function if capture 2 event is missing)"]
    CAPTURE_2_OR_COMPARE_1,
    #[doc = "CMP4 value is recomputed and is active following a capture event, or is recomputed and active after Compare 3 match (timeout function if capture event is missing)"]
    CAPTURE_2_OR_COMPARE_3,
}
impl DELCMP4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DELCMP4W::ALWAYS_ACTIVE => 0,
            DELCMP4W::CAPTURE_2 => 1,
            DELCMP4W::CAPTURE_2_OR_COMPARE_1 => 2,
            DELCMP4W::CAPTURE_2_OR_COMPARE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DELCMP4W<'a> {
    w: &'a mut W,
}
impl<'a> _DELCMP4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DELCMP4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CMP4 register is always active (standard compare mode)"]
    #[inline]
    pub fn always_active(self) -> &'a mut W {
        self.variant(DELCMP4W::ALWAYS_ACTIVE)
    }
    #[doc = "CMP4 value is recomputed and is active following a capture 2 event"]
    #[inline]
    pub fn capture_2(self) -> &'a mut W {
        self.variant(DELCMP4W::CAPTURE_2)
    }
    #[doc = "CMP4 value is recomputed and is active following a capture 2 event, or is recomputed and active after Compare 1 match (timeout function if capture 2 event is missing)"]
    #[inline]
    pub fn capture_2_or_compare_1(self) -> &'a mut W {
        self.variant(DELCMP4W::CAPTURE_2_OR_COMPARE_1)
    }
    #[doc = "CMP4 value is recomputed and is active following a capture event, or is recomputed and active after Compare 3 match (timeout function if capture event is missing)"]
    #[inline]
    pub fn capture_2_or_compare_3(self) -> &'a mut W {
        self.variant(DELCMP4W::CAPTURE_2_OR_COMPARE_3)
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
#[doc = "Values that can be written to the field `DELCMP2`"]
pub enum DELCMP2W {
    #[doc = "CMP4 register is always active (standard compare mode)"]
    ALWAYS_ACTIVE,
    #[doc = "CMP2 value is recomputed and is active following a capture 1 event"]
    CAPTURE_1,
    #[doc = "CMP2 value is recomputed and is active following a capture 1 event, or is recomputed and active after Compare 1 match (timeout function if capture event is missing)"]
    CAPTURE_1_OR_COMPARE_1,
    #[doc = "CMP2 value is recomputed and is active following a capture 1 event, or is recomputed and active after Compare 3 match (timeout function if capture event is missing)"]
    CAPTURE_1_OR_COMPARE_3,
}
impl DELCMP2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DELCMP2W::ALWAYS_ACTIVE => 0,
            DELCMP2W::CAPTURE_1 => 1,
            DELCMP2W::CAPTURE_1_OR_COMPARE_1 => 2,
            DELCMP2W::CAPTURE_1_OR_COMPARE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DELCMP2W<'a> {
    w: &'a mut W,
}
impl<'a> _DELCMP2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DELCMP2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CMP4 register is always active (standard compare mode)"]
    #[inline]
    pub fn always_active(self) -> &'a mut W {
        self.variant(DELCMP2W::ALWAYS_ACTIVE)
    }
    #[doc = "CMP2 value is recomputed and is active following a capture 1 event"]
    #[inline]
    pub fn capture_1(self) -> &'a mut W {
        self.variant(DELCMP2W::CAPTURE_1)
    }
    #[doc = "CMP2 value is recomputed and is active following a capture 1 event, or is recomputed and active after Compare 1 match (timeout function if capture event is missing)"]
    #[inline]
    pub fn capture_1_or_compare_1(self) -> &'a mut W {
        self.variant(DELCMP2W::CAPTURE_1_OR_COMPARE_1)
    }
    #[doc = "CMP2 value is recomputed and is active following a capture 1 event, or is recomputed and active after Compare 3 match (timeout function if capture event is missing)"]
    #[inline]
    pub fn capture_1_or_compare_3(self) -> &'a mut W {
        self.variant(DELCMP2W::CAPTURE_1_OR_COMPARE_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYNCSTRTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCSTRTW<'a> {
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
pub struct _SYNCRSTXW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCRSTXW<'a> {
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
#[doc = r" Proxy"]
pub struct _PSHPLLW<'a> {
    w: &'a mut W,
}
impl<'a> _PSHPLLW<'a> {
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
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline]
    pub fn updgat(&self) -> UPDGATR {
        UPDGATR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bit 24 - Master Timer update"]
    #[inline]
    pub fn mstu(&self) -> MSTUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSTUR { bits }
    }
    #[doc = "Bit 23 - TEU"]
    #[inline]
    pub fn teu(&self) -> TEUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEUR { bits }
    }
    #[doc = "Bit 22 - TDU"]
    #[inline]
    pub fn tdu(&self) -> TDUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TDUR { bits }
    }
    #[doc = "Bit 21 - TCU"]
    #[inline]
    pub fn tcu(&self) -> TCUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCUR { bits }
    }
    #[doc = "Bit 20 - TBU"]
    #[inline]
    pub fn tbu(&self) -> TBUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBUR { bits }
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline]
    pub fn rstu(&self) -> RSTUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSTUR { bits }
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline]
    pub fn repu(&self) -> REPUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REPUR { bits }
    }
    #[doc = "Bits 14:15 - CMP4 auto-delayed mode"]
    #[inline]
    pub fn delcmp4(&self) -> DELCMP4R {
        DELCMP4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline]
    pub fn delcmp2(&self) -> DELCMP2R {
        DELCMP2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline]
    pub fn syncstrt(&self) -> SYNCSTRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNCSTRTR { bits }
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline]
    pub fn syncrstx(&self) -> SYNCRSTXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNCRSTXR { bits }
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline]
    pub fn pshpll(&self) -> PSHPLLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PSHPLLR { bits }
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
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline]
    pub fn retrig(&self) -> RETRIGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RETRIGR { bits }
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline]
    pub fn cont(&self) -> CONTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONTR { bits }
    }
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
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
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline]
    pub fn updgat(&mut self) -> _UPDGATW {
        _UPDGATW { w: self }
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
    #[doc = "Bit 24 - Master Timer update"]
    #[inline]
    pub fn mstu(&mut self) -> _MSTUW {
        _MSTUW { w: self }
    }
    #[doc = "Bit 23 - TEU"]
    #[inline]
    pub fn teu(&mut self) -> _TEUW {
        _TEUW { w: self }
    }
    #[doc = "Bit 22 - TDU"]
    #[inline]
    pub fn tdu(&mut self) -> _TDUW {
        _TDUW { w: self }
    }
    #[doc = "Bit 21 - TCU"]
    #[inline]
    pub fn tcu(&mut self) -> _TCUW {
        _TCUW { w: self }
    }
    #[doc = "Bit 20 - TBU"]
    #[inline]
    pub fn tbu(&mut self) -> _TBUW {
        _TBUW { w: self }
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline]
    pub fn rstu(&mut self) -> _RSTUW {
        _RSTUW { w: self }
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline]
    pub fn repu(&mut self) -> _REPUW {
        _REPUW { w: self }
    }
    #[doc = "Bits 14:15 - CMP4 auto-delayed mode"]
    #[inline]
    pub fn delcmp4(&mut self) -> _DELCMP4W {
        _DELCMP4W { w: self }
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline]
    pub fn delcmp2(&mut self) -> _DELCMP2W {
        _DELCMP2W { w: self }
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline]
    pub fn syncstrt(&mut self) -> _SYNCSTRTW {
        _SYNCSTRTW { w: self }
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline]
    pub fn syncrstx(&mut self) -> _SYNCRSTXW {
        _SYNCRSTXW { w: self }
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline]
    pub fn pshpll(&mut self) -> _PSHPLLW {
        _PSHPLLW { w: self }
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline]
    pub fn half(&mut self) -> _HALFW {
        _HALFW { w: self }
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline]
    pub fn retrig(&mut self) -> _RETRIGW {
        _RETRIGW { w: self }
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline]
    pub fn cont(&mut self) -> _CONTW {
        _CONTW { w: self }
    }
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline]
    pub fn ckpsc(&mut self) -> _CKPSCW {
        _CKPSCW { w: self }
    }
}
