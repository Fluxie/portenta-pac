///Register `ECCMODE` reader
pub type R = crate::R<EccmodeSpec>;
///Register `ECCMODE` writer
pub type W = crate::W<EccmodeSpec>;
/**ECC Operating Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eccmod {
    ///0: Disable ECC function
    _00 = 0,
    ///1: Setting prohibited
    _01 = 1,
    ///2: Enable ECC function without error checking
    _10 = 2,
    ///3: Enable ECC function with error checking
    _11 = 3,
}
impl From<Eccmod> for u8 {
    #[inline(always)]
    fn from(variant: Eccmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eccmod {
    type Ux = u8;
}
impl crate::IsEnum for Eccmod {}
///Field `ECCMOD` reader - ECC Operating Mode Select
pub type EccmodR = crate::FieldReader<Eccmod>;
impl EccmodR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eccmod {
        match self.bits {
            0 => Eccmod::_00,
            1 => Eccmod::_01,
            2 => Eccmod::_10,
            3 => Eccmod::_11,
            _ => unreachable!(),
        }
    }
    ///Disable ECC function
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Eccmod::_00
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Eccmod::_01
    }
    ///Enable ECC function without error checking
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Eccmod::_10
    }
    ///Enable ECC function with error checking
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Eccmod::_11
    }
}
///Field `ECCMOD` writer - ECC Operating Mode Select
pub type EccmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eccmod, crate::Safe>;
impl<'a, REG> EccmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disable ECC function
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Eccmod::_00)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Eccmod::_01)
    }
    ///Enable ECC function without error checking
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Eccmod::_10)
    }
    ///Enable ECC function with error checking
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Eccmod::_11)
    }
}
impl R {
    ///Bits 0:1 - ECC Operating Mode Select
    #[inline(always)]
    pub fn eccmod(&self) -> EccmodR {
        EccmodR::new(self.bits & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCMODE").field("eccmod", &self.eccmod()).finish()
    }
}
impl W {
    ///Bits 0:1 - ECC Operating Mode Select
    #[inline(always)]
    pub fn eccmod(&mut self) -> EccmodW<EccmodeSpec> {
        EccmodW::new(self, 0)
    }
}
/**ECC Operating Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`eccmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EccmodeSpec;
impl crate::RegisterSpec for EccmodeSpec {
    type Ux = u8;
}
///`read()` method returns [`eccmode::R`](R) reader structure
impl crate::Readable for EccmodeSpec {}
///`write(|w| ..)` method takes [`eccmode::W`](W) writer structure
impl crate::Writable for EccmodeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCMODE to value 0
impl crate::Resettable for EccmodeSpec {}
