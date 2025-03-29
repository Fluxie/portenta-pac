///Register `BUSWAIT` reader
pub type R = crate::R<BuswaitSpec>;
///Register `BUSWAIT` writer
pub type W = crate::W<BuswaitSpec>;
///Field `BWAIT` reader - CPU Bus Access Wait Specification
pub type BwaitR = crate::FieldReader;
///Field `BWAIT` writer - CPU Bus Access Wait Specification
pub type BwaitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - CPU Bus Access Wait Specification
    #[inline(always)]
    pub fn bwait(&self) -> BwaitR {
        BwaitR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSWAIT").field("bwait", &self.bwait()).finish()
    }
}
impl W {
    ///Bits 0:3 - CPU Bus Access Wait Specification
    #[inline(always)]
    pub fn bwait(&mut self) -> BwaitW<BuswaitSpec> {
        BwaitW::new(self, 0)
    }
}
/**CPU Bus Wait Register

You can [`read`](crate::Reg::read) this register and get [`buswait::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buswait::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BuswaitSpec;
impl crate::RegisterSpec for BuswaitSpec {
    type Ux = u16;
}
///`read()` method returns [`buswait::R`](R) reader structure
impl crate::Readable for BuswaitSpec {}
///`write(|w| ..)` method takes [`buswait::W`](W) writer structure
impl crate::Writable for BuswaitSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSWAIT to value 0x0f
impl crate::Resettable for BuswaitSpec {
    const RESET_VALUE: u16 = 0x0f;
}
