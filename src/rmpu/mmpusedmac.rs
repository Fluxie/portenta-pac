///Register `MMPUSEDMAC%s` reader
pub type R = crate::R<MmpusedmacSpec>;
///Register `MMPUSEDMAC%s` writer
pub type W = crate::W<MmpusedmacSpec>;
///Field `MMPUS` reader - Region start address register for EDMAC
pub type MmpusR = crate::FieldReader<u32>;
///Field `MMPUS` writer - Region start address register for EDMAC
pub type MmpusW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bits 5:31 - Region start address register for EDMAC
    #[inline(always)]
    pub fn mmpus(&self) -> MmpusR {
        MmpusR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUSEDMAC").field("mmpus", &self.mmpus()).finish()
    }
}
impl W {
    ///Bits 5:31 - Region start address register for EDMAC
    #[inline(always)]
    pub fn mmpus(&mut self) -> MmpusW<MmpusedmacSpec> {
        MmpusW::new(self, 5)
    }
}
/**MMPU Start Address Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpusedmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusedmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmpusedmacSpec;
impl crate::RegisterSpec for MmpusedmacSpec {
    type Ux = u32;
}
///`read()` method returns [`mmpusedmac::R`](R) reader structure
impl crate::Readable for MmpusedmacSpec {}
///`write(|w| ..)` method takes [`mmpusedmac::W`](W) writer structure
impl crate::Writable for MmpusedmacSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUSEDMAC%s to value 0
impl crate::Resettable for MmpusedmacSpec {}
