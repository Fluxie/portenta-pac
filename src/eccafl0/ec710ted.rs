///Register `EC710TED` reader
pub type R = crate::R<Ec710tedSpec>;
///Register `EC710TED` writer
pub type W = crate::W<Ec710tedSpec>;
///Field `ECEDB` reader - ECC Test Substitute Data
pub type EcedbR = crate::FieldReader<u32>;
///Field `ECEDB` writer - ECC Test Substitute Data
pub type EcedbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ECC Test Substitute Data
    #[inline(always)]
    pub fn ecedb(&self) -> EcedbR {
        EcedbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EC710TED").field("ecedb", &self.ecedb()).finish()
    }
}
impl W {
    ///Bits 0:31 - ECC Test Substitute Data
    #[inline(always)]
    pub fn ecedb(&mut self) -> EcedbW<Ec710tedSpec> {
        EcedbW::new(self, 0)
    }
}
/**ECC Test Substitute Data Register

You can [`read`](crate::Reg::read) this register and get [`ec710ted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ec710ted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ec710tedSpec;
impl crate::RegisterSpec for Ec710tedSpec {
    type Ux = u32;
}
///`read()` method returns [`ec710ted::R`](R) reader structure
impl crate::Readable for Ec710tedSpec {}
///`write(|w| ..)` method takes [`ec710ted::W`](W) writer structure
impl crate::Writable for Ec710tedSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EC710TED to value 0
impl crate::Resettable for Ec710tedSpec {}
