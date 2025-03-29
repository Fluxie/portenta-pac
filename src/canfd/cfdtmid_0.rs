///Register `CFDTMID%s_0` reader
pub type R = crate::R<Cfdtmid0Spec>;
///Register `CFDTMID%s_0` writer
pub type W = crate::W<Cfdtmid0Spec>;
///Field `TMID` reader - TX Message Buffer ID Field
pub type TmidR = crate::FieldReader<u32>;
///Field `TMID` writer - TX Message Buffer ID Field
pub type TmidW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
/**Tx History List Entry

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thlen {
    ///0: Entry not stored in THL after successful TX
    _0 = 0,
    ///1: Entry stored in THL after successful TX
    _1 = 1,
}
impl From<Thlen> for bool {
    #[inline(always)]
    fn from(variant: Thlen) -> Self {
        variant as u8 != 0
    }
}
///Field `THLEN` reader - Tx History List Entry
pub type ThlenR = crate::BitReader<Thlen>;
impl ThlenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thlen {
        match self.bits {
            false => Thlen::_0,
            true => Thlen::_1,
        }
    }
    ///Entry not stored in THL after successful TX
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thlen::_0
    }
    ///Entry stored in THL after successful TX
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thlen::_1
    }
}
///Field `THLEN` writer - Tx History List Entry
pub type ThlenW<'a, REG> = crate::BitWriter<'a, REG, Thlen>;
impl<'a, REG> ThlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Entry not stored in THL after successful TX
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Thlen::_0)
    }
    ///Entry stored in THL after successful TX
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Thlen::_1)
    }
}
/**TX Message Buffer RTR bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmrtr {
    ///0: Data frame
    _0 = 0,
    ///1: Remote frame
    _1 = 1,
}
impl From<Tmrtr> for bool {
    #[inline(always)]
    fn from(variant: Tmrtr) -> Self {
        variant as u8 != 0
    }
}
///Field `TMRTR` reader - TX Message Buffer RTR bit
pub type TmrtrR = crate::BitReader<Tmrtr>;
impl TmrtrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmrtr {
        match self.bits {
            false => Tmrtr::_0,
            true => Tmrtr::_1,
        }
    }
    ///Data frame
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmrtr::_0
    }
    ///Remote frame
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmrtr::_1
    }
}
///Field `TMRTR` writer - TX Message Buffer RTR bit
pub type TmrtrW<'a, REG> = crate::BitWriter<'a, REG, Tmrtr>;
impl<'a, REG> TmrtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data frame
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmrtr::_0)
    }
    ///Remote frame
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmrtr::_1)
    }
}
/**TX Message Buffer IDE bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmide {
    ///0: STD-ID is transmitted
    _0 = 0,
    ///1: EXT-ID is transmitted
    _1 = 1,
}
impl From<Tmide> for bool {
    #[inline(always)]
    fn from(variant: Tmide) -> Self {
        variant as u8 != 0
    }
}
///Field `TMIDE` reader - TX Message Buffer IDE bit
pub type TmideR = crate::BitReader<Tmide>;
impl TmideR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmide {
        match self.bits {
            false => Tmide::_0,
            true => Tmide::_1,
        }
    }
    ///STD-ID is transmitted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmide::_0
    }
    ///EXT-ID is transmitted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmide::_1
    }
}
///Field `TMIDE` writer - TX Message Buffer IDE bit
pub type TmideW<'a, REG> = crate::BitWriter<'a, REG, Tmide>;
impl<'a, REG> TmideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///STD-ID is transmitted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmide::_0)
    }
    ///EXT-ID is transmitted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmide::_1)
    }
}
impl R {
    ///Bits 0:28 - TX Message Buffer ID Field
    #[inline(always)]
    pub fn tmid(&self) -> TmidR {
        TmidR::new(self.bits & 0x1fff_ffff)
    }
    ///Bit 29 - Tx History List Entry
    #[inline(always)]
    pub fn thlen(&self) -> ThlenR {
        ThlenR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TX Message Buffer RTR bit
    #[inline(always)]
    pub fn tmrtr(&self) -> TmrtrR {
        TmrtrR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - TX Message Buffer IDE bit
    #[inline(always)]
    pub fn tmide(&self) -> TmideR {
        TmideR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTMID_0")
            .field("tmid", &self.tmid())
            .field("thlen", &self.thlen())
            .field("tmrtr", &self.tmrtr())
            .field("tmide", &self.tmide())
            .finish()
    }
}
impl W {
    ///Bits 0:28 - TX Message Buffer ID Field
    #[inline(always)]
    pub fn tmid(&mut self) -> TmidW<Cfdtmid0Spec> {
        TmidW::new(self, 0)
    }
    ///Bit 29 - Tx History List Entry
    #[inline(always)]
    pub fn thlen(&mut self) -> ThlenW<Cfdtmid0Spec> {
        ThlenW::new(self, 29)
    }
    ///Bit 30 - TX Message Buffer RTR bit
    #[inline(always)]
    pub fn tmrtr(&mut self) -> TmrtrW<Cfdtmid0Spec> {
        TmrtrW::new(self, 30)
    }
    ///Bit 31 - TX Message Buffer IDE bit
    #[inline(always)]
    pub fn tmide(&mut self) -> TmideW<Cfdtmid0Spec> {
        TmideW::new(self, 31)
    }
}
/**TX Message Buffer ID Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmid_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmid_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdtmid0Spec;
impl crate::RegisterSpec for Cfdtmid0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdtmid_0::R`](R) reader structure
impl crate::Readable for Cfdtmid0Spec {}
///`write(|w| ..)` method takes [`cfdtmid_0::W`](W) writer structure
impl crate::Writable for Cfdtmid0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTMID%s_0 to value 0
impl crate::Resettable for Cfdtmid0Spec {}
