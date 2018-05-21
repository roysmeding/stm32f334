#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFRL {
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
#[doc = "Possible values of the field `AFRL7`"]
pub type AFRL7R = ::gpioa::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRL6`"]
pub type AFRL6R = ::gpioa::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRL5`"]
pub type AFRL5R = ::gpioa::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRL4`"]
pub type AFRL4R = ::gpioa::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRL3`"]
pub type AFRL3R = ::gpioa::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRL2`"]
pub type AFRL2R = ::gpioa::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRL1`"]
pub type AFRL1R = ::gpioa::afrl::AFRL7R;
#[doc = "Possible values of the field `AFRL0`"]
pub type AFRL0R = ::gpioa::afrl::AFRL7R;
#[doc = "Values that can be written to the field `AFRL7`"]
pub type AFRL7W = ::gpioa::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL7W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF15)
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
#[doc = "Values that can be written to the field `AFRL6`"]
pub type AFRL6W = ::gpioa::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL6W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF15)
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
#[doc = "Values that can be written to the field `AFRL5`"]
pub type AFRL5W = ::gpioa::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL5W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF15)
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
#[doc = "Values that can be written to the field `AFRL4`"]
pub type AFRL4W = ::gpioa::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL4W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF15)
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
#[doc = "Values that can be written to the field `AFRL3`"]
pub type AFRL3W = ::gpioa::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL3W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF15)
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
#[doc = "Values that can be written to the field `AFRL2`"]
pub type AFRL2W = ::gpioa::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL2W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF15)
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
#[doc = "Values that can be written to the field `AFRL1`"]
pub type AFRL1W = ::gpioa::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL1W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF15)
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
#[doc = "Values that can be written to the field `AFRL0`"]
pub type AFRL0W = ::gpioa::afrl::AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL0W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(::gpioa::afrl::AFRL7W::AF15)
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
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl7(&self) -> AFRL7R {
        AFRL7R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl6(&self) -> AFRL6R {
        AFRL6R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl5(&self) -> AFRL5R {
        AFRL5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl4(&self) -> AFRL4R {
        AFRL4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl3(&self) -> AFRL3R {
        AFRL3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl2(&self) -> AFRL2R {
        AFRL2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl1(&self) -> AFRL1R {
        AFRL1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl0(&self) -> AFRL0R {
        AFRL0R::_from({
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
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl7(&mut self) -> _AFRL7W {
        _AFRL7W { w: self }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl6(&mut self) -> _AFRL6W {
        _AFRL6W { w: self }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl5(&mut self) -> _AFRL5W {
        _AFRL5W { w: self }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl4(&mut self) -> _AFRL4W {
        _AFRL4W { w: self }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl3(&mut self) -> _AFRL3W {
        _AFRL3W { w: self }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl2(&mut self) -> _AFRL2W {
        _AFRL2W { w: self }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl1(&mut self) -> _AFRL1W {
        _AFRL1W { w: self }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline]
    pub fn afrl0(&mut self) -> _AFRL0W {
        _AFRL0W { w: self }
    }
}
