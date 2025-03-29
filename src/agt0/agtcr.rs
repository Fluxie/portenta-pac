///Register `AGTCR` reader
pub type R = crate::R<AgtcrSpec>;
///Register `AGTCR` writer
pub type W = crate::W<AgtcrSpec>;
/**AGT Count Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstart {
    ///0: Count stops
    _0 = 0,
    ///1: Count starts
    _1 = 1,
}
impl From<Tstart> for bool {
    #[inline(always)]
    fn from(variant: Tstart) -> Self {
        variant as u8 != 0
    }
}
///Field `TSTART` reader - AGT Count Start
pub type TstartR = crate::BitReader<Tstart>;
impl TstartR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tstart {
        match self.bits {
            false => Tstart::_0,
            true => Tstart::_1,
        }
    }
    ///Count stops
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tstart::_0
    }
    ///Count starts
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tstart::_1
    }
}
///Field `TSTART` writer - AGT Count Start
pub type TstartW<'a, REG> = crate::BitWriter<'a, REG, Tstart>;
impl<'a, REG> TstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Count stops
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::_0)
    }
    ///Count starts
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::_1)
    }
}
/**AGT Count Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcstf {
    ///0: Count stopped
    _0 = 0,
    ///1: Count in progress
    _1 = 1,
}
impl From<Tcstf> for bool {
    #[inline(always)]
    fn from(variant: Tcstf) -> Self {
        variant as u8 != 0
    }
}
///Field `TCSTF` reader - AGT Count Status Flag
pub type TcstfR = crate::BitReader<Tcstf>;
impl TcstfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcstf {
        match self.bits {
            false => Tcstf::_0,
            true => Tcstf::_1,
        }
    }
    ///Count stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcstf::_0
    }
    ///Count in progress
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcstf::_1
    }
}
/**AGT Count Forced Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstop {
    ///0: Writing is invalid
    _0 = 0,
    ///1: The count is forcibly stopped
    _1 = 1,
}
impl From<Tstop> for bool {
    #[inline(always)]
    fn from(variant: Tstop) -> Self {
        variant as u8 != 0
    }
}
///Field `TSTOP` writer - AGT Count Forced Stop
pub type TstopW<'a, REG> = crate::BitWriter<'a, REG, Tstop>;
impl<'a, REG> TstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing is invalid
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::_0)
    }
    ///The count is forcibly stopped
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::_1)
    }
}
/**Active Edge Judgment Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tedgf {
    ///0: No active edge received
    _0 = 0,
    ///1: Active edge received
    _1 = 1,
}
impl From<Tedgf> for bool {
    #[inline(always)]
    fn from(variant: Tedgf) -> Self {
        variant as u8 != 0
    }
}
///Field `TEDGF` reader - Active Edge Judgment Flag
pub type TedgfR = crate::BitReader<Tedgf>;
impl TedgfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tedgf {
        match self.bits {
            false => Tedgf::_0,
            true => Tedgf::_1,
        }
    }
    ///No active edge received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tedgf::_0
    }
    ///Active edge received
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tedgf::_1
    }
}
///Field `TEDGF` writer - Active Edge Judgment Flag
pub type TedgfW<'a, REG> = crate::BitWriter<'a, REG, Tedgf>;
impl<'a, REG> TedgfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No active edge received
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tedgf::_0)
    }
    ///Active edge received
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tedgf::_1)
    }
}
/**Underflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tundf {
    ///0: No underflow
    _0 = 0,
    ///1: Underflow
    _1 = 1,
}
impl From<Tundf> for bool {
    #[inline(always)]
    fn from(variant: Tundf) -> Self {
        variant as u8 != 0
    }
}
///Field `TUNDF` reader - Underflow Flag
pub type TundfR = crate::BitReader<Tundf>;
impl TundfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tundf {
        match self.bits {
            false => Tundf::_0,
            true => Tundf::_1,
        }
    }
    ///No underflow
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tundf::_0
    }
    ///Underflow
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tundf::_1
    }
}
///Field `TUNDF` writer - Underflow Flag
pub type TundfW<'a, REG> = crate::BitWriter<'a, REG, Tundf>;
impl<'a, REG> TundfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No underflow
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tundf::_0)
    }
    ///Underflow
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tundf::_1)
    }
}
/**Compare Match A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcmaf {
    ///0: No match
    _0 = 0,
    ///1: Match
    _1 = 1,
}
impl From<Tcmaf> for bool {
    #[inline(always)]
    fn from(variant: Tcmaf) -> Self {
        variant as u8 != 0
    }
}
///Field `TCMAF` reader - Compare Match A Flag
pub type TcmafR = crate::BitReader<Tcmaf>;
impl TcmafR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcmaf {
        match self.bits {
            false => Tcmaf::_0,
            true => Tcmaf::_1,
        }
    }
    ///No match
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcmaf::_0
    }
    ///Match
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcmaf::_1
    }
}
///Field `TCMAF` writer - Compare Match A Flag
pub type TcmafW<'a, REG> = crate::BitWriter<'a, REG, Tcmaf>;
impl<'a, REG> TcmafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No match
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmaf::_0)
    }
    ///Match
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmaf::_1)
    }
}
/**Compare Match B Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcmbf {
    ///0: No match
    _0 = 0,
    ///1: Match
    _1 = 1,
}
impl From<Tcmbf> for bool {
    #[inline(always)]
    fn from(variant: Tcmbf) -> Self {
        variant as u8 != 0
    }
}
///Field `TCMBF` reader - Compare Match B Flag
pub type TcmbfR = crate::BitReader<Tcmbf>;
impl TcmbfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcmbf {
        match self.bits {
            false => Tcmbf::_0,
            true => Tcmbf::_1,
        }
    }
    ///No match
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcmbf::_0
    }
    ///Match
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcmbf::_1
    }
}
///Field `TCMBF` writer - Compare Match B Flag
pub type TcmbfW<'a, REG> = crate::BitWriter<'a, REG, Tcmbf>;
impl<'a, REG> TcmbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No match
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmbf::_0)
    }
    ///Match
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmbf::_1)
    }
}
impl R {
    ///Bit 0 - AGT Count Start
    #[inline(always)]
    pub fn tstart(&self) -> TstartR {
        TstartR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AGT Count Status Flag
    #[inline(always)]
    pub fn tcstf(&self) -> TcstfR {
        TcstfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Active Edge Judgment Flag
    #[inline(always)]
    pub fn tedgf(&self) -> TedgfR {
        TedgfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Underflow Flag
    #[inline(always)]
    pub fn tundf(&self) -> TundfR {
        TundfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Compare Match A Flag
    #[inline(always)]
    pub fn tcmaf(&self) -> TcmafR {
        TcmafR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Compare Match B Flag
    #[inline(always)]
    pub fn tcmbf(&self) -> TcmbfR {
        TcmbfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTCR")
            .field("tstart", &self.tstart())
            .field("tcstf", &self.tcstf())
            .field("tedgf", &self.tedgf())
            .field("tundf", &self.tundf())
            .field("tcmaf", &self.tcmaf())
            .field("tcmbf", &self.tcmbf())
            .finish()
    }
}
impl W {
    ///Bit 0 - AGT Count Start
    #[inline(always)]
    pub fn tstart(&mut self) -> TstartW<AgtcrSpec> {
        TstartW::new(self, 0)
    }
    ///Bit 2 - AGT Count Forced Stop
    #[inline(always)]
    pub fn tstop(&mut self) -> TstopW<AgtcrSpec> {
        TstopW::new(self, 2)
    }
    ///Bit 4 - Active Edge Judgment Flag
    #[inline(always)]
    pub fn tedgf(&mut self) -> TedgfW<AgtcrSpec> {
        TedgfW::new(self, 4)
    }
    ///Bit 5 - Underflow Flag
    #[inline(always)]
    pub fn tundf(&mut self) -> TundfW<AgtcrSpec> {
        TundfW::new(self, 5)
    }
    ///Bit 6 - Compare Match A Flag
    #[inline(always)]
    pub fn tcmaf(&mut self) -> TcmafW<AgtcrSpec> {
        TcmafW::new(self, 6)
    }
    ///Bit 7 - Compare Match B Flag
    #[inline(always)]
    pub fn tcmbf(&mut self) -> TcmbfW<AgtcrSpec> {
        TcmbfW::new(self, 7)
    }
}
/**AGT Control Register

You can [`read`](crate::Reg::read) this register and get [`agtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AgtcrSpec;
impl crate::RegisterSpec for AgtcrSpec {
    type Ux = u8;
}
///`read()` method returns [`agtcr::R`](R) reader structure
impl crate::Readable for AgtcrSpec {}
///`write(|w| ..)` method takes [`agtcr::W`](W) writer structure
impl crate::Writable for AgtcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTCR to value 0
impl crate::Resettable for AgtcrSpec {}
