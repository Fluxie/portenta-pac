///Register `MMPUEDMAC%s` reader
pub type R = crate::R<MmpuedmacSpec>;
///Register `MMPUEDMAC%s` writer
pub type W = crate::W<MmpuedmacSpec>;
///Field `MMPUE` reader - Region end address register
pub type MmpueR = crate::FieldReader<u32>;
///Field `MMPUE` writer - Region end address register
pub type MmpueW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bits 5:31 - Region end address register
    #[inline(always)]
    pub fn mmpue(&self) -> MmpueR {
        MmpueR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUEDMAC").field("mmpue", &self.mmpue()).finish()
    }
}
impl W {
    ///Bits 5:31 - Region end address register
    #[inline(always)]
    pub fn mmpue(&mut self) -> MmpueW<MmpuedmacSpec> {
        MmpueW::new(self, 5)
    }
}
/**MMPU End Address Register for DMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuedmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuedmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmpuedmacSpec;
impl crate::RegisterSpec for MmpuedmacSpec {
    type Ux = u32;
}
///`read()` method returns [`mmpuedmac::R`](R) reader structure
impl crate::Readable for MmpuedmacSpec {}
///`write(|w| ..)` method takes [`mmpuedmac::W`](W) writer structure
impl crate::Writable for MmpuedmacSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUEDMAC%s to value 0x1f
impl crate::Resettable for MmpuedmacSpec {
    const RESET_VALUE: u32 = 0x1f;
}
