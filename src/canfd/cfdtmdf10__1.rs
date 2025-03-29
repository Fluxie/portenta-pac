///Register `CFDTMDF10_%s_1` reader
pub type R = crate::R<Cfdtmdf10_1Spec>;
///Register `CFDTMDF10_%s_1` writer
pub type W = crate::W<Cfdtmdf10_1Spec>;
///Field `TMDB_LL` reader - TX Message Buffer Data Byte 40
pub type TmdbLlR = crate::FieldReader;
///Field `TMDB_LL` writer - TX Message Buffer Data Byte 40
pub type TmdbLlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TMDB_LH` reader - TX Message Buffer Data Byte 41
pub type TmdbLhR = crate::FieldReader;
///Field `TMDB_LH` writer - TX Message Buffer Data Byte 41
pub type TmdbLhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TMDB_HL` reader - TX Message Buffer Data Byte 42
pub type TmdbHlR = crate::FieldReader;
///Field `TMDB_HL` writer - TX Message Buffer Data Byte 42
pub type TmdbHlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TMDB_HH` reader - TX Message Buffer Data Byte 43
pub type TmdbHhR = crate::FieldReader;
///Field `TMDB_HH` writer - TX Message Buffer Data Byte 43
pub type TmdbHhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - TX Message Buffer Data Byte 40
    #[inline(always)]
    pub fn tmdb_ll(&self) -> TmdbLlR {
        TmdbLlR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - TX Message Buffer Data Byte 41
    #[inline(always)]
    pub fn tmdb_lh(&self) -> TmdbLhR {
        TmdbLhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - TX Message Buffer Data Byte 42
    #[inline(always)]
    pub fn tmdb_hl(&self) -> TmdbHlR {
        TmdbHlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - TX Message Buffer Data Byte 43
    #[inline(always)]
    pub fn tmdb_hh(&self) -> TmdbHhR {
        TmdbHhR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTMDF10__1")
            .field("tmdb_ll", &self.tmdb_ll())
            .field("tmdb_lh", &self.tmdb_lh())
            .field("tmdb_hl", &self.tmdb_hl())
            .field("tmdb_hh", &self.tmdb_hh())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - TX Message Buffer Data Byte 40
    #[inline(always)]
    pub fn tmdb_ll(&mut self) -> TmdbLlW<Cfdtmdf10_1Spec> {
        TmdbLlW::new(self, 0)
    }
    ///Bits 8:15 - TX Message Buffer Data Byte 41
    #[inline(always)]
    pub fn tmdb_lh(&mut self) -> TmdbLhW<Cfdtmdf10_1Spec> {
        TmdbLhW::new(self, 8)
    }
    ///Bits 16:23 - TX Message Buffer Data Byte 42
    #[inline(always)]
    pub fn tmdb_hl(&mut self) -> TmdbHlW<Cfdtmdf10_1Spec> {
        TmdbHlW::new(self, 16)
    }
    ///Bits 24:31 - TX Message Buffer Data Byte 43
    #[inline(always)]
    pub fn tmdb_hh(&mut self) -> TmdbHhW<Cfdtmdf10_1Spec> {
        TmdbHhW::new(self, 24)
    }
}
/**TX Message Buffer Data Field 10 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdtmdf10__1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmdf10__1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdtmdf10_1Spec;
impl crate::RegisterSpec for Cfdtmdf10_1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdtmdf10__1::R`](R) reader structure
impl crate::Readable for Cfdtmdf10_1Spec {}
///`write(|w| ..)` method takes [`cfdtmdf10__1::W`](W) writer structure
impl crate::Writable for Cfdtmdf10_1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTMDF10_%s_1 to value 0
impl crate::Resettable for Cfdtmdf10_1Spec {}
