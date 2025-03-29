///Register `DCR` reader
pub type R = crate::R<DcrSpec>;
///Register `DCR` writer
pub type W = crate::W<DcrSpec>;
///Field `DVCMD0` reader - Device Command data
pub type Dvcmd0R = crate::FieldReader;
///Field `DVCMD0` writer - Device Command data
pub type Dvcmd0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DVCMD1` reader - Device Command data
pub type Dvcmd1R = crate::FieldReader;
///Field `DVCMD1` writer - Device Command data
pub type Dvcmd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Device Command data
    #[inline(always)]
    pub fn dvcmd0(&self) -> Dvcmd0R {
        Dvcmd0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Device Command data
    #[inline(always)]
    pub fn dvcmd1(&self) -> Dvcmd1R {
        Dvcmd1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR")
            .field("dvcmd0", &self.dvcmd0())
            .field("dvcmd1", &self.dvcmd1())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Device Command data
    #[inline(always)]
    pub fn dvcmd0(&mut self) -> Dvcmd0W<DcrSpec> {
        Dvcmd0W::new(self, 0)
    }
    ///Bits 8:15 - Device Command data
    #[inline(always)]
    pub fn dvcmd1(&mut self) -> Dvcmd1W<DcrSpec> {
        Dvcmd1W::new(self, 8)
    }
}
/**Device Command Register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcrSpec;
impl crate::RegisterSpec for DcrSpec {
    type Ux = u32;
}
///`read()` method returns [`dcr::R`](R) reader structure
impl crate::Readable for DcrSpec {}
///`write(|w| ..)` method takes [`dcr::W`](W) writer structure
impl crate::Writable for DcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCR to value 0
impl crate::Resettable for DcrSpec {}
