///Register `CECEXMON` reader
pub type R = crate::R<CecexmonSpec>;
///Register `CECEXMON` writer
pub type W = crate::W<CecexmonSpec>;
/**CEC Line Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceclnmon {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Ceclnmon> for bool {
    #[inline(always)]
    fn from(variant: Ceclnmon) -> Self {
        variant as u8 != 0
    }
}
///Field `CECLNMON` reader - CEC Line Monitor
pub type CeclnmonR = crate::BitReader<Ceclnmon>;
impl CeclnmonR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ceclnmon {
        match self.bits {
            false => Ceclnmon::_0,
            true => Ceclnmon::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ceclnmon::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ceclnmon::_1
    }
}
///Field `ACKF` reader - ACK Flag
pub type AckfR = crate::BitReader;
impl R {
    ///Bit 0 - CEC Line Monitor
    #[inline(always)]
    pub fn ceclnmon(&self) -> CeclnmonR {
        CeclnmonR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ACK Flag
    #[inline(always)]
    pub fn ackf(&self) -> AckfR {
        AckfR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CECEXMON")
            .field("ceclnmon", &self.ceclnmon())
            .field("ackf", &self.ackf())
            .finish()
    }
}
impl W {}
/**CEC Extension Monitor Register

You can [`read`](crate::Reg::read) this register and get [`cecexmon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecexmon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CecexmonSpec;
impl crate::RegisterSpec for CecexmonSpec {
    type Ux = u8;
}
///`read()` method returns [`cecexmon::R`](R) reader structure
impl crate::Readable for CecexmonSpec {}
///`write(|w| ..)` method takes [`cecexmon::W`](W) writer structure
impl crate::Writable for CecexmonSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CECEXMON to value 0
impl crate::Resettable for CecexmonSpec {}
