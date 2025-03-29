///Register `GTUDDTYC` reader
pub type R = crate::R<GtuddtycSpec>;
///Register `GTUDDTYC` writer
pub type W = crate::W<GtuddtycSpec>;
/**Count Direction Setting

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ud {
    ///0: GTCNT counts down
    _0 = 0,
    ///1: GTCNT counts up
    _1 = 1,
}
impl From<Ud> for bool {
    #[inline(always)]
    fn from(variant: Ud) -> Self {
        variant as u8 != 0
    }
}
///Field `UD` reader - Count Direction Setting
pub type UdR = crate::BitReader<Ud>;
impl UdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ud {
        match self.bits {
            false => Ud::_0,
            true => Ud::_1,
        }
    }
    ///GTCNT counts down
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ud::_0
    }
    ///GTCNT counts up
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ud::_1
    }
}
///Field `UD` writer - Count Direction Setting
pub type UdW<'a, REG> = crate::BitWriter<'a, REG, Ud>;
impl<'a, REG> UdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counts down
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ud::_0)
    }
    ///GTCNT counts up
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ud::_1)
    }
}
/**Forcible Count Direction Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Udf {
    ///0: Not forcibly set
    _0 = 0,
    ///1: Forcibly set
    _1 = 1,
}
impl From<Udf> for bool {
    #[inline(always)]
    fn from(variant: Udf) -> Self {
        variant as u8 != 0
    }
}
///Field `UDF` reader - Forcible Count Direction Setting
pub type UdfR = crate::BitReader<Udf>;
impl UdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Udf {
        match self.bits {
            false => Udf::_0,
            true => Udf::_1,
        }
    }
    ///Not forcibly set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Udf::_0
    }
    ///Forcibly set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Udf::_1
    }
}
///Field `UDF` writer - Forcible Count Direction Setting
pub type UdfW<'a, REG> = crate::BitWriter<'a, REG, Udf>;
impl<'a, REG> UdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not forcibly set
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Udf::_0)
    }
    ///Forcibly set
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Udf::_1)
    }
}
/**GTIOCnA Output Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oadty {
    ///0: GTIOCnA pin duty depends on the compare match
    _00 = 0,
    ///1: GTIOCnA pin duty depends on the compare match
    _01 = 1,
    ///2: GTIOCnA pin duty 0%
    _10 = 2,
    ///3: GTIOCnA pin duty 100%
    _11 = 3,
}
impl From<Oadty> for u8 {
    #[inline(always)]
    fn from(variant: Oadty) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oadty {
    type Ux = u8;
}
impl crate::IsEnum for Oadty {}
///Field `OADTY` reader - GTIOCnA Output Duty Setting
pub type OadtyR = crate::FieldReader<Oadty>;
impl OadtyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oadty {
        match self.bits {
            0 => Oadty::_00,
            1 => Oadty::_01,
            2 => Oadty::_10,
            3 => Oadty::_11,
            _ => unreachable!(),
        }
    }
    ///GTIOCnA pin duty depends on the compare match
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Oadty::_00
    }
    ///GTIOCnA pin duty depends on the compare match
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Oadty::_01
    }
    ///GTIOCnA pin duty 0%
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Oadty::_10
    }
    ///GTIOCnA pin duty 100%
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Oadty::_11
    }
}
///Field `OADTY` writer - GTIOCnA Output Duty Setting
pub type OadtyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Oadty, crate::Safe>;
impl<'a, REG> OadtyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///GTIOCnA pin duty depends on the compare match
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Oadty::_00)
    }
    ///GTIOCnA pin duty depends on the compare match
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Oadty::_01)
    }
    ///GTIOCnA pin duty 0%
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Oadty::_10)
    }
    ///GTIOCnA pin duty 100%
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Oadty::_11)
    }
}
/**Forcible GTIOCnA Output Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oadtyf {
    ///0: Not forcibly set
    _0 = 0,
    ///1: Forcibly set
    _1 = 1,
}
impl From<Oadtyf> for bool {
    #[inline(always)]
    fn from(variant: Oadtyf) -> Self {
        variant as u8 != 0
    }
}
///Field `OADTYF` reader - Forcible GTIOCnA Output Duty Setting
pub type OadtyfR = crate::BitReader<Oadtyf>;
impl OadtyfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oadtyf {
        match self.bits {
            false => Oadtyf::_0,
            true => Oadtyf::_1,
        }
    }
    ///Not forcibly set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oadtyf::_0
    }
    ///Forcibly set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oadtyf::_1
    }
}
///Field `OADTYF` writer - Forcible GTIOCnA Output Duty Setting
pub type OadtyfW<'a, REG> = crate::BitWriter<'a, REG, Oadtyf>;
impl<'a, REG> OadtyfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not forcibly set
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oadtyf::_0)
    }
    ///Forcibly set
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oadtyf::_1)
    }
}
/**GTIOCnA Output Value Selecting after Releasing 0%/100% Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oadtyr {
    ///0: The function selected by the GTIOA\[3:2\] bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting.
    _0 = 0,
    ///1: The function selected by the GTIOA\[3:2\] bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting.
    _1 = 1,
}
impl From<Oadtyr> for bool {
    #[inline(always)]
    fn from(variant: Oadtyr) -> Self {
        variant as u8 != 0
    }
}
///Field `OADTYR` reader - GTIOCnA Output Value Selecting after Releasing 0%/100% Duty Setting
pub type OadtyrR = crate::BitReader<Oadtyr>;
impl OadtyrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oadtyr {
        match self.bits {
            false => Oadtyr::_0,
            true => Oadtyr::_1,
        }
    }
    ///The function selected by the GTIOA\[3:2\] bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oadtyr::_0
    }
    ///The function selected by the GTIOA\[3:2\] bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oadtyr::_1
    }
}
///Field `OADTYR` writer - GTIOCnA Output Value Selecting after Releasing 0%/100% Duty Setting
pub type OadtyrW<'a, REG> = crate::BitWriter<'a, REG, Oadtyr>;
impl<'a, REG> OadtyrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The function selected by the GTIOA\[3:2\] bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oadtyr::_0)
    }
    ///The function selected by the GTIOA\[3:2\] bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oadtyr::_1)
    }
}
/**GTIOCnB Output Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Obdty {
    ///0: GTIOCnB pin duty depends on the compare match
    _00 = 0,
    ///1: GTIOCnB pin duty depends on the compare match
    _01 = 1,
    ///2: GTIOCnB pin duty 0%
    _10 = 2,
    ///3: GTIOCnB pin duty 100%
    _11 = 3,
}
impl From<Obdty> for u8 {
    #[inline(always)]
    fn from(variant: Obdty) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Obdty {
    type Ux = u8;
}
impl crate::IsEnum for Obdty {}
///Field `OBDTY` reader - GTIOCnB Output Duty Setting
pub type ObdtyR = crate::FieldReader<Obdty>;
impl ObdtyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Obdty {
        match self.bits {
            0 => Obdty::_00,
            1 => Obdty::_01,
            2 => Obdty::_10,
            3 => Obdty::_11,
            _ => unreachable!(),
        }
    }
    ///GTIOCnB pin duty depends on the compare match
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Obdty::_00
    }
    ///GTIOCnB pin duty depends on the compare match
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Obdty::_01
    }
    ///GTIOCnB pin duty 0%
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Obdty::_10
    }
    ///GTIOCnB pin duty 100%
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Obdty::_11
    }
}
///Field `OBDTY` writer - GTIOCnB Output Duty Setting
pub type ObdtyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Obdty, crate::Safe>;
impl<'a, REG> ObdtyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///GTIOCnB pin duty depends on the compare match
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Obdty::_00)
    }
    ///GTIOCnB pin duty depends on the compare match
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Obdty::_01)
    }
    ///GTIOCnB pin duty 0%
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Obdty::_10)
    }
    ///GTIOCnB pin duty 100%
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Obdty::_11)
    }
}
/**Forcible GTIOCnB Output Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obdtyf {
    ///0: Not forcibly set
    _0 = 0,
    ///1: Forcibly set
    _1 = 1,
}
impl From<Obdtyf> for bool {
    #[inline(always)]
    fn from(variant: Obdtyf) -> Self {
        variant as u8 != 0
    }
}
///Field `OBDTYF` reader - Forcible GTIOCnB Output Duty Setting
pub type ObdtyfR = crate::BitReader<Obdtyf>;
impl ObdtyfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Obdtyf {
        match self.bits {
            false => Obdtyf::_0,
            true => Obdtyf::_1,
        }
    }
    ///Not forcibly set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Obdtyf::_0
    }
    ///Forcibly set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Obdtyf::_1
    }
}
///Field `OBDTYF` writer - Forcible GTIOCnB Output Duty Setting
pub type ObdtyfW<'a, REG> = crate::BitWriter<'a, REG, Obdtyf>;
impl<'a, REG> ObdtyfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not forcibly set
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Obdtyf::_0)
    }
    ///Forcibly set
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Obdtyf::_1)
    }
}
/**GTIOCnB Output Value Selecting after Releasing 0%/100% Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obdtyr {
    ///0: The function selected by the GTIOB\[3:2\] bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting.
    _0 = 0,
    ///1: The function selected by the GTIOB\[3:2\] bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting.
    _1 = 1,
}
impl From<Obdtyr> for bool {
    #[inline(always)]
    fn from(variant: Obdtyr) -> Self {
        variant as u8 != 0
    }
}
///Field `OBDTYR` reader - GTIOCnB Output Value Selecting after Releasing 0%/100% Duty Setting
pub type ObdtyrR = crate::BitReader<Obdtyr>;
impl ObdtyrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Obdtyr {
        match self.bits {
            false => Obdtyr::_0,
            true => Obdtyr::_1,
        }
    }
    ///The function selected by the GTIOB\[3:2\] bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Obdtyr::_0
    }
    ///The function selected by the GTIOB\[3:2\] bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Obdtyr::_1
    }
}
///Field `OBDTYR` writer - GTIOCnB Output Value Selecting after Releasing 0%/100% Duty Setting
pub type ObdtyrW<'a, REG> = crate::BitWriter<'a, REG, Obdtyr>;
impl<'a, REG> ObdtyrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The function selected by the GTIOB\[3:2\] bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Obdtyr::_0)
    }
    ///The function selected by the GTIOB\[3:2\] bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Obdtyr::_1)
    }
}
impl R {
    ///Bit 0 - Count Direction Setting
    #[inline(always)]
    pub fn ud(&self) -> UdR {
        UdR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Forcible Count Direction Setting
    #[inline(always)]
    pub fn udf(&self) -> UdfR {
        UdfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 16:17 - GTIOCnA Output Duty Setting
    #[inline(always)]
    pub fn oadty(&self) -> OadtyR {
        OadtyR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Forcible GTIOCnA Output Duty Setting
    #[inline(always)]
    pub fn oadtyf(&self) -> OadtyfR {
        OadtyfR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - GTIOCnA Output Value Selecting after Releasing 0%/100% Duty Setting
    #[inline(always)]
    pub fn oadtyr(&self) -> OadtyrR {
        OadtyrR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 24:25 - GTIOCnB Output Duty Setting
    #[inline(always)]
    pub fn obdty(&self) -> ObdtyR {
        ObdtyR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - Forcible GTIOCnB Output Duty Setting
    #[inline(always)]
    pub fn obdtyf(&self) -> ObdtyfR {
        ObdtyfR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - GTIOCnB Output Value Selecting after Releasing 0%/100% Duty Setting
    #[inline(always)]
    pub fn obdtyr(&self) -> ObdtyrR {
        ObdtyrR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTUDDTYC")
            .field("ud", &self.ud())
            .field("udf", &self.udf())
            .field("oadty", &self.oadty())
            .field("oadtyf", &self.oadtyf())
            .field("oadtyr", &self.oadtyr())
            .field("obdty", &self.obdty())
            .field("obdtyf", &self.obdtyf())
            .field("obdtyr", &self.obdtyr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Count Direction Setting
    #[inline(always)]
    pub fn ud(&mut self) -> UdW<GtuddtycSpec> {
        UdW::new(self, 0)
    }
    ///Bit 1 - Forcible Count Direction Setting
    #[inline(always)]
    pub fn udf(&mut self) -> UdfW<GtuddtycSpec> {
        UdfW::new(self, 1)
    }
    ///Bits 16:17 - GTIOCnA Output Duty Setting
    #[inline(always)]
    pub fn oadty(&mut self) -> OadtyW<GtuddtycSpec> {
        OadtyW::new(self, 16)
    }
    ///Bit 18 - Forcible GTIOCnA Output Duty Setting
    #[inline(always)]
    pub fn oadtyf(&mut self) -> OadtyfW<GtuddtycSpec> {
        OadtyfW::new(self, 18)
    }
    ///Bit 19 - GTIOCnA Output Value Selecting after Releasing 0%/100% Duty Setting
    #[inline(always)]
    pub fn oadtyr(&mut self) -> OadtyrW<GtuddtycSpec> {
        OadtyrW::new(self, 19)
    }
    ///Bits 24:25 - GTIOCnB Output Duty Setting
    #[inline(always)]
    pub fn obdty(&mut self) -> ObdtyW<GtuddtycSpec> {
        ObdtyW::new(self, 24)
    }
    ///Bit 26 - Forcible GTIOCnB Output Duty Setting
    #[inline(always)]
    pub fn obdtyf(&mut self) -> ObdtyfW<GtuddtycSpec> {
        ObdtyfW::new(self, 26)
    }
    ///Bit 27 - GTIOCnB Output Value Selecting after Releasing 0%/100% Duty Setting
    #[inline(always)]
    pub fn obdtyr(&mut self) -> ObdtyrW<GtuddtycSpec> {
        ObdtyrW::new(self, 27)
    }
}
/**General PWM Timer Count Direction and Duty Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtuddtyc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtuddtyc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtuddtycSpec;
impl crate::RegisterSpec for GtuddtycSpec {
    type Ux = u32;
}
///`read()` method returns [`gtuddtyc::R`](R) reader structure
impl crate::Readable for GtuddtycSpec {}
///`write(|w| ..)` method takes [`gtuddtyc::W`](W) writer structure
impl crate::Writable for GtuddtycSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTUDDTYC to value 0x01
impl crate::Resettable for GtuddtycSpec {
    const RESET_VALUE: u32 = 0x01;
}
