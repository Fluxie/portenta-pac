///Register `STCR` reader
pub type R = crate::R<StcrSpec>;
///Register `STCR` writer
pub type W = crate::W<StcrSpec>;
///Field `BFDCL` reader - BFDF Clear
pub type BfdclR = crate::BitReader;
///Field `BFDCL` writer - BFDF Clear
pub type BfdclW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CF0MCL` reader - CF0MF Clear
pub type Cf0mclR = crate::BitReader;
///Field `CF0MCL` writer - CF0MF Clear
pub type Cf0mclW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CF1MCL` reader - CF1MF Clear
pub type Cf1mclR = crate::BitReader;
///Field `CF1MCL` writer - CF1MF Clear
pub type Cf1mclW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIBDCL` reader - PIBDF Clear
pub type PibdclR = crate::BitReader;
///Field `PIBDCL` writer - PIBDF Clear
pub type PibdclW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BCDCL` reader - BCDF Clear
pub type BcdclR = crate::BitReader;
///Field `BCDCL` writer - BCDF Clear
pub type BcdclW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AEDCL` reader - AEDF Clear
pub type AedclR = crate::BitReader;
///Field `AEDCL` writer - AEDF Clear
pub type AedclW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BFDF Clear
    #[inline(always)]
    pub fn bfdcl(&self) -> BfdclR {
        BfdclR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CF0MF Clear
    #[inline(always)]
    pub fn cf0mcl(&self) -> Cf0mclR {
        Cf0mclR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CF1MF Clear
    #[inline(always)]
    pub fn cf1mcl(&self) -> Cf1mclR {
        Cf1mclR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PIBDF Clear
    #[inline(always)]
    pub fn pibdcl(&self) -> PibdclR {
        PibdclR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - BCDF Clear
    #[inline(always)]
    pub fn bcdcl(&self) -> BcdclR {
        BcdclR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AEDF Clear
    #[inline(always)]
    pub fn aedcl(&self) -> AedclR {
        AedclR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STCR")
            .field("bfdcl", &self.bfdcl())
            .field("cf0mcl", &self.cf0mcl())
            .field("cf1mcl", &self.cf1mcl())
            .field("pibdcl", &self.pibdcl())
            .field("bcdcl", &self.bcdcl())
            .field("aedcl", &self.aedcl())
            .finish()
    }
}
impl W {
    ///Bit 0 - BFDF Clear
    #[inline(always)]
    pub fn bfdcl(&mut self) -> BfdclW<StcrSpec> {
        BfdclW::new(self, 0)
    }
    ///Bit 1 - CF0MF Clear
    #[inline(always)]
    pub fn cf0mcl(&mut self) -> Cf0mclW<StcrSpec> {
        Cf0mclW::new(self, 1)
    }
    ///Bit 2 - CF1MF Clear
    #[inline(always)]
    pub fn cf1mcl(&mut self) -> Cf1mclW<StcrSpec> {
        Cf1mclW::new(self, 2)
    }
    ///Bit 3 - PIBDF Clear
    #[inline(always)]
    pub fn pibdcl(&mut self) -> PibdclW<StcrSpec> {
        PibdclW::new(self, 3)
    }
    ///Bit 4 - BCDF Clear
    #[inline(always)]
    pub fn bcdcl(&mut self) -> BcdclW<StcrSpec> {
        BcdclW::new(self, 4)
    }
    ///Bit 5 - AEDF Clear
    #[inline(always)]
    pub fn aedcl(&mut self) -> AedclW<StcrSpec> {
        AedclW::new(self, 5)
    }
}
/**Status Clear Register

You can [`read`](crate::Reg::read) this register and get [`stcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StcrSpec;
impl crate::RegisterSpec for StcrSpec {
    type Ux = u8;
}
///`read()` method returns [`stcr::R`](R) reader structure
impl crate::Readable for StcrSpec {}
///`write(|w| ..)` method takes [`stcr::W`](W) writer structure
impl crate::Writable for StcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STCR to value 0
impl crate::Resettable for StcrSpec {}
