///Register `RMINCNT` reader
pub type R = crate::R<RmincntSpec>;
///Register `RMINCNT` writer
pub type W = crate::W<RmincntSpec>;
///Field `MIN1` reader - 1-Minute Count
pub type Min1R = crate::FieldReader;
///Field `MIN1` writer - 1-Minute Count
pub type Min1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MIN10` reader - 10-Minute Count
pub type Min10R = crate::FieldReader;
///Field `MIN10` writer - 10-Minute Count
pub type Min10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - 1-Minute Count
    #[inline(always)]
    pub fn min1(&self) -> Min1R {
        Min1R::new(self.bits & 0x0f)
    }
    ///Bits 4:6 - 10-Minute Count
    #[inline(always)]
    pub fn min10(&self) -> Min10R {
        Min10R::new((self.bits >> 4) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMINCNT")
            .field("min1", &self.min1())
            .field("min10", &self.min10())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 1-Minute Count
    #[inline(always)]
    pub fn min1(&mut self) -> Min1W<RmincntSpec> {
        Min1W::new(self, 0)
    }
    ///Bits 4:6 - 10-Minute Count
    #[inline(always)]
    pub fn min10(&mut self) -> Min10W<RmincntSpec> {
        Min10W::new(self, 4)
    }
}
/**Minute Counter (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rmincnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmincnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RmincntSpec;
impl crate::RegisterSpec for RmincntSpec {
    type Ux = u8;
}
///`read()` method returns [`rmincnt::R`](R) reader structure
impl crate::Readable for RmincntSpec {}
///`write(|w| ..)` method takes [`rmincnt::W`](W) writer structure
impl crate::Writable for RmincntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RMINCNT to value 0
impl crate::Resettable for RmincntSpec {}
