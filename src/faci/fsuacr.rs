///Register `FSUACR` reader
pub type R = crate::R<FsuacrSpec>;
///Register `FSUACR` writer
pub type W = crate::W<FsuacrSpec>;
/**Startup Area Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sas {
    ///0: Startup area is selected by BTFLG bit
    _00 = 0,
    ///1: Startup area is selected by BTFLG bit
    _01 = 1,
    ///2: Startup area is temporarily switched to the default area (block 0)
    _10 = 2,
    ///3: Startup area is temporarily switched to the alternate area (block 1).
    _11 = 3,
}
impl From<Sas> for u8 {
    #[inline(always)]
    fn from(variant: Sas) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sas {
    type Ux = u8;
}
impl crate::IsEnum for Sas {}
///Field `SAS` reader - Startup Area Select
pub type SasR = crate::FieldReader<Sas>;
impl SasR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sas {
        match self.bits {
            0 => Sas::_00,
            1 => Sas::_01,
            2 => Sas::_10,
            3 => Sas::_11,
            _ => unreachable!(),
        }
    }
    ///Startup area is selected by BTFLG bit
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sas::_00
    }
    ///Startup area is selected by BTFLG bit
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sas::_01
    }
    ///Startup area is temporarily switched to the default area (block 0)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sas::_10
    }
    ///Startup area is temporarily switched to the alternate area (block 1).
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sas::_11
    }
}
///Field `SAS` writer - Startup Area Select
pub type SasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sas, crate::Safe>;
impl<'a, REG> SasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Startup area is selected by BTFLG bit
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sas::_00)
    }
    ///Startup area is selected by BTFLG bit
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sas::_01)
    }
    ///Startup area is temporarily switched to the default area (block 0)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sas::_10)
    }
    ///Startup area is temporarily switched to the alternate area (block 1).
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sas::_11)
    }
}
///Field `KEY` writer - Key Code
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:1 - Startup Area Select
    #[inline(always)]
    pub fn sas(&self) -> SasR {
        SasR::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSUACR").field("sas", &self.sas()).finish()
    }
}
impl W {
    ///Bits 0:1 - Startup Area Select
    #[inline(always)]
    pub fn sas(&mut self) -> SasW<FsuacrSpec> {
        SasW::new(self, 0)
    }
    ///Bits 8:15 - Key Code
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<FsuacrSpec> {
        KeyW::new(self, 8)
    }
}
/**Flash Startup Area Control Register

You can [`read`](crate::Reg::read) this register and get [`fsuacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsuacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FsuacrSpec;
impl crate::RegisterSpec for FsuacrSpec {
    type Ux = u16;
}
///`read()` method returns [`fsuacr::R`](R) reader structure
impl crate::Readable for FsuacrSpec {}
///`write(|w| ..)` method takes [`fsuacr::W`](W) writer structure
impl crate::Writable for FsuacrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FSUACR to value 0
impl crate::Resettable for FsuacrSpec {}
