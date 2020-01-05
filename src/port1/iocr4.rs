#[doc = "Reader of register IOCR4"]
pub type R = crate::R<u32, super::IOCR4>;
#[doc = "Writer for register IOCR4"]
pub type W = crate::W<u32, super::IOCR4>;
#[doc = "Register IOCR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Control for Port n Pin 4 to 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC4_A {
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
impl From<PC4_A> for u8 {
    #[inline(always)]
    fn from(variant: PC4_A) -> Self {
        match variant {
            PC4_A::VALUE1 => 0,
            PC4_A::VALUE2 => 1,
            PC4_A::VALUE3 => 2,
            PC4_A::VALUE4 => 3,
            PC4_A::VALUE5 => 4,
            PC4_A::VALUE6 => 5,
            PC4_A::VALUE7 => 6,
            PC4_A::VALUE8 => 7,
            PC4_A::VALUE9 => 16,
            PC4_A::VALUE10 => 17,
            PC4_A::VALUE11 => 18,
            PC4_A::VALUE12 => 19,
            PC4_A::VALUE13 => 20,
            PC4_A::VALUE14 => 21,
            PC4_A::VALUE15 => 22,
            PC4_A::VALUE16 => 23,
            PC4_A::VALUE17 => 24,
            PC4_A::VALUE18 => 25,
            PC4_A::VALUE19 => 26,
            PC4_A::VALUE20 => 27,
            PC4_A::VALUE21 => 28,
            PC4_A::VALUE22 => 29,
            PC4_A::VALUE23 => 30,
            PC4_A::VALUE24 => 31,
        }
    }
}
#[doc = "Reader of field `PC4`"]
pub type PC4_R = crate::R<u8, PC4_A>;
impl PC4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PC4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PC4_A::VALUE1),
            1 => Val(PC4_A::VALUE2),
            2 => Val(PC4_A::VALUE3),
            3 => Val(PC4_A::VALUE4),
            4 => Val(PC4_A::VALUE5),
            5 => Val(PC4_A::VALUE6),
            6 => Val(PC4_A::VALUE7),
            7 => Val(PC4_A::VALUE8),
            16 => Val(PC4_A::VALUE9),
            17 => Val(PC4_A::VALUE10),
            18 => Val(PC4_A::VALUE11),
            19 => Val(PC4_A::VALUE12),
            20 => Val(PC4_A::VALUE13),
            21 => Val(PC4_A::VALUE14),
            22 => Val(PC4_A::VALUE15),
            23 => Val(PC4_A::VALUE16),
            24 => Val(PC4_A::VALUE17),
            25 => Val(PC4_A::VALUE18),
            26 => Val(PC4_A::VALUE19),
            27 => Val(PC4_A::VALUE20),
            28 => Val(PC4_A::VALUE21),
            29 => Val(PC4_A::VALUE22),
            30 => Val(PC4_A::VALUE23),
            31 => Val(PC4_A::VALUE24),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC4_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC4_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC4_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC4_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC4_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC4_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC4_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC4_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC4_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC4_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC4_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC4_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC4_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC4_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC4_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC4_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC4_A::VALUE18
    }
    #[doc = "Checks if the value of the field is `VALUE19`"]
    #[inline(always)]
    pub fn is_value19(&self) -> bool {
        *self == PC4_A::VALUE19
    }
    #[doc = "Checks if the value of the field is `VALUE20`"]
    #[inline(always)]
    pub fn is_value20(&self) -> bool {
        *self == PC4_A::VALUE20
    }
    #[doc = "Checks if the value of the field is `VALUE21`"]
    #[inline(always)]
    pub fn is_value21(&self) -> bool {
        *self == PC4_A::VALUE21
    }
    #[doc = "Checks if the value of the field is `VALUE22`"]
    #[inline(always)]
    pub fn is_value22(&self) -> bool {
        *self == PC4_A::VALUE22
    }
    #[doc = "Checks if the value of the field is `VALUE23`"]
    #[inline(always)]
    pub fn is_value23(&self) -> bool {
        *self == PC4_A::VALUE23
    }
    #[doc = "Checks if the value of the field is `VALUE24`"]
    #[inline(always)]
    pub fn is_value24(&self) -> bool {
        *self == PC4_A::VALUE24
    }
}
#[doc = "Write proxy for field `PC4`"]
pub struct PC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC4_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC4_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC4_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC4_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC4_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC4_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC4_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC4_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC4_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC4_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC4_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC4_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC4_A::VALUE13)
    }
    #[doc = "Output Push-Pull - Alternate output function 5"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC4_A::VALUE14)
    }
    #[doc = "Output Push-Pull - Alternate output function 6"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC4_A::VALUE15)
    }
    #[doc = "Output Push-Pull - Alternate output function 7"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC4_A::VALUE16)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC4_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC4_A::VALUE18)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value19(self) -> &'a mut W {
        self.variant(PC4_A::VALUE19)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value20(self) -> &'a mut W {
        self.variant(PC4_A::VALUE20)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value21(self) -> &'a mut W {
        self.variant(PC4_A::VALUE21)
    }
    #[doc = "Output Open Drain - Alternate output function 5"]
    #[inline(always)]
    pub fn value22(self) -> &'a mut W {
        self.variant(PC4_A::VALUE22)
    }
    #[doc = "Output Open Drain - Alternate output function 6"]
    #[inline(always)]
    pub fn value23(self) -> &'a mut W {
        self.variant(PC4_A::VALUE23)
    }
    #[doc = "Output Open Drain - Alternate output function 7"]
    #[inline(always)]
    pub fn value24(self) -> &'a mut W {
        self.variant(PC4_A::VALUE24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 4 to 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC5_A {
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
impl From<PC5_A> for u8 {
    #[inline(always)]
    fn from(variant: PC5_A) -> Self {
        match variant {
            PC5_A::VALUE1 => 0,
            PC5_A::VALUE2 => 1,
            PC5_A::VALUE3 => 2,
            PC5_A::VALUE4 => 3,
            PC5_A::VALUE5 => 4,
            PC5_A::VALUE6 => 5,
            PC5_A::VALUE7 => 6,
            PC5_A::VALUE8 => 7,
            PC5_A::VALUE9 => 16,
            PC5_A::VALUE10 => 17,
            PC5_A::VALUE11 => 18,
            PC5_A::VALUE12 => 19,
            PC5_A::VALUE13 => 20,
            PC5_A::VALUE14 => 21,
            PC5_A::VALUE15 => 22,
            PC5_A::VALUE16 => 23,
            PC5_A::VALUE17 => 24,
            PC5_A::VALUE18 => 25,
            PC5_A::VALUE19 => 26,
            PC5_A::VALUE20 => 27,
            PC5_A::VALUE21 => 28,
            PC5_A::VALUE22 => 29,
            PC5_A::VALUE23 => 30,
            PC5_A::VALUE24 => 31,
        }
    }
}
#[doc = "Reader of field `PC5`"]
pub type PC5_R = crate::R<u8, PC5_A>;
impl PC5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PC5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PC5_A::VALUE1),
            1 => Val(PC5_A::VALUE2),
            2 => Val(PC5_A::VALUE3),
            3 => Val(PC5_A::VALUE4),
            4 => Val(PC5_A::VALUE5),
            5 => Val(PC5_A::VALUE6),
            6 => Val(PC5_A::VALUE7),
            7 => Val(PC5_A::VALUE8),
            16 => Val(PC5_A::VALUE9),
            17 => Val(PC5_A::VALUE10),
            18 => Val(PC5_A::VALUE11),
            19 => Val(PC5_A::VALUE12),
            20 => Val(PC5_A::VALUE13),
            21 => Val(PC5_A::VALUE14),
            22 => Val(PC5_A::VALUE15),
            23 => Val(PC5_A::VALUE16),
            24 => Val(PC5_A::VALUE17),
            25 => Val(PC5_A::VALUE18),
            26 => Val(PC5_A::VALUE19),
            27 => Val(PC5_A::VALUE20),
            28 => Val(PC5_A::VALUE21),
            29 => Val(PC5_A::VALUE22),
            30 => Val(PC5_A::VALUE23),
            31 => Val(PC5_A::VALUE24),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC5_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC5_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC5_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC5_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC5_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC5_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC5_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC5_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC5_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC5_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC5_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC5_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC5_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC5_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC5_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC5_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC5_A::VALUE18
    }
    #[doc = "Checks if the value of the field is `VALUE19`"]
    #[inline(always)]
    pub fn is_value19(&self) -> bool {
        *self == PC5_A::VALUE19
    }
    #[doc = "Checks if the value of the field is `VALUE20`"]
    #[inline(always)]
    pub fn is_value20(&self) -> bool {
        *self == PC5_A::VALUE20
    }
    #[doc = "Checks if the value of the field is `VALUE21`"]
    #[inline(always)]
    pub fn is_value21(&self) -> bool {
        *self == PC5_A::VALUE21
    }
    #[doc = "Checks if the value of the field is `VALUE22`"]
    #[inline(always)]
    pub fn is_value22(&self) -> bool {
        *self == PC5_A::VALUE22
    }
    #[doc = "Checks if the value of the field is `VALUE23`"]
    #[inline(always)]
    pub fn is_value23(&self) -> bool {
        *self == PC5_A::VALUE23
    }
    #[doc = "Checks if the value of the field is `VALUE24`"]
    #[inline(always)]
    pub fn is_value24(&self) -> bool {
        *self == PC5_A::VALUE24
    }
}
#[doc = "Write proxy for field `PC5`"]
pub struct PC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PC5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC5_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC5_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC5_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC5_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC5_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC5_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC5_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC5_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC5_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC5_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC5_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC5_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC5_A::VALUE13)
    }
    #[doc = "Output Push-Pull - Alternate output function 5"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC5_A::VALUE14)
    }
    #[doc = "Output Push-Pull - Alternate output function 6"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC5_A::VALUE15)
    }
    #[doc = "Output Push-Pull - Alternate output function 7"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC5_A::VALUE16)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC5_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC5_A::VALUE18)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value19(self) -> &'a mut W {
        self.variant(PC5_A::VALUE19)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value20(self) -> &'a mut W {
        self.variant(PC5_A::VALUE20)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value21(self) -> &'a mut W {
        self.variant(PC5_A::VALUE21)
    }
    #[doc = "Output Open Drain - Alternate output function 5"]
    #[inline(always)]
    pub fn value22(self) -> &'a mut W {
        self.variant(PC5_A::VALUE22)
    }
    #[doc = "Output Open Drain - Alternate output function 6"]
    #[inline(always)]
    pub fn value23(self) -> &'a mut W {
        self.variant(PC5_A::VALUE23)
    }
    #[doc = "Output Open Drain - Alternate output function 7"]
    #[inline(always)]
    pub fn value24(self) -> &'a mut W {
        self.variant(PC5_A::VALUE24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 4 to 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC6_A {
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
impl From<PC6_A> for u8 {
    #[inline(always)]
    fn from(variant: PC6_A) -> Self {
        match variant {
            PC6_A::VALUE1 => 0,
            PC6_A::VALUE2 => 1,
            PC6_A::VALUE3 => 2,
            PC6_A::VALUE4 => 3,
            PC6_A::VALUE5 => 4,
            PC6_A::VALUE6 => 5,
            PC6_A::VALUE7 => 6,
            PC6_A::VALUE8 => 7,
            PC6_A::VALUE9 => 16,
            PC6_A::VALUE10 => 17,
            PC6_A::VALUE11 => 18,
            PC6_A::VALUE12 => 19,
            PC6_A::VALUE13 => 20,
            PC6_A::VALUE14 => 21,
            PC6_A::VALUE15 => 22,
            PC6_A::VALUE16 => 23,
            PC6_A::VALUE17 => 24,
            PC6_A::VALUE18 => 25,
            PC6_A::VALUE19 => 26,
            PC6_A::VALUE20 => 27,
            PC6_A::VALUE21 => 28,
            PC6_A::VALUE22 => 29,
            PC6_A::VALUE23 => 30,
            PC6_A::VALUE24 => 31,
        }
    }
}
#[doc = "Reader of field `PC6`"]
pub type PC6_R = crate::R<u8, PC6_A>;
impl PC6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PC6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PC6_A::VALUE1),
            1 => Val(PC6_A::VALUE2),
            2 => Val(PC6_A::VALUE3),
            3 => Val(PC6_A::VALUE4),
            4 => Val(PC6_A::VALUE5),
            5 => Val(PC6_A::VALUE6),
            6 => Val(PC6_A::VALUE7),
            7 => Val(PC6_A::VALUE8),
            16 => Val(PC6_A::VALUE9),
            17 => Val(PC6_A::VALUE10),
            18 => Val(PC6_A::VALUE11),
            19 => Val(PC6_A::VALUE12),
            20 => Val(PC6_A::VALUE13),
            21 => Val(PC6_A::VALUE14),
            22 => Val(PC6_A::VALUE15),
            23 => Val(PC6_A::VALUE16),
            24 => Val(PC6_A::VALUE17),
            25 => Val(PC6_A::VALUE18),
            26 => Val(PC6_A::VALUE19),
            27 => Val(PC6_A::VALUE20),
            28 => Val(PC6_A::VALUE21),
            29 => Val(PC6_A::VALUE22),
            30 => Val(PC6_A::VALUE23),
            31 => Val(PC6_A::VALUE24),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC6_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC6_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC6_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC6_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC6_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC6_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC6_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC6_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC6_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC6_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC6_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC6_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC6_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC6_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC6_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC6_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC6_A::VALUE18
    }
    #[doc = "Checks if the value of the field is `VALUE19`"]
    #[inline(always)]
    pub fn is_value19(&self) -> bool {
        *self == PC6_A::VALUE19
    }
    #[doc = "Checks if the value of the field is `VALUE20`"]
    #[inline(always)]
    pub fn is_value20(&self) -> bool {
        *self == PC6_A::VALUE20
    }
    #[doc = "Checks if the value of the field is `VALUE21`"]
    #[inline(always)]
    pub fn is_value21(&self) -> bool {
        *self == PC6_A::VALUE21
    }
    #[doc = "Checks if the value of the field is `VALUE22`"]
    #[inline(always)]
    pub fn is_value22(&self) -> bool {
        *self == PC6_A::VALUE22
    }
    #[doc = "Checks if the value of the field is `VALUE23`"]
    #[inline(always)]
    pub fn is_value23(&self) -> bool {
        *self == PC6_A::VALUE23
    }
    #[doc = "Checks if the value of the field is `VALUE24`"]
    #[inline(always)]
    pub fn is_value24(&self) -> bool {
        *self == PC6_A::VALUE24
    }
}
#[doc = "Write proxy for field `PC6`"]
pub struct PC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PC6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC6_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC6_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC6_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC6_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC6_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC6_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC6_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC6_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC6_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC6_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC6_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC6_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC6_A::VALUE13)
    }
    #[doc = "Output Push-Pull - Alternate output function 5"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC6_A::VALUE14)
    }
    #[doc = "Output Push-Pull - Alternate output function 6"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC6_A::VALUE15)
    }
    #[doc = "Output Push-Pull - Alternate output function 7"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC6_A::VALUE16)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC6_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC6_A::VALUE18)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value19(self) -> &'a mut W {
        self.variant(PC6_A::VALUE19)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value20(self) -> &'a mut W {
        self.variant(PC6_A::VALUE20)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value21(self) -> &'a mut W {
        self.variant(PC6_A::VALUE21)
    }
    #[doc = "Output Open Drain - Alternate output function 5"]
    #[inline(always)]
    pub fn value22(self) -> &'a mut W {
        self.variant(PC6_A::VALUE22)
    }
    #[doc = "Output Open Drain - Alternate output function 6"]
    #[inline(always)]
    pub fn value23(self) -> &'a mut W {
        self.variant(PC6_A::VALUE23)
    }
    #[doc = "Output Open Drain - Alternate output function 7"]
    #[inline(always)]
    pub fn value24(self) -> &'a mut W {
        self.variant(PC6_A::VALUE24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 6"]
    #[inline(always)]
    pub fn pc4(&self) -> PC4_R {
        PC4_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 6"]
    #[inline(always)]
    pub fn pc5(&self) -> PC5_R {
        PC5_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 6"]
    #[inline(always)]
    pub fn pc6(&self) -> PC6_R {
        PC6_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 6"]
    #[inline(always)]
    pub fn pc4(&mut self) -> PC4_W {
        PC4_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 6"]
    #[inline(always)]
    pub fn pc5(&mut self) -> PC5_W {
        PC5_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 6"]
    #[inline(always)]
    pub fn pc6(&mut self) -> PC6_W {
        PC6_W { w: self }
    }
}
