///Register `CFDCFDF9%s_0` reader
pub type R = crate::R<Cfdcfdf9_0Spec>;
///Register `CFDCFDF9%s_0` writer
pub type W = crate::W<Cfdcfdf9_0Spec>;
///Field `CFDB_LL` reader - Common FIFO Buffer Data Bytes 36
pub type CfdbLlR = crate::FieldReader;
///Field `CFDB_LL` writer - Common FIFO Buffer Data Bytes 36
pub type CfdbLlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CFDB_LH` reader - Common FIFO Buffer Data Bytes 37
pub type CfdbLhR = crate::FieldReader;
///Field `CFDB_LH` writer - Common FIFO Buffer Data Bytes 37
pub type CfdbLhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CFDB_HL` reader - Common FIFO Buffer Data Bytes 38
pub type CfdbHlR = crate::FieldReader;
///Field `CFDB_HL` writer - Common FIFO Buffer Data Bytes 38
pub type CfdbHlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CFDB_HH` reader - Common FIFO Buffer Data Bytes 39
pub type CfdbHhR = crate::FieldReader;
///Field `CFDB_HH` writer - Common FIFO Buffer Data Bytes 39
pub type CfdbHhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Common FIFO Buffer Data Bytes 36
    #[inline(always)]
    pub fn cfdb_ll(&self) -> CfdbLlR {
        CfdbLlR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Common FIFO Buffer Data Bytes 37
    #[inline(always)]
    pub fn cfdb_lh(&self) -> CfdbLhR {
        CfdbLhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Common FIFO Buffer Data Bytes 38
    #[inline(always)]
    pub fn cfdb_hl(&self) -> CfdbHlR {
        CfdbHlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Common FIFO Buffer Data Bytes 39
    #[inline(always)]
    pub fn cfdb_hh(&self) -> CfdbHhR {
        CfdbHhR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFDF9_0")
            .field("cfdb_ll", &self.cfdb_ll())
            .field("cfdb_lh", &self.cfdb_lh())
            .field("cfdb_hl", &self.cfdb_hl())
            .field("cfdb_hh", &self.cfdb_hh())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Common FIFO Buffer Data Bytes 36
    #[inline(always)]
    pub fn cfdb_ll(&mut self) -> CfdbLlW<Cfdcfdf9_0Spec> {
        CfdbLlW::new(self, 0)
    }
    ///Bits 8:15 - Common FIFO Buffer Data Bytes 37
    #[inline(always)]
    pub fn cfdb_lh(&mut self) -> CfdbLhW<Cfdcfdf9_0Spec> {
        CfdbLhW::new(self, 8)
    }
    ///Bits 16:23 - Common FIFO Buffer Data Bytes 38
    #[inline(always)]
    pub fn cfdb_hl(&mut self) -> CfdbHlW<Cfdcfdf9_0Spec> {
        CfdbHlW::new(self, 16)
    }
    ///Bits 24:31 - Common FIFO Buffer Data Bytes 39
    #[inline(always)]
    pub fn cfdb_hh(&mut self) -> CfdbHhW<Cfdcfdf9_0Spec> {
        CfdbHhW::new(self, 24)
    }
}
/**Common FIFO Access Data Field 9 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdcfdf9_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdf9_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdcfdf9_0Spec;
impl crate::RegisterSpec for Cfdcfdf9_0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfdf9_0::R`](R) reader structure
impl crate::Readable for Cfdcfdf9_0Spec {}
///`write(|w| ..)` method takes [`cfdcfdf9_0::W`](W) writer structure
impl crate::Writable for Cfdcfdf9_0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCFDF9%s_0 to value 0
impl crate::Resettable for Cfdcfdf9_0Spec {}
