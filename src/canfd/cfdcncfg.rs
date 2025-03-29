///Register `CFDC%sNCFG` reader
pub type R = crate::R<CfdcncfgSpec>;
///Register `CFDC%sNCFG` writer
pub type W = crate::W<CfdcncfgSpec>;
///Field `NBRP` reader - Channel Nominal Baud Rate Prescaler
pub type NbrpR = crate::FieldReader<u16>;
///Field `NBRP` writer - Channel Nominal Baud Rate Prescaler
pub type NbrpW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `NSJW` reader - Resynchronization Jump Width
pub type NsjwR = crate::FieldReader;
///Field `NSJW` writer - Resynchronization Jump Width
pub type NsjwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `NTSEG1` reader - Timing Segment 1
pub type Ntseg1R = crate::FieldReader;
///Field `NTSEG1` writer - Timing Segment 1
pub type Ntseg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `NTSEG2` reader - Timing Segment 2
pub type Ntseg2R = crate::FieldReader;
///Field `NTSEG2` writer - Timing Segment 2
pub type Ntseg2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:9 - Channel Nominal Baud Rate Prescaler
    #[inline(always)]
    pub fn nbrp(&self) -> NbrpR {
        NbrpR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:16 - Resynchronization Jump Width
    #[inline(always)]
    pub fn nsjw(&self) -> NsjwR {
        NsjwR::new(((self.bits >> 10) & 0x7f) as u8)
    }
    ///Bits 17:24 - Timing Segment 1
    #[inline(always)]
    pub fn ntseg1(&self) -> Ntseg1R {
        Ntseg1R::new(((self.bits >> 17) & 0xff) as u8)
    }
    ///Bits 25:31 - Timing Segment 2
    #[inline(always)]
    pub fn ntseg2(&self) -> Ntseg2R {
        Ntseg2R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCNCFG")
            .field("nbrp", &self.nbrp())
            .field("nsjw", &self.nsjw())
            .field("ntseg1", &self.ntseg1())
            .field("ntseg2", &self.ntseg2())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Channel Nominal Baud Rate Prescaler
    #[inline(always)]
    pub fn nbrp(&mut self) -> NbrpW<CfdcncfgSpec> {
        NbrpW::new(self, 0)
    }
    ///Bits 10:16 - Resynchronization Jump Width
    #[inline(always)]
    pub fn nsjw(&mut self) -> NsjwW<CfdcncfgSpec> {
        NsjwW::new(self, 10)
    }
    ///Bits 17:24 - Timing Segment 1
    #[inline(always)]
    pub fn ntseg1(&mut self) -> Ntseg1W<CfdcncfgSpec> {
        Ntseg1W::new(self, 17)
    }
    ///Bits 25:31 - Timing Segment 2
    #[inline(always)]
    pub fn ntseg2(&mut self) -> Ntseg2W<CfdcncfgSpec> {
        Ntseg2W::new(self, 25)
    }
}
/**Channel %s Nominal Bitrate Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdcncfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcncfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcncfgSpec;
impl crate::RegisterSpec for CfdcncfgSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcncfg::R`](R) reader structure
impl crate::Readable for CfdcncfgSpec {}
///`write(|w| ..)` method takes [`cfdcncfg::W`](W) writer structure
impl crate::Writable for CfdcncfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDC%sNCFG to value 0
impl crate::Resettable for CfdcncfgSpec {}
