///Register `CFDC%sFDCTR` reader
pub type R = crate::R<CfdcfdctrSpec>;
///Register `CFDC%sFDCTR` writer
pub type W = crate::W<CfdcfdctrSpec>;
/**Error Occurrence Counter Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eocclr {
    ///0: No error occurrence counter clear
    _0 = 0,
    ///1: Clear error occurrence counter
    _1 = 1,
}
impl From<Eocclr> for bool {
    #[inline(always)]
    fn from(variant: Eocclr) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCCLR` reader - Error Occurrence Counter Clear
pub type EocclrR = crate::BitReader<Eocclr>;
impl EocclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eocclr {
        match self.bits {
            false => Eocclr::_0,
            true => Eocclr::_1,
        }
    }
    ///No error occurrence counter clear
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eocclr::_0
    }
    ///Clear error occurrence counter
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eocclr::_1
    }
}
///Field `EOCCLR` writer - Error Occurrence Counter Clear
pub type EocclrW<'a, REG> = crate::BitWriter<'a, REG, Eocclr>;
impl<'a, REG> EocclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No error occurrence counter clear
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eocclr::_0)
    }
    ///Clear error occurrence counter
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eocclr::_1)
    }
}
/**Successful Occurrence Counter Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Socclr {
    ///0: No successful occurrence counter clear
    _0 = 0,
    ///1: Clear successful occurrence counter
    _1 = 1,
}
impl From<Socclr> for bool {
    #[inline(always)]
    fn from(variant: Socclr) -> Self {
        variant as u8 != 0
    }
}
///Field `SOCCLR` reader - Successful Occurrence Counter Clear
pub type SocclrR = crate::BitReader<Socclr>;
impl SocclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Socclr {
        match self.bits {
            false => Socclr::_0,
            true => Socclr::_1,
        }
    }
    ///No successful occurrence counter clear
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Socclr::_0
    }
    ///Clear successful occurrence counter
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Socclr::_1
    }
}
///Field `SOCCLR` writer - Successful Occurrence Counter Clear
pub type SocclrW<'a, REG> = crate::BitWriter<'a, REG, Socclr>;
impl<'a, REG> SocclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No successful occurrence counter clear
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Socclr::_0)
    }
    ///Clear successful occurrence counter
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Socclr::_1)
    }
}
impl R {
    ///Bit 0 - Error Occurrence Counter Clear
    #[inline(always)]
    pub fn eocclr(&self) -> EocclrR {
        EocclrR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Successful Occurrence Counter Clear
    #[inline(always)]
    pub fn socclr(&self) -> SocclrR {
        SocclrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFDCTR")
            .field("eocclr", &self.eocclr())
            .field("socclr", &self.socclr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error Occurrence Counter Clear
    #[inline(always)]
    pub fn eocclr(&mut self) -> EocclrW<CfdcfdctrSpec> {
        EocclrW::new(self, 0)
    }
    ///Bit 1 - Successful Occurrence Counter Clear
    #[inline(always)]
    pub fn socclr(&mut self) -> SocclrW<CfdcfdctrSpec> {
        SocclrW::new(self, 1)
    }
}
/**Channel %s CANFD Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfdctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfdctrSpec;
impl crate::RegisterSpec for CfdcfdctrSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfdctr::R`](R) reader structure
impl crate::Readable for CfdcfdctrSpec {}
///`write(|w| ..)` method takes [`cfdcfdctr::W`](W) writer structure
impl crate::Writable for CfdcfdctrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDC%sFDCTR to value 0
impl crate::Resettable for CfdcfdctrSpec {}
