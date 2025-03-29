///Register `RMONCNT` reader
pub type R = crate::R<RmoncntSpec>;
///Register `RMONCNT` writer
pub type W = crate::W<RmoncntSpec>;
///Field `MON1` reader - 1-Month Count
pub type Mon1R = crate::FieldReader;
///Field `MON1` writer - 1-Month Count
pub type Mon1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MON10` reader - 10-Month Count
pub type Mon10R = crate::BitReader;
///Field `MON10` writer - 10-Month Count
pub type Mon10W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - 1-Month Count
    #[inline(always)]
    pub fn mon1(&self) -> Mon1R {
        Mon1R::new(self.bits & 0x0f)
    }
    ///Bit 4 - 10-Month Count
    #[inline(always)]
    pub fn mon10(&self) -> Mon10R {
        Mon10R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMONCNT")
            .field("mon1", &self.mon1())
            .field("mon10", &self.mon10())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 1-Month Count
    #[inline(always)]
    pub fn mon1(&mut self) -> Mon1W<RmoncntSpec> {
        Mon1W::new(self, 0)
    }
    ///Bit 4 - 10-Month Count
    #[inline(always)]
    pub fn mon10(&mut self) -> Mon10W<RmoncntSpec> {
        Mon10W::new(self, 4)
    }
}
/**Month Counter

You can [`read`](crate::Reg::read) this register and get [`rmoncnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmoncnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RmoncntSpec;
impl crate::RegisterSpec for RmoncntSpec {
    type Ux = u8;
}
///`read()` method returns [`rmoncnt::R`](R) reader structure
impl crate::Readable for RmoncntSpec {}
///`write(|w| ..)` method takes [`rmoncnt::W`](W) writer structure
impl crate::Writable for RmoncntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RMONCNT to value 0
impl crate::Resettable for RmoncntSpec {}
