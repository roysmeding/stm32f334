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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRL7R {
    #[doc = "undocumented"]
    AF0,
    #[doc = "undocumented"]
    AF1,
    #[doc = "undocumented"]
    AF2,
    #[doc = "undocumented"]
    AF3,
    #[doc = "undocumented"]
    AF4,
    #[doc = "undocumented"]
    AF5,
    #[doc = "undocumented"]
    AF6,
    #[doc = "undocumented"]
    AF7,
    #[doc = "undocumented"]
    AF8,
    #[doc = "undocumented"]
    AF9,
    #[doc = "undocumented"]
    AF10,
    #[doc = "undocumented"]
    AF11,
    #[doc = "undocumented"]
    AF12,
    #[doc = "undocumented"]
    AF13,
    #[doc = "undocumented"]
    AF14,
    #[doc = "undocumented"]
    AF15,
}
impl AFRL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRL7R::AF0 => 0,
            AFRL7R::AF1 => 1,
            AFRL7R::AF2 => 2,
            AFRL7R::AF3 => 3,
            AFRL7R::AF4 => 4,
            AFRL7R::AF5 => 5,
            AFRL7R::AF6 => 6,
            AFRL7R::AF7 => 7,
            AFRL7R::AF8 => 8,
            AFRL7R::AF9 => 9,
            AFRL7R::AF10 => 10,
            AFRL7R::AF11 => 11,
            AFRL7R::AF12 => 12,
            AFRL7R::AF13 => 13,
            AFRL7R::AF14 => 14,
            AFRL7R::AF15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRL7R {
        match value {
            0 => AFRL7R::AF0,
            1 => AFRL7R::AF1,
            2 => AFRL7R::AF2,
            3 => AFRL7R::AF3,
            4 => AFRL7R::AF4,
            5 => AFRL7R::AF5,
            6 => AFRL7R::AF6,
            7 => AFRL7R::AF7,
            8 => AFRL7R::AF8,
            9 => AFRL7R::AF9,
            10 => AFRL7R::AF10,
            11 => AFRL7R::AF11,
            12 => AFRL7R::AF12,
            13 => AFRL7R::AF13,
            14 => AFRL7R::AF14,
            15 => AFRL7R::AF15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AF0`"]
    #[inline]
    pub fn is_af0(&self) -> bool {
        *self == AFRL7R::AF0
    }
    #[doc = "Checks if the value of the field is `AF1`"]
    #[inline]
    pub fn is_af1(&self) -> bool {
        *self == AFRL7R::AF1
    }
    #[doc = "Checks if the value of the field is `AF2`"]
    #[inline]
    pub fn is_af2(&self) -> bool {
        *self == AFRL7R::AF2
    }
    #[doc = "Checks if the value of the field is `AF3`"]
    #[inline]
    pub fn is_af3(&self) -> bool {
        *self == AFRL7R::AF3
    }
    #[doc = "Checks if the value of the field is `AF4`"]
    #[inline]
    pub fn is_af4(&self) -> bool {
        *self == AFRL7R::AF4
    }
    #[doc = "Checks if the value of the field is `AF5`"]
    #[inline]
    pub fn is_af5(&self) -> bool {
        *self == AFRL7R::AF5
    }
    #[doc = "Checks if the value of the field is `AF6`"]
    #[inline]
    pub fn is_af6(&self) -> bool {
        *self == AFRL7R::AF6
    }
    #[doc = "Checks if the value of the field is `AF7`"]
    #[inline]
    pub fn is_af7(&self) -> bool {
        *self == AFRL7R::AF7
    }
    #[doc = "Checks if the value of the field is `AF8`"]
    #[inline]
    pub fn is_af8(&self) -> bool {
        *self == AFRL7R::AF8
    }
    #[doc = "Checks if the value of the field is `AF9`"]
    #[inline]
    pub fn is_af9(&self) -> bool {
        *self == AFRL7R::AF9
    }
    #[doc = "Checks if the value of the field is `AF10`"]
    #[inline]
    pub fn is_af10(&self) -> bool {
        *self == AFRL7R::AF10
    }
    #[doc = "Checks if the value of the field is `AF11`"]
    #[inline]
    pub fn is_af11(&self) -> bool {
        *self == AFRL7R::AF11
    }
    #[doc = "Checks if the value of the field is `AF12`"]
    #[inline]
    pub fn is_af12(&self) -> bool {
        *self == AFRL7R::AF12
    }
    #[doc = "Checks if the value of the field is `AF13`"]
    #[inline]
    pub fn is_af13(&self) -> bool {
        *self == AFRL7R::AF13
    }
    #[doc = "Checks if the value of the field is `AF14`"]
    #[inline]
    pub fn is_af14(&self) -> bool {
        *self == AFRL7R::AF14
    }
    #[doc = "Checks if the value of the field is `AF15`"]
    #[inline]
    pub fn is_af15(&self) -> bool {
        *self == AFRL7R::AF15
    }
}
#[doc = "Possible values of the field `AFRL6`"]
pub type AFRL6R = AFRL7R;
#[doc = "Possible values of the field `AFRL5`"]
pub type AFRL5R = AFRL7R;
#[doc = "Possible values of the field `AFRL4`"]
pub type AFRL4R = AFRL7R;
#[doc = "Possible values of the field `AFRL3`"]
pub type AFRL3R = AFRL7R;
#[doc = "Possible values of the field `AFRL2`"]
pub type AFRL2R = AFRL7R;
#[doc = "Possible values of the field `AFRL1`"]
pub type AFRL1R = AFRL7R;
#[doc = "Possible values of the field `AFRL0`"]
pub type AFRL0R = AFRL7R;
#[doc = "Values that can be written to the field `AFRL7`"]
pub enum AFRL7W {
    #[doc = "`0`"]
    AF0,
    #[doc = "`1`"]
    AF1,
    #[doc = "`10`"]
    AF2,
    #[doc = "`11`"]
    AF3,
    #[doc = "`100`"]
    AF4,
    #[doc = "`101`"]
    AF5,
    #[doc = "`110`"]
    AF6,
    #[doc = "`111`"]
    AF7,
    #[doc = "`1000`"]
    AF8,
    #[doc = "`1001`"]
    AF9,
    #[doc = "`1010`"]
    AF10,
    #[doc = "`1011`"]
    AF11,
    #[doc = "`1100`"]
    AF12,
    #[doc = "`1101`"]
    AF13,
    #[doc = "`1110`"]
    AF14,
    #[doc = "`1111`"]
    AF15,
}
impl AFRL7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRL7W::AF0 => 0,
            AFRL7W::AF1 => 1,
            AFRL7W::AF2 => 2,
            AFRL7W::AF3 => 3,
            AFRL7W::AF4 => 4,
            AFRL7W::AF5 => 5,
            AFRL7W::AF6 => 6,
            AFRL7W::AF7 => 7,
            AFRL7W::AF8 => 8,
            AFRL7W::AF9 => 9,
            AFRL7W::AF10 => 10,
            AFRL7W::AF11 => 11,
            AFRL7W::AF12 => 12,
            AFRL7W::AF13 => 13,
            AFRL7W::AF14 => 14,
            AFRL7W::AF15 => 15,
        }
    }
}
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
        self.variant(AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7W::AF15)
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
pub type AFRL6W = AFRL7W;
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
        self.variant(AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7W::AF15)
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
pub type AFRL5W = AFRL7W;
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
        self.variant(AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7W::AF15)
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
pub type AFRL4W = AFRL7W;
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
        self.variant(AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7W::AF15)
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
pub type AFRL3W = AFRL7W;
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
        self.variant(AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7W::AF15)
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
pub type AFRL2W = AFRL7W;
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
        self.variant(AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7W::AF15)
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
pub type AFRL1W = AFRL7W;
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
        self.variant(AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7W::AF15)
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
pub type AFRL0W = AFRL7W;
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
        self.variant(AFRL7W::AF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7W::AF8)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7W::AF9)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7W::AF10)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7W::AF11)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7W::AF12)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7W::AF13)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7W::AF14)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7W::AF15)
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
    #[doc = "Bits 28:31 - Alternate function for pin 7"]
    #[inline]
    pub fn afrl7(&self) -> AFRL7R {
        AFRL7R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27"]
    #[inline]
    pub fn afrl6(&self) -> AFRL6R {
        AFRL6R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23"]
    #[inline]
    pub fn afrl5(&self) -> AFRL5R {
        AFRL5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn afrl4(&self) -> AFRL4R {
        AFRL4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15"]
    #[inline]
    pub fn afrl3(&self) -> AFRL3R {
        AFRL3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11"]
    #[inline]
    pub fn afrl2(&self) -> AFRL2R {
        AFRL2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7"]
    #[inline]
    pub fn afrl1(&self) -> AFRL1R {
        AFRL1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:3"]
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
    #[doc = "Bits 28:31 - Alternate function for pin 7"]
    #[inline]
    pub fn afrl7(&mut self) -> _AFRL7W {
        _AFRL7W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline]
    pub fn afrl6(&mut self) -> _AFRL6W {
        _AFRL6W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline]
    pub fn afrl5(&mut self) -> _AFRL5W {
        _AFRL5W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn afrl4(&mut self) -> _AFRL4W {
        _AFRL4W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline]
    pub fn afrl3(&mut self) -> _AFRL3W {
        _AFRL3W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline]
    pub fn afrl2(&mut self) -> _AFRL2W {
        _AFRL2W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline]
    pub fn afrl1(&mut self) -> _AFRL1W {
        _AFRL1W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline]
    pub fn afrl0(&mut self) -> _AFRL0W {
        _AFRL0W { w: self }
    }
}
