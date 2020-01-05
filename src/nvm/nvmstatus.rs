#[doc = "Reader of register NVMSTATUS"]
pub type R = crate::R<u16, super::NVMSTATUS>;
#[doc = "Write Protocol Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPERR_A {
    #[doc = "0: No write protocol failure occurred."]
    VALUE1,
    #[doc = "1: At least one write protocol failure was detected."]
    VALUE2,
}
impl From<WRPERR_A> for bool {
    #[inline(always)]
    fn from(variant: WRPERR_A) -> Self {
        match variant {
            WRPERR_A::VALUE1 => false,
            WRPERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WRPERR`"]
pub type WRPERR_R = crate::R<bool, WRPERR_A>;
impl WRPERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRPERR_A {
        match self.bits {
            false => WRPERR_A::VALUE1,
            true => WRPERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WRPERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WRPERR_A::VALUE2
    }
}
#[doc = "ECC2 Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC2READ_A {
    #[doc = "0: No ECC two bit failure during memory read operations."]
    VALUE1,
    #[doc = "1: At least one ECC two bit failure was detected."]
    VALUE2,
}
impl From<ECC2READ_A> for bool {
    #[inline(always)]
    fn from(variant: ECC2READ_A) -> Self {
        match variant {
            ECC2READ_A::VALUE1 => false,
            ECC2READ_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ECC2READ`"]
pub type ECC2READ_R = crate::R<bool, ECC2READ_A>;
impl ECC2READ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC2READ_A {
        match self.bits {
            false => ECC2READ_A::VALUE1,
            true => ECC2READ_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECC2READ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECC2READ_A::VALUE2
    }
}
#[doc = "ECC1 Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC1READ_A {
    #[doc = "0: No ECC single bit failure occurred."]
    VALUE1,
    #[doc = "1: At least one ECC single bit failure was detected and corrected."]
    VALUE2,
}
impl From<ECC1READ_A> for bool {
    #[inline(always)]
    fn from(variant: ECC1READ_A) -> Self {
        match variant {
            ECC1READ_A::VALUE1 => false,
            ECC1READ_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ECC1READ`"]
pub type ECC1READ_R = crate::R<bool, ECC1READ_A>;
impl ECC1READ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC1READ_A {
        match self.bits {
            false => ECC1READ_A::VALUE1,
            true => ECC1READ_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECC1READ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECC1READ_A::VALUE2
    }
}
#[doc = "Verify Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VERR_A {
    #[doc = "0: No fail bit."]
    VALUE1,
    #[doc = "1: One fail bit in one data block."]
    VALUE2,
    #[doc = "2: Two fail bits in two different data blocks."]
    VALUE3,
    #[doc = "3: Two or more fail bits in one data block, or three or more fail bits overall."]
    VALUE4,
}
impl From<VERR_A> for u8 {
    #[inline(always)]
    fn from(variant: VERR_A) -> Self {
        match variant {
            VERR_A::VALUE1 => 0,
            VERR_A::VALUE2 => 1,
            VERR_A::VALUE3 => 2,
            VERR_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `VERR`"]
pub type VERR_R = crate::R<u8, VERR_A>;
impl VERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VERR_A {
        match self.bits {
            0 => VERR_A::VALUE1,
            1 => VERR_A::VALUE2,
            2 => VERR_A::VALUE3,
            3 => VERR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VERR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == VERR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == VERR_A::VALUE4
    }
}
#[doc = "Sleep Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: NVM not in sleep mode, and no sleep or wake up procedure in progress."]
    VALUE1,
    #[doc = "1: NVM in sleep mode, or busy due to a sleep or wake up procedure."]
    VALUE2,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        match variant {
            SLEEP_A::VALUE1 => false,
            SLEEP_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<bool, SLEEP_A>;
impl SLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::VALUE1,
            true => SLEEP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLEEP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SLEEP_A::VALUE2
    }
}
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: The NVM is not busy. Memory reads from the cell array and register write accesses are possible."]
    VALUE1,
    #[doc = "1: The NVM is busy. Memory reads and register write accesses are not possible."]
    VALUE2,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        match variant {
            BUSY_A::VALUE1 => false,
            BUSY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 6 - Write Protocol Error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ECC2 Read"]
    #[inline(always)]
    pub fn ecc2read(&self) -> ECC2READ_R {
        ECC2READ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ECC1 Read"]
    #[inline(always)]
    pub fn ecc1read(&self) -> ECC1READ_R {
        ECC1READ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Verify Error"]
    #[inline(always)]
    pub fn verr(&self) -> VERR_R {
        VERR_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
}
