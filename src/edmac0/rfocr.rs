///Register `RFOCR` reader
pub type R = crate::R<RfocrSpec>;
///Register `RFOCR` writer
pub type W = crate::W<RfocrSpec>;
///Field `OVER` reader - Receive FIFO Overflow Count
pub type OverR = crate::FieldReader<u16>;
///Field `OVER` writer - Receive FIFO Overflow Count
pub type OverW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Receive FIFO Overflow Count
    #[inline(always)]
    pub fn over(&self) -> OverR {
        OverR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFOCR").field("over", &self.over()).finish()
    }
}
impl W {
    ///Bits 0:15 - Receive FIFO Overflow Count
    #[inline(always)]
    pub fn over(&mut self) -> OverW<RfocrSpec> {
        OverW::new(self, 0)
    }
}
/**Receive FIFO Overflow Counter

You can [`read`](crate::Reg::read) this register and get [`rfocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RfocrSpec;
impl crate::RegisterSpec for RfocrSpec {
    type Ux = u32;
}
///`read()` method returns [`rfocr::R`](R) reader structure
impl crate::Readable for RfocrSpec {}
///`write(|w| ..)` method takes [`rfocr::W`](W) writer structure
impl crate::Writable for RfocrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFOCR to value 0
impl crate::Resettable for RfocrSpec {}
