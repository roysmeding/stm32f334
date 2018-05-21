#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFRH {
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
#[doc = "Possible values of the field `AFRH15`"]
pub type AFRH15R = super::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRH14`"]
pub type AFRH14R = super::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRH13`"]
pub type AFRH13R = super::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRH12`"]
pub type AFRH12R = super::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRH11`"]
pub type AFRH11R = super::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRH10`"]
pub type AFRH10R = super::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRH9`"]
pub type AFRH9R = super::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRH8`"]
pub type AFRH8R = super::afrl::AFRL7R;
#[doc = "Values that can be written to the field `AFRH15`"]
pub type AFRH15W = super::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRH15W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH14`"]
pub type AFRH14W = super::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRH14W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH13`"]
pub type AFRH13W = super::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRH13W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH12`"]
pub type AFRH12W = super::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRH12W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH11`"]
pub type AFRH11W = super::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRH11W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH10`"]
pub type AFRH10W = super::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRH10W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH9`"]
pub type AFRH9W = super::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRH9W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH8`"]
pub type AFRH8W = super::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRH8W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(super::afrl::AFRL7W::AF15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 28:31"]
    #[inline]
    pub fn afrh15(&self) -> AFRH15R {
        AFRH15R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27"]
    #[inline]
    pub fn afrh14(&self) -> AFRH14R {
        AFRH14R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23"]
    #[inline]
    pub fn afrh13(&self) -> AFRH13R {
        AFRH13R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn afrh12(&self) -> AFRH12R {
        AFRH12R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15"]
    #[inline]
    pub fn afrh11(&self) -> AFRH11R {
        AFRH11R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11"]
    #[inline]
    pub fn afrh10(&self) -> AFRH10R {
        AFRH10R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7"]
    #[inline]
    pub fn afrh9(&self) -> AFRH9R {
        AFRH9R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:3"]
    #[inline]
    pub fn afrh8(&self) -> AFRH8R {
        AFRH8R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 28:31"]
    #[inline]
    pub fn afrh15(&mut self) -> _AFRH15W {
        _AFRH15W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline]
    pub fn afrh14(&mut self) -> _AFRH14W {
        _AFRH14W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline]
    pub fn afrh13(&mut self) -> _AFRH13W {
        _AFRH13W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn afrh12(&mut self) -> _AFRH12W {
        _AFRH12W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline]
    pub fn afrh11(&mut self) -> _AFRH11W {
        _AFRH11W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline]
    pub fn afrh10(&mut self) -> _AFRH10W {
        _AFRH10W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline]
    pub fn afrh9(&mut self) -> _AFRH9W {
        _AFRH9W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline]
    pub fn afrh8(&mut self) -> _AFRH8W {
        _AFRH8W { w: self }
    }
}
