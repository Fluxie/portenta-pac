///Register `EC710CTL` reader
pub type R = crate::R<Ec710ctlSpec>;
///Register `EC710CTL` writer
pub type W = crate::W<Ec710ctlSpec>;
/**ECC Error Message Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecemf {
    ///0: There is no bit error in present RAM output data
    _0 = 0,
    ///1: There is bit error in present RAM output data
    _1 = 1,
}
impl From<Ecemf> for bool {
    #[inline(always)]
    fn from(variant: Ecemf) -> Self {
        variant as u8 != 0
    }
}
///Field `ECEMF` reader - ECC Error Message Flag
pub type EcemfR = crate::BitReader<Ecemf>;
impl EcemfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ecemf {
        match self.bits {
            false => Ecemf::_0,
            true => Ecemf::_1,
        }
    }
    ///There is no bit error in present RAM output data
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecemf::_0
    }
    ///There is bit error in present RAM output data
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecemf::_1
    }
}
/**ECC Error Detection and Correction Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecer1f {
    ///0: After clearing this bit, 1-bit error correction has not occurred
    _0 = 0,
    ///1: 1-bit error has occurred
    _1 = 1,
}
impl From<Ecer1f> for bool {
    #[inline(always)]
    fn from(variant: Ecer1f) -> Self {
        variant as u8 != 0
    }
}
///Field `ECER1F` reader - ECC Error Detection and Correction Flag
pub type Ecer1fR = crate::BitReader<Ecer1f>;
impl Ecer1fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ecer1f {
        match self.bits {
            false => Ecer1f::_0,
            true => Ecer1f::_1,
        }
    }
    ///After clearing this bit, 1-bit error correction has not occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecer1f::_0
    }
    ///1-bit error has occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecer1f::_1
    }
}
/**2-bit ECC Error Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecer2f {
    ///0: After clearing this bit, 2-bit error has not occurred
    _0 = 0,
    ///1: 2-bit error has occurred
    _1 = 1,
}
impl From<Ecer2f> for bool {
    #[inline(always)]
    fn from(variant: Ecer2f) -> Self {
        variant as u8 != 0
    }
}
///Field `ECER2F` reader - 2-bit ECC Error Detection Flag
pub type Ecer2fR = crate::BitReader<Ecer2f>;
impl Ecer2fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ecer2f {
        match self.bits {
            false => Ecer2f::_0,
            true => Ecer2f::_1,
        }
    }
    ///After clearing this bit, 2-bit error has not occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecer2f::_0
    }
    ///2-bit error has occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecer2f::_1
    }
}
/**ECC 1-bit Error Detection Interrupt Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ec1edic {
    ///0: Disable 1-bit error detection interrupt request
    _0 = 0,
    ///1: Enable 1-bit error detection interrupt request
    _1 = 1,
}
impl From<Ec1edic> for bool {
    #[inline(always)]
    fn from(variant: Ec1edic) -> Self {
        variant as u8 != 0
    }
}
///Field `EC1EDIC` reader - ECC 1-bit Error Detection Interrupt Control
pub type Ec1edicR = crate::BitReader<Ec1edic>;
impl Ec1edicR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ec1edic {
        match self.bits {
            false => Ec1edic::_0,
            true => Ec1edic::_1,
        }
    }
    ///Disable 1-bit error detection interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ec1edic::_0
    }
    ///Enable 1-bit error detection interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ec1edic::_1
    }
}
///Field `EC1EDIC` writer - ECC 1-bit Error Detection Interrupt Control
pub type Ec1edicW<'a, REG> = crate::BitWriter<'a, REG, Ec1edic>;
impl<'a, REG> Ec1edicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable 1-bit error detection interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ec1edic::_0)
    }
    ///Enable 1-bit error detection interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ec1edic::_1)
    }
}
/**ECC 2-bit Error Detection Interrupt Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ec2edic {
    ///0: Disable 2-bit error detection interrupt request
    _0 = 0,
    ///1: Enable 2-bit error detection interrupt request
    _1 = 1,
}
impl From<Ec2edic> for bool {
    #[inline(always)]
    fn from(variant: Ec2edic) -> Self {
        variant as u8 != 0
    }
}
///Field `EC2EDIC` reader - ECC 2-bit Error Detection Interrupt Control
pub type Ec2edicR = crate::BitReader<Ec2edic>;
impl Ec2edicR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ec2edic {
        match self.bits {
            false => Ec2edic::_0,
            true => Ec2edic::_1,
        }
    }
    ///Disable 2-bit error detection interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ec2edic::_0
    }
    ///Enable 2-bit error detection interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ec2edic::_1
    }
}
///Field `EC2EDIC` writer - ECC 2-bit Error Detection Interrupt Control
pub type Ec2edicW<'a, REG> = crate::BitWriter<'a, REG, Ec2edic>;
impl<'a, REG> Ec2edicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable 2-bit error detection interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ec2edic::_0)
    }
    ///Enable 2-bit error detection interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ec2edic::_1)
    }
}
/**ECC 1-bit Error Correction Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ec1ecp {
    ///0: At 1-bit error detection, the error correction is executed
    _0 = 0,
    ///1: At 1-bit error detection, the error correction is not executed
    _1 = 1,
}
impl From<Ec1ecp> for bool {
    #[inline(always)]
    fn from(variant: Ec1ecp) -> Self {
        variant as u8 != 0
    }
}
///Field `EC1ECP` reader - ECC 1-bit Error Correction Permission
pub type Ec1ecpR = crate::BitReader<Ec1ecp>;
impl Ec1ecpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ec1ecp {
        match self.bits {
            false => Ec1ecp::_0,
            true => Ec1ecp::_1,
        }
    }
    ///At 1-bit error detection, the error correction is executed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ec1ecp::_0
    }
    ///At 1-bit error detection, the error correction is not executed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ec1ecp::_1
    }
}
///Field `EC1ECP` writer - ECC 1-bit Error Correction Permission
pub type Ec1ecpW<'a, REG> = crate::BitWriter<'a, REG, Ec1ecp>;
impl<'a, REG> Ec1ecpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///At 1-bit error detection, the error correction is executed
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ec1ecp::_0)
    }
    ///At 1-bit error detection, the error correction is not executed
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ec1ecp::_1)
    }
}
/**ECC Error Judgment Enable Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecervf {
    ///0: Error judgment disable
    _0 = 0,
    ///1: Error judgment enable
    _1 = 1,
}
impl From<Ecervf> for bool {
    #[inline(always)]
    fn from(variant: Ecervf) -> Self {
        variant as u8 != 0
    }
}
///Field `ECERVF` reader - ECC Error Judgment Enable Flag
pub type EcervfR = crate::BitReader<Ecervf>;
impl EcervfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ecervf {
        match self.bits {
            false => Ecervf::_0,
            true => Ecervf::_1,
        }
    }
    ///Error judgment disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecervf::_0
    }
    ///Error judgment enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecervf::_1
    }
}
///Field `ECERVF` writer - ECC Error Judgment Enable Flag
pub type EcervfW<'a, REG> = crate::BitWriter<'a, REG, Ecervf>;
impl<'a, REG> EcervfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error judgment disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecervf::_0)
    }
    ///Error judgment enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecervf::_1)
    }
}
/**Accumulating ECC Error Detection and Correction Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecer1c {
    ///0: No effect
    _0 = 0,
    ///1: Clear accumulating ECC error detection and correction flag
    _1 = 1,
}
impl From<Ecer1c> for bool {
    #[inline(always)]
    fn from(variant: Ecer1c) -> Self {
        variant as u8 != 0
    }
}
///Field `ECER1C` reader - Accumulating ECC Error Detection and Correction Flag Clear
pub type Ecer1cR = crate::BitReader<Ecer1c>;
impl Ecer1cR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ecer1c {
        match self.bits {
            false => Ecer1c::_0,
            true => Ecer1c::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecer1c::_0
    }
    ///Clear accumulating ECC error detection and correction flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecer1c::_1
    }
}
///Field `ECER1C` writer - Accumulating ECC Error Detection and Correction Flag Clear
pub type Ecer1cW<'a, REG> = crate::BitWriter<'a, REG, Ecer1c>;
impl<'a, REG> Ecer1cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecer1c::_0)
    }
    ///Clear accumulating ECC error detection and correction flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecer1c::_1)
    }
}
/**2-bit ECC Error Detection Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecer2c {
    ///0: No effect
    _0 = 0,
    ///1: Clear 2-bit ECC error detection flag
    _1 = 1,
}
impl From<Ecer2c> for bool {
    #[inline(always)]
    fn from(variant: Ecer2c) -> Self {
        variant as u8 != 0
    }
}
///Field `ECER2C` reader - 2-bit ECC Error Detection Flag Clear
pub type Ecer2cR = crate::BitReader<Ecer2c>;
impl Ecer2cR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ecer2c {
        match self.bits {
            false => Ecer2c::_0,
            true => Ecer2c::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecer2c::_0
    }
    ///Clear 2-bit ECC error detection flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecer2c::_1
    }
}
///Field `ECER2C` writer - 2-bit ECC Error Detection Flag Clear
pub type Ecer2cW<'a, REG> = crate::BitWriter<'a, REG, Ecer2c>;
impl<'a, REG> Ecer2cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecer2c::_0)
    }
    ///Clear 2-bit ECC error detection flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecer2c::_1)
    }
}
/**ECC Overflow Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecovff {
    ///0: No effect
    _0 = 0,
    ///1: ECC overflow detection flag
    _1 = 1,
}
impl From<Ecovff> for bool {
    #[inline(always)]
    fn from(variant: Ecovff) -> Self {
        variant as u8 != 0
    }
}
///Field `ECOVFF` reader - ECC Overflow Detection Flag
pub type EcovffR = crate::BitReader<Ecovff>;
impl EcovffR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ecovff {
        match self.bits {
            false => Ecovff::_0,
            true => Ecovff::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecovff::_0
    }
    ///ECC overflow detection flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecovff::_1
    }
}
///Field `EMCA` reader - Access Control to ECC Mode Select bit
pub type EmcaR = crate::FieldReader;
///Field `EMCA` writer - Access Control to ECC Mode Select bit
pub type EmcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**ECC Single bit Error Address Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecsedf0 {
    ///0: There is no bit error in EC710EAD0 after reset or clearing ECER1F bit
    _0 = 0,
    ///1: Address captured in EC710EAD0 shows that 1-bit error occurred and captured
    _1 = 1,
}
impl From<Ecsedf0> for bool {
    #[inline(always)]
    fn from(variant: Ecsedf0) -> Self {
        variant as u8 != 0
    }
}
///Field `ECSEDF0` reader - ECC Single bit Error Address Detection Flag
pub type Ecsedf0R = crate::BitReader<Ecsedf0>;
impl Ecsedf0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ecsedf0 {
        match self.bits {
            false => Ecsedf0::_0,
            true => Ecsedf0::_1,
        }
    }
    ///There is no bit error in EC710EAD0 after reset or clearing ECER1F bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecsedf0::_0
    }
    ///Address captured in EC710EAD0 shows that 1-bit error occurred and captured
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecsedf0::_1
    }
}
/**ECC Dual Bit Error Address Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecdedf0 {
    ///0: There is no bit error in EC710EAD0 after reset or clearing ECER2F bit
    _0 = 0,
    ///1: Address captured in EC710EAD0 shows that 2-bit error occurred and captured
    _1 = 1,
}
impl From<Ecdedf0> for bool {
    #[inline(always)]
    fn from(variant: Ecdedf0) -> Self {
        variant as u8 != 0
    }
}
///Field `ECDEDF0` reader - ECC Dual Bit Error Address Detection Flag
pub type Ecdedf0R = crate::BitReader<Ecdedf0>;
impl Ecdedf0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ecdedf0 {
        match self.bits {
            false => Ecdedf0::_0,
            true => Ecdedf0::_1,
        }
    }
    ///There is no bit error in EC710EAD0 after reset or clearing ECER2F bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecdedf0::_0
    }
    ///Address captured in EC710EAD0 shows that 2-bit error occurred and captured
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecdedf0::_1
    }
}
impl R {
    ///Bit 0 - ECC Error Message Flag
    #[inline(always)]
    pub fn ecemf(&self) -> EcemfR {
        EcemfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECC Error Detection and Correction Flag
    #[inline(always)]
    pub fn ecer1f(&self) -> Ecer1fR {
        Ecer1fR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 2-bit ECC Error Detection Flag
    #[inline(always)]
    pub fn ecer2f(&self) -> Ecer2fR {
        Ecer2fR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC 1-bit Error Detection Interrupt Control
    #[inline(always)]
    pub fn ec1edic(&self) -> Ec1edicR {
        Ec1edicR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ECC 2-bit Error Detection Interrupt Control
    #[inline(always)]
    pub fn ec2edic(&self) -> Ec2edicR {
        Ec2edicR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ECC 1-bit Error Correction Permission
    #[inline(always)]
    pub fn ec1ecp(&self) -> Ec1ecpR {
        Ec1ecpR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ECC Error Judgment Enable Flag
    #[inline(always)]
    pub fn ecervf(&self) -> EcervfR {
        EcervfR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Accumulating ECC Error Detection and Correction Flag Clear
    #[inline(always)]
    pub fn ecer1c(&self) -> Ecer1cR {
        Ecer1cR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - 2-bit ECC Error Detection Flag Clear
    #[inline(always)]
    pub fn ecer2c(&self) -> Ecer2cR {
        Ecer2cR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ECC Overflow Detection Flag
    #[inline(always)]
    pub fn ecovff(&self) -> EcovffR {
        EcovffR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 14:15 - Access Control to ECC Mode Select bit
    #[inline(always)]
    pub fn emca(&self) -> EmcaR {
        EmcaR::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - ECC Single bit Error Address Detection Flag
    #[inline(always)]
    pub fn ecsedf0(&self) -> Ecsedf0R {
        Ecsedf0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ECC Dual Bit Error Address Detection Flag
    #[inline(always)]
    pub fn ecdedf0(&self) -> Ecdedf0R {
        Ecdedf0R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EC710CTL")
            .field("ecemf", &self.ecemf())
            .field("ecer1f", &self.ecer1f())
            .field("ecer2f", &self.ecer2f())
            .field("ec1edic", &self.ec1edic())
            .field("ec2edic", &self.ec2edic())
            .field("ec1ecp", &self.ec1ecp())
            .field("ecervf", &self.ecervf())
            .field("ecer1c", &self.ecer1c())
            .field("ecer2c", &self.ecer2c())
            .field("ecovff", &self.ecovff())
            .field("emca", &self.emca())
            .field("ecsedf0", &self.ecsedf0())
            .field("ecdedf0", &self.ecdedf0())
            .finish()
    }
}
impl W {
    ///Bit 3 - ECC 1-bit Error Detection Interrupt Control
    #[inline(always)]
    pub fn ec1edic(&mut self) -> Ec1edicW<Ec710ctlSpec> {
        Ec1edicW::new(self, 3)
    }
    ///Bit 4 - ECC 2-bit Error Detection Interrupt Control
    #[inline(always)]
    pub fn ec2edic(&mut self) -> Ec2edicW<Ec710ctlSpec> {
        Ec2edicW::new(self, 4)
    }
    ///Bit 5 - ECC 1-bit Error Correction Permission
    #[inline(always)]
    pub fn ec1ecp(&mut self) -> Ec1ecpW<Ec710ctlSpec> {
        Ec1ecpW::new(self, 5)
    }
    ///Bit 6 - ECC Error Judgment Enable Flag
    #[inline(always)]
    pub fn ecervf(&mut self) -> EcervfW<Ec710ctlSpec> {
        EcervfW::new(self, 6)
    }
    ///Bit 9 - Accumulating ECC Error Detection and Correction Flag Clear
    #[inline(always)]
    pub fn ecer1c(&mut self) -> Ecer1cW<Ec710ctlSpec> {
        Ecer1cW::new(self, 9)
    }
    ///Bit 10 - 2-bit ECC Error Detection Flag Clear
    #[inline(always)]
    pub fn ecer2c(&mut self) -> Ecer2cW<Ec710ctlSpec> {
        Ecer2cW::new(self, 10)
    }
    ///Bits 14:15 - Access Control to ECC Mode Select bit
    #[inline(always)]
    pub fn emca(&mut self) -> EmcaW<Ec710ctlSpec> {
        EmcaW::new(self, 14)
    }
}
/**ECC Control Register

You can [`read`](crate::Reg::read) this register and get [`ec710ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ec710ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ec710ctlSpec;
impl crate::RegisterSpec for Ec710ctlSpec {
    type Ux = u32;
}
///`read()` method returns [`ec710ctl::R`](R) reader structure
impl crate::Readable for Ec710ctlSpec {}
///`write(|w| ..)` method takes [`ec710ctl::W`](W) writer structure
impl crate::Writable for Ec710ctlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EC710CTL to value 0x10
impl crate::Resettable for Ec710ctlSpec {
    const RESET_VALUE: u32 = 0x10;
}
