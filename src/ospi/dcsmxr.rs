///Register `DCSMXR` reader
pub type R = crate::R<DcsmxrSpec>;
///Register `DCSMXR` writer
pub type W = crate::W<DcsmxrSpec>;
///Field `CTWMX0` reader - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous write of OctaRAM.
pub type Ctwmx0R = crate::FieldReader<u16>;
///Field `CTWMX0` writer - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous write of OctaRAM.
pub type Ctwmx0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `CTWMX1` reader - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous read of OctaRAM.
pub type Ctwmx1R = crate::FieldReader<u16>;
///Field `CTWMX1` writer - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous read of OctaRAM.
pub type Ctwmx1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous write of OctaRAM.
    #[inline(always)]
    pub fn ctwmx0(&self) -> Ctwmx0R {
        Ctwmx0R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous read of OctaRAM.
    #[inline(always)]
    pub fn ctwmx1(&self) -> Ctwmx1R {
        Ctwmx1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCSMXR")
            .field("ctwmx0", &self.ctwmx0())
            .field("ctwmx1", &self.ctwmx1())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous write of OctaRAM.
    #[inline(always)]
    pub fn ctwmx0(&mut self) -> Ctwmx0W<DcsmxrSpec> {
        Ctwmx0W::new(self, 0)
    }
    ///Bits 16:24 - Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous read of OctaRAM.
    #[inline(always)]
    pub fn ctwmx1(&mut self) -> Ctwmx1W<DcsmxrSpec> {
        Ctwmx1W::new(self, 16)
    }
}
/**Device Chip Select Maximum Period Register

You can [`read`](crate::Reg::read) this register and get [`dcsmxr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsmxr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcsmxrSpec;
impl crate::RegisterSpec for DcsmxrSpec {
    type Ux = u32;
}
///`read()` method returns [`dcsmxr::R`](R) reader structure
impl crate::Readable for DcsmxrSpec {}
///`write(|w| ..)` method takes [`dcsmxr::W`](W) writer structure
impl crate::Writable for DcsmxrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCSMXR to value 0
impl crate::Resettable for DcsmxrSpec {}
