///Register `TMR` reader
pub type R = crate::R<TmrSpec>;
///Register `TMR` writer
pub type W = crate::W<TmrSpec>;
/**Timer Operating Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Toms {
    ///0: Timer mode
    _00 = 0,
    ///1: Break Field low width determination mode
    _01 = 1,
    ///2: Break Field low width output mode
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Toms> for u8 {
    #[inline(always)]
    fn from(variant: Toms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Toms {
    type Ux = u8;
}
impl crate::IsEnum for Toms {}
///Field `TOMS` reader - Timer Operating Mode Select
pub type TomsR = crate::FieldReader<Toms>;
impl TomsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Toms {
        match self.bits {
            0 => Toms::_00,
            1 => Toms::_01,
            2 => Toms::_10,
            3 => Toms::_11,
            _ => unreachable!(),
        }
    }
    ///Timer mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Toms::_00
    }
    ///Break Field low width determination mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Toms::_01
    }
    ///Break Field low width output mode
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Toms::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Toms::_11
    }
}
///Field `TOMS` writer - Timer Operating Mode Select
pub type TomsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Toms, crate::Safe>;
impl<'a, REG> TomsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Toms::_00)
    }
    ///Break Field low width determination mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Toms::_01)
    }
    ///Break Field low width output mode
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Toms::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Toms::_11)
    }
}
/**Counter Write Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Twrc {
    ///0: Data is written to the reload register and counter
    _0 = 0,
    ///1: Data is written to the reload register only
    _1 = 1,
}
impl From<Twrc> for bool {
    #[inline(always)]
    fn from(variant: Twrc) -> Self {
        variant as u8 != 0
    }
}
///Field `TWRC` reader - Counter Write Control
pub type TwrcR = crate::BitReader<Twrc>;
impl TwrcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Twrc {
        match self.bits {
            false => Twrc::_0,
            true => Twrc::_1,
        }
    }
    ///Data is written to the reload register and counter
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Twrc::_0
    }
    ///Data is written to the reload register only
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Twrc::_1
    }
}
///Field `TWRC` writer - Counter Write Control
pub type TwrcW<'a, REG> = crate::BitWriter<'a, REG, Twrc>;
impl<'a, REG> TwrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data is written to the reload register and counter
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Twrc::_0)
    }
    ///Data is written to the reload register only
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Twrc::_1)
    }
}
/**Timer Count Clock Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcss {
    ///0: PCLK
    _000 = 0,
    ///1: PCLK/2
    _001 = 1,
    ///2: PCLK/4
    _010 = 2,
    ///3: PCLK/8
    _011 = 3,
    ///4: PCLK/16
    _100 = 4,
    ///5: PCLK/32
    _101 = 5,
    ///6: PCLK/64
    _110 = 6,
    ///7: PCLK/128
    _111 = 7,
}
impl From<Tcss> for u8 {
    #[inline(always)]
    fn from(variant: Tcss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcss {
    type Ux = u8;
}
impl crate::IsEnum for Tcss {}
///Field `TCSS` reader - Timer Count Clock Source Select
pub type TcssR = crate::FieldReader<Tcss>;
impl TcssR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcss {
        match self.bits {
            0 => Tcss::_000,
            1 => Tcss::_001,
            2 => Tcss::_010,
            3 => Tcss::_011,
            4 => Tcss::_100,
            5 => Tcss::_101,
            6 => Tcss::_110,
            7 => Tcss::_111,
            _ => unreachable!(),
        }
    }
    ///PCLK
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Tcss::_000
    }
    ///PCLK/2
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Tcss::_001
    }
    ///PCLK/4
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Tcss::_010
    }
    ///PCLK/8
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Tcss::_011
    }
    ///PCLK/16
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Tcss::_100
    }
    ///PCLK/32
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Tcss::_101
    }
    ///PCLK/64
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Tcss::_110
    }
    ///PCLK/128
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Tcss::_111
    }
}
///Field `TCSS` writer - Timer Count Clock Source Select
pub type TcssW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tcss, crate::Safe>;
impl<'a, REG> TcssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_000)
    }
    ///PCLK/2
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_001)
    }
    ///PCLK/4
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_010)
    }
    ///PCLK/8
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_011)
    }
    ///PCLK/16
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_100)
    }
    ///PCLK/32
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_101)
    }
    ///PCLK/64
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_110)
    }
    ///PCLK/128
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_111)
    }
}
impl R {
    ///Bits 0:1 - Timer Operating Mode Select
    #[inline(always)]
    pub fn toms(&self) -> TomsR {
        TomsR::new(self.bits & 3)
    }
    ///Bit 3 - Counter Write Control
    #[inline(always)]
    pub fn twrc(&self) -> TwrcR {
        TwrcR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Timer Count Clock Source Select
    #[inline(always)]
    pub fn tcss(&self) -> TcssR {
        TcssR::new((self.bits >> 4) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMR")
            .field("toms", &self.toms())
            .field("twrc", &self.twrc())
            .field("tcss", &self.tcss())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Timer Operating Mode Select
    #[inline(always)]
    pub fn toms(&mut self) -> TomsW<TmrSpec> {
        TomsW::new(self, 0)
    }
    ///Bit 3 - Counter Write Control
    #[inline(always)]
    pub fn twrc(&mut self) -> TwrcW<TmrSpec> {
        TwrcW::new(self, 3)
    }
    ///Bits 4:6 - Timer Count Clock Source Select
    #[inline(always)]
    pub fn tcss(&mut self) -> TcssW<TmrSpec> {
        TcssW::new(self, 4)
    }
}
/**Timer Mode Register

You can [`read`](crate::Reg::read) this register and get [`tmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TmrSpec;
impl crate::RegisterSpec for TmrSpec {
    type Ux = u8;
}
///`read()` method returns [`tmr::R`](R) reader structure
impl crate::Readable for TmrSpec {}
///`write(|w| ..)` method takes [`tmr::W`](W) writer structure
impl crate::Writable for TmrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TMR to value 0
impl crate::Resettable for TmrSpec {}
