///Register `TRCKCR` reader
pub type R = crate::R<TrckcrSpec>;
///Register `TRCKCR` writer
pub type W = crate::W<TrckcrSpec>;
/**Trace Clock operating frequency select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trck {
    ///0: /1
    _0x0 = 0,
    ///1: /2 (value after reset)
    _0x1 = 1,
    ///2: /4
    _0x2 = 2,
    ///3: Setting prohibited
    Others = 3,
}
impl From<Trck> for u8 {
    #[inline(always)]
    fn from(variant: Trck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trck {
    type Ux = u8;
}
impl crate::IsEnum for Trck {}
///Field `TRCK` reader - Trace Clock operating frequency select
pub type TrckR = crate::FieldReader<Trck>;
impl TrckR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Trck {
        match self.bits {
            0 => Trck::_0x0,
            1 => Trck::_0x1,
            2 => Trck::_0x2,
            _ => Trck::Others,
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Trck::_0x0
    }
    #[doc = "/2 (value after reset)"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Trck::_0x1
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Trck::_0x2
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Trck::Others)
    }
}
///Field `TRCK` writer - Trace Clock operating frequency select
pub type TrckW<'a, REG> = crate::FieldWriter<'a, REG, 4, Trck, crate::Safe>;
impl<'a, REG> TrckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Trck::_0x0)
    }
    #[doc = "/2 (value after reset)"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Trck::_0x1)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Trck::_0x2)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Trck::Others)
    }
}
/**Trace Clock operating Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trcken {
    ///0: Stop
    _0 = 0,
    ///1: Operation enable
    _1 = 1,
}
impl From<Trcken> for bool {
    #[inline(always)]
    fn from(variant: Trcken) -> Self {
        variant as u8 != 0
    }
}
///Field `TRCKEN` reader - Trace Clock operating Enable
pub type TrckenR = crate::BitReader<Trcken>;
impl TrckenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Trcken {
        match self.bits {
            false => Trcken::_0,
            true => Trcken::_1,
        }
    }
    ///Stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trcken::_0
    }
    ///Operation enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trcken::_1
    }
}
///Field `TRCKEN` writer - Trace Clock operating Enable
pub type TrckenW<'a, REG> = crate::BitWriter<'a, REG, Trcken>;
impl<'a, REG> TrckenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trcken::_0)
    }
    ///Operation enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trcken::_1)
    }
}
impl R {
    ///Bits 0:3 - Trace Clock operating frequency select
    #[inline(always)]
    pub fn trck(&self) -> TrckR {
        TrckR::new(self.bits & 0x0f)
    }
    ///Bit 7 - Trace Clock operating Enable
    #[inline(always)]
    pub fn trcken(&self) -> TrckenR {
        TrckenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRCKCR")
            .field("trck", &self.trck())
            .field("trcken", &self.trcken())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Trace Clock operating frequency select
    #[inline(always)]
    pub fn trck(&mut self) -> TrckW<TrckcrSpec> {
        TrckW::new(self, 0)
    }
    ///Bit 7 - Trace Clock operating Enable
    #[inline(always)]
    pub fn trcken(&mut self) -> TrckenW<TrckcrSpec> {
        TrckenW::new(self, 7)
    }
}
/**Trace Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`trckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TrckcrSpec;
impl crate::RegisterSpec for TrckcrSpec {
    type Ux = u8;
}
///`read()` method returns [`trckcr::R`](R) reader structure
impl crate::Readable for TrckcrSpec {}
///`write(|w| ..)` method takes [`trckcr::W`](W) writer structure
impl crate::Writable for TrckcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRCKCR to value 0x01
impl crate::Resettable for TrckcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
