///Register `DMACSAR` reader
pub type R = crate::R<DmacsarSpec>;
///Register `DMACSAR` writer
pub type W = crate::W<DmacsarSpec>;
/**DMAST Security Attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmastsa {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Dmastsa> for bool {
    #[inline(always)]
    fn from(variant: Dmastsa) -> Self {
        variant as u8 != 0
    }
}
///Field `DMASTSA` reader - DMAST Security Attribution
pub type DmastsaR = crate::BitReader<Dmastsa>;
impl DmastsaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dmastsa {
        match self.bits {
            false => Dmastsa::_0,
            true => Dmastsa::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmastsa::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmastsa::_1
    }
}
///Field `DMASTSA` writer - DMAST Security Attribution
pub type DmastsaW<'a, REG> = crate::BitWriter<'a, REG, Dmastsa>;
impl<'a, REG> DmastsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmastsa::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmastsa::_1)
    }
}
impl R {
    ///Bit 0 - DMAST Security Attribution
    #[inline(always)]
    pub fn dmastsa(&self) -> DmastsaR {
        DmastsaR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACSAR").field("dmastsa", &self.dmastsa()).finish()
    }
}
impl W {
    ///Bit 0 - DMAST Security Attribution
    #[inline(always)]
    pub fn dmastsa(&mut self) -> DmastsaW<DmacsarSpec> {
        DmastsaW::new(self, 0)
    }
}
/**DMAC Controller Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`dmacsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacsarSpec;
impl crate::RegisterSpec for DmacsarSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacsar::R`](R) reader structure
impl crate::Readable for DmacsarSpec {}
///`write(|w| ..)` method takes [`dmacsar::W`](W) writer structure
impl crate::Writable for DmacsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACSAR to value 0xffff_ffff
impl crate::Resettable for DmacsarSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
