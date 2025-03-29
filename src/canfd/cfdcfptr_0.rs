///Register `CFDCFPTR%s_0` reader
pub type R = crate::R<Cfdcfptr0Spec>;
///Register `CFDCFPTR%s_0` writer
pub type W = crate::W<Cfdcfptr0Spec>;
///Field `CFTS` reader - Common FIFO Timestamp Value
pub type CftsR = crate::FieldReader<u16>;
///Field `CFTS` writer - Common FIFO Timestamp Value
pub type CftsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CFDLC` reader - Common FIFO Buffer DLC Field
pub type CfdlcR = crate::FieldReader;
///Field `CFDLC` writer - Common FIFO Buffer DLC Field
pub type CfdlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:15 - Common FIFO Timestamp Value
    #[inline(always)]
    pub fn cfts(&self) -> CftsR {
        CftsR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 28:31 - Common FIFO Buffer DLC Field
    #[inline(always)]
    pub fn cfdlc(&self) -> CfdlcR {
        CfdlcR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFPTR_0")
            .field("cfts", &self.cfts())
            .field("cfdlc", &self.cfdlc())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Common FIFO Timestamp Value
    #[inline(always)]
    pub fn cfts(&mut self) -> CftsW<Cfdcfptr0Spec> {
        CftsW::new(self, 0)
    }
    ///Bits 28:31 - Common FIFO Buffer DLC Field
    #[inline(always)]
    pub fn cfdlc(&mut self) -> CfdlcW<Cfdcfptr0Spec> {
        CfdlcW::new(self, 28)
    }
}
/**Common FIFO Access Pointer Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfptr_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfptr_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdcfptr0Spec;
impl crate::RegisterSpec for Cfdcfptr0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfptr_0::R`](R) reader structure
impl crate::Readable for Cfdcfptr0Spec {}
///`write(|w| ..)` method takes [`cfdcfptr_0::W`](W) writer structure
impl crate::Writable for Cfdcfptr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCFPTR%s_0 to value 0
impl crate::Resettable for Cfdcfptr0Spec {}
