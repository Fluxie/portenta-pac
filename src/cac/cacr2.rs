///Register `CACR2` reader
pub type R = crate::R<Cacr2Spec>;
///Register `CACR2` writer
pub type W = crate::W<Cacr2Spec>;
/**Reference Signal Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rps {
    ///0: CACREF pin input
    _0 = 0,
    ///1: Internal clock (internally generated signal)
    _1 = 1,
}
impl From<Rps> for bool {
    #[inline(always)]
    fn from(variant: Rps) -> Self {
        variant as u8 != 0
    }
}
///Field `RPS` reader - Reference Signal Select
pub type RpsR = crate::BitReader<Rps>;
impl RpsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rps {
        match self.bits {
            false => Rps::_0,
            true => Rps::_1,
        }
    }
    ///CACREF pin input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rps::_0
    }
    ///Internal clock (internally generated signal)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rps::_1
    }
}
///Field `RPS` writer - Reference Signal Select
pub type RpsW<'a, REG> = crate::BitWriter<'a, REG, Rps>;
impl<'a, REG> RpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CACREF pin input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rps::_0)
    }
    ///Internal clock (internally generated signal)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rps::_1)
    }
}
/**Measurement Reference Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rscs {
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
impl From<Rscs> for u8 {
    #[inline(always)]
    fn from(variant: Rscs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rscs {
    type Ux = u8;
}
impl crate::IsEnum for Rscs {}
///Field `RSCS` reader - Measurement Reference Clock Select
pub type RscsR = crate::FieldReader<Rscs>;
impl RscsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rscs {
        match self.bits {
            0 => Rscs::_000,
            1 => Rscs::_001,
            2 => Rscs::_010,
            3 => Rscs::_011,
            4 => Rscs::_100,
            5 => Rscs::_101,
            6 => Rscs::_110,
            7 => Rscs::_111,
            _ => unreachable!(),
        }
    }
    ///Main clock oscillator
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Rscs::_000
    }
    ///Sub-clock oscillator
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Rscs::_001
    }
    ///HOCO clock
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Rscs::_010
    }
    ///MOCO clock
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Rscs::_011
    }
    ///LOCO clock
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Rscs::_100
    }
    ///Peripheral module clock B (PCLKB)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Rscs::_101
    }
    ///IWDT-dedicated clock
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Rscs::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Rscs::_111
    }
}
///Field `RSCS` writer - Measurement Reference Clock Select
pub type RscsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rscs, crate::Safe>;
impl<'a, REG> RscsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Main clock oscillator
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_000)
    }
    ///Sub-clock oscillator
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_001)
    }
    ///HOCO clock
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_010)
    }
    ///MOCO clock
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_011)
    }
    ///LOCO clock
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_100)
    }
    ///Peripheral module clock B (PCLKB)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_101)
    }
    ///IWDT-dedicated clock
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_111)
    }
}
/**Measurement Reference Clock Frequency Division Ratio Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rcds {
    ///0: x 1/32 clock
    _00 = 0,
    ///1: x 1/128 clock
    _01 = 1,
    ///2: x 1/1024 clock
    _10 = 2,
    ///3: x 1/8192 clock
    _11 = 3,
}
impl From<Rcds> for u8 {
    #[inline(always)]
    fn from(variant: Rcds) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rcds {
    type Ux = u8;
}
impl crate::IsEnum for Rcds {}
///Field `RCDS` reader - Measurement Reference Clock Frequency Division Ratio Select
pub type RcdsR = crate::FieldReader<Rcds>;
impl RcdsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcds {
        match self.bits {
            0 => Rcds::_00,
            1 => Rcds::_01,
            2 => Rcds::_10,
            3 => Rcds::_11,
            _ => unreachable!(),
        }
    }
    ///x 1/32 clock
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rcds::_00
    }
    ///x 1/128 clock
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rcds::_01
    }
    ///x 1/1024 clock
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rcds::_10
    }
    ///x 1/8192 clock
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rcds::_11
    }
}
///Field `RCDS` writer - Measurement Reference Clock Frequency Division Ratio Select
pub type RcdsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rcds, crate::Safe>;
impl<'a, REG> RcdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///x 1/32 clock
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rcds::_00)
    }
    ///x 1/128 clock
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rcds::_01)
    }
    ///x 1/1024 clock
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rcds::_10)
    }
    ///x 1/8192 clock
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rcds::_11)
    }
}
/**Digital Filter Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dfs {
    ///0: Disable digital filtering
    _00 = 0,
    ///1: Use sampling clock for the digital filter as the frequency measuring clock
    _01 = 1,
    ///2: Use sampling clock for the digital filter as the frequency measuring clock divided by 4
    _10 = 2,
    ///3: Use sampling clock for the digital filter as the frequency measuring clock divided by 16.
    _11 = 3,
}
impl From<Dfs> for u8 {
    #[inline(always)]
    fn from(variant: Dfs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dfs {
    type Ux = u8;
}
impl crate::IsEnum for Dfs {}
///Field `DFS` reader - Digital Filter Select
pub type DfsR = crate::FieldReader<Dfs>;
impl DfsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dfs {
        match self.bits {
            0 => Dfs::_00,
            1 => Dfs::_01,
            2 => Dfs::_10,
            3 => Dfs::_11,
            _ => unreachable!(),
        }
    }
    ///Disable digital filtering
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dfs::_00
    }
    ///Use sampling clock for the digital filter as the frequency measuring clock
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dfs::_01
    }
    ///Use sampling clock for the digital filter as the frequency measuring clock divided by 4
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dfs::_10
    }
    ///Use sampling clock for the digital filter as the frequency measuring clock divided by 16.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dfs::_11
    }
}
///Field `DFS` writer - Digital Filter Select
pub type DfsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dfs, crate::Safe>;
impl<'a, REG> DfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disable digital filtering
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dfs::_00)
    }
    ///Use sampling clock for the digital filter as the frequency measuring clock
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dfs::_01)
    }
    ///Use sampling clock for the digital filter as the frequency measuring clock divided by 4
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dfs::_10)
    }
    ///Use sampling clock for the digital filter as the frequency measuring clock divided by 16.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dfs::_11)
    }
}
impl R {
    ///Bit 0 - Reference Signal Select
    #[inline(always)]
    pub fn rps(&self) -> RpsR {
        RpsR::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Measurement Reference Clock Select
    #[inline(always)]
    pub fn rscs(&self) -> RscsR {
        RscsR::new((self.bits >> 1) & 7)
    }
    ///Bits 4:5 - Measurement Reference Clock Frequency Division Ratio Select
    #[inline(always)]
    pub fn rcds(&self) -> RcdsR {
        RcdsR::new((self.bits >> 4) & 3)
    }
    ///Bits 6:7 - Digital Filter Select
    #[inline(always)]
    pub fn dfs(&self) -> DfsR {
        DfsR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACR2")
            .field("rps", &self.rps())
            .field("rscs", &self.rscs())
            .field("rcds", &self.rcds())
            .field("dfs", &self.dfs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reference Signal Select
    #[inline(always)]
    pub fn rps(&mut self) -> RpsW<Cacr2Spec> {
        RpsW::new(self, 0)
    }
    ///Bits 1:3 - Measurement Reference Clock Select
    #[inline(always)]
    pub fn rscs(&mut self) -> RscsW<Cacr2Spec> {
        RscsW::new(self, 1)
    }
    ///Bits 4:5 - Measurement Reference Clock Frequency Division Ratio Select
    #[inline(always)]
    pub fn rcds(&mut self) -> RcdsW<Cacr2Spec> {
        RcdsW::new(self, 4)
    }
    ///Bits 6:7 - Digital Filter Select
    #[inline(always)]
    pub fn dfs(&mut self) -> DfsW<Cacr2Spec> {
        DfsW::new(self, 6)
    }
}
/**CAC Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cacr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cacr2Spec;
impl crate::RegisterSpec for Cacr2Spec {
    type Ux = u8;
}
///`read()` method returns [`cacr2::R`](R) reader structure
impl crate::Readable for Cacr2Spec {}
///`write(|w| ..)` method takes [`cacr2::W`](W) writer structure
impl crate::Writable for Cacr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CACR2 to value 0
impl crate::Resettable for Cacr2Spec {}
