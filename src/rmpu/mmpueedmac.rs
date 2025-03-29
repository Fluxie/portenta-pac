///Register `MMPUEEDMAC%s` reader
pub type R = crate::R<MmpueedmacSpec>;
///Register `MMPUEEDMAC%s` writer
pub type W = crate::W<MmpueedmacSpec>;
///Field `MMPUE` reader - Region end address register for EDMAC
pub type MmpueR = crate::FieldReader<u32>;
///Field `MMPUE` writer - Region end address register for EDMAC
pub type MmpueW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bits 5:31 - Region end address register for EDMAC
    #[inline(always)]
    pub fn mmpue(&self) -> MmpueR {
        MmpueR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUEEDMAC").field("mmpue", &self.mmpue()).finish()
    }
}
impl W {
    ///Bits 5:31 - Region end address register for EDMAC
    #[inline(always)]
    pub fn mmpue(&mut self) -> MmpueW<MmpueedmacSpec> {
        MmpueW::new(self, 5)
    }
}
/**MMPU End Address Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpueedmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpueedmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmpueedmacSpec;
impl crate::RegisterSpec for MmpueedmacSpec {
    type Ux = u32;
}
///`read()` method returns [`mmpueedmac::R`](R) reader structure
impl crate::Readable for MmpueedmacSpec {}
///`write(|w| ..)` method takes [`mmpueedmac::W`](W) writer structure
impl crate::Writable for MmpueedmacSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUEEDMAC%s to value 0x1f
impl crate::Resettable for MmpueedmacSpec {
    const RESET_VALUE: u32 = 0x1f;
}
