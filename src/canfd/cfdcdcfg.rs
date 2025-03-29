///Register `CFDC%sDCFG` reader
pub type R = crate::R<CfdcdcfgSpec>;
///Register `CFDC%sDCFG` writer
pub type W = crate::W<CfdcdcfgSpec>;
///Field `DBRP` reader - Channel Data Baud Rate Prescaler
pub type DbrpR = crate::FieldReader;
///Field `DBRP` writer - Channel Data Baud Rate Prescaler
pub type DbrpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DTSEG1` reader - Timing Segment 1
pub type Dtseg1R = crate::FieldReader;
///Field `DTSEG1` writer - Timing Segment 1
pub type Dtseg1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DTSEG2` reader - Timing Segment 2
pub type Dtseg2R = crate::FieldReader;
///Field `DTSEG2` writer - Timing Segment 2
pub type Dtseg2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DSJW` reader - Resynchronization Jump Width
pub type DsjwR = crate::FieldReader;
///Field `DSJW` writer - Resynchronization Jump Width
pub type DsjwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Channel Data Baud Rate Prescaler
    #[inline(always)]
    pub fn dbrp(&self) -> DbrpR {
        DbrpR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:12 - Timing Segment 1
    #[inline(always)]
    pub fn dtseg1(&self) -> Dtseg1R {
        Dtseg1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:19 - Timing Segment 2
    #[inline(always)]
    pub fn dtseg2(&self) -> Dtseg2R {
        Dtseg2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - Resynchronization Jump Width
    #[inline(always)]
    pub fn dsjw(&self) -> DsjwR {
        DsjwR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCDCFG")
            .field("dbrp", &self.dbrp())
            .field("dtseg1", &self.dtseg1())
            .field("dtseg2", &self.dtseg2())
            .field("dsjw", &self.dsjw())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Channel Data Baud Rate Prescaler
    #[inline(always)]
    pub fn dbrp(&mut self) -> DbrpW<CfdcdcfgSpec> {
        DbrpW::new(self, 0)
    }
    ///Bits 8:12 - Timing Segment 1
    #[inline(always)]
    pub fn dtseg1(&mut self) -> Dtseg1W<CfdcdcfgSpec> {
        Dtseg1W::new(self, 8)
    }
    ///Bits 16:19 - Timing Segment 2
    #[inline(always)]
    pub fn dtseg2(&mut self) -> Dtseg2W<CfdcdcfgSpec> {
        Dtseg2W::new(self, 16)
    }
    ///Bits 24:27 - Resynchronization Jump Width
    #[inline(always)]
    pub fn dsjw(&mut self) -> DsjwW<CfdcdcfgSpec> {
        DsjwW::new(self, 24)
    }
}
/**Channel %s Data Bitrate Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdcdcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcdcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcdcfgSpec;
impl crate::RegisterSpec for CfdcdcfgSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcdcfg::R`](R) reader structure
impl crate::Readable for CfdcdcfgSpec {}
///`write(|w| ..)` method takes [`cfdcdcfg::W`](W) writer structure
impl crate::Writable for CfdcdcfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDC%sDCFG to value 0
impl crate::Resettable for CfdcdcfgSpec {}
