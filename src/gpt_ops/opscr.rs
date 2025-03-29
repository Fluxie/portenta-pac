///Register `OPSCR` reader
pub type R = crate::R<OpscrSpec>;
///Register `OPSCR` writer
pub type W = crate::W<OpscrSpec>;
///Field `UF` reader -
pub type UfR = crate::BitReader;
///Field `UF` writer -
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VF` reader -
pub type VfR = crate::BitReader;
///Field `VF` writer -
pub type VfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WF` reader -
pub type WfR = crate::BitReader;
///Field `WF` writer -
pub type WfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `U` reader - Input U-Phase Monitor
pub type UR = crate::BitReader;
///Field `V` reader - Input V-Phase Monitor
pub type VR = crate::BitReader;
///Field `W` reader - Input W-Phase Monitor
pub type WR = crate::BitReader;
/**Output Phase Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    ///0: Do not output (Hi-Z external pin)
    _0 = 0,
    ///1: Output
    _1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Output Phase Enable
pub type EnR = crate::BitReader<En>;
impl EnR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::_0,
            true => En::_1,
        }
    }
    ///Do not output (Hi-Z external pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En::_0
    }
    ///Output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En::_1
    }
}
///Field `EN` writer - Output Phase Enable
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not output (Hi-Z external pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En::_0)
    }
    ///Output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En::_1)
    }
}
/**External Feedback Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fb {
    ///0: Select the external input
    _0 = 0,
    ///1: Select the soft setting (OPSCR.UF, VF, WF)
    _1 = 1,
}
impl From<Fb> for bool {
    #[inline(always)]
    fn from(variant: Fb) -> Self {
        variant as u8 != 0
    }
}
///Field `FB` reader - External Feedback Signal Enable
pub type FbR = crate::BitReader<Fb>;
impl FbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fb {
        match self.bits {
            false => Fb::_0,
            true => Fb::_1,
        }
    }
    ///Select the external input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fb::_0
    }
    ///Select the soft setting (OPSCR.UF, VF, WF)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fb::_1
    }
}
///Field `FB` writer - External Feedback Signal Enable
pub type FbW<'a, REG> = crate::BitWriter<'a, REG, Fb>;
impl<'a, REG> FbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select the external input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fb::_0)
    }
    ///Select the soft setting (OPSCR.UF, VF, WF)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fb::_1)
    }
}
/**Positive-Phase Output (P) Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P {
    ///0: Level signal output
    _0 = 0,
    ///1: PWM signal output
    _1 = 1,
}
impl From<P> for bool {
    #[inline(always)]
    fn from(variant: P) -> Self {
        variant as u8 != 0
    }
}
///Field `P` reader - Positive-Phase Output (P) Control
pub type PR = crate::BitReader<P>;
impl PR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P {
        match self.bits {
            false => P::_0,
            true => P::_1,
        }
    }
    ///Level signal output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P::_0
    }
    ///PWM signal output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P::_1
    }
}
///Field `P` writer - Positive-Phase Output (P) Control
pub type PW<'a, REG> = crate::BitWriter<'a, REG, P>;
impl<'a, REG> PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Level signal output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P::_0)
    }
    ///PWM signal output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P::_1)
    }
}
/**Negative-Phase Output (N) Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N {
    ///0: Level signal output
    _0 = 0,
    ///1: PWM signal output
    _1 = 1,
}
impl From<N> for bool {
    #[inline(always)]
    fn from(variant: N) -> Self {
        variant as u8 != 0
    }
}
///Field `N` reader - Negative-Phase Output (N) Control
pub type NR = crate::BitReader<N>;
impl NR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N {
        match self.bits {
            false => N::_0,
            true => N::_1,
        }
    }
    ///Level signal output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == N::_0
    }
    ///PWM signal output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == N::_1
    }
}
///Field `N` writer - Negative-Phase Output (N) Control
pub type NW<'a, REG> = crate::BitWriter<'a, REG, N>;
impl<'a, REG> NW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Level signal output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(N::_0)
    }
    ///PWM signal output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(N::_1)
    }
}
/**Output Phase Invert Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    ///0: Positive logic (active-high) output
    _0 = 0,
    ///1: Negative logic (active-low) output
    _1 = 1,
}
impl From<Inv> for bool {
    #[inline(always)]
    fn from(variant: Inv) -> Self {
        variant as u8 != 0
    }
}
///Field `INV` reader - Output Phase Invert Control
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
    ///Positive logic (active-high) output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Inv::_0
    }
    ///Negative logic (active-low) output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Inv::_1
    }
}
///Field `INV` writer - Output Phase Invert Control
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Inv>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Positive logic (active-high) output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::_0)
    }
    ///Negative logic (active-low) output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::_1)
    }
}
/**Output Phase Rotation Direction Reversal Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rv {
    ///0: Positive rotation
    _0 = 0,
    ///1: Reverse rotation
    _1 = 1,
}
impl From<Rv> for bool {
    #[inline(always)]
    fn from(variant: Rv) -> Self {
        variant as u8 != 0
    }
}
///Field `RV` reader - Output Phase Rotation Direction Reversal Control
pub type RvR = crate::BitReader<Rv>;
impl RvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rv {
        match self.bits {
            false => Rv::_0,
            true => Rv::_1,
        }
    }
    ///Positive rotation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rv::_0
    }
    ///Reverse rotation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rv::_1
    }
}
///Field `RV` writer - Output Phase Rotation Direction Reversal Control
pub type RvW<'a, REG> = crate::BitWriter<'a, REG, Rv>;
impl<'a, REG> RvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Positive rotation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rv::_0)
    }
    ///Reverse rotation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rv::_1)
    }
}
/**Input Phase Alignment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    ///0: Input phase aligned to PCLKD
    _0 = 0,
    ///1: Input phase aligned to the falling edge of PWM
    _1 = 1,
}
impl From<Align> for bool {
    #[inline(always)]
    fn from(variant: Align) -> Self {
        variant as u8 != 0
    }
}
///Field `ALIGN` reader - Input Phase Alignment
pub type AlignR = crate::BitReader<Align>;
impl AlignR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Align {
        match self.bits {
            false => Align::_0,
            true => Align::_1,
        }
    }
    ///Input phase aligned to PCLKD
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Align::_0
    }
    ///Input phase aligned to the falling edge of PWM
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Align::_1
    }
}
///Field `ALIGN` writer - Input Phase Alignment
pub type AlignW<'a, REG> = crate::BitWriter<'a, REG, Align>;
impl<'a, REG> AlignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input phase aligned to PCLKD
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Align::_0)
    }
    ///Input phase aligned to the falling edge of PWM
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Align::_1)
    }
}
///Field `GRP` reader - Output Disabled Source Selection
pub type GrpR = crate::FieldReader;
///Field `GRP` writer - Output Disabled Source Selection
pub type GrpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Group Output Disable Function

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Godf {
    ///0: This bit function is ignored
    _0 = 0,
    ///1: Group disable clears the OPSCR.EN bit
    _1 = 1,
}
impl From<Godf> for bool {
    #[inline(always)]
    fn from(variant: Godf) -> Self {
        variant as u8 != 0
    }
}
///Field `GODF` reader - Group Output Disable Function
pub type GodfR = crate::BitReader<Godf>;
impl GodfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Godf {
        match self.bits {
            false => Godf::_0,
            true => Godf::_1,
        }
    }
    ///This bit function is ignored
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Godf::_0
    }
    ///Group disable clears the OPSCR.EN bit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Godf::_1
    }
}
///Field `GODF` writer - Group Output Disable Function
pub type GodfW<'a, REG> = crate::BitWriter<'a, REG, Godf>;
impl<'a, REG> GodfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit function is ignored
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Godf::_0)
    }
    ///Group disable clears the OPSCR.EN bit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Godf::_1)
    }
}
/**External Input Noise Filter Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfen {
    ///0: Do not use a noise filter on the external input
    _0 = 0,
    ///1: Use a noise filter on the external input
    _1 = 1,
}
impl From<Nfen> for bool {
    #[inline(always)]
    fn from(variant: Nfen) -> Self {
        variant as u8 != 0
    }
}
///Field `NFEN` reader - External Input Noise Filter Enable
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
    ///Do not use a noise filter on the external input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfen::_0
    }
    ///Use a noise filter on the external input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfen::_1
    }
}
///Field `NFEN` writer - External Input Noise Filter Enable
pub type NfenW<'a, REG> = crate::BitWriter<'a, REG, Nfen>;
impl<'a, REG> NfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use a noise filter on the external input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_0)
    }
    ///Use a noise filter on the external input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_1)
    }
}
/**External Input Noise Filter Clock Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfcs {
    ///0: PCLKD/1
    _00 = 0,
    ///1: PCLKD/4
    _01 = 1,
    ///2: PCLKD/16
    _10 = 2,
    ///3: PCLKD/64
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
///Field `NFCS` reader - External Input Noise Filter Clock Selection
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
    ///PCLKD/1
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Nfcs::_00
    }
    ///PCLKD/4
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Nfcs::_01
    }
    ///PCLKD/16
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Nfcs::_10
    }
    ///PCLKD/64
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Nfcs::_11
    }
}
///Field `NFCS` writer - External Input Noise Filter Clock Selection
pub type NfcsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nfcs, crate::Safe>;
impl<'a, REG> NfcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKD/1
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_00)
    }
    ///PCLKD/4
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_01)
    }
    ///PCLKD/16
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_10)
    }
    ///PCLKD/64
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_11)
    }
}
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn vf(&self) -> VfR {
        VfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn wf(&self) -> WfR {
        WfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Input U-Phase Monitor
    #[inline(always)]
    pub fn u(&self) -> UR {
        UR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Input V-Phase Monitor
    #[inline(always)]
    pub fn v(&self) -> VR {
        VR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Input W-Phase Monitor
    #[inline(always)]
    pub fn w(&self) -> WR {
        WR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Output Phase Enable
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - External Feedback Signal Enable
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Positive-Phase Output (P) Control
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Negative-Phase Output (N) Control
    #[inline(always)]
    pub fn n(&self) -> NR {
        NR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Output Phase Invert Control
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output Phase Rotation Direction Reversal Control
    #[inline(always)]
    pub fn rv(&self) -> RvR {
        RvR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Input Phase Alignment
    #[inline(always)]
    pub fn align(&self) -> AlignR {
        AlignR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 24:25 - Output Disabled Source Selection
    #[inline(always)]
    pub fn grp(&self) -> GrpR {
        GrpR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - Group Output Disable Function
    #[inline(always)]
    pub fn godf(&self) -> GodfR {
        GodfR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 29 - External Input Noise Filter Enable
    #[inline(always)]
    pub fn nfen(&self) -> NfenR {
        NfenR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - External Input Noise Filter Clock Selection
    #[inline(always)]
    pub fn nfcs(&self) -> NfcsR {
        NfcsR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPSCR")
            .field("uf", &self.uf())
            .field("vf", &self.vf())
            .field("wf", &self.wf())
            .field("u", &self.u())
            .field("v", &self.v())
            .field("w", &self.w())
            .field("en", &self.en())
            .field("fb", &self.fb())
            .field("p", &self.p())
            .field("n", &self.n())
            .field("inv", &self.inv())
            .field("rv", &self.rv())
            .field("align", &self.align())
            .field("grp", &self.grp())
            .field("godf", &self.godf())
            .field("nfen", &self.nfen())
            .field("nfcs", &self.nfcs())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<OpscrSpec> {
        UfW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn vf(&mut self) -> VfW<OpscrSpec> {
        VfW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn wf(&mut self) -> WfW<OpscrSpec> {
        WfW::new(self, 2)
    }
    ///Bit 8 - Output Phase Enable
    #[inline(always)]
    pub fn en(&mut self) -> EnW<OpscrSpec> {
        EnW::new(self, 8)
    }
    ///Bit 16 - External Feedback Signal Enable
    #[inline(always)]
    pub fn fb(&mut self) -> FbW<OpscrSpec> {
        FbW::new(self, 16)
    }
    ///Bit 17 - Positive-Phase Output (P) Control
    #[inline(always)]
    pub fn p(&mut self) -> PW<OpscrSpec> {
        PW::new(self, 17)
    }
    ///Bit 18 - Negative-Phase Output (N) Control
    #[inline(always)]
    pub fn n(&mut self) -> NW<OpscrSpec> {
        NW::new(self, 18)
    }
    ///Bit 19 - Output Phase Invert Control
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<OpscrSpec> {
        InvW::new(self, 19)
    }
    ///Bit 20 - Output Phase Rotation Direction Reversal Control
    #[inline(always)]
    pub fn rv(&mut self) -> RvW<OpscrSpec> {
        RvW::new(self, 20)
    }
    ///Bit 21 - Input Phase Alignment
    #[inline(always)]
    pub fn align(&mut self) -> AlignW<OpscrSpec> {
        AlignW::new(self, 21)
    }
    ///Bits 24:25 - Output Disabled Source Selection
    #[inline(always)]
    pub fn grp(&mut self) -> GrpW<OpscrSpec> {
        GrpW::new(self, 24)
    }
    ///Bit 26 - Group Output Disable Function
    #[inline(always)]
    pub fn godf(&mut self) -> GodfW<OpscrSpec> {
        GodfW::new(self, 26)
    }
    ///Bit 29 - External Input Noise Filter Enable
    #[inline(always)]
    pub fn nfen(&mut self) -> NfenW<OpscrSpec> {
        NfenW::new(self, 29)
    }
    ///Bits 30:31 - External Input Noise Filter Clock Selection
    #[inline(always)]
    pub fn nfcs(&mut self) -> NfcsW<OpscrSpec> {
        NfcsW::new(self, 30)
    }
}
/**Output Phase Switching Control Register

You can [`read`](crate::Reg::read) this register and get [`opscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OpscrSpec;
impl crate::RegisterSpec for OpscrSpec {
    type Ux = u32;
}
///`read()` method returns [`opscr::R`](R) reader structure
impl crate::Readable for OpscrSpec {}
///`write(|w| ..)` method takes [`opscr::W`](W) writer structure
impl crate::Writable for OpscrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPSCR to value 0
impl crate::Resettable for OpscrSpec {}
