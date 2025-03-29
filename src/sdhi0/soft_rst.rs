///Register `SOFT_RST` reader
pub type R = crate::R<SoftRstSpec>;
///Register `SOFT_RST` writer
pub type W = crate::W<SoftRstSpec>;
/**Software Reset Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdrst {
    ///0: Reset SD/MMC Host Interface software
    _0 = 0,
    ///1: Cancel reset of SD/MMC Host Interface software
    _1 = 1,
}
impl From<Sdrst> for bool {
    #[inline(always)]
    fn from(variant: Sdrst) -> Self {
        variant as u8 != 0
    }
}
///Field `SDRST` reader - Software Reset Control
pub type SdrstR = crate::BitReader<Sdrst>;
impl SdrstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdrst {
        match self.bits {
            false => Sdrst::_0,
            true => Sdrst::_1,
        }
    }
    ///Reset SD/MMC Host Interface software
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdrst::_0
    }
    ///Cancel reset of SD/MMC Host Interface software
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdrst::_1
    }
}
///Field `SDRST` writer - Software Reset Control
pub type SdrstW<'a, REG> = crate::BitWriter<'a, REG, Sdrst>;
impl<'a, REG> SdrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset SD/MMC Host Interface software
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdrst::_0)
    }
    ///Cancel reset of SD/MMC Host Interface software
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdrst::_1)
    }
}
impl R {
    ///Bit 0 - Software Reset Control
    #[inline(always)]
    pub fn sdrst(&self) -> SdrstR {
        SdrstR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOFT_RST").field("sdrst", &self.sdrst()).finish()
    }
}
impl W {
    ///Bit 0 - Software Reset Control
    #[inline(always)]
    pub fn sdrst(&mut self) -> SdrstW<SoftRstSpec> {
        SdrstW::new(self, 0)
    }
}
/**Software Reset Register

You can [`read`](crate::Reg::read) this register and get [`soft_rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SoftRstSpec;
impl crate::RegisterSpec for SoftRstSpec {
    type Ux = u32;
}
///`read()` method returns [`soft_rst::R`](R) reader structure
impl crate::Readable for SoftRstSpec {}
///`write(|w| ..)` method takes [`soft_rst::W`](W) writer structure
impl crate::Writable for SoftRstSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOFT_RST to value 0x07
impl crate::Resettable for SoftRstSpec {
    const RESET_VALUE: u32 = 0x07;
}
