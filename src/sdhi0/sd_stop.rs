///Register `SD_STOP` reader
pub type R = crate::R<SdStopSpec>;
///Register `SD_STOP` writer
pub type W = crate::W<SdStopSpec>;
///Field `STP` reader - Transfer Stop
pub type StpR = crate::BitReader;
///Field `STP` writer - Transfer Stop
pub type StpW<'a, REG> = crate::BitWriter<'a, REG>;
/**Block Count Register Value Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sec {
    ///0: Disable SD_SECCNT register value
    _0 = 0,
    ///1: Enable SD_SECCNT register value
    _1 = 1,
}
impl From<Sec> for bool {
    #[inline(always)]
    fn from(variant: Sec) -> Self {
        variant as u8 != 0
    }
}
///Field `SEC` reader - Block Count Register Value Select
pub type SecR = crate::BitReader<Sec>;
impl SecR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sec {
        match self.bits {
            false => Sec::_0,
            true => Sec::_1,
        }
    }
    ///Disable SD_SECCNT register value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sec::_0
    }
    ///Enable SD_SECCNT register value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sec::_1
    }
}
///Field `SEC` writer - Block Count Register Value Select
pub type SecW<'a, REG> = crate::BitWriter<'a, REG, Sec>;
impl<'a, REG> SecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable SD_SECCNT register value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sec::_0)
    }
    ///Enable SD_SECCNT register value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sec::_1)
    }
}
impl R {
    ///Bit 0 - Transfer Stop
    #[inline(always)]
    pub fn stp(&self) -> StpR {
        StpR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Block Count Register Value Select
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_STOP")
            .field("stp", &self.stp())
            .field("sec", &self.sec())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer Stop
    #[inline(always)]
    pub fn stp(&mut self) -> StpW<SdStopSpec> {
        StpW::new(self, 0)
    }
    ///Bit 8 - Block Count Register Value Select
    #[inline(always)]
    pub fn sec(&mut self) -> SecW<SdStopSpec> {
        SecW::new(self, 8)
    }
}
/**Data Stop Register

You can [`read`](crate::Reg::read) this register and get [`sd_stop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_stop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdStopSpec;
impl crate::RegisterSpec for SdStopSpec {
    type Ux = u32;
}
///`read()` method returns [`sd_stop::R`](R) reader structure
impl crate::Readable for SdStopSpec {}
///`write(|w| ..)` method takes [`sd_stop::W`](W) writer structure
impl crate::Writable for SdStopSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_STOP to value 0
impl crate::Resettable for SdStopSpec {}
