///Register `CACR1` reader
pub type R = crate::R<Cacr1Spec>;
///Register `CACR1` writer
pub type W = crate::W<Cacr1Spec>;
/**CACREF Pin Input Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cacrefe {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Cacrefe> for bool {
    #[inline(always)]
    fn from(variant: Cacrefe) -> Self {
        variant as u8 != 0
    }
}
///Field `CACREFE` reader - CACREF Pin Input Enable
pub type CacrefeR = crate::BitReader<Cacrefe>;
impl CacrefeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cacrefe {
        match self.bits {
            false => Cacrefe::_0,
            true => Cacrefe::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cacrefe::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cacrefe::_1
    }
}
///Field `CACREFE` writer - CACREF Pin Input Enable
pub type CacrefeW<'a, REG> = crate::BitWriter<'a, REG, Cacrefe>;
impl<'a, REG> CacrefeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cacrefe::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cacrefe::_1)
    }
}
/**Measurement Target Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fmcs {
    ///0: Main clock oscillator
    _000 = 0,
    ///1: Sub-clock oscillator
    _001 = 1,
    ///2: HOCO clock
    _010 = 2,
    ///3: MOCO clock
    _011 = 3,
    ///4: LOCO clock
    _100 = 4,
    ///5: Peripheral module clock B (PCLKB)
    _101 = 5,
    ///6: IWDT-dedicated clock
    _110 = 6,
    ///7: Setting prohibited
    _111 = 7,
}
impl From<Fmcs> for u8 {
    #[inline(always)]
    fn from(variant: Fmcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fmcs {
    type Ux = u8;
}
impl crate::IsEnum for Fmcs {}
///Field `FMCS` reader - Measurement Target Clock Select
pub type FmcsR = crate::FieldReader<Fmcs>;
impl FmcsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fmcs {
        match self.bits {
            0 => Fmcs::_000,
            1 => Fmcs::_001,
            2 => Fmcs::_010,
            3 => Fmcs::_011,
            4 => Fmcs::_100,
            5 => Fmcs::_101,
            6 => Fmcs::_110,
            7 => Fmcs::_111,
            _ => unreachable!(),
        }
    }
    ///Main clock oscillator
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Fmcs::_000
    }
    ///Sub-clock oscillator
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Fmcs::_001
    }
    ///HOCO clock
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Fmcs::_010
    }
    ///MOCO clock
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Fmcs::_011
    }
    ///LOCO clock
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Fmcs::_100
    }
    ///Peripheral module clock B (PCLKB)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Fmcs::_101
    }
    ///IWDT-dedicated clock
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Fmcs::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Fmcs::_111
    }
}
///Field `FMCS` writer - Measurement Target Clock Select
pub type FmcsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fmcs, crate::Safe>;
impl<'a, REG> FmcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Main clock oscillator
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_000)
    }
    ///Sub-clock oscillator
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_001)
    }
    ///HOCO clock
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_010)
    }
    ///MOCO clock
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_011)
    }
    ///LOCO clock
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_100)
    }
    ///Peripheral module clock B (PCLKB)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_101)
    }
    ///IWDT-dedicated clock
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_111)
    }
}
/**Timer Count Clock Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcss {
    ///0: No division
    _00 = 0,
    ///1: x 1/4 clock
    _01 = 1,
    ///2: x 1/8 clock
    _10 = 2,
    ///3: x 1/32 clock
    _11 = 3,
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
            0 => Tcss::_00,
            1 => Tcss::_01,
            2 => Tcss::_10,
            3 => Tcss::_11,
            _ => unreachable!(),
        }
    }
    ///No division
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tcss::_00
    }
    ///x 1/4 clock
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tcss::_01
    }
    ///x 1/8 clock
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tcss::_10
    }
    ///x 1/32 clock
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tcss::_11
    }
}
///Field `TCSS` writer - Timer Count Clock Source Select
pub type TcssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tcss, crate::Safe>;
impl<'a, REG> TcssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No division
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_00)
    }
    ///x 1/4 clock
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_01)
    }
    ///x 1/8 clock
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_10)
    }
    ///x 1/32 clock
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_11)
    }
}
/**Valid Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edges {
    ///0: Rising edge
    _00 = 0,
    ///1: Falling edge
    _01 = 1,
    ///2: Both rising and falling edges
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Edges> for u8 {
    #[inline(always)]
    fn from(variant: Edges) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edges {
    type Ux = u8;
}
impl crate::IsEnum for Edges {}
///Field `EDGES` reader - Valid Edge Select
pub type EdgesR = crate::FieldReader<Edges>;
impl EdgesR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Edges {
        match self.bits {
            0 => Edges::_00,
            1 => Edges::_01,
            2 => Edges::_10,
            3 => Edges::_11,
            _ => unreachable!(),
        }
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Edges::_00
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Edges::_01
    }
    ///Both rising and falling edges
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Edges::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Edges::_11
    }
}
///Field `EDGES` writer - Valid Edge Select
pub type EdgesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edges, crate::Safe>;
impl<'a, REG> EdgesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Rising edge
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Edges::_00)
    }
    ///Falling edge
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Edges::_01)
    }
    ///Both rising and falling edges
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Edges::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Edges::_11)
    }
}
impl R {
    ///Bit 0 - CACREF Pin Input Enable
    #[inline(always)]
    pub fn cacrefe(&self) -> CacrefeR {
        CacrefeR::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Measurement Target Clock Select
    #[inline(always)]
    pub fn fmcs(&self) -> FmcsR {
        FmcsR::new((self.bits >> 1) & 7)
    }
    ///Bits 4:5 - Timer Count Clock Source Select
    #[inline(always)]
    pub fn tcss(&self) -> TcssR {
        TcssR::new((self.bits >> 4) & 3)
    }
    ///Bits 6:7 - Valid Edge Select
    #[inline(always)]
    pub fn edges(&self) -> EdgesR {
        EdgesR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACR1")
            .field("cacrefe", &self.cacrefe())
            .field("fmcs", &self.fmcs())
            .field("tcss", &self.tcss())
            .field("edges", &self.edges())
            .finish()
    }
}
impl W {
    ///Bit 0 - CACREF Pin Input Enable
    #[inline(always)]
    pub fn cacrefe(&mut self) -> CacrefeW<Cacr1Spec> {
        CacrefeW::new(self, 0)
    }
    ///Bits 1:3 - Measurement Target Clock Select
    #[inline(always)]
    pub fn fmcs(&mut self) -> FmcsW<Cacr1Spec> {
        FmcsW::new(self, 1)
    }
    ///Bits 4:5 - Timer Count Clock Source Select
    #[inline(always)]
    pub fn tcss(&mut self) -> TcssW<Cacr1Spec> {
        TcssW::new(self, 4)
    }
    ///Bits 6:7 - Valid Edge Select
    #[inline(always)]
    pub fn edges(&mut self) -> EdgesW<Cacr1Spec> {
        EdgesW::new(self, 6)
    }
}
/**CAC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cacr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cacr1Spec;
impl crate::RegisterSpec for Cacr1Spec {
    type Ux = u8;
}
///`read()` method returns [`cacr1::R`](R) reader structure
impl crate::Readable for Cacr1Spec {}
///`write(|w| ..)` method takes [`cacr1::W`](W) writer structure
impl crate::Writable for Cacr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CACR1 to value 0
impl crate::Resettable for Cacr1Spec {}
