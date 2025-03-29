///Register `GTIOR` reader
pub type R = crate::R<GtiorSpec>;
///Register `GTIOR` writer
pub type W = crate::W<GtiorSpec>;
///Field `GTIOA` reader - GTIOCnA Pin Function Select
pub type GtioaR = crate::FieldReader;
///Field `GTIOA` writer - GTIOCnA Pin Function Select
pub type GtioaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**GTIOCnA Pin Output Value Setting at the Count Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oadflt {
    ///0: The GTIOCnA pin outputs low when counting stops
    _0 = 0,
    ///1: The GTIOCnA pin outputs high when counting stops
    _1 = 1,
}
impl From<Oadflt> for bool {
    #[inline(always)]
    fn from(variant: Oadflt) -> Self {
        variant as u8 != 0
    }
}
///Field `OADFLT` reader - GTIOCnA Pin Output Value Setting at the Count Stop
pub type OadfltR = crate::BitReader<Oadflt>;
impl OadfltR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oadflt {
        match self.bits {
            false => Oadflt::_0,
            true => Oadflt::_1,
        }
    }
    ///The GTIOCnA pin outputs low when counting stops
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oadflt::_0
    }
    ///The GTIOCnA pin outputs high when counting stops
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oadflt::_1
    }
}
///Field `OADFLT` writer - GTIOCnA Pin Output Value Setting at the Count Stop
pub type OadfltW<'a, REG> = crate::BitWriter<'a, REG, Oadflt>;
impl<'a, REG> OadfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The GTIOCnA pin outputs low when counting stops
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oadflt::_0)
    }
    ///The GTIOCnA pin outputs high when counting stops
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oadflt::_1)
    }
}
/**GTIOCnA Pin Output Setting at the Start/Stop Count

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oahld {
    ///0: The GTIOCnA pin output level at the start or stop of counting depends on the register setting
    _0 = 0,
    ///1: The GTIOCnA pin output level is retained at the start or stop of counting
    _1 = 1,
}
impl From<Oahld> for bool {
    #[inline(always)]
    fn from(variant: Oahld) -> Self {
        variant as u8 != 0
    }
}
///Field `OAHLD` reader - GTIOCnA Pin Output Setting at the Start/Stop Count
pub type OahldR = crate::BitReader<Oahld>;
impl OahldR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oahld {
        match self.bits {
            false => Oahld::_0,
            true => Oahld::_1,
        }
    }
    ///The GTIOCnA pin output level at the start or stop of counting depends on the register setting
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oahld::_0
    }
    ///The GTIOCnA pin output level is retained at the start or stop of counting
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oahld::_1
    }
}
///Field `OAHLD` writer - GTIOCnA Pin Output Setting at the Start/Stop Count
pub type OahldW<'a, REG> = crate::BitWriter<'a, REG, Oahld>;
impl<'a, REG> OahldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The GTIOCnA pin output level at the start or stop of counting depends on the register setting
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oahld::_0)
    }
    ///The GTIOCnA pin output level is retained at the start or stop of counting
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oahld::_1)
    }
}
/**GTIOCnA Pin Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oae {
    ///0: Output is disabled
    _0 = 0,
    ///1: Output is enabled
    _1 = 1,
}
impl From<Oae> for bool {
    #[inline(always)]
    fn from(variant: Oae) -> Self {
        variant as u8 != 0
    }
}
///Field `OAE` reader - GTIOCnA Pin Output Enable
pub type OaeR = crate::BitReader<Oae>;
impl OaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oae {
        match self.bits {
            false => Oae::_0,
            true => Oae::_1,
        }
    }
    ///Output is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oae::_0
    }
    ///Output is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oae::_1
    }
}
///Field `OAE` writer - GTIOCnA Pin Output Enable
pub type OaeW<'a, REG> = crate::BitWriter<'a, REG, Oae>;
impl<'a, REG> OaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oae::_0)
    }
    ///Output is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oae::_1)
    }
}
/**GTIOCnA Pin Disable Value Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oadf {
    ///0: None of the below options are specified
    _00 = 0,
    ///1: GTIOCnA pin is set to Hi-Z in response to controlling the output negation
    _01 = 1,
    ///2: GTIOCnA pin is set to 0 in response to controlling the output negation
    _10 = 2,
    ///3: GTIOCnA pin is set to 1 in response to controlling the output negation
    _11 = 3,
}
impl From<Oadf> for u8 {
    #[inline(always)]
    fn from(variant: Oadf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oadf {
    type Ux = u8;
}
impl crate::IsEnum for Oadf {}
///Field `OADF` reader - GTIOCnA Pin Disable Value Setting
pub type OadfR = crate::FieldReader<Oadf>;
impl OadfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oadf {
        match self.bits {
            0 => Oadf::_00,
            1 => Oadf::_01,
            2 => Oadf::_10,
            3 => Oadf::_11,
            _ => unreachable!(),
        }
    }
    ///None of the below options are specified
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Oadf::_00
    }
    ///GTIOCnA pin is set to Hi-Z in response to controlling the output negation
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Oadf::_01
    }
    ///GTIOCnA pin is set to 0 in response to controlling the output negation
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Oadf::_10
    }
    ///GTIOCnA pin is set to 1 in response to controlling the output negation
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Oadf::_11
    }
}
///Field `OADF` writer - GTIOCnA Pin Disable Value Setting
pub type OadfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Oadf, crate::Safe>;
impl<'a, REG> OadfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///None of the below options are specified
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Oadf::_00)
    }
    ///GTIOCnA pin is set to Hi-Z in response to controlling the output negation
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Oadf::_01)
    }
    ///GTIOCnA pin is set to 0 in response to controlling the output negation
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Oadf::_10)
    }
    ///GTIOCnA pin is set to 1 in response to controlling the output negation
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Oadf::_11)
    }
}
/**Noise Filter A Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfaen {
    ///0: The noise filter for the GTIOCnA pin is disabled
    _0 = 0,
    ///1: The noise filter for the GTIOCnA pin is enabled
    _1 = 1,
}
impl From<Nfaen> for bool {
    #[inline(always)]
    fn from(variant: Nfaen) -> Self {
        variant as u8 != 0
    }
}
///Field `NFAEN` reader - Noise Filter A Enable
pub type NfaenR = crate::BitReader<Nfaen>;
impl NfaenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nfaen {
        match self.bits {
            false => Nfaen::_0,
            true => Nfaen::_1,
        }
    }
    ///The noise filter for the GTIOCnA pin is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfaen::_0
    }
    ///The noise filter for the GTIOCnA pin is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfaen::_1
    }
}
///Field `NFAEN` writer - Noise Filter A Enable
pub type NfaenW<'a, REG> = crate::BitWriter<'a, REG, Nfaen>;
impl<'a, REG> NfaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The noise filter for the GTIOCnA pin is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfaen::_0)
    }
    ///The noise filter for the GTIOCnA pin is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfaen::_1)
    }
}
/**Noise Filter A Sampling Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfcsa {
    ///0: PCLKD/1
    _00 = 0,
    ///1: PCLKD/4
    _01 = 1,
    ///2: PCLKD/16
    _10 = 2,
    ///3: PCLKD/64
    _11 = 3,
}
impl From<Nfcsa> for u8 {
    #[inline(always)]
    fn from(variant: Nfcsa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nfcsa {
    type Ux = u8;
}
impl crate::IsEnum for Nfcsa {}
///Field `NFCSA` reader - Noise Filter A Sampling Clock Select
pub type NfcsaR = crate::FieldReader<Nfcsa>;
impl NfcsaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nfcsa {
        match self.bits {
            0 => Nfcsa::_00,
            1 => Nfcsa::_01,
            2 => Nfcsa::_10,
            3 => Nfcsa::_11,
            _ => unreachable!(),
        }
    }
    ///PCLKD/1
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Nfcsa::_00
    }
    ///PCLKD/4
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Nfcsa::_01
    }
    ///PCLKD/16
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Nfcsa::_10
    }
    ///PCLKD/64
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Nfcsa::_11
    }
}
///Field `NFCSA` writer - Noise Filter A Sampling Clock Select
pub type NfcsaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nfcsa, crate::Safe>;
impl<'a, REG> NfcsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKD/1
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsa::_00)
    }
    ///PCLKD/4
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsa::_01)
    }
    ///PCLKD/16
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsa::_10)
    }
    ///PCLKD/64
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsa::_11)
    }
}
///Field `GTIOB` reader - GTIOCnB Pin Function Select
pub type GtiobR = crate::FieldReader;
///Field `GTIOB` writer - GTIOCnB Pin Function Select
pub type GtiobW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**GTIOCnB Pin Output Value Setting at the Count Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obdflt {
    ///0: The GTIOCnB pin outputs low when counting stops
    _0 = 0,
    ///1: The GTIOCnB pin outputs high when counting stops
    _1 = 1,
}
impl From<Obdflt> for bool {
    #[inline(always)]
    fn from(variant: Obdflt) -> Self {
        variant as u8 != 0
    }
}
///Field `OBDFLT` reader - GTIOCnB Pin Output Value Setting at the Count Stop
pub type ObdfltR = crate::BitReader<Obdflt>;
impl ObdfltR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Obdflt {
        match self.bits {
            false => Obdflt::_0,
            true => Obdflt::_1,
        }
    }
    ///The GTIOCnB pin outputs low when counting stops
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Obdflt::_0
    }
    ///The GTIOCnB pin outputs high when counting stops
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Obdflt::_1
    }
}
///Field `OBDFLT` writer - GTIOCnB Pin Output Value Setting at the Count Stop
pub type ObdfltW<'a, REG> = crate::BitWriter<'a, REG, Obdflt>;
impl<'a, REG> ObdfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The GTIOCnB pin outputs low when counting stops
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Obdflt::_0)
    }
    ///The GTIOCnB pin outputs high when counting stops
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Obdflt::_1)
    }
}
/**GTIOCnB Pin Output Setting at the Start/Stop Count

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obhld {
    ///0: The GTIOCnB pin output level at the start/stop of counting depends on the register setting
    _0 = 0,
    ///1: The GTIOCnB pin output level is retained at the start/stop of counting
    _1 = 1,
}
impl From<Obhld> for bool {
    #[inline(always)]
    fn from(variant: Obhld) -> Self {
        variant as u8 != 0
    }
}
///Field `OBHLD` reader - GTIOCnB Pin Output Setting at the Start/Stop Count
pub type ObhldR = crate::BitReader<Obhld>;
impl ObhldR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Obhld {
        match self.bits {
            false => Obhld::_0,
            true => Obhld::_1,
        }
    }
    ///The GTIOCnB pin output level at the start/stop of counting depends on the register setting
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Obhld::_0
    }
    ///The GTIOCnB pin output level is retained at the start/stop of counting
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Obhld::_1
    }
}
///Field `OBHLD` writer - GTIOCnB Pin Output Setting at the Start/Stop Count
pub type ObhldW<'a, REG> = crate::BitWriter<'a, REG, Obhld>;
impl<'a, REG> ObhldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The GTIOCnB pin output level at the start/stop of counting depends on the register setting
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Obhld::_0)
    }
    ///The GTIOCnB pin output level is retained at the start/stop of counting
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Obhld::_1)
    }
}
/**GTIOCnB Pin Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obe {
    ///0: Output is disabled
    _0 = 0,
    ///1: Output is enabled
    _1 = 1,
}
impl From<Obe> for bool {
    #[inline(always)]
    fn from(variant: Obe) -> Self {
        variant as u8 != 0
    }
}
///Field `OBE` reader - GTIOCnB Pin Output Enable
pub type ObeR = crate::BitReader<Obe>;
impl ObeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Obe {
        match self.bits {
            false => Obe::_0,
            true => Obe::_1,
        }
    }
    ///Output is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Obe::_0
    }
    ///Output is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Obe::_1
    }
}
///Field `OBE` writer - GTIOCnB Pin Output Enable
pub type ObeW<'a, REG> = crate::BitWriter<'a, REG, Obe>;
impl<'a, REG> ObeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Obe::_0)
    }
    ///Output is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Obe::_1)
    }
}
/**GTIOCnB Pin Disable Value Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Obdf {
    ///0: None of the below options are specified
    _00 = 0,
    ///1: GTIOCnB pin is set to Hi-Z in response to controlling the output negation
    _01 = 1,
    ///2: GTIOCnB pin is set to 0 in response to controlling the output negation
    _10 = 2,
    ///3: GTIOCnB pin is set to 1 in response to controlling the output negation
    _11 = 3,
}
impl From<Obdf> for u8 {
    #[inline(always)]
    fn from(variant: Obdf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Obdf {
    type Ux = u8;
}
impl crate::IsEnum for Obdf {}
///Field `OBDF` reader - GTIOCnB Pin Disable Value Setting
pub type ObdfR = crate::FieldReader<Obdf>;
impl ObdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Obdf {
        match self.bits {
            0 => Obdf::_00,
            1 => Obdf::_01,
            2 => Obdf::_10,
            3 => Obdf::_11,
            _ => unreachable!(),
        }
    }
    ///None of the below options are specified
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Obdf::_00
    }
    ///GTIOCnB pin is set to Hi-Z in response to controlling the output negation
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Obdf::_01
    }
    ///GTIOCnB pin is set to 0 in response to controlling the output negation
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Obdf::_10
    }
    ///GTIOCnB pin is set to 1 in response to controlling the output negation
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Obdf::_11
    }
}
///Field `OBDF` writer - GTIOCnB Pin Disable Value Setting
pub type ObdfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Obdf, crate::Safe>;
impl<'a, REG> ObdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///None of the below options are specified
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Obdf::_00)
    }
    ///GTIOCnB pin is set to Hi-Z in response to controlling the output negation
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Obdf::_01)
    }
    ///GTIOCnB pin is set to 0 in response to controlling the output negation
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Obdf::_10)
    }
    ///GTIOCnB pin is set to 1 in response to controlling the output negation
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Obdf::_11)
    }
}
/**Noise Filter B Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfben {
    ///0: The noise filter for the GTIOCnB pin is disabled
    _0 = 0,
    ///1: The noise filter for the GTIOCnB pin is enabled
    _1 = 1,
}
impl From<Nfben> for bool {
    #[inline(always)]
    fn from(variant: Nfben) -> Self {
        variant as u8 != 0
    }
}
///Field `NFBEN` reader - Noise Filter B Enable
pub type NfbenR = crate::BitReader<Nfben>;
impl NfbenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nfben {
        match self.bits {
            false => Nfben::_0,
            true => Nfben::_1,
        }
    }
    ///The noise filter for the GTIOCnB pin is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfben::_0
    }
    ///The noise filter for the GTIOCnB pin is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfben::_1
    }
}
///Field `NFBEN` writer - Noise Filter B Enable
pub type NfbenW<'a, REG> = crate::BitWriter<'a, REG, Nfben>;
impl<'a, REG> NfbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The noise filter for the GTIOCnB pin is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfben::_0)
    }
    ///The noise filter for the GTIOCnB pin is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfben::_1)
    }
}
/**Noise Filter B Sampling Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfcsb {
    ///0: PCLKD/1
    _00 = 0,
    ///1: PCLKD/4
    _01 = 1,
    ///2: PCLKD/16
    _10 = 2,
    ///3: PCLKD/64
    _11 = 3,
}
impl From<Nfcsb> for u8 {
    #[inline(always)]
    fn from(variant: Nfcsb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nfcsb {
    type Ux = u8;
}
impl crate::IsEnum for Nfcsb {}
///Field `NFCSB` reader - Noise Filter B Sampling Clock Select
pub type NfcsbR = crate::FieldReader<Nfcsb>;
impl NfcsbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nfcsb {
        match self.bits {
            0 => Nfcsb::_00,
            1 => Nfcsb::_01,
            2 => Nfcsb::_10,
            3 => Nfcsb::_11,
            _ => unreachable!(),
        }
    }
    ///PCLKD/1
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Nfcsb::_00
    }
    ///PCLKD/4
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Nfcsb::_01
    }
    ///PCLKD/16
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Nfcsb::_10
    }
    ///PCLKD/64
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Nfcsb::_11
    }
}
///Field `NFCSB` writer - Noise Filter B Sampling Clock Select
pub type NfcsbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nfcsb, crate::Safe>;
impl<'a, REG> NfcsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKD/1
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsb::_00)
    }
    ///PCLKD/4
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsb::_01)
    }
    ///PCLKD/16
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsb::_10)
    }
    ///PCLKD/64
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcsb::_11)
    }
}
impl R {
    ///Bits 0:4 - GTIOCnA Pin Function Select
    #[inline(always)]
    pub fn gtioa(&self) -> GtioaR {
        GtioaR::new((self.bits & 0x1f) as u8)
    }
    ///Bit 6 - GTIOCnA Pin Output Value Setting at the Count Stop
    #[inline(always)]
    pub fn oadflt(&self) -> OadfltR {
        OadfltR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTIOCnA Pin Output Setting at the Start/Stop Count
    #[inline(always)]
    pub fn oahld(&self) -> OahldR {
        OahldR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCnA Pin Output Enable
    #[inline(always)]
    pub fn oae(&self) -> OaeR {
        OaeR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - GTIOCnA Pin Disable Value Setting
    #[inline(always)]
    pub fn oadf(&self) -> OadfR {
        OadfR::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 13 - Noise Filter A Enable
    #[inline(always)]
    pub fn nfaen(&self) -> NfaenR {
        NfaenR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Noise Filter A Sampling Clock Select
    #[inline(always)]
    pub fn nfcsa(&self) -> NfcsaR {
        NfcsaR::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:20 - GTIOCnB Pin Function Select
    #[inline(always)]
    pub fn gtiob(&self) -> GtiobR {
        GtiobR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 22 - GTIOCnB Pin Output Value Setting at the Count Stop
    #[inline(always)]
    pub fn obdflt(&self) -> ObdfltR {
        ObdfltR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - GTIOCnB Pin Output Setting at the Start/Stop Count
    #[inline(always)]
    pub fn obhld(&self) -> ObhldR {
        ObhldR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - GTIOCnB Pin Output Enable
    #[inline(always)]
    pub fn obe(&self) -> ObeR {
        ObeR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - GTIOCnB Pin Disable Value Setting
    #[inline(always)]
    pub fn obdf(&self) -> ObdfR {
        ObdfR::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 29 - Noise Filter B Enable
    #[inline(always)]
    pub fn nfben(&self) -> NfbenR {
        NfbenR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - Noise Filter B Sampling Clock Select
    #[inline(always)]
    pub fn nfcsb(&self) -> NfcsbR {
        NfcsbR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTIOR")
            .field("gtioa", &self.gtioa())
            .field("oadflt", &self.oadflt())
            .field("oahld", &self.oahld())
            .field("oae", &self.oae())
            .field("oadf", &self.oadf())
            .field("nfaen", &self.nfaen())
            .field("nfcsa", &self.nfcsa())
            .field("gtiob", &self.gtiob())
            .field("obdflt", &self.obdflt())
            .field("obhld", &self.obhld())
            .field("obe", &self.obe())
            .field("obdf", &self.obdf())
            .field("nfben", &self.nfben())
            .field("nfcsb", &self.nfcsb())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - GTIOCnA Pin Function Select
    #[inline(always)]
    pub fn gtioa(&mut self) -> GtioaW<GtiorSpec> {
        GtioaW::new(self, 0)
    }
    ///Bit 6 - GTIOCnA Pin Output Value Setting at the Count Stop
    #[inline(always)]
    pub fn oadflt(&mut self) -> OadfltW<GtiorSpec> {
        OadfltW::new(self, 6)
    }
    ///Bit 7 - GTIOCnA Pin Output Setting at the Start/Stop Count
    #[inline(always)]
    pub fn oahld(&mut self) -> OahldW<GtiorSpec> {
        OahldW::new(self, 7)
    }
    ///Bit 8 - GTIOCnA Pin Output Enable
    #[inline(always)]
    pub fn oae(&mut self) -> OaeW<GtiorSpec> {
        OaeW::new(self, 8)
    }
    ///Bits 9:10 - GTIOCnA Pin Disable Value Setting
    #[inline(always)]
    pub fn oadf(&mut self) -> OadfW<GtiorSpec> {
        OadfW::new(self, 9)
    }
    ///Bit 13 - Noise Filter A Enable
    #[inline(always)]
    pub fn nfaen(&mut self) -> NfaenW<GtiorSpec> {
        NfaenW::new(self, 13)
    }
    ///Bits 14:15 - Noise Filter A Sampling Clock Select
    #[inline(always)]
    pub fn nfcsa(&mut self) -> NfcsaW<GtiorSpec> {
        NfcsaW::new(self, 14)
    }
    ///Bits 16:20 - GTIOCnB Pin Function Select
    #[inline(always)]
    pub fn gtiob(&mut self) -> GtiobW<GtiorSpec> {
        GtiobW::new(self, 16)
    }
    ///Bit 22 - GTIOCnB Pin Output Value Setting at the Count Stop
    #[inline(always)]
    pub fn obdflt(&mut self) -> ObdfltW<GtiorSpec> {
        ObdfltW::new(self, 22)
    }
    ///Bit 23 - GTIOCnB Pin Output Setting at the Start/Stop Count
    #[inline(always)]
    pub fn obhld(&mut self) -> ObhldW<GtiorSpec> {
        ObhldW::new(self, 23)
    }
    ///Bit 24 - GTIOCnB Pin Output Enable
    #[inline(always)]
    pub fn obe(&mut self) -> ObeW<GtiorSpec> {
        ObeW::new(self, 24)
    }
    ///Bits 25:26 - GTIOCnB Pin Disable Value Setting
    #[inline(always)]
    pub fn obdf(&mut self) -> ObdfW<GtiorSpec> {
        ObdfW::new(self, 25)
    }
    ///Bit 29 - Noise Filter B Enable
    #[inline(always)]
    pub fn nfben(&mut self) -> NfbenW<GtiorSpec> {
        NfbenW::new(self, 29)
    }
    ///Bits 30:31 - Noise Filter B Sampling Clock Select
    #[inline(always)]
    pub fn nfcsb(&mut self) -> NfcsbW<GtiorSpec> {
        NfcsbW::new(self, 30)
    }
}
/**General PWM Timer I/O Control Register

You can [`read`](crate::Reg::read) this register and get [`gtior::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtior::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtiorSpec;
impl crate::RegisterSpec for GtiorSpec {
    type Ux = u32;
}
///`read()` method returns [`gtior::R`](R) reader structure
impl crate::Readable for GtiorSpec {}
///`write(|w| ..)` method takes [`gtior::W`](W) writer structure
impl crate::Writable for GtiorSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTIOR to value 0
impl crate::Resettable for GtiorSpec {}
