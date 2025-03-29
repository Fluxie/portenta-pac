///Register `RSECCNT` reader
pub type R = crate::R<RseccntSpec>;
///Register `RSECCNT` writer
pub type W = crate::W<RseccntSpec>;
///Field `SEC1` reader - 1-Second Count
pub type Sec1R = crate::FieldReader;
///Field `SEC1` writer - 1-Second Count
pub type Sec1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEC10` reader - 10-Second Count
pub type Sec10R = crate::FieldReader;
///Field `SEC10` writer - 10-Second Count
pub type Sec10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - 1-Second Count
    #[inline(always)]
    pub fn sec1(&self) -> Sec1R {
        Sec1R::new(self.bits & 0x0f)
    }
    ///Bits 4:6 - 10-Second Count
    #[inline(always)]
    pub fn sec10(&self) -> Sec10R {
        Sec10R::new((self.bits >> 4) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSECCNT")
            .field("sec1", &self.sec1())
            .field("sec10", &self.sec10())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 1-Second Count
    #[inline(always)]
    pub fn sec1(&mut self) -> Sec1W<RseccntSpec> {
        Sec1W::new(self, 0)
    }
    ///Bits 4:6 - 10-Second Count
    #[inline(always)]
    pub fn sec10(&mut self) -> Sec10W<RseccntSpec> {
        Sec10W::new(self, 4)
    }
}
/**Second Counter (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rseccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rseccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RseccntSpec;
impl crate::RegisterSpec for RseccntSpec {
    type Ux = u8;
}
///`read()` method returns [`rseccnt::R`](R) reader structure
impl crate::Readable for RseccntSpec {}
///`write(|w| ..)` method takes [`rseccnt::W`](W) writer structure
impl crate::Writable for RseccntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSECCNT to value 0
impl crate::Resettable for RseccntSpec {}
