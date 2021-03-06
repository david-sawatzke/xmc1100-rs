#[doc = "Reader of register IOCR12"]
pub type R = crate::R<u32, super::IOCR12>;
#[doc = "Writer for register IOCR12"]
pub type W = crate::W<u32, super::IOCR12>;
#[doc = "Register IOCR12 `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCR12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC12_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "21: Output Push-Pull - Alternate output function 5"]
    VALUE14,
    #[doc = "22: Output Push-Pull - Alternate output function 6"]
    VALUE15,
    #[doc = "23: Output Push-Pull - Alternate output function 7"]
    VALUE16,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE17,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE18,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE19,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE20,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE21,
    #[doc = "29: Output Open Drain - Alternate output function 5"]
    VALUE22,
    #[doc = "30: Output Open Drain - Alternate output function 6"]
    VALUE23,
    #[doc = "31: Output Open Drain - Alternate output function 7"]
    VALUE24,
}
impl From<PC12_A> for u8 {
    #[inline(always)]
    fn from(variant: PC12_A) -> Self {
        match variant {
            PC12_A::VALUE1 => 0,
            PC12_A::VALUE2 => 1,
            PC12_A::VALUE3 => 2,
            PC12_A::VALUE4 => 3,
            PC12_A::VALUE5 => 4,
            PC12_A::VALUE6 => 5,
            PC12_A::VALUE7 => 6,
            PC12_A::VALUE8 => 7,
            PC12_A::VALUE9 => 16,
            PC12_A::VALUE10 => 17,
            PC12_A::VALUE11 => 18,
            PC12_A::VALUE12 => 19,
            PC12_A::VALUE13 => 20,
            PC12_A::VALUE14 => 21,
            PC12_A::VALUE15 => 22,
            PC12_A::VALUE16 => 23,
            PC12_A::VALUE17 => 24,
            PC12_A::VALUE18 => 25,
            PC12_A::VALUE19 => 26,
            PC12_A::VALUE20 => 27,
            PC12_A::VALUE21 => 28,
            PC12_A::VALUE22 => 29,
            PC12_A::VALUE23 => 30,
            PC12_A::VALUE24 => 31,
        }
    }
}
#[doc = "Reader of field `PC12`"]
pub type PC12_R = crate::R<u8, PC12_A>;
impl PC12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PC12_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PC12_A::VALUE1),
            1 => Val(PC12_A::VALUE2),
            2 => Val(PC12_A::VALUE3),
            3 => Val(PC12_A::VALUE4),
            4 => Val(PC12_A::VALUE5),
            5 => Val(PC12_A::VALUE6),
            6 => Val(PC12_A::VALUE7),
            7 => Val(PC12_A::VALUE8),
            16 => Val(PC12_A::VALUE9),
            17 => Val(PC12_A::VALUE10),
            18 => Val(PC12_A::VALUE11),
            19 => Val(PC12_A::VALUE12),
            20 => Val(PC12_A::VALUE13),
            21 => Val(PC12_A::VALUE14),
            22 => Val(PC12_A::VALUE15),
            23 => Val(PC12_A::VALUE16),
            24 => Val(PC12_A::VALUE17),
            25 => Val(PC12_A::VALUE18),
            26 => Val(PC12_A::VALUE19),
            27 => Val(PC12_A::VALUE20),
            28 => Val(PC12_A::VALUE21),
            29 => Val(PC12_A::VALUE22),
            30 => Val(PC12_A::VALUE23),
            31 => Val(PC12_A::VALUE24),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC12_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC12_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC12_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC12_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC12_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC12_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC12_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC12_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC12_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC12_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC12_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC12_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC12_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC12_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC12_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC12_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC12_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC12_A::VALUE18
    }
    #[doc = "Checks if the value of the field is `VALUE19`"]
    #[inline(always)]
    pub fn is_value19(&self) -> bool {
        *self == PC12_A::VALUE19
    }
    #[doc = "Checks if the value of the field is `VALUE20`"]
    #[inline(always)]
    pub fn is_value20(&self) -> bool {
        *self == PC12_A::VALUE20
    }
    #[doc = "Checks if the value of the field is `VALUE21`"]
    #[inline(always)]
    pub fn is_value21(&self) -> bool {
        *self == PC12_A::VALUE21
    }
    #[doc = "Checks if the value of the field is `VALUE22`"]
    #[inline(always)]
    pub fn is_value22(&self) -> bool {
        *self == PC12_A::VALUE22
    }
    #[doc = "Checks if the value of the field is `VALUE23`"]
    #[inline(always)]
    pub fn is_value23(&self) -> bool {
        *self == PC12_A::VALUE23
    }
    #[doc = "Checks if the value of the field is `VALUE24`"]
    #[inline(always)]
    pub fn is_value24(&self) -> bool {
        *self == PC12_A::VALUE24
    }
}
#[doc = "Write proxy for field `PC12`"]
pub struct PC12_W<'a> {
    w: &'a mut W,
}
impl<'a> PC12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC12_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC12_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC12_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC12_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC12_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC12_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC12_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC12_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC12_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC12_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC12_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC12_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC12_A::VALUE13)
    }
    #[doc = "Output Push-Pull - Alternate output function 5"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC12_A::VALUE14)
    }
    #[doc = "Output Push-Pull - Alternate output function 6"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC12_A::VALUE15)
    }
    #[doc = "Output Push-Pull - Alternate output function 7"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC12_A::VALUE16)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC12_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC12_A::VALUE18)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value19(self) -> &'a mut W {
        self.variant(PC12_A::VALUE19)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value20(self) -> &'a mut W {
        self.variant(PC12_A::VALUE20)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value21(self) -> &'a mut W {
        self.variant(PC12_A::VALUE21)
    }
    #[doc = "Output Open Drain - Alternate output function 5"]
    #[inline(always)]
    pub fn value22(self) -> &'a mut W {
        self.variant(PC12_A::VALUE22)
    }
    #[doc = "Output Open Drain - Alternate output function 6"]
    #[inline(always)]
    pub fn value23(self) -> &'a mut W {
        self.variant(PC12_A::VALUE23)
    }
    #[doc = "Output Open Drain - Alternate output function 7"]
    #[inline(always)]
    pub fn value24(self) -> &'a mut W {
        self.variant(PC12_A::VALUE24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC13_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "21: Output Push-Pull - Alternate output function 5"]
    VALUE14,
    #[doc = "22: Output Push-Pull - Alternate output function 6"]
    VALUE15,
    #[doc = "23: Output Push-Pull - Alternate output function 7"]
    VALUE16,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE17,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE18,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE19,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE20,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE21,
    #[doc = "29: Output Open Drain - Alternate output function 5"]
    VALUE22,
    #[doc = "30: Output Open Drain - Alternate output function 6"]
    VALUE23,
    #[doc = "31: Output Open Drain - Alternate output function 7"]
    VALUE24,
}
impl From<PC13_A> for u8 {
    #[inline(always)]
    fn from(variant: PC13_A) -> Self {
        match variant {
            PC13_A::VALUE1 => 0,
            PC13_A::VALUE2 => 1,
            PC13_A::VALUE3 => 2,
            PC13_A::VALUE4 => 3,
            PC13_A::VALUE5 => 4,
            PC13_A::VALUE6 => 5,
            PC13_A::VALUE7 => 6,
            PC13_A::VALUE8 => 7,
            PC13_A::VALUE9 => 16,
            PC13_A::VALUE10 => 17,
            PC13_A::VALUE11 => 18,
            PC13_A::VALUE12 => 19,
            PC13_A::VALUE13 => 20,
            PC13_A::VALUE14 => 21,
            PC13_A::VALUE15 => 22,
            PC13_A::VALUE16 => 23,
            PC13_A::VALUE17 => 24,
            PC13_A::VALUE18 => 25,
            PC13_A::VALUE19 => 26,
            PC13_A::VALUE20 => 27,
            PC13_A::VALUE21 => 28,
            PC13_A::VALUE22 => 29,
            PC13_A::VALUE23 => 30,
            PC13_A::VALUE24 => 31,
        }
    }
}
#[doc = "Reader of field `PC13`"]
pub type PC13_R = crate::R<u8, PC13_A>;
impl PC13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PC13_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PC13_A::VALUE1),
            1 => Val(PC13_A::VALUE2),
            2 => Val(PC13_A::VALUE3),
            3 => Val(PC13_A::VALUE4),
            4 => Val(PC13_A::VALUE5),
            5 => Val(PC13_A::VALUE6),
            6 => Val(PC13_A::VALUE7),
            7 => Val(PC13_A::VALUE8),
            16 => Val(PC13_A::VALUE9),
            17 => Val(PC13_A::VALUE10),
            18 => Val(PC13_A::VALUE11),
            19 => Val(PC13_A::VALUE12),
            20 => Val(PC13_A::VALUE13),
            21 => Val(PC13_A::VALUE14),
            22 => Val(PC13_A::VALUE15),
            23 => Val(PC13_A::VALUE16),
            24 => Val(PC13_A::VALUE17),
            25 => Val(PC13_A::VALUE18),
            26 => Val(PC13_A::VALUE19),
            27 => Val(PC13_A::VALUE20),
            28 => Val(PC13_A::VALUE21),
            29 => Val(PC13_A::VALUE22),
            30 => Val(PC13_A::VALUE23),
            31 => Val(PC13_A::VALUE24),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC13_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC13_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC13_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC13_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC13_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC13_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC13_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC13_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC13_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC13_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC13_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC13_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC13_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC13_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC13_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC13_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC13_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC13_A::VALUE18
    }
    #[doc = "Checks if the value of the field is `VALUE19`"]
    #[inline(always)]
    pub fn is_value19(&self) -> bool {
        *self == PC13_A::VALUE19
    }
    #[doc = "Checks if the value of the field is `VALUE20`"]
    #[inline(always)]
    pub fn is_value20(&self) -> bool {
        *self == PC13_A::VALUE20
    }
    #[doc = "Checks if the value of the field is `VALUE21`"]
    #[inline(always)]
    pub fn is_value21(&self) -> bool {
        *self == PC13_A::VALUE21
    }
    #[doc = "Checks if the value of the field is `VALUE22`"]
    #[inline(always)]
    pub fn is_value22(&self) -> bool {
        *self == PC13_A::VALUE22
    }
    #[doc = "Checks if the value of the field is `VALUE23`"]
    #[inline(always)]
    pub fn is_value23(&self) -> bool {
        *self == PC13_A::VALUE23
    }
    #[doc = "Checks if the value of the field is `VALUE24`"]
    #[inline(always)]
    pub fn is_value24(&self) -> bool {
        *self == PC13_A::VALUE24
    }
}
#[doc = "Write proxy for field `PC13`"]
pub struct PC13_W<'a> {
    w: &'a mut W,
}
impl<'a> PC13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC13_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC13_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC13_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC13_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC13_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC13_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC13_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC13_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC13_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC13_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC13_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC13_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC13_A::VALUE13)
    }
    #[doc = "Output Push-Pull - Alternate output function 5"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC13_A::VALUE14)
    }
    #[doc = "Output Push-Pull - Alternate output function 6"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC13_A::VALUE15)
    }
    #[doc = "Output Push-Pull - Alternate output function 7"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC13_A::VALUE16)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC13_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC13_A::VALUE18)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value19(self) -> &'a mut W {
        self.variant(PC13_A::VALUE19)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value20(self) -> &'a mut W {
        self.variant(PC13_A::VALUE20)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value21(self) -> &'a mut W {
        self.variant(PC13_A::VALUE21)
    }
    #[doc = "Output Open Drain - Alternate output function 5"]
    #[inline(always)]
    pub fn value22(self) -> &'a mut W {
        self.variant(PC13_A::VALUE22)
    }
    #[doc = "Output Open Drain - Alternate output function 6"]
    #[inline(always)]
    pub fn value23(self) -> &'a mut W {
        self.variant(PC13_A::VALUE23)
    }
    #[doc = "Output Open Drain - Alternate output function 7"]
    #[inline(always)]
    pub fn value24(self) -> &'a mut W {
        self.variant(PC13_A::VALUE24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC14_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "21: Output Push-Pull - Alternate output function 5"]
    VALUE14,
    #[doc = "22: Output Push-Pull - Alternate output function 6"]
    VALUE15,
    #[doc = "23: Output Push-Pull - Alternate output function 7"]
    VALUE16,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE17,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE18,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE19,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE20,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE21,
    #[doc = "29: Output Open Drain - Alternate output function 5"]
    VALUE22,
    #[doc = "30: Output Open Drain - Alternate output function 6"]
    VALUE23,
    #[doc = "31: Output Open Drain - Alternate output function 7"]
    VALUE24,
}
impl From<PC14_A> for u8 {
    #[inline(always)]
    fn from(variant: PC14_A) -> Self {
        match variant {
            PC14_A::VALUE1 => 0,
            PC14_A::VALUE2 => 1,
            PC14_A::VALUE3 => 2,
            PC14_A::VALUE4 => 3,
            PC14_A::VALUE5 => 4,
            PC14_A::VALUE6 => 5,
            PC14_A::VALUE7 => 6,
            PC14_A::VALUE8 => 7,
            PC14_A::VALUE9 => 16,
            PC14_A::VALUE10 => 17,
            PC14_A::VALUE11 => 18,
            PC14_A::VALUE12 => 19,
            PC14_A::VALUE13 => 20,
            PC14_A::VALUE14 => 21,
            PC14_A::VALUE15 => 22,
            PC14_A::VALUE16 => 23,
            PC14_A::VALUE17 => 24,
            PC14_A::VALUE18 => 25,
            PC14_A::VALUE19 => 26,
            PC14_A::VALUE20 => 27,
            PC14_A::VALUE21 => 28,
            PC14_A::VALUE22 => 29,
            PC14_A::VALUE23 => 30,
            PC14_A::VALUE24 => 31,
        }
    }
}
#[doc = "Reader of field `PC14`"]
pub type PC14_R = crate::R<u8, PC14_A>;
impl PC14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PC14_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PC14_A::VALUE1),
            1 => Val(PC14_A::VALUE2),
            2 => Val(PC14_A::VALUE3),
            3 => Val(PC14_A::VALUE4),
            4 => Val(PC14_A::VALUE5),
            5 => Val(PC14_A::VALUE6),
            6 => Val(PC14_A::VALUE7),
            7 => Val(PC14_A::VALUE8),
            16 => Val(PC14_A::VALUE9),
            17 => Val(PC14_A::VALUE10),
            18 => Val(PC14_A::VALUE11),
            19 => Val(PC14_A::VALUE12),
            20 => Val(PC14_A::VALUE13),
            21 => Val(PC14_A::VALUE14),
            22 => Val(PC14_A::VALUE15),
            23 => Val(PC14_A::VALUE16),
            24 => Val(PC14_A::VALUE17),
            25 => Val(PC14_A::VALUE18),
            26 => Val(PC14_A::VALUE19),
            27 => Val(PC14_A::VALUE20),
            28 => Val(PC14_A::VALUE21),
            29 => Val(PC14_A::VALUE22),
            30 => Val(PC14_A::VALUE23),
            31 => Val(PC14_A::VALUE24),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC14_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC14_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC14_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC14_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC14_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC14_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC14_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC14_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC14_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC14_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC14_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC14_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC14_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC14_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC14_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC14_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC14_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC14_A::VALUE18
    }
    #[doc = "Checks if the value of the field is `VALUE19`"]
    #[inline(always)]
    pub fn is_value19(&self) -> bool {
        *self == PC14_A::VALUE19
    }
    #[doc = "Checks if the value of the field is `VALUE20`"]
    #[inline(always)]
    pub fn is_value20(&self) -> bool {
        *self == PC14_A::VALUE20
    }
    #[doc = "Checks if the value of the field is `VALUE21`"]
    #[inline(always)]
    pub fn is_value21(&self) -> bool {
        *self == PC14_A::VALUE21
    }
    #[doc = "Checks if the value of the field is `VALUE22`"]
    #[inline(always)]
    pub fn is_value22(&self) -> bool {
        *self == PC14_A::VALUE22
    }
    #[doc = "Checks if the value of the field is `VALUE23`"]
    #[inline(always)]
    pub fn is_value23(&self) -> bool {
        *self == PC14_A::VALUE23
    }
    #[doc = "Checks if the value of the field is `VALUE24`"]
    #[inline(always)]
    pub fn is_value24(&self) -> bool {
        *self == PC14_A::VALUE24
    }
}
#[doc = "Write proxy for field `PC14`"]
pub struct PC14_W<'a> {
    w: &'a mut W,
}
impl<'a> PC14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC14_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC14_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC14_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC14_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC14_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC14_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC14_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC14_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC14_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC14_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC14_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC14_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC14_A::VALUE13)
    }
    #[doc = "Output Push-Pull - Alternate output function 5"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC14_A::VALUE14)
    }
    #[doc = "Output Push-Pull - Alternate output function 6"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC14_A::VALUE15)
    }
    #[doc = "Output Push-Pull - Alternate output function 7"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC14_A::VALUE16)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC14_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC14_A::VALUE18)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value19(self) -> &'a mut W {
        self.variant(PC14_A::VALUE19)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value20(self) -> &'a mut W {
        self.variant(PC14_A::VALUE20)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value21(self) -> &'a mut W {
        self.variant(PC14_A::VALUE21)
    }
    #[doc = "Output Open Drain - Alternate output function 5"]
    #[inline(always)]
    pub fn value22(self) -> &'a mut W {
        self.variant(PC14_A::VALUE22)
    }
    #[doc = "Output Open Drain - Alternate output function 6"]
    #[inline(always)]
    pub fn value23(self) -> &'a mut W {
        self.variant(PC14_A::VALUE23)
    }
    #[doc = "Output Open Drain - Alternate output function 7"]
    #[inline(always)]
    pub fn value24(self) -> &'a mut W {
        self.variant(PC14_A::VALUE24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC15_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "21: Output Push-Pull - Alternate output function 5"]
    VALUE14,
    #[doc = "22: Output Push-Pull - Alternate output function 6"]
    VALUE15,
    #[doc = "23: Output Push-Pull - Alternate output function 7"]
    VALUE16,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE17,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE18,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE19,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE20,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE21,
    #[doc = "29: Output Open Drain - Alternate output function 5"]
    VALUE22,
    #[doc = "30: Output Open Drain - Alternate output function 6"]
    VALUE23,
    #[doc = "31: Output Open Drain - Alternate output function 7"]
    VALUE24,
}
impl From<PC15_A> for u8 {
    #[inline(always)]
    fn from(variant: PC15_A) -> Self {
        match variant {
            PC15_A::VALUE1 => 0,
            PC15_A::VALUE2 => 1,
            PC15_A::VALUE3 => 2,
            PC15_A::VALUE4 => 3,
            PC15_A::VALUE5 => 4,
            PC15_A::VALUE6 => 5,
            PC15_A::VALUE7 => 6,
            PC15_A::VALUE8 => 7,
            PC15_A::VALUE9 => 16,
            PC15_A::VALUE10 => 17,
            PC15_A::VALUE11 => 18,
            PC15_A::VALUE12 => 19,
            PC15_A::VALUE13 => 20,
            PC15_A::VALUE14 => 21,
            PC15_A::VALUE15 => 22,
            PC15_A::VALUE16 => 23,
            PC15_A::VALUE17 => 24,
            PC15_A::VALUE18 => 25,
            PC15_A::VALUE19 => 26,
            PC15_A::VALUE20 => 27,
            PC15_A::VALUE21 => 28,
            PC15_A::VALUE22 => 29,
            PC15_A::VALUE23 => 30,
            PC15_A::VALUE24 => 31,
        }
    }
}
#[doc = "Reader of field `PC15`"]
pub type PC15_R = crate::R<u8, PC15_A>;
impl PC15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PC15_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PC15_A::VALUE1),
            1 => Val(PC15_A::VALUE2),
            2 => Val(PC15_A::VALUE3),
            3 => Val(PC15_A::VALUE4),
            4 => Val(PC15_A::VALUE5),
            5 => Val(PC15_A::VALUE6),
            6 => Val(PC15_A::VALUE7),
            7 => Val(PC15_A::VALUE8),
            16 => Val(PC15_A::VALUE9),
            17 => Val(PC15_A::VALUE10),
            18 => Val(PC15_A::VALUE11),
            19 => Val(PC15_A::VALUE12),
            20 => Val(PC15_A::VALUE13),
            21 => Val(PC15_A::VALUE14),
            22 => Val(PC15_A::VALUE15),
            23 => Val(PC15_A::VALUE16),
            24 => Val(PC15_A::VALUE17),
            25 => Val(PC15_A::VALUE18),
            26 => Val(PC15_A::VALUE19),
            27 => Val(PC15_A::VALUE20),
            28 => Val(PC15_A::VALUE21),
            29 => Val(PC15_A::VALUE22),
            30 => Val(PC15_A::VALUE23),
            31 => Val(PC15_A::VALUE24),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC15_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC15_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC15_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC15_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC15_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC15_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC15_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC15_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC15_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC15_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC15_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC15_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC15_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC15_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC15_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC15_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC15_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC15_A::VALUE18
    }
    #[doc = "Checks if the value of the field is `VALUE19`"]
    #[inline(always)]
    pub fn is_value19(&self) -> bool {
        *self == PC15_A::VALUE19
    }
    #[doc = "Checks if the value of the field is `VALUE20`"]
    #[inline(always)]
    pub fn is_value20(&self) -> bool {
        *self == PC15_A::VALUE20
    }
    #[doc = "Checks if the value of the field is `VALUE21`"]
    #[inline(always)]
    pub fn is_value21(&self) -> bool {
        *self == PC15_A::VALUE21
    }
    #[doc = "Checks if the value of the field is `VALUE22`"]
    #[inline(always)]
    pub fn is_value22(&self) -> bool {
        *self == PC15_A::VALUE22
    }
    #[doc = "Checks if the value of the field is `VALUE23`"]
    #[inline(always)]
    pub fn is_value23(&self) -> bool {
        *self == PC15_A::VALUE23
    }
    #[doc = "Checks if the value of the field is `VALUE24`"]
    #[inline(always)]
    pub fn is_value24(&self) -> bool {
        *self == PC15_A::VALUE24
    }
}
#[doc = "Write proxy for field `PC15`"]
pub struct PC15_W<'a> {
    w: &'a mut W,
}
impl<'a> PC15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC15_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC15_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC15_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC15_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC15_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC15_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC15_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC15_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC15_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC15_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC15_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC15_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC15_A::VALUE13)
    }
    #[doc = "Output Push-Pull - Alternate output function 5"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC15_A::VALUE14)
    }
    #[doc = "Output Push-Pull - Alternate output function 6"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC15_A::VALUE15)
    }
    #[doc = "Output Push-Pull - Alternate output function 7"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC15_A::VALUE16)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC15_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC15_A::VALUE18)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value19(self) -> &'a mut W {
        self.variant(PC15_A::VALUE19)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value20(self) -> &'a mut W {
        self.variant(PC15_A::VALUE20)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value21(self) -> &'a mut W {
        self.variant(PC15_A::VALUE21)
    }
    #[doc = "Output Open Drain - Alternate output function 5"]
    #[inline(always)]
    pub fn value22(self) -> &'a mut W {
        self.variant(PC15_A::VALUE22)
    }
    #[doc = "Output Open Drain - Alternate output function 6"]
    #[inline(always)]
    pub fn value23(self) -> &'a mut W {
        self.variant(PC15_A::VALUE23)
    }
    #[doc = "Output Open Drain - Alternate output function 7"]
    #[inline(always)]
    pub fn value24(self) -> &'a mut W {
        self.variant(PC15_A::VALUE24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&self) -> PC12_R {
        PC12_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&self) -> PC13_R {
        PC13_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&self) -> PC14_R {
        PC14_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&self) -> PC15_R {
        PC15_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&mut self) -> PC12_W {
        PC12_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&mut self) -> PC13_W {
        PC13_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&mut self) -> PC14_W {
        PC14_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&mut self) -> PC15_W {
        PC15_W { w: self }
    }
}
