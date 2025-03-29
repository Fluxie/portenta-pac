///Register `CDSR` reader
pub type R = crate::R<CdsrSpec>;
///Register `CDSR` writer
pub type W = crate::W<CdsrSpec>;
/**Device0_transfer_type setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dv0ttyp {
    ///0: SPI mode
    _00 = 0,
    ///1: SOPI mode
    _01 = 1,
    ///2: DOPI mode
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Dv0ttyp> for u8 {
    #[inline(always)]
    fn from(variant: Dv0ttyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dv0ttyp {
    type Ux = u8;
}
impl crate::IsEnum for Dv0ttyp {}
///Field `DV0TTYP` reader - Device0_transfer_type setting
pub type Dv0ttypR = crate::FieldReader<Dv0ttyp>;
impl Dv0ttypR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dv0ttyp {
        match self.bits {
            0 => Dv0ttyp::_00,
            1 => Dv0ttyp::_01,
            2 => Dv0ttyp::_10,
            3 => Dv0ttyp::_11,
            _ => unreachable!(),
        }
    }
    ///SPI mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dv0ttyp::_00
    }
    ///SOPI mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dv0ttyp::_01
    }
    ///DOPI mode
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dv0ttyp::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dv0ttyp::_11
    }
}
///Field `DV0TTYP` writer - Device0_transfer_type setting
pub type Dv0ttypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dv0ttyp, crate::Safe>;
impl<'a, REG> Dv0ttypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SPI mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dv0ttyp::_00)
    }
    ///SOPI mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dv0ttyp::_01)
    }
    ///DOPI mode
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dv0ttyp::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dv0ttyp::_11)
    }
}
/**Device1_transfer_type setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dv1ttyp {
    ///0: SPI mode
    _00 = 0,
    ///1: SOPI mode
    _01 = 1,
    ///2: DOPI mode
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Dv1ttyp> for u8 {
    #[inline(always)]
    fn from(variant: Dv1ttyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dv1ttyp {
    type Ux = u8;
}
impl crate::IsEnum for Dv1ttyp {}
///Field `DV1TTYP` reader - Device1_transfer_type setting
pub type Dv1ttypR = crate::FieldReader<Dv1ttyp>;
impl Dv1ttypR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dv1ttyp {
        match self.bits {
            0 => Dv1ttyp::_00,
            1 => Dv1ttyp::_01,
            2 => Dv1ttyp::_10,
            3 => Dv1ttyp::_11,
            _ => unreachable!(),
        }
    }
    ///SPI mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dv1ttyp::_00
    }
    ///SOPI mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dv1ttyp::_01
    }
    ///DOPI mode
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dv1ttyp::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dv1ttyp::_11
    }
}
///Field `DV1TTYP` writer - Device1_transfer_type setting
pub type Dv1ttypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dv1ttyp, crate::Safe>;
impl<'a, REG> Dv1ttypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SPI mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dv1ttyp::_00)
    }
    ///SOPI mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dv1ttyp::_01)
    }
    ///DOPI mode
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dv1ttyp::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dv1ttyp::_11)
    }
}
/**Device0_memory precycle setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dv0pc {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Dv0pc> for bool {
    #[inline(always)]
    fn from(variant: Dv0pc) -> Self {
        variant as u8 != 0
    }
}
///Field `DV0PC` reader - Device0_memory precycle setting
pub type Dv0pcR = crate::BitReader<Dv0pc>;
impl Dv0pcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dv0pc {
        match self.bits {
            false => Dv0pc::_0,
            true => Dv0pc::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dv0pc::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dv0pc::_1
    }
}
///Field `DV0PC` writer - Device0_memory precycle setting
pub type Dv0pcW<'a, REG> = crate::BitWriter<'a, REG, Dv0pc>;
impl<'a, REG> Dv0pcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dv0pc::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dv0pc::_1)
    }
}
/**Device1_memory precycle setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dv1pc {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Dv1pc> for bool {
    #[inline(always)]
    fn from(variant: Dv1pc) -> Self {
        variant as u8 != 0
    }
}
///Field `DV1PC` reader - Device1_memory precycle setting
pub type Dv1pcR = crate::BitReader<Dv1pc>;
impl Dv1pcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dv1pc {
        match self.bits {
            false => Dv1pc::_0,
            true => Dv1pc::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dv1pc::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dv1pc::_1
    }
}
///Field `DV1PC` writer - Device1_memory precycle setting
pub type Dv1pcW<'a, REG> = crate::BitWriter<'a, REG, Dv1pc>;
impl<'a, REG> Dv1pcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dv1pc::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dv1pc::_1)
    }
}
/**Automatic calibration memory enable setting for device 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmeme0 {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Acmeme0> for bool {
    #[inline(always)]
    fn from(variant: Acmeme0) -> Self {
        variant as u8 != 0
    }
}
///Field `ACMEME0` reader - Automatic calibration memory enable setting for device 0
pub type Acmeme0R = crate::BitReader<Acmeme0>;
impl Acmeme0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Acmeme0 {
        match self.bits {
            false => Acmeme0::_0,
            true => Acmeme0::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acmeme0::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acmeme0::_1
    }
}
///Field `ACMEME0` writer - Automatic calibration memory enable setting for device 0
pub type Acmeme0W<'a, REG> = crate::BitWriter<'a, REG, Acmeme0>;
impl<'a, REG> Acmeme0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acmeme0::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acmeme0::_1)
    }
}
/**Automatic calibration memory enable setting for device 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmeme1 {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Acmeme1> for bool {
    #[inline(always)]
    fn from(variant: Acmeme1) -> Self {
        variant as u8 != 0
    }
}
///Field `ACMEME1` reader - Automatic calibration memory enable setting for device 1
pub type Acmeme1R = crate::BitReader<Acmeme1>;
impl Acmeme1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Acmeme1 {
        match self.bits {
            false => Acmeme1::_0,
            true => Acmeme1::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acmeme1::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acmeme1::_1
    }
}
///Field `ACMEME1` writer - Automatic calibration memory enable setting for device 1
pub type Acmeme1W<'a, REG> = crate::BitWriter<'a, REG, Acmeme1>;
impl<'a, REG> Acmeme1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acmeme1::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acmeme1::_1)
    }
}
/**Automatic calibration mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acmode {
    ///0: Automatic calibration is disabled
    _00 = 0,
    ///1: Automatic calibration is enabled and modify MDTR
    _01 = 1,
    ///2: Automatic calibration immediately is executed for all trim code, but it will not modify MDTR
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Acmode> for u8 {
    #[inline(always)]
    fn from(variant: Acmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acmode {
    type Ux = u8;
}
impl crate::IsEnum for Acmode {}
///Field `ACMODE` reader - Automatic calibration mode
pub type AcmodeR = crate::FieldReader<Acmode>;
impl AcmodeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Acmode {
        match self.bits {
            0 => Acmode::_00,
            1 => Acmode::_01,
            2 => Acmode::_10,
            3 => Acmode::_11,
            _ => unreachable!(),
        }
    }
    ///Automatic calibration is disabled
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Acmode::_00
    }
    ///Automatic calibration is enabled and modify MDTR
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Acmode::_01
    }
    ///Automatic calibration immediately is executed for all trim code, but it will not modify MDTR
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Acmode::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Acmode::_11
    }
}
///Field `ACMODE` writer - Automatic calibration mode
pub type AcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Acmode, crate::Safe>;
impl<'a, REG> AcmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Automatic calibration is disabled
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Acmode::_00)
    }
    ///Automatic calibration is enabled and modify MDTR
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Acmode::_01)
    }
    ///Automatic calibration immediately is executed for all trim code, but it will not modify MDTR
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Acmode::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Acmode::_11)
    }
}
/**Deadlock Free Timer Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlft {
    ///0: Enable timer
    _0 = 0,
    ///1: Disable timer
    _1 = 1,
}
impl From<Dlft> for bool {
    #[inline(always)]
    fn from(variant: Dlft) -> Self {
        variant as u8 != 0
    }
}
///Field `DLFT` reader - Deadlock Free Timer Enable
pub type DlftR = crate::BitReader<Dlft>;
impl DlftR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dlft {
        match self.bits {
            false => Dlft::_0,
            true => Dlft::_1,
        }
    }
    ///Enable timer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dlft::_0
    }
    ///Disable timer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dlft::_1
    }
}
///Field `DLFT` writer - Deadlock Free Timer Enable
pub type DlftW<'a, REG> = crate::BitWriter<'a, REG, Dlft>;
impl<'a, REG> DlftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable timer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dlft::_0)
    }
    ///Disable timer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlft::_1)
    }
}
impl R {
    ///Bits 0:1 - Device0_transfer_type setting
    #[inline(always)]
    pub fn dv0ttyp(&self) -> Dv0ttypR {
        Dv0ttypR::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Device1_transfer_type setting
    #[inline(always)]
    pub fn dv1ttyp(&self) -> Dv1ttypR {
        Dv1ttypR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - Device0_memory precycle setting
    #[inline(always)]
    pub fn dv0pc(&self) -> Dv0pcR {
        Dv0pcR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Device1_memory precycle setting
    #[inline(always)]
    pub fn dv1pc(&self) -> Dv1pcR {
        Dv1pcR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 10 - Automatic calibration memory enable setting for device 0
    #[inline(always)]
    pub fn acmeme0(&self) -> Acmeme0R {
        Acmeme0R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Automatic calibration memory enable setting for device 1
    #[inline(always)]
    pub fn acmeme1(&self) -> Acmeme1R {
        Acmeme1R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Automatic calibration mode
    #[inline(always)]
    pub fn acmode(&self) -> AcmodeR {
        AcmodeR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 31 - Deadlock Free Timer Enable
    #[inline(always)]
    pub fn dlft(&self) -> DlftR {
        DlftR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDSR")
            .field("dv0ttyp", &self.dv0ttyp())
            .field("dv1ttyp", &self.dv1ttyp())
            .field("dv0pc", &self.dv0pc())
            .field("dv1pc", &self.dv1pc())
            .field("acmeme0", &self.acmeme0())
            .field("acmeme1", &self.acmeme1())
            .field("acmode", &self.acmode())
            .field("dlft", &self.dlft())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Device0_transfer_type setting
    #[inline(always)]
    pub fn dv0ttyp(&mut self) -> Dv0ttypW<CdsrSpec> {
        Dv0ttypW::new(self, 0)
    }
    ///Bits 2:3 - Device1_transfer_type setting
    #[inline(always)]
    pub fn dv1ttyp(&mut self) -> Dv1ttypW<CdsrSpec> {
        Dv1ttypW::new(self, 2)
    }
    ///Bit 4 - Device0_memory precycle setting
    #[inline(always)]
    pub fn dv0pc(&mut self) -> Dv0pcW<CdsrSpec> {
        Dv0pcW::new(self, 4)
    }
    ///Bit 5 - Device1_memory precycle setting
    #[inline(always)]
    pub fn dv1pc(&mut self) -> Dv1pcW<CdsrSpec> {
        Dv1pcW::new(self, 5)
    }
    ///Bit 10 - Automatic calibration memory enable setting for device 0
    #[inline(always)]
    pub fn acmeme0(&mut self) -> Acmeme0W<CdsrSpec> {
        Acmeme0W::new(self, 10)
    }
    ///Bit 11 - Automatic calibration memory enable setting for device 1
    #[inline(always)]
    pub fn acmeme1(&mut self) -> Acmeme1W<CdsrSpec> {
        Acmeme1W::new(self, 11)
    }
    ///Bits 12:13 - Automatic calibration mode
    #[inline(always)]
    pub fn acmode(&mut self) -> AcmodeW<CdsrSpec> {
        AcmodeW::new(self, 12)
    }
    ///Bit 31 - Deadlock Free Timer Enable
    #[inline(always)]
    pub fn dlft(&mut self) -> DlftW<CdsrSpec> {
        DlftW::new(self, 31)
    }
}
/**Controller and Device Setting Register

You can [`read`](crate::Reg::read) this register and get [`cdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CdsrSpec;
impl crate::RegisterSpec for CdsrSpec {
    type Ux = u32;
}
///`read()` method returns [`cdsr::R`](R) reader structure
impl crate::Readable for CdsrSpec {}
///`write(|w| ..)` method takes [`cdsr::W`](W) writer structure
impl crate::Writable for CdsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CDSR to value 0
impl crate::Resettable for CdsrSpec {}
