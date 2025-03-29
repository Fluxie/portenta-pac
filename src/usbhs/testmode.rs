///Register `TESTMODE` reader
pub type R = crate::R<TestmodeSpec>;
///Register `TESTMODE` writer
pub type W = crate::W<TestmodeSpec>;
///Field `UTST` reader - Test Mode
pub type UtstR = crate::FieldReader;
///Field `UTST` writer - Test Mode
pub type UtstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Test Mode
    #[inline(always)]
    pub fn utst(&self) -> UtstR {
        UtstR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TESTMODE").field("utst", &self.utst()).finish()
    }
}
impl W {
    ///Bits 0:3 - Test Mode
    #[inline(always)]
    pub fn utst(&mut self) -> UtstW<TestmodeSpec> {
        UtstW::new(self, 0)
    }
}
/**USB Test Mode Register

You can [`read`](crate::Reg::read) this register and get [`testmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TestmodeSpec;
impl crate::RegisterSpec for TestmodeSpec {
    type Ux = u16;
}
///`read()` method returns [`testmode::R`](R) reader structure
impl crate::Readable for TestmodeSpec {}
///`write(|w| ..)` method takes [`testmode::W`](W) writer structure
impl crate::Writable for TestmodeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TESTMODE to value 0
impl crate::Resettable for TestmodeSpec {}
