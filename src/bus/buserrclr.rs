///Register `BUS%sERRCLR` reader
pub type R = crate::R<BuserrclrSpec>;
///Register `BUS%sERRCLR` writer
pub type W = crate::W<BuserrclrSpec>;
///Field `SLERRCLR` reader - Slave bus Error Clear
pub type SlerrclrR = crate::BitReader;
///Field `SLERRCLR` writer - Slave bus Error Clear
pub type SlerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STERRCLR` reader - Slave TrustZone filter Error Clear
pub type SterrclrR = crate::BitReader;
///Field `STERRCLR` writer - Slave TrustZone filter Error Clear
pub type SterrclrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMERRCLR` reader - Master MPU Error Clear
pub type MmerrclrR = crate::BitReader;
///Field `MMERRCLR` writer - Master MPU Error Clear
pub type MmerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ILERRCLR` reader - Illegal Address Access Error Clear
pub type IlerrclrR = crate::BitReader;
///Field `ILERRCLR` writer - Illegal Address Access Error Clear
pub type IlerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Slave bus Error Clear
    #[inline(always)]
    pub fn slerrclr(&self) -> SlerrclrR {
        SlerrclrR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Slave TrustZone filter Error Clear
    #[inline(always)]
    pub fn sterrclr(&self) -> SterrclrR {
        SterrclrR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Master MPU Error Clear
    #[inline(always)]
    pub fn mmerrclr(&self) -> MmerrclrR {
        MmerrclrR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Illegal Address Access Error Clear
    #[inline(always)]
    pub fn ilerrclr(&self) -> IlerrclrR {
        IlerrclrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSERRCLR")
            .field("slerrclr", &self.slerrclr())
            .field("sterrclr", &self.sterrclr())
            .field("mmerrclr", &self.mmerrclr())
            .field("ilerrclr", &self.ilerrclr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Slave bus Error Clear
    #[inline(always)]
    pub fn slerrclr(&mut self) -> SlerrclrW<BuserrclrSpec> {
        SlerrclrW::new(self, 0)
    }
    ///Bit 1 - Slave TrustZone filter Error Clear
    #[inline(always)]
    pub fn sterrclr(&mut self) -> SterrclrW<BuserrclrSpec> {
        SterrclrW::new(self, 1)
    }
    ///Bit 3 - Master MPU Error Clear
    #[inline(always)]
    pub fn mmerrclr(&mut self) -> MmerrclrW<BuserrclrSpec> {
        MmerrclrW::new(self, 3)
    }
    ///Bit 4 - Illegal Address Access Error Clear
    #[inline(always)]
    pub fn ilerrclr(&mut self) -> IlerrclrW<BuserrclrSpec> {
        IlerrclrW::new(self, 4)
    }
}
/**BUS Error Clear Register %s

You can [`read`](crate::Reg::read) this register and get [`buserrclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserrclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BuserrclrSpec;
impl crate::RegisterSpec for BuserrclrSpec {
    type Ux = u8;
}
///`read()` method returns [`buserrclr::R`](R) reader structure
impl crate::Readable for BuserrclrSpec {}
///`write(|w| ..)` method takes [`buserrclr::W`](W) writer structure
impl crate::Writable for BuserrclrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUS%sERRCLR to value 0
impl crate::Resettable for BuserrclrSpec {}
