///Register `DWSCTSR` reader
pub type R = crate::R<DwsctsrSpec>;
///Register `DWSCTSR` writer
pub type W = crate::W<DwsctsrSpec>;
///Field `CTSN0` reader - Indicates the number of bytes to translate in single continuous write of device 0.
pub type Ctsn0R = crate::FieldReader<u16>;
///Field `CTSN0` writer - Indicates the number of bytes to translate in single continuous write of device 0.
pub type Ctsn0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `CTSN1` reader - Indicates the number of bytes to translate in single continuous write of device 1.
pub type Ctsn1R = crate::FieldReader<u16>;
///Field `CTSN1` writer - Indicates the number of bytes to translate in single continuous write of device 1.
pub type Ctsn1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - Indicates the number of bytes to translate in single continuous write of device 0.
    #[inline(always)]
    pub fn ctsn0(&self) -> Ctsn0R {
        Ctsn0R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Indicates the number of bytes to translate in single continuous write of device 1.
    #[inline(always)]
    pub fn ctsn1(&self) -> Ctsn1R {
        Ctsn1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DWSCTSR")
            .field("ctsn0", &self.ctsn0())
            .field("ctsn1", &self.ctsn1())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Indicates the number of bytes to translate in single continuous write of device 0.
    #[inline(always)]
    pub fn ctsn0(&mut self) -> Ctsn0W<DwsctsrSpec> {
        Ctsn0W::new(self, 0)
    }
    ///Bits 16:26 - Indicates the number of bytes to translate in single continuous write of device 1.
    #[inline(always)]
    pub fn ctsn1(&mut self) -> Ctsn1W<DwsctsrSpec> {
        Ctsn1W::new(self, 16)
    }
}
/**Device Memory Map Write single continuous translating size Register

You can [`read`](crate::Reg::read) this register and get [`dwsctsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwsctsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DwsctsrSpec;
impl crate::RegisterSpec for DwsctsrSpec {
    type Ux = u32;
}
///`read()` method returns [`dwsctsr::R`](R) reader structure
impl crate::Readable for DwsctsrSpec {}
///`write(|w| ..)` method takes [`dwsctsr::W`](W) writer structure
impl crate::Writable for DwsctsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DWSCTSR to value 0
impl crate::Resettable for DwsctsrSpec {}
