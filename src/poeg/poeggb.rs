///Register `POEGGB` reader
pub type R = crate::R<PoeggbSpec>;
///Register `POEGGB` writer
pub type W = crate::W<PoeggbSpec>;
/**Port Input Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidf {
    ///0: No output-disable request from the GTETRGn pin occurred
    _0 = 0,
    ///1: Output-disable request from the GTETRGn pin occurred.
    _1 = 1,
}
impl From<Pidf> for bool {
    #[inline(always)]
    fn from(variant: Pidf) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDF` reader - Port Input Detection Flag
pub type PidfR = crate::BitReader<Pidf>;
impl PidfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidf {
        match self.bits {
            false => Pidf::_0,
            true => Pidf::_1,
        }
    }
    ///No output-disable request from the GTETRGn pin occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidf::_0
    }
    ///Output-disable request from the GTETRGn pin occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidf::_1
    }
}
///Field `PIDF` writer - Port Input Detection Flag
pub type PidfW<'a, REG> = crate::BitWriter<'a, REG, Pidf>;
impl<'a, REG> PidfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No output-disable request from the GTETRGn pin occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pidf::_0)
    }
    ///Output-disable request from the GTETRGn pin occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pidf::_1)
    }
}
/**Detection Flag for GPT Output-Disable Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iocf {
    ///0: No output-disable request from GPT occurred.
    _0 = 0,
    ///1: Output-disable request from GPT occurred.
    _1 = 1,
}
impl From<Iocf> for bool {
    #[inline(always)]
    fn from(variant: Iocf) -> Self {
        variant as u8 != 0
    }
}
///Field `IOCF` reader - Detection Flag for GPT Output-Disable Request
pub type IocfR = crate::BitReader<Iocf>;
impl IocfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iocf {
        match self.bits {
            false => Iocf::_0,
            true => Iocf::_1,
        }
    }
    ///No output-disable request from GPT occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iocf::_0
    }
    ///Output-disable request from GPT occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iocf::_1
    }
}
///Field `IOCF` writer - Detection Flag for GPT Output-Disable Request
pub type IocfW<'a, REG> = crate::BitWriter<'a, REG, Iocf>;
impl<'a, REG> IocfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No output-disable request from GPT occurred.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iocf::_0)
    }
    ///Output-disable request from GPT occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iocf::_1)
    }
}
/**Oscillation Stop Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostpf {
    ///0: No output-disable request from oscillation stop detection occurred
    _0 = 0,
    ///1: Output-disable request from oscillation stop detection occurred
    _1 = 1,
}
impl From<Ostpf> for bool {
    #[inline(always)]
    fn from(variant: Ostpf) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTPF` reader - Oscillation Stop Detection Flag
pub type OstpfR = crate::BitReader<Ostpf>;
impl OstpfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ostpf {
        match self.bits {
            false => Ostpf::_0,
            true => Ostpf::_1,
        }
    }
    ///No output-disable request from oscillation stop detection occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ostpf::_0
    }
    ///Output-disable request from oscillation stop detection occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ostpf::_1
    }
}
///Field `OSTPF` writer - Oscillation Stop Detection Flag
pub type OstpfW<'a, REG> = crate::BitWriter<'a, REG, Ostpf>;
impl<'a, REG> OstpfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No output-disable request from oscillation stop detection occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ostpf::_0)
    }
    ///Output-disable request from oscillation stop detection occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ostpf::_1)
    }
}
/**Software Stop Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssf {
    ///0: No output-disable request from software occurred
    _0 = 0,
    ///1: Output-disable request from software occurred
    _1 = 1,
}
impl From<Ssf> for bool {
    #[inline(always)]
    fn from(variant: Ssf) -> Self {
        variant as u8 != 0
    }
}
///Field `SSF` reader - Software Stop Flag
pub type SsfR = crate::BitReader<Ssf>;
impl SsfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssf {
        match self.bits {
            false => Ssf::_0,
            true => Ssf::_1,
        }
    }
    ///No output-disable request from software occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssf::_0
    }
    ///Output-disable request from software occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssf::_1
    }
}
///Field `SSF` writer - Software Stop Flag
pub type SsfW<'a, REG> = crate::BitWriter<'a, REG, Ssf>;
impl<'a, REG> SsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No output-disable request from software occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssf::_0)
    }
    ///Output-disable request from software occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssf::_1)
    }
}
/**Port Input Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pide {
    ///0: Disable output-disable requests from the GTETRGn pins
    _0 = 0,
    ///1: Enable output-disable requests from the GTETRGn pins
    _1 = 1,
}
impl From<Pide> for bool {
    #[inline(always)]
    fn from(variant: Pide) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDE` reader - Port Input Detection Enable
pub type PideR = crate::BitReader<Pide>;
impl PideR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pide {
        match self.bits {
            false => Pide::_0,
            true => Pide::_1,
        }
    }
    ///Disable output-disable requests from the GTETRGn pins
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pide::_0
    }
    ///Enable output-disable requests from the GTETRGn pins
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pide::_1
    }
}
///Field `PIDE` writer - Port Input Detection Enable
pub type PideW<'a, REG> = crate::BitWriter<'a, REG, Pide>;
impl<'a, REG> PideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable output-disable requests from the GTETRGn pins
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pide::_0)
    }
    ///Enable output-disable requests from the GTETRGn pins
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pide::_1)
    }
}
/**Enable for GPT Output-Disable Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ioce {
    ///0: Disable output-disable requests from GPT
    _0 = 0,
    ///1: Enable output-disable requests from GPT
    _1 = 1,
}
impl From<Ioce> for bool {
    #[inline(always)]
    fn from(variant: Ioce) -> Self {
        variant as u8 != 0
    }
}
///Field `IOCE` reader - Enable for GPT Output-Disable Request
pub type IoceR = crate::BitReader<Ioce>;
impl IoceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ioce {
        match self.bits {
            false => Ioce::_0,
            true => Ioce::_1,
        }
    }
    ///Disable output-disable requests from GPT
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ioce::_0
    }
    ///Enable output-disable requests from GPT
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ioce::_1
    }
}
///Field `IOCE` writer - Enable for GPT Output-Disable Request
pub type IoceW<'a, REG> = crate::BitWriter<'a, REG, Ioce>;
impl<'a, REG> IoceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable output-disable requests from GPT
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ioce::_0)
    }
    ///Enable output-disable requests from GPT
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ioce::_1)
    }
}
/**Oscillation Stop Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostpe {
    ///0: Disable output-disable requests from oscillation stop detection
    _0 = 0,
    ///1: Enable output-disable requests from oscillation stop detection
    _1 = 1,
}
impl From<Ostpe> for bool {
    #[inline(always)]
    fn from(variant: Ostpe) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTPE` reader - Oscillation Stop Detection Enable
pub type OstpeR = crate::BitReader<Ostpe>;
impl OstpeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ostpe {
        match self.bits {
            false => Ostpe::_0,
            true => Ostpe::_1,
        }
    }
    ///Disable output-disable requests from oscillation stop detection
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ostpe::_0
    }
    ///Enable output-disable requests from oscillation stop detection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ostpe::_1
    }
}
///Field `OSTPE` writer - Oscillation Stop Detection Enable
pub type OstpeW<'a, REG> = crate::BitWriter<'a, REG, Ostpe>;
impl<'a, REG> OstpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable output-disable requests from oscillation stop detection
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ostpe::_0)
    }
    ///Enable output-disable requests from oscillation stop detection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ostpe::_1)
    }
}
/**GTETRGn Input Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum St {
    ///0: GTETRGn input after filtering was 0
    _0 = 0,
    ///1: GTETRGn input after filtering was 1
    _1 = 1,
}
impl From<St> for bool {
    #[inline(always)]
    fn from(variant: St) -> Self {
        variant as u8 != 0
    }
}
///Field `ST` reader - GTETRGn Input Status Flag
pub type StR = crate::BitReader<St>;
impl StR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> St {
        match self.bits {
            false => St::_0,
            true => St::_1,
        }
    }
    ///GTETRGn input after filtering was 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == St::_0
    }
    ///GTETRGn input after filtering was 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == St::_1
    }
}
/**GTETRGn Input Reverse

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    ///0: Input GTETRGn as-is
    _0 = 0,
    ///1: Input GTETRGn in reverse
    _1 = 1,
}
impl From<Inv> for bool {
    #[inline(always)]
    fn from(variant: Inv) -> Self {
        variant as u8 != 0
    }
}
///Field `INV` reader - GTETRGn Input Reverse
pub type InvR = crate::BitReader<Inv>;
impl InvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Inv {
        match self.bits {
            false => Inv::_0,
            true => Inv::_1,
        }
    }
    ///Input GTETRGn as-is
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Inv::_0
    }
    ///Input GTETRGn in reverse
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Inv::_1
    }
}
///Field `INV` writer - GTETRGn Input Reverse
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Inv>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input GTETRGn as-is
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::_0)
    }
    ///Input GTETRGn in reverse
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::_1)
    }
}
/**Noise Filter Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfen {
    ///0: Disable noise filtering
    _0 = 0,
    ///1: Enable noise filtering
    _1 = 1,
}
impl From<Nfen> for bool {
    #[inline(always)]
    fn from(variant: Nfen) -> Self {
        variant as u8 != 0
    }
}
///Field `NFEN` reader - Noise Filter Enable
pub type NfenR = crate::BitReader<Nfen>;
impl NfenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nfen {
        match self.bits {
            false => Nfen::_0,
            true => Nfen::_1,
        }
    }
    ///Disable noise filtering
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfen::_0
    }
    ///Enable noise filtering
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfen::_1
    }
}
///Field `NFEN` writer - Noise Filter Enable
pub type NfenW<'a, REG> = crate::BitWriter<'a, REG, Nfen>;
impl<'a, REG> NfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable noise filtering
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_0)
    }
    ///Enable noise filtering
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_1)
    }
}
/**Noise Filter Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfcs {
    ///0: Sample GTETRGn pin input level three times every PCLKB
    _00 = 0,
    ///1: Sample GTETRGn pin input level three times every PCLKB/8
    _01 = 1,
    ///2: Sample GTETRGn pin input level three times every PCLKB/32
    _10 = 2,
    ///3: Sample GTETRGn pin input level three times every PCLKB/128
    _11 = 3,
}
impl From<Nfcs> for u8 {
    #[inline(always)]
    fn from(variant: Nfcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nfcs {
    type Ux = u8;
}
impl crate::IsEnum for Nfcs {}
///Field `NFCS` reader - Noise Filter Clock Select
pub type NfcsR = crate::FieldReader<Nfcs>;
impl NfcsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nfcs {
        match self.bits {
            0 => Nfcs::_00,
            1 => Nfcs::_01,
            2 => Nfcs::_10,
            3 => Nfcs::_11,
            _ => unreachable!(),
        }
    }
    ///Sample GTETRGn pin input level three times every PCLKB
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Nfcs::_00
    }
    ///Sample GTETRGn pin input level three times every PCLKB/8
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Nfcs::_01
    }
    ///Sample GTETRGn pin input level three times every PCLKB/32
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Nfcs::_10
    }
    ///Sample GTETRGn pin input level three times every PCLKB/128
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Nfcs::_11
    }
}
///Field `NFCS` writer - Noise Filter Clock Select
pub type NfcsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nfcs, crate::Safe>;
impl<'a, REG> NfcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Sample GTETRGn pin input level three times every PCLKB
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_00)
    }
    ///Sample GTETRGn pin input level three times every PCLKB/8
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_01)
    }
    ///Sample GTETRGn pin input level three times every PCLKB/32
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_10)
    }
    ///Sample GTETRGn pin input level three times every PCLKB/128
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_11)
    }
}
impl R {
    ///Bit 0 - Port Input Detection Flag
    #[inline(always)]
    pub fn pidf(&self) -> PidfR {
        PidfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Detection Flag for GPT Output-Disable Request
    #[inline(always)]
    pub fn iocf(&self) -> IocfR {
        IocfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Oscillation Stop Detection Flag
    #[inline(always)]
    pub fn ostpf(&self) -> OstpfR {
        OstpfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Software Stop Flag
    #[inline(always)]
    pub fn ssf(&self) -> SsfR {
        SsfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port Input Detection Enable
    #[inline(always)]
    pub fn pide(&self) -> PideR {
        PideR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable for GPT Output-Disable Request
    #[inline(always)]
    pub fn ioce(&self) -> IoceR {
        IoceR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Oscillation Stop Detection Enable
    #[inline(always)]
    pub fn ostpe(&self) -> OstpeR {
        OstpeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - GTETRGn Input Status Flag
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 28 - GTETRGn Input Reverse
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Noise Filter Enable
    #[inline(always)]
    pub fn nfen(&self) -> NfenR {
        NfenR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - Noise Filter Clock Select
    #[inline(always)]
    pub fn nfcs(&self) -> NfcsR {
        NfcsR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POEGGB")
            .field("pidf", &self.pidf())
            .field("iocf", &self.iocf())
            .field("ostpf", &self.ostpf())
            .field("ssf", &self.ssf())
            .field("pide", &self.pide())
            .field("ioce", &self.ioce())
            .field("ostpe", &self.ostpe())
            .field("st", &self.st())
            .field("inv", &self.inv())
            .field("nfen", &self.nfen())
            .field("nfcs", &self.nfcs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port Input Detection Flag
    #[inline(always)]
    pub fn pidf(&mut self) -> PidfW<PoeggbSpec> {
        PidfW::new(self, 0)
    }
    ///Bit 1 - Detection Flag for GPT Output-Disable Request
    #[inline(always)]
    pub fn iocf(&mut self) -> IocfW<PoeggbSpec> {
        IocfW::new(self, 1)
    }
    ///Bit 2 - Oscillation Stop Detection Flag
    #[inline(always)]
    pub fn ostpf(&mut self) -> OstpfW<PoeggbSpec> {
        OstpfW::new(self, 2)
    }
    ///Bit 3 - Software Stop Flag
    #[inline(always)]
    pub fn ssf(&mut self) -> SsfW<PoeggbSpec> {
        SsfW::new(self, 3)
    }
    ///Bit 4 - Port Input Detection Enable
    #[inline(always)]
    pub fn pide(&mut self) -> PideW<PoeggbSpec> {
        PideW::new(self, 4)
    }
    ///Bit 5 - Enable for GPT Output-Disable Request
    #[inline(always)]
    pub fn ioce(&mut self) -> IoceW<PoeggbSpec> {
        IoceW::new(self, 5)
    }
    ///Bit 6 - Oscillation Stop Detection Enable
    #[inline(always)]
    pub fn ostpe(&mut self) -> OstpeW<PoeggbSpec> {
        OstpeW::new(self, 6)
    }
    ///Bit 28 - GTETRGn Input Reverse
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<PoeggbSpec> {
        InvW::new(self, 28)
    }
    ///Bit 29 - Noise Filter Enable
    #[inline(always)]
    pub fn nfen(&mut self) -> NfenW<PoeggbSpec> {
        NfenW::new(self, 29)
    }
    ///Bits 30:31 - Noise Filter Clock Select
    #[inline(always)]
    pub fn nfcs(&mut self) -> NfcsW<PoeggbSpec> {
        NfcsW::new(self, 30)
    }
}
/**POEG Group B Setting Register

You can [`read`](crate::Reg::read) this register and get [`poeggb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poeggb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PoeggbSpec;
impl crate::RegisterSpec for PoeggbSpec {
    type Ux = u32;
}
///`read()` method returns [`poeggb::R`](R) reader structure
impl crate::Readable for PoeggbSpec {}
///`write(|w| ..)` method takes [`poeggb::W`](W) writer structure
impl crate::Writable for PoeggbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POEGGB to value 0
impl crate::Resettable for PoeggbSpec {}
