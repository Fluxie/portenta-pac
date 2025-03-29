///Register `SFMCST` reader
pub type R = crate::R<SfmcstSpec>;
///Register `SFMCST` writer
pub type W = crate::W<SfmcstSpec>;
/**SPI bus cycle completion state in direct communication

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Combsy {
    ///0: No serial transfer being processed
    _0 = 0,
    ///1: Serial transfer being processed
    _1 = 1,
}
impl From<Combsy> for bool {
    #[inline(always)]
    fn from(variant: Combsy) -> Self {
        variant as u8 != 0
    }
}
///Field `COMBSY` reader - SPI bus cycle completion state in direct communication
pub type CombsyR = crate::BitReader<Combsy>;
impl CombsyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Combsy {
        match self.bits {
            false => Combsy::_0,
            true => Combsy::_1,
        }
    }
    ///No serial transfer being processed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Combsy::_0
    }
    ///Serial transfer being processed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Combsy::_1
    }
}
/**ROM access detection status in direct communication mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eromr {
    ///0: ROM access not detected
    _0 = 0,
    ///1: ROM access detected
    _1 = 1,
}
impl From<Eromr> for bool {
    #[inline(always)]
    fn from(variant: Eromr) -> Self {
        variant as u8 != 0
    }
}
///Field `EROMR` reader - ROM access detection status in direct communication mode
pub type EromrR = crate::BitReader<Eromr>;
impl EromrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eromr {
        match self.bits {
            false => Eromr::_0,
            true => Eromr::_1,
        }
    }
    ///ROM access not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eromr::_0
    }
    ///ROM access detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eromr::_1
    }
}
///Field `EROMR` writer - ROM access detection status in direct communication mode
pub type EromrW<'a, REG> = crate::BitWriter<'a, REG, Eromr>;
impl<'a, REG> EromrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ROM access not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eromr::_0)
    }
    ///ROM access detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eromr::_1)
    }
}
impl R {
    ///Bit 0 - SPI bus cycle completion state in direct communication
    #[inline(always)]
    pub fn combsy(&self) -> CombsyR {
        CombsyR::new((self.bits & 1) != 0)
    }
    ///Bit 7 - ROM access detection status in direct communication mode
    #[inline(always)]
    pub fn eromr(&self) -> EromrR {
        EromrR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMCST")
            .field("combsy", &self.combsy())
            .field("eromr", &self.eromr())
            .finish()
    }
}
impl W {
    ///Bit 7 - ROM access detection status in direct communication mode
    #[inline(always)]
    pub fn eromr(&mut self) -> EromrW<SfmcstSpec> {
        EromrW::new(self, 7)
    }
}
/**Communication Status Register

You can [`read`](crate::Reg::read) this register and get [`sfmcst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmcstSpec;
impl crate::RegisterSpec for SfmcstSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmcst::R`](R) reader structure
impl crate::Readable for SfmcstSpec {}
///`write(|w| ..)` method takes [`sfmcst::W`](W) writer structure
impl crate::Writable for SfmcstSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMCST to value 0
impl crate::Resettable for SfmcstSpec {}
