///Register `CS%sWCR1` reader
pub type R = crate::R<Cswcr1Spec>;
///Register `CS%sWCR1` writer
pub type W = crate::W<Cswcr1Spec>;
///Field `CSPWWAIT` reader - Page Write Cycle Wait Select
pub type CspwwaitR = crate::FieldReader;
///Field `CSPWWAIT` writer - Page Write Cycle Wait Select
pub type CspwwaitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CSPRWAIT` reader - Page Read Cycle Wait Select
pub type CsprwaitR = crate::FieldReader;
///Field `CSPRWAIT` writer - Page Read Cycle Wait Select
pub type CsprwaitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CSWWAIT` reader - Normal Write Cycle Wait Select
pub type CswwaitR = crate::FieldReader;
///Field `CSWWAIT` writer - Normal Write Cycle Wait Select
pub type CswwaitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CSRWAIT` reader - Normal Read Cycle Wait Select
pub type CsrwaitR = crate::FieldReader;
///Field `CSRWAIT` writer - Normal Read Cycle Wait Select
pub type CsrwaitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:2 - Page Write Cycle Wait Select
    #[inline(always)]
    pub fn cspwwait(&self) -> CspwwaitR {
        CspwwaitR::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - Page Read Cycle Wait Select
    #[inline(always)]
    pub fn csprwait(&self) -> CsprwaitR {
        CsprwaitR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:20 - Normal Write Cycle Wait Select
    #[inline(always)]
    pub fn cswwait(&self) -> CswwaitR {
        CswwaitR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28 - Normal Read Cycle Wait Select
    #[inline(always)]
    pub fn csrwait(&self) -> CsrwaitR {
        CsrwaitR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSWCR1")
            .field("cspwwait", &self.cspwwait())
            .field("csprwait", &self.csprwait())
            .field("cswwait", &self.cswwait())
            .field("csrwait", &self.csrwait())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Page Write Cycle Wait Select
    #[inline(always)]
    pub fn cspwwait(&mut self) -> CspwwaitW<Cswcr1Spec> {
        CspwwaitW::new(self, 0)
    }
    ///Bits 8:10 - Page Read Cycle Wait Select
    #[inline(always)]
    pub fn csprwait(&mut self) -> CsprwaitW<Cswcr1Spec> {
        CsprwaitW::new(self, 8)
    }
    ///Bits 16:20 - Normal Write Cycle Wait Select
    #[inline(always)]
    pub fn cswwait(&mut self) -> CswwaitW<Cswcr1Spec> {
        CswwaitW::new(self, 16)
    }
    ///Bits 24:28 - Normal Read Cycle Wait Select
    #[inline(always)]
    pub fn csrwait(&mut self) -> CsrwaitW<Cswcr1Spec> {
        CsrwaitW::new(self, 24)
    }
}
/**CS%s Wait Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cswcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cswcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cswcr1Spec;
impl crate::RegisterSpec for Cswcr1Spec {
    type Ux = u32;
}
///`read()` method returns [`cswcr1::R`](R) reader structure
impl crate::Readable for Cswcr1Spec {}
///`write(|w| ..)` method takes [`cswcr1::W`](W) writer structure
impl crate::Writable for Cswcr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CS%sWCR1 to value 0x0707_0707
impl crate::Resettable for Cswcr1Spec {
    const RESET_VALUE: u32 = 0x0707_0707;
}
