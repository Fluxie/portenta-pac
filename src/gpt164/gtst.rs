///Register `GTST` reader
pub type R = crate::R<GtstSpec>;
///Register `GTST` writer
pub type W = crate::W<GtstSpec>;
/**Input Capture/Compare Match Flag A

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfa {
    ///0: No input capture/compare match of GTCCRA is generated
    _0 = 0,
    ///1: An input capture/compare match of GTCCRA is generated
    _1 = 1,
}
impl From<Tcfa> for bool {
    #[inline(always)]
    fn from(variant: Tcfa) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFA` reader - Input Capture/Compare Match Flag A
pub type TcfaR = crate::BitReader<Tcfa>;
impl TcfaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcfa {
        match self.bits {
            false => Tcfa::_0,
            true => Tcfa::_1,
        }
    }
    ///No input capture/compare match of GTCCRA is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfa::_0
    }
    ///An input capture/compare match of GTCCRA is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfa::_1
    }
}
///Field `TCFA` writer - Input Capture/Compare Match Flag A
pub type TcfaW<'a, REG> = crate::BitWriter<'a, REG, Tcfa>;
impl<'a, REG> TcfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No input capture/compare match of GTCCRA is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfa::_0)
    }
    ///An input capture/compare match of GTCCRA is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfa::_1)
    }
}
/**Input Capture/Compare Match Flag B

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfb {
    ///0: No input capture/compare match of GTCCRB is generated
    _0 = 0,
    ///1: An input capture/compare match of GTCCRB is generated
    _1 = 1,
}
impl From<Tcfb> for bool {
    #[inline(always)]
    fn from(variant: Tcfb) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFB` reader - Input Capture/Compare Match Flag B
pub type TcfbR = crate::BitReader<Tcfb>;
impl TcfbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcfb {
        match self.bits {
            false => Tcfb::_0,
            true => Tcfb::_1,
        }
    }
    ///No input capture/compare match of GTCCRB is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfb::_0
    }
    ///An input capture/compare match of GTCCRB is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfb::_1
    }
}
///Field `TCFB` writer - Input Capture/Compare Match Flag B
pub type TcfbW<'a, REG> = crate::BitWriter<'a, REG, Tcfb>;
impl<'a, REG> TcfbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No input capture/compare match of GTCCRB is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfb::_0)
    }
    ///An input capture/compare match of GTCCRB is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfb::_1)
    }
}
/**Input Compare Match Flag C

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfc {
    ///0: No compare match of GTCCRC is generated
    _0 = 0,
    ///1: A compare match of GTCCRC is generated
    _1 = 1,
}
impl From<Tcfc> for bool {
    #[inline(always)]
    fn from(variant: Tcfc) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFC` reader - Input Compare Match Flag C
pub type TcfcR = crate::BitReader<Tcfc>;
impl TcfcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcfc {
        match self.bits {
            false => Tcfc::_0,
            true => Tcfc::_1,
        }
    }
    ///No compare match of GTCCRC is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfc::_0
    }
    ///A compare match of GTCCRC is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfc::_1
    }
}
///Field `TCFC` writer - Input Compare Match Flag C
pub type TcfcW<'a, REG> = crate::BitWriter<'a, REG, Tcfc>;
impl<'a, REG> TcfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No compare match of GTCCRC is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfc::_0)
    }
    ///A compare match of GTCCRC is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfc::_1)
    }
}
/**Input Compare Match Flag D

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfd {
    ///0: No compare match of GTCCRD is generated
    _0 = 0,
    ///1: A compare match of GTCCRD is generated
    _1 = 1,
}
impl From<Tcfd> for bool {
    #[inline(always)]
    fn from(variant: Tcfd) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFD` reader - Input Compare Match Flag D
pub type TcfdR = crate::BitReader<Tcfd>;
impl TcfdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcfd {
        match self.bits {
            false => Tcfd::_0,
            true => Tcfd::_1,
        }
    }
    ///No compare match of GTCCRD is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfd::_0
    }
    ///A compare match of GTCCRD is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfd::_1
    }
}
///Field `TCFD` writer - Input Compare Match Flag D
pub type TcfdW<'a, REG> = crate::BitWriter<'a, REG, Tcfd>;
impl<'a, REG> TcfdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No compare match of GTCCRD is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfd::_0)
    }
    ///A compare match of GTCCRD is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfd::_1)
    }
}
/**Input Compare Match Flag E

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfe {
    ///0: No compare match of GTCCRE is generated
    _0 = 0,
    ///1: A compare match of GTCCRE is generated
    _1 = 1,
}
impl From<Tcfe> for bool {
    #[inline(always)]
    fn from(variant: Tcfe) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFE` reader - Input Compare Match Flag E
pub type TcfeR = crate::BitReader<Tcfe>;
impl TcfeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcfe {
        match self.bits {
            false => Tcfe::_0,
            true => Tcfe::_1,
        }
    }
    ///No compare match of GTCCRE is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfe::_0
    }
    ///A compare match of GTCCRE is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfe::_1
    }
}
///Field `TCFE` writer - Input Compare Match Flag E
pub type TcfeW<'a, REG> = crate::BitWriter<'a, REG, Tcfe>;
impl<'a, REG> TcfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No compare match of GTCCRE is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfe::_0)
    }
    ///A compare match of GTCCRE is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfe::_1)
    }
}
/**Input Compare Match Flag F

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcff {
    ///0: No compare match of GTCCRF is generated
    _0 = 0,
    ///1: A compare match of GTCCRF is generated
    _1 = 1,
}
impl From<Tcff> for bool {
    #[inline(always)]
    fn from(variant: Tcff) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFF` reader - Input Compare Match Flag F
pub type TcffR = crate::BitReader<Tcff>;
impl TcffR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcff {
        match self.bits {
            false => Tcff::_0,
            true => Tcff::_1,
        }
    }
    ///No compare match of GTCCRF is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcff::_0
    }
    ///A compare match of GTCCRF is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcff::_1
    }
}
///Field `TCFF` writer - Input Compare Match Flag F
pub type TcffW<'a, REG> = crate::BitWriter<'a, REG, Tcff>;
impl<'a, REG> TcffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No compare match of GTCCRF is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcff::_0)
    }
    ///A compare match of GTCCRF is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcff::_1)
    }
}
/**Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfpo {
    ///0: No overflow (crest) occurred
    _0 = 0,
    ///1: An overflow (crest) occurred
    _1 = 1,
}
impl From<Tcfpo> for bool {
    #[inline(always)]
    fn from(variant: Tcfpo) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFPO` reader - Overflow Flag
pub type TcfpoR = crate::BitReader<Tcfpo>;
impl TcfpoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcfpo {
        match self.bits {
            false => Tcfpo::_0,
            true => Tcfpo::_1,
        }
    }
    ///No overflow (crest) occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfpo::_0
    }
    ///An overflow (crest) occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfpo::_1
    }
}
///Field `TCFPO` writer - Overflow Flag
pub type TcfpoW<'a, REG> = crate::BitWriter<'a, REG, Tcfpo>;
impl<'a, REG> TcfpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overflow (crest) occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfpo::_0)
    }
    ///An overflow (crest) occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfpo::_1)
    }
}
/**Underflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfpu {
    ///0: No underflow (trough) occurred
    _0 = 0,
    ///1: An underflow (trough) occurred
    _1 = 1,
}
impl From<Tcfpu> for bool {
    #[inline(always)]
    fn from(variant: Tcfpu) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFPU` reader - Underflow Flag
pub type TcfpuR = crate::BitReader<Tcfpu>;
impl TcfpuR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcfpu {
        match self.bits {
            false => Tcfpu::_0,
            true => Tcfpu::_1,
        }
    }
    ///No underflow (trough) occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcfpu::_0
    }
    ///An underflow (trough) occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcfpu::_1
    }
}
///Field `TCFPU` writer - Underflow Flag
pub type TcfpuW<'a, REG> = crate::BitWriter<'a, REG, Tcfpu>;
impl<'a, REG> TcfpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No underflow (trough) occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfpu::_0)
    }
    ///An underflow (trough) occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfpu::_1)
    }
}
/**Count Direction Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tucf {
    ///0: GTCNT counter counts downward
    _0 = 0,
    ///1: GTCNT counter counts upward
    _1 = 1,
}
impl From<Tucf> for bool {
    #[inline(always)]
    fn from(variant: Tucf) -> Self {
        variant as u8 != 0
    }
}
///Field `TUCF` reader - Count Direction Flag
pub type TucfR = crate::BitReader<Tucf>;
impl TucfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tucf {
        match self.bits {
            false => Tucf::_0,
            true => Tucf::_1,
        }
    }
    ///GTCNT counter counts downward
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tucf::_0
    }
    ///GTCNT counter counts upward
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tucf::_1
    }
}
/**Output Disable Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Odf {
    ///0: No output disable request is generated
    _0 = 0,
    ///1: An output disable request is generated
    _1 = 1,
}
impl From<Odf> for bool {
    #[inline(always)]
    fn from(variant: Odf) -> Self {
        variant as u8 != 0
    }
}
///Field `ODF` reader - Output Disable Flag
pub type OdfR = crate::BitReader<Odf>;
impl OdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Odf {
        match self.bits {
            false => Odf::_0,
            true => Odf::_1,
        }
    }
    ///No output disable request is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Odf::_0
    }
    ///An output disable request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Odf::_1
    }
}
/**Same Time Output Level High Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oabhf {
    ///0: No simultaneous generation of 1 both for the GTIOCA and GTIOCB pins has occurred.
    _0 = 0,
    ///1: A simultaneous generation of 1 both for the GTIOCA and GTIOCB pins has occurred.
    _1 = 1,
}
impl From<Oabhf> for bool {
    #[inline(always)]
    fn from(variant: Oabhf) -> Self {
        variant as u8 != 0
    }
}
///Field `OABHF` reader - Same Time Output Level High Flag
pub type OabhfR = crate::BitReader<Oabhf>;
impl OabhfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oabhf {
        match self.bits {
            false => Oabhf::_0,
            true => Oabhf::_1,
        }
    }
    ///No simultaneous generation of 1 both for the GTIOCA and GTIOCB pins has occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oabhf::_0
    }
    ///A simultaneous generation of 1 both for the GTIOCA and GTIOCB pins has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oabhf::_1
    }
}
/**Same Time Output Level Low Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oablf {
    ///0: No simultaneous generation of 0 both for the GTIOCA and GTIOCB pins has occurred.
    _0 = 0,
    ///1: A simultaneous generation of 0 both for the GTIOCA and GTIOCB pins has occurred.
    _1 = 1,
}
impl From<Oablf> for bool {
    #[inline(always)]
    fn from(variant: Oablf) -> Self {
        variant as u8 != 0
    }
}
///Field `OABLF` reader - Same Time Output Level Low Flag
pub type OablfR = crate::BitReader<Oablf>;
impl OablfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oablf {
        match self.bits {
            false => Oablf::_0,
            true => Oablf::_1,
        }
    }
    ///No simultaneous generation of 0 both for the GTIOCA and GTIOCB pins has occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oablf::_0
    }
    ///A simultaneous generation of 0 both for the GTIOCA and GTIOCB pins has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oablf::_1
    }
}
/**Period Count Function Finish Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcf {
    ///0: No period count function finish has occurred
    _0 = 0,
    ///1: A period count function finish has occurred
    _1 = 1,
}
impl From<Pcf> for bool {
    #[inline(always)]
    fn from(variant: Pcf) -> Self {
        variant as u8 != 0
    }
}
///Field `PCF` reader - Period Count Function Finish Flag
pub type PcfR = crate::BitReader<Pcf>;
impl PcfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pcf {
        match self.bits {
            false => Pcf::_0,
            true => Pcf::_1,
        }
    }
    ///No period count function finish has occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pcf::_0
    }
    ///A period count function finish has occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pcf::_1
    }
}
///Field `PCF` writer - Period Count Function Finish Flag
pub type PcfW<'a, REG> = crate::BitWriter<'a, REG, Pcf>;
impl<'a, REG> PcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No period count function finish has occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pcf::_0)
    }
    ///A period count function finish has occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcf::_1)
    }
}
impl R {
    ///Bit 0 - Input Capture/Compare Match Flag A
    #[inline(always)]
    pub fn tcfa(&self) -> TcfaR {
        TcfaR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Input Capture/Compare Match Flag B
    #[inline(always)]
    pub fn tcfb(&self) -> TcfbR {
        TcfbR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Input Compare Match Flag C
    #[inline(always)]
    pub fn tcfc(&self) -> TcfcR {
        TcfcR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Input Compare Match Flag D
    #[inline(always)]
    pub fn tcfd(&self) -> TcfdR {
        TcfdR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Input Compare Match Flag E
    #[inline(always)]
    pub fn tcfe(&self) -> TcfeR {
        TcfeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Input Compare Match Flag F
    #[inline(always)]
    pub fn tcff(&self) -> TcffR {
        TcffR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overflow Flag
    #[inline(always)]
    pub fn tcfpo(&self) -> TcfpoR {
        TcfpoR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Underflow Flag
    #[inline(always)]
    pub fn tcfpu(&self) -> TcfpuR {
        TcfpuR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - Count Direction Flag
    #[inline(always)]
    pub fn tucf(&self) -> TucfR {
        TucfR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - Output Disable Flag
    #[inline(always)]
    pub fn odf(&self) -> OdfR {
        OdfR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 29 - Same Time Output Level High Flag
    #[inline(always)]
    pub fn oabhf(&self) -> OabhfR {
        OabhfR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Same Time Output Level Low Flag
    #[inline(always)]
    pub fn oablf(&self) -> OablfR {
        OablfR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Period Count Function Finish Flag
    #[inline(always)]
    pub fn pcf(&self) -> PcfR {
        PcfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTST")
            .field("tcfa", &self.tcfa())
            .field("tcfb", &self.tcfb())
            .field("tcfc", &self.tcfc())
            .field("tcfd", &self.tcfd())
            .field("tcfe", &self.tcfe())
            .field("tcff", &self.tcff())
            .field("tcfpo", &self.tcfpo())
            .field("tcfpu", &self.tcfpu())
            .field("tucf", &self.tucf())
            .field("odf", &self.odf())
            .field("oabhf", &self.oabhf())
            .field("oablf", &self.oablf())
            .field("pcf", &self.pcf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Input Capture/Compare Match Flag A
    #[inline(always)]
    pub fn tcfa(&mut self) -> TcfaW<GtstSpec> {
        TcfaW::new(self, 0)
    }
    ///Bit 1 - Input Capture/Compare Match Flag B
    #[inline(always)]
    pub fn tcfb(&mut self) -> TcfbW<GtstSpec> {
        TcfbW::new(self, 1)
    }
    ///Bit 2 - Input Compare Match Flag C
    #[inline(always)]
    pub fn tcfc(&mut self) -> TcfcW<GtstSpec> {
        TcfcW::new(self, 2)
    }
    ///Bit 3 - Input Compare Match Flag D
    #[inline(always)]
    pub fn tcfd(&mut self) -> TcfdW<GtstSpec> {
        TcfdW::new(self, 3)
    }
    ///Bit 4 - Input Compare Match Flag E
    #[inline(always)]
    pub fn tcfe(&mut self) -> TcfeW<GtstSpec> {
        TcfeW::new(self, 4)
    }
    ///Bit 5 - Input Compare Match Flag F
    #[inline(always)]
    pub fn tcff(&mut self) -> TcffW<GtstSpec> {
        TcffW::new(self, 5)
    }
    ///Bit 6 - Overflow Flag
    #[inline(always)]
    pub fn tcfpo(&mut self) -> TcfpoW<GtstSpec> {
        TcfpoW::new(self, 6)
    }
    ///Bit 7 - Underflow Flag
    #[inline(always)]
    pub fn tcfpu(&mut self) -> TcfpuW<GtstSpec> {
        TcfpuW::new(self, 7)
    }
    ///Bit 31 - Period Count Function Finish Flag
    #[inline(always)]
    pub fn pcf(&mut self) -> PcfW<GtstSpec> {
        PcfW::new(self, 31)
    }
}
/**General PWM Timer Status Register

You can [`read`](crate::Reg::read) this register and get [`gtst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtstSpec;
impl crate::RegisterSpec for GtstSpec {
    type Ux = u32;
}
///`read()` method returns [`gtst::R`](R) reader structure
impl crate::Readable for GtstSpec {}
///`write(|w| ..)` method takes [`gtst::W`](W) writer structure
impl crate::Writable for GtstSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTST to value 0x8000
impl crate::Resettable for GtstSpec {
    const RESET_VALUE: u32 = 0x8000;
}
