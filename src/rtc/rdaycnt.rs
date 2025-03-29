///Register `RDAYCNT` reader
pub type R = crate::R<RdaycntSpec>;
///Register `RDAYCNT` writer
pub type W = crate::W<RdaycntSpec>;
///Field `DATE1` reader - 1-Day Count
pub type Date1R = crate::FieldReader;
///Field `DATE1` writer - 1-Day Count
pub type Date1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATE10` reader - 10-Day Count
pub type Date10R = crate::FieldReader;
///Field `DATE10` writer - 10-Day Count
pub type Date10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - 1-Day Count
    #[inline(always)]
    pub fn date1(&self) -> Date1R {
        Date1R::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - 10-Day Count
    #[inline(always)]
    pub fn date10(&self) -> Date10R {
        Date10R::new((self.bits >> 4) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDAYCNT")
            .field("date1", &self.date1())
            .field("date10", &self.date10())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 1-Day Count
    #[inline(always)]
    pub fn date1(&mut self) -> Date1W<RdaycntSpec> {
        Date1W::new(self, 0)
    }
    ///Bits 4:5 - 10-Day Count
    #[inline(always)]
    pub fn date10(&mut self) -> Date10W<RdaycntSpec> {
        Date10W::new(self, 4)
    }
}
/**Day Counter

You can [`read`](crate::Reg::read) this register and get [`rdaycnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdaycnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdaycntSpec;
impl crate::RegisterSpec for RdaycntSpec {
    type Ux = u8;
}
///`read()` method returns [`rdaycnt::R`](R) reader structure
impl crate::Readable for RdaycntSpec {}
///`write(|w| ..)` method takes [`rdaycnt::W`](W) writer structure
impl crate::Writable for RdaycntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RDAYCNT to value 0
impl crate::Resettable for RdaycntSpec {}
