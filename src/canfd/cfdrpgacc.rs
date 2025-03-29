///Register `CFDRPGACC%s` reader
pub type R = crate::R<CfdrpgaccSpec>;
///Register `CFDRPGACC%s` writer
pub type W = crate::W<CfdrpgaccSpec>;
///Field `RDTA` reader - RAM Data Test Access
pub type RdtaR = crate::FieldReader<u32>;
///Field `RDTA` writer - RAM Data Test Access
pub type RdtaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - RAM Data Test Access
    #[inline(always)]
    pub fn rdta(&self) -> RdtaR {
        RdtaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRPGACC").field("rdta", &self.rdta()).finish()
    }
}
impl W {
    ///Bits 0:31 - RAM Data Test Access
    #[inline(always)]
    pub fn rdta(&mut self) -> RdtaW<CfdrpgaccSpec> {
        RdtaW::new(self, 0)
    }
}
/**RAM Test Page Access Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdrpgacc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrpgacc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdrpgaccSpec;
impl crate::RegisterSpec for CfdrpgaccSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdrpgacc::R`](R) reader structure
impl crate::Readable for CfdrpgaccSpec {}
///`write(|w| ..)` method takes [`cfdrpgacc::W`](W) writer structure
impl crate::Writable for CfdrpgaccSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDRPGACC%s to value 0
impl crate::Resettable for CfdrpgaccSpec {}
