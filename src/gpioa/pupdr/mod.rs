#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PUPDR {
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
#[doc = "Possible values of the field `PUPDR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUPDR15R {
    #[doc = "No pull-up, pull-down"]
    NONE,
    #[doc = "Pull-up"]
    PULL_UP,
    #[doc = "Pull-down"]
    PULL_DOWN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PUPDR15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PUPDR15R::NONE => 0,
            PUPDR15R::PULL_UP => 1,
            PUPDR15R::PULL_DOWN => 2,
            PUPDR15R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PUPDR15R {
        match value {
            0 => PUPDR15R::NONE,
            1 => PUPDR15R::PULL_UP,
            2 => PUPDR15R::PULL_DOWN,
            i => PUPDR15R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PUPDR15R::NONE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == PUPDR15R::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == PUPDR15R::PULL_DOWN
    }
}
#[doc = "Possible values of the field `PUPDR14`"]
pub type PUPDR14R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR13`"]
pub type PUPDR13R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR12`"]
pub type PUPDR12R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR11`"]
pub type PUPDR11R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR10`"]
pub type PUPDR10R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR9`"]
pub type PUPDR9R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR8`"]
pub type PUPDR8R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR7`"]
pub type PUPDR7R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR6`"]
pub type PUPDR6R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR5`"]
pub type PUPDR5R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR4`"]
pub type PUPDR4R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR3`"]
pub type PUPDR3R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR2`"]
pub type PUPDR2R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR1`"]
pub type PUPDR1R = PUPDR15R;
#[doc = "Possible values of the field `PUPDR0`"]
pub type PUPDR0R = PUPDR15R;
#[doc = "Values that can be written to the field `PUPDR15`"]
pub enum PUPDR15W {
    #[doc = "No pull-up, pull-down"]
    NONE,
    #[doc = "Pull-up"]
    PULL_UP,
    #[doc = "Pull-down"]
    PULL_DOWN,
}
impl PUPDR15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PUPDR15W::NONE => 0,
            PUPDR15W::PULL_UP => 1,
            PUPDR15W::PULL_DOWN => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUPDR15W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
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
#[doc = "Values that can be written to the field `PUPDR14`"]
pub type PUPDR14W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR14W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR13`"]
pub type PUPDR13W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR13W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR12`"]
pub type PUPDR12W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR12W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
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
#[doc = "Values that can be written to the field `PUPDR11`"]
pub type PUPDR11W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR11W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR10`"]
pub type PUPDR10W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR10W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR9`"]
pub type PUPDR9W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR9W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
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
#[doc = "Values that can be written to the field `PUPDR8`"]
pub type PUPDR8W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR8W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR7`"]
pub type PUPDR7W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR7W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR6`"]
pub type PUPDR6W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR6W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
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
#[doc = "Values that can be written to the field `PUPDR5`"]
pub type PUPDR5W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR5W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR4`"]
pub type PUPDR4W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR4W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
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
#[doc = "Values that can be written to the field `PUPDR3`"]
pub type PUPDR3W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR3W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
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
#[doc = "Values that can be written to the field `PUPDR2`"]
pub type PUPDR2W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR2W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR1`"]
pub type PUPDR1W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR1W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR0`"]
pub type PUPDR0W = PUPDR15W;
#[doc = r" Proxy"]
pub struct _PUPDR0W<'a> {
    w: &'a mut W,
}
impl<'a> _PUPDR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUPDR0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PUPDR15W::NONE)
    }
    #[doc = "Pull-up"]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15W::PULL_DOWN)
    }
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
    #[doc = "Bits 30:31 - Pin 15 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr15(&self) -> PUPDR15R {
        PUPDR15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Pin 14 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr14(&self) -> PUPDR14R {
        PUPDR14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Pin 13 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr13(&self) -> PUPDR13R {
        PUPDR13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Pin 12 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr12(&self) -> PUPDR12R {
        PUPDR12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Pin 11 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr11(&self) -> PUPDR11R {
        PUPDR11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Pin 10 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr10(&self) -> PUPDR10R {
        PUPDR10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Pin 9 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr9(&self) -> PUPDR9R {
        PUPDR9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Pin 8 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr8(&self) -> PUPDR8R {
        PUPDR8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Pin 7 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr7(&self) -> PUPDR7R {
        PUPDR7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Pin 6 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr6(&self) -> PUPDR6R {
        PUPDR6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Pin 5 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr5(&self) -> PUPDR5R {
        PUPDR5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Pin 4 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr4(&self) -> PUPDR4R {
        PUPDR4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Pin 3 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr3(&self) -> PUPDR3R {
        PUPDR3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Pin 2 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr2(&self) -> PUPDR2R {
        PUPDR2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Pin 1 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr1(&self) -> PUPDR1R {
        PUPDR1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1 - Pin 0 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr0(&self) -> PUPDR0R {
        PUPDR0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1677721600 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 30:31 - Pin 15 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr15(&mut self) -> _PUPDR15W {
        _PUPDR15W { w: self }
    }
    #[doc = "Bits 28:29 - Pin 14 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr14(&mut self) -> _PUPDR14W {
        _PUPDR14W { w: self }
    }
    #[doc = "Bits 26:27 - Pin 13 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr13(&mut self) -> _PUPDR13W {
        _PUPDR13W { w: self }
    }
    #[doc = "Bits 24:25 - Pin 12 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr12(&mut self) -> _PUPDR12W {
        _PUPDR12W { w: self }
    }
    #[doc = "Bits 22:23 - Pin 11 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr11(&mut self) -> _PUPDR11W {
        _PUPDR11W { w: self }
    }
    #[doc = "Bits 20:21 - Pin 10 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr10(&mut self) -> _PUPDR10W {
        _PUPDR10W { w: self }
    }
    #[doc = "Bits 18:19 - Pin 9 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr9(&mut self) -> _PUPDR9W {
        _PUPDR9W { w: self }
    }
    #[doc = "Bits 16:17 - Pin 8 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr8(&mut self) -> _PUPDR8W {
        _PUPDR8W { w: self }
    }
    #[doc = "Bits 14:15 - Pin 7 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr7(&mut self) -> _PUPDR7W {
        _PUPDR7W { w: self }
    }
    #[doc = "Bits 12:13 - Pin 6 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr6(&mut self) -> _PUPDR6W {
        _PUPDR6W { w: self }
    }
    #[doc = "Bits 10:11 - Pin 5 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr5(&mut self) -> _PUPDR5W {
        _PUPDR5W { w: self }
    }
    #[doc = "Bits 8:9 - Pin 4 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr4(&mut self) -> _PUPDR4W {
        _PUPDR4W { w: self }
    }
    #[doc = "Bits 6:7 - Pin 3 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr3(&mut self) -> _PUPDR3W {
        _PUPDR3W { w: self }
    }
    #[doc = "Bits 4:5 - Pin 2 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr2(&mut self) -> _PUPDR2W {
        _PUPDR2W { w: self }
    }
    #[doc = "Bits 2:3 - Pin 1 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr1(&mut self) -> _PUPDR1W {
        _PUPDR1W { w: self }
    }
    #[doc = "Bits 0:1 - Pin 0 pull-up/pull-down state"]
    #[inline]
    pub fn pupdr0(&mut self) -> _PUPDR0W {
        _PUPDR0W { w: self }
    }
}
