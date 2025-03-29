///Register `MDTR` reader
pub type R = crate::R<MdtrSpec>;
///Register `MDTR` writer
pub type W = crate::W<MdtrSpec>;
///Field `DV0DEL` reader - Device 0 delay setting
pub type Dv0delR = crate::FieldReader;
///Field `DV0DEL` writer - Device 0 delay setting
pub type Dv0delW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DQSERAM` reader - OM_DQS enable counter
pub type DqseramR = crate::FieldReader;
///Field `DQSERAM` writer - OM_DQS enable counter
pub type DqseramW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DQSESOPI` reader - OM_DQS enable counter
pub type DqsesopiR = crate::FieldReader;
///Field `DQSESOPI` writer - OM_DQS enable counter
pub type DqsesopiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DV1DEL` reader - Device 1 delay setting
pub type Dv1delR = crate::FieldReader;
///Field `DV1DEL` writer - Device 1 delay setting
pub type Dv1delW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DQSEDOPI` reader - OM_DQS enable counter
pub type DqsedopiR = crate::FieldReader;
///Field `DQSEDOPI` writer - OM_DQS enable counter
pub type DqsedopiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Device 0 delay setting
    #[inline(always)]
    pub fn dv0del(&self) -> Dv0delR {
        Dv0delR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - OM_DQS enable counter
    #[inline(always)]
    pub fn dqseram(&self) -> DqseramR {
        DqseramR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - OM_DQS enable counter
    #[inline(always)]
    pub fn dqsesopi(&self) -> DqsesopiR {
        DqsesopiR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:23 - Device 1 delay setting
    #[inline(always)]
    pub fn dv1del(&self) -> Dv1delR {
        Dv1delR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - OM_DQS enable counter
    #[inline(always)]
    pub fn dqsedopi(&self) -> DqsedopiR {
        DqsedopiR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDTR")
            .field("dv0del", &self.dv0del())
            .field("dqseram", &self.dqseram())
            .field("dqsesopi", &self.dqsesopi())
            .field("dv1del", &self.dv1del())
            .field("dqsedopi", &self.dqsedopi())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Device 0 delay setting
    #[inline(always)]
    pub fn dv0del(&mut self) -> Dv0delW<MdtrSpec> {
        Dv0delW::new(self, 0)
    }
    ///Bits 8:11 - OM_DQS enable counter
    #[inline(always)]
    pub fn dqseram(&mut self) -> DqseramW<MdtrSpec> {
        DqseramW::new(self, 8)
    }
    ///Bits 12:15 - OM_DQS enable counter
    #[inline(always)]
    pub fn dqsesopi(&mut self) -> DqsesopiW<MdtrSpec> {
        DqsesopiW::new(self, 12)
    }
    ///Bits 16:23 - Device 1 delay setting
    #[inline(always)]
    pub fn dv1del(&mut self) -> Dv1delW<MdtrSpec> {
        Dv1delW::new(self, 16)
    }
    ///Bits 24:27 - OM_DQS enable counter
    #[inline(always)]
    pub fn dqsedopi(&mut self) -> DqsedopiW<MdtrSpec> {
        DqsedopiW::new(self, 24)
    }
}
/**Memory Delay Trim Register

You can [`read`](crate::Reg::read) this register and get [`mdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MdtrSpec;
impl crate::RegisterSpec for MdtrSpec {
    type Ux = u32;
}
///`read()` method returns [`mdtr::R`](R) reader structure
impl crate::Readable for MdtrSpec {}
///`write(|w| ..)` method takes [`mdtr::W`](W) writer structure
impl crate::Writable for MdtrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MDTR to value 0x0600_9400
impl crate::Resettable for MdtrSpec {
    const RESET_VALUE: u32 = 0x0600_9400;
}
