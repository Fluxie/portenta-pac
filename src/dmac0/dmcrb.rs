///Register `DMCRB` reader
pub type R = crate::R<DmcrbSpec>;
///Register `DMCRB` writer
pub type W = crate::W<DmcrbSpec>;
///Field `DMCRBL` reader - Functions as a number of block, repeat or repeat-block transfer counter.
pub type DmcrblR = crate::FieldReader<u16>;
///Field `DMCRBL` writer - Functions as a number of block, repeat or repeat-block transfer counter.
pub type DmcrblW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DMCRBH` reader - Specifies the number of block, repeat or repeat-block transfer operations.
pub type DmcrbhR = crate::FieldReader<u16>;
///Field `DMCRBH` writer - Specifies the number of block, repeat or repeat-block transfer operations.
pub type DmcrbhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Functions as a number of block, repeat or repeat-block transfer counter.
    #[inline(always)]
    pub fn dmcrbl(&self) -> DmcrblR {
        DmcrblR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Specifies the number of block, repeat or repeat-block transfer operations.
    #[inline(always)]
    pub fn dmcrbh(&self) -> DmcrbhR {
        DmcrbhR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMCRB")
            .field("dmcrbl", &self.dmcrbl())
            .field("dmcrbh", &self.dmcrbh())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Functions as a number of block, repeat or repeat-block transfer counter.
    #[inline(always)]
    pub fn dmcrbl(&mut self) -> DmcrblW<DmcrbSpec> {
        DmcrblW::new(self, 0)
    }
    ///Bits 16:31 - Specifies the number of block, repeat or repeat-block transfer operations.
    #[inline(always)]
    pub fn dmcrbh(&mut self) -> DmcrbhW<DmcrbSpec> {
        DmcrbhW::new(self, 16)
    }
}
/**DMA Block Transfer Count Register

You can [`read`](crate::Reg::read) this register and get [`dmcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmcrbSpec;
impl crate::RegisterSpec for DmcrbSpec {
    type Ux = u32;
}
///`read()` method returns [`dmcrb::R`](R) reader structure
impl crate::Readable for DmcrbSpec {}
///`write(|w| ..)` method takes [`dmcrb::W`](W) writer structure
impl crate::Writable for DmcrbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMCRB to value 0
impl crate::Resettable for DmcrbSpec {}
