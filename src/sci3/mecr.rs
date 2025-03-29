///Register `MECR` reader
pub type R = crate::R<MecrSpec>;
///Register `MECR` writer
pub type W = crate::W<MecrSpec>;
/**Preface Error Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pferen {
    ///0: Does not handle a preface error as an interrupt source
    _0 = 0,
    ///1: Handles a preface error as an interrupt source
    _1 = 1,
}
impl From<Pferen> for bool {
    #[inline(always)]
    fn from(variant: Pferen) -> Self {
        variant as u8 != 0
    }
}
///Field `PFEREN` reader - Preface Error Enable
pub type PferenR = crate::BitReader<Pferen>;
impl PferenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pferen {
        match self.bits {
            false => Pferen::_0,
            true => Pferen::_1,
        }
    }
    ///Does not handle a preface error as an interrupt source
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pferen::_0
    }
    ///Handles a preface error as an interrupt source
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pferen::_1
    }
}
///Field `PFEREN` writer - Preface Error Enable
pub type PferenW<'a, REG> = crate::BitWriter<'a, REG, Pferen>;
impl<'a, REG> PferenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not handle a preface error as an interrupt source
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pferen::_0)
    }
    ///Handles a preface error as an interrupt source
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pferen::_1)
    }
}
/**Receive SYNC Error Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syeren {
    ///0: Does not handle a receive SYNC error as an interrupt source
    _0 = 0,
    ///1: Handles a receive SYNC error as an interrupt source
    _1 = 1,
}
impl From<Syeren> for bool {
    #[inline(always)]
    fn from(variant: Syeren) -> Self {
        variant as u8 != 0
    }
}
///Field `SYEREN` reader - Receive SYNC Error Enable
pub type SyerenR = crate::BitReader<Syeren>;
impl SyerenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Syeren {
        match self.bits {
            false => Syeren::_0,
            true => Syeren::_1,
        }
    }
    ///Does not handle a receive SYNC error as an interrupt source
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Syeren::_0
    }
    ///Handles a receive SYNC error as an interrupt source
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Syeren::_1
    }
}
///Field `SYEREN` writer - Receive SYNC Error Enable
pub type SyerenW<'a, REG> = crate::BitWriter<'a, REG, Syeren>;
impl<'a, REG> SyerenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not handle a receive SYNC error as an interrupt source
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Syeren::_0)
    }
    ///Handles a receive SYNC error as an interrupt source
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Syeren::_1)
    }
}
/**Start Bit Error Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sberen {
    ///0: Does not handle a start bit error as an interrupt source
    _0 = 0,
    ///1: Handles a start bit error as an interrupt source
    _1 = 1,
}
impl From<Sberen> for bool {
    #[inline(always)]
    fn from(variant: Sberen) -> Self {
        variant as u8 != 0
    }
}
///Field `SBEREN` reader - Start Bit Error Enable
pub type SberenR = crate::BitReader<Sberen>;
impl SberenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sberen {
        match self.bits {
            false => Sberen::_0,
            true => Sberen::_1,
        }
    }
    ///Does not handle a start bit error as an interrupt source
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sberen::_0
    }
    ///Handles a start bit error as an interrupt source
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sberen::_1
    }
}
///Field `SBEREN` writer - Start Bit Error Enable
pub type SberenW<'a, REG> = crate::BitWriter<'a, REG, Sberen>;
impl<'a, REG> SberenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not handle a start bit error as an interrupt source
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sberen::_0)
    }
    ///Handles a start bit error as an interrupt source
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sberen::_1)
    }
}
impl R {
    ///Bit 0 - Preface Error Enable
    #[inline(always)]
    pub fn pferen(&self) -> PferenR {
        PferenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receive SYNC Error Enable
    #[inline(always)]
    pub fn syeren(&self) -> SyerenR {
        SyerenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Start Bit Error Enable
    #[inline(always)]
    pub fn sberen(&self) -> SberenR {
        SberenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MECR")
            .field("pferen", &self.pferen())
            .field("syeren", &self.syeren())
            .field("sberen", &self.sberen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Preface Error Enable
    #[inline(always)]
    pub fn pferen(&mut self) -> PferenW<MecrSpec> {
        PferenW::new(self, 0)
    }
    ///Bit 1 - Receive SYNC Error Enable
    #[inline(always)]
    pub fn syeren(&mut self) -> SyerenW<MecrSpec> {
        SyerenW::new(self, 1)
    }
    ///Bit 2 - Start Bit Error Enable
    #[inline(always)]
    pub fn sberen(&mut self) -> SberenW<MecrSpec> {
        SberenW::new(self, 2)
    }
}
/**Manchester Extended Error Control Register

You can [`read`](crate::Reg::read) this register and get [`mecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MecrSpec;
impl crate::RegisterSpec for MecrSpec {
    type Ux = u8;
}
///`read()` method returns [`mecr::R`](R) reader structure
impl crate::Readable for MecrSpec {}
///`write(|w| ..)` method takes [`mecr::W`](W) writer structure
impl crate::Writable for MecrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MECR to value 0
impl crate::Resettable for MecrSpec {}
