///Register `SRAMPRCR` reader
pub type R = crate::R<SramprcrSpec>;
///Register `SRAMPRCR` writer
pub type W = crate::W<SramprcrSpec>;
/**Register Write Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramprcr {
    ///0: Disable writes to protected registers
    _0 = 0,
    ///1: Enable writes to protected registers
    _1 = 1,
}
impl From<Sramprcr> for bool {
    #[inline(always)]
    fn from(variant: Sramprcr) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAMPRCR` reader - Register Write Control
pub type SramprcrR = crate::BitReader<Sramprcr>;
impl SramprcrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sramprcr {
        match self.bits {
            false => Sramprcr::_0,
            true => Sramprcr::_1,
        }
    }
    ///Disable writes to protected registers
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sramprcr::_0
    }
    ///Enable writes to protected registers
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sramprcr::_1
    }
}
///Field `SRAMPRCR` writer - Register Write Control
pub type SramprcrW<'a, REG> = crate::BitWriter<'a, REG, Sramprcr>;
impl<'a, REG> SramprcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable writes to protected registers
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sramprcr::_0)
    }
    ///Enable writes to protected registers
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sramprcr::_1)
    }
}
///Field `KW` writer - Write Key Code
pub type KwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn sramprcr(&self) -> SramprcrR {
        SramprcrR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAMPRCR").field("sramprcr", &self.sramprcr()).finish()
    }
}
impl W {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn sramprcr(&mut self) -> SramprcrW<SramprcrSpec> {
        SramprcrW::new(self, 0)
    }
    ///Bits 1:7 - Write Key Code
    #[inline(always)]
    pub fn kw(&mut self) -> KwW<SramprcrSpec> {
        KwW::new(self, 1)
    }
}
/**SRAM Protection Register

You can [`read`](crate::Reg::read) this register and get [`sramprcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramprcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SramprcrSpec;
impl crate::RegisterSpec for SramprcrSpec {
    type Ux = u8;
}
///`read()` method returns [`sramprcr::R`](R) reader structure
impl crate::Readable for SramprcrSpec {}
///`write(|w| ..)` method takes [`sramprcr::W`](W) writer structure
impl crate::Writable for SramprcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRAMPRCR to value 0
impl crate::Resettable for SramprcrSpec {}
