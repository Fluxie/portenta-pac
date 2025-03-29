///Register `PIPE%sTRN` reader
pub type R = crate::R<PipetrnSpec>;
///Register `PIPE%sTRN` writer
pub type W = crate::W<PipetrnSpec>;
///Field `TRNCNT` reader - Transaction Counter
pub type TrncntR = crate::FieldReader<u16>;
///Field `TRNCNT` writer - Transaction Counter
pub type TrncntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Transaction Counter
    #[inline(always)]
    pub fn trncnt(&self) -> TrncntR {
        TrncntR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPETRN").field("trncnt", &self.trncnt()).finish()
    }
}
impl W {
    ///Bits 0:15 - Transaction Counter
    #[inline(always)]
    pub fn trncnt(&mut self) -> TrncntW<PipetrnSpec> {
        TrncntW::new(self, 0)
    }
}
/**PIPE%s Transaction Counter Register

You can [`read`](crate::Reg::read) this register and get [`pipetrn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetrn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PipetrnSpec;
impl crate::RegisterSpec for PipetrnSpec {
    type Ux = u16;
}
///`read()` method returns [`pipetrn::R`](R) reader structure
impl crate::Readable for PipetrnSpec {}
///`write(|w| ..)` method takes [`pipetrn::W`](W) writer structure
impl crate::Writable for PipetrnSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPE%sTRN to value 0
impl crate::Resettable for PipetrnSpec {}
