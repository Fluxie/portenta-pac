///Register `MMPUSDMAC%s` reader
pub type R = crate::R<MmpusdmacSpec>;
///Register `MMPUSDMAC%s` writer
pub type W = crate::W<MmpusdmacSpec>;
///Field `MMPUS` reader - Region start address register
pub type MmpusR = crate::FieldReader<u32>;
///Field `MMPUS` writer - Region start address register
pub type MmpusW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bits 5:31 - Region start address register
    #[inline(always)]
    pub fn mmpus(&self) -> MmpusR {
        MmpusR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUSDMAC").field("mmpus", &self.mmpus()).finish()
    }
}
impl W {
    ///Bits 5:31 - Region start address register
    #[inline(always)]
    pub fn mmpus(&mut self) -> MmpusW<MmpusdmacSpec> {
        MmpusW::new(self, 5)
    }
}
/**MMPU Start Address Register for DMAC

You can [`read`](crate::Reg::read) this register and get [`mmpusdmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusdmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmpusdmacSpec;
impl crate::RegisterSpec for MmpusdmacSpec {
    type Ux = u32;
}
///`read()` method returns [`mmpusdmac::R`](R) reader structure
impl crate::Readable for MmpusdmacSpec {}
///`write(|w| ..)` method takes [`mmpusdmac::W`](W) writer structure
impl crate::Writable for MmpusdmacSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUSDMAC%s to value 0
impl crate::Resettable for MmpusdmacSpec {}
