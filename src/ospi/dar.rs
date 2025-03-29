///Register `DAR` reader
pub type R = crate::R<DarSpec>;
///Register `DAR` writer
pub type W = crate::W<DarSpec>;
///Field `DVAD0` reader - Device Address data 0
pub type Dvad0R = crate::FieldReader;
///Field `DVAD0` writer - Device Address data 0
pub type Dvad0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DVAD1` reader - Device Address data 1
pub type Dvad1R = crate::FieldReader;
///Field `DVAD1` writer - Device Address data 1
pub type Dvad1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DVAD2` reader - Device Address data 2
pub type Dvad2R = crate::FieldReader;
///Field `DVAD2` writer - Device Address data 2
pub type Dvad2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DVAD3` reader - Device Address data 3
pub type Dvad3R = crate::FieldReader;
///Field `DVAD3` writer - Device Address data 3
pub type Dvad3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Device Address data 0
    #[inline(always)]
    pub fn dvad0(&self) -> Dvad0R {
        Dvad0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Device Address data 1
    #[inline(always)]
    pub fn dvad1(&self) -> Dvad1R {
        Dvad1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Device Address data 2
    #[inline(always)]
    pub fn dvad2(&self) -> Dvad2R {
        Dvad2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Device Address data 3
    #[inline(always)]
    pub fn dvad3(&self) -> Dvad3R {
        Dvad3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAR")
            .field("dvad0", &self.dvad0())
            .field("dvad1", &self.dvad1())
            .field("dvad2", &self.dvad2())
            .field("dvad3", &self.dvad3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Device Address data 0
    #[inline(always)]
    pub fn dvad0(&mut self) -> Dvad0W<DarSpec> {
        Dvad0W::new(self, 0)
    }
    ///Bits 8:15 - Device Address data 1
    #[inline(always)]
    pub fn dvad1(&mut self) -> Dvad1W<DarSpec> {
        Dvad1W::new(self, 8)
    }
    ///Bits 16:23 - Device Address data 2
    #[inline(always)]
    pub fn dvad2(&mut self) -> Dvad2W<DarSpec> {
        Dvad2W::new(self, 16)
    }
    ///Bits 24:31 - Device Address data 3
    #[inline(always)]
    pub fn dvad3(&mut self) -> Dvad3W<DarSpec> {
        Dvad3W::new(self, 24)
    }
}
/**Device Address Register

You can [`read`](crate::Reg::read) this register and get [`dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DarSpec;
impl crate::RegisterSpec for DarSpec {
    type Ux = u32;
}
///`read()` method returns [`dar::R`](R) reader structure
impl crate::Readable for DarSpec {}
///`write(|w| ..)` method takes [`dar::W`](W) writer structure
impl crate::Writable for DarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAR to value 0
impl crate::Resettable for DarSpec {}
