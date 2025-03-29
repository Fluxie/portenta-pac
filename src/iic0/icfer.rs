///Register `ICFER` reader
pub type R = crate::R<IcferSpec>;
///Register `ICFER` writer
pub type W = crate::W<IcferSpec>;
/**Timeout Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmoe {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Tmoe> for bool {
    #[inline(always)]
    fn from(variant: Tmoe) -> Self {
        variant as u8 != 0
    }
}
///Field `TMOE` reader - Timeout Function Enable
pub type TmoeR = crate::BitReader<Tmoe>;
impl TmoeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmoe {
        match self.bits {
            false => Tmoe::_0,
            true => Tmoe::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmoe::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmoe::_1
    }
}
///Field `TMOE` writer - Timeout Function Enable
pub type TmoeW<'a, REG> = crate::BitWriter<'a, REG, Tmoe>;
impl<'a, REG> TmoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmoe::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmoe::_1)
    }
}
/**Master Arbitration-Lost Detection Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Male {
    ///0: Disable the arbitration-lost detection function and disable automatic clearing of the MST and TRS bits in ICCR2 when arbitration is lost
    _0 = 0,
    ///1: Enable the arbitration-lost detection function and enable automatic clearing of the MST and TRS bits in ICCR2 when arbitration is lost
    _1 = 1,
}
impl From<Male> for bool {
    #[inline(always)]
    fn from(variant: Male) -> Self {
        variant as u8 != 0
    }
}
///Field `MALE` reader - Master Arbitration-Lost Detection Enable
pub type MaleR = crate::BitReader<Male>;
impl MaleR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Male {
        match self.bits {
            false => Male::_0,
            true => Male::_1,
        }
    }
    ///Disable the arbitration-lost detection function and disable automatic clearing of the MST and TRS bits in ICCR2 when arbitration is lost
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Male::_0
    }
    ///Enable the arbitration-lost detection function and enable automatic clearing of the MST and TRS bits in ICCR2 when arbitration is lost
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Male::_1
    }
}
///Field `MALE` writer - Master Arbitration-Lost Detection Enable
pub type MaleW<'a, REG> = crate::BitWriter<'a, REG, Male>;
impl<'a, REG> MaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the arbitration-lost detection function and disable automatic clearing of the MST and TRS bits in ICCR2 when arbitration is lost
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Male::_0)
    }
    ///Enable the arbitration-lost detection function and enable automatic clearing of the MST and TRS bits in ICCR2 when arbitration is lost
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Male::_1)
    }
}
/**NACK Transmission Arbitration-Lost Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nale {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Nale> for bool {
    #[inline(always)]
    fn from(variant: Nale) -> Self {
        variant as u8 != 0
    }
}
///Field `NALE` reader - NACK Transmission Arbitration-Lost Detection Enable
pub type NaleR = crate::BitReader<Nale>;
impl NaleR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nale {
        match self.bits {
            false => Nale::_0,
            true => Nale::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nale::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nale::_1
    }
}
///Field `NALE` writer - NACK Transmission Arbitration-Lost Detection Enable
pub type NaleW<'a, REG> = crate::BitWriter<'a, REG, Nale>;
impl<'a, REG> NaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nale::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nale::_1)
    }
}
/**Slave Arbitration-Lost Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sale {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Sale> for bool {
    #[inline(always)]
    fn from(variant: Sale) -> Self {
        variant as u8 != 0
    }
}
///Field `SALE` reader - Slave Arbitration-Lost Detection Enable
pub type SaleR = crate::BitReader<Sale>;
impl SaleR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sale {
        match self.bits {
            false => Sale::_0,
            true => Sale::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sale::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sale::_1
    }
}
///Field `SALE` writer - Slave Arbitration-Lost Detection Enable
pub type SaleW<'a, REG> = crate::BitWriter<'a, REG, Sale>;
impl<'a, REG> SaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sale::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sale::_1)
    }
}
/**NACK Reception Transfer Suspension Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nacke {
    ///0: Do not suspend transfer operation during NACK reception (disable transfer suspension)
    _0 = 0,
    ///1: Suspend transfer operation during NACK reception (enable transfer suspension)
    _1 = 1,
}
impl From<Nacke> for bool {
    #[inline(always)]
    fn from(variant: Nacke) -> Self {
        variant as u8 != 0
    }
}
///Field `NACKE` reader - NACK Reception Transfer Suspension Enable
pub type NackeR = crate::BitReader<Nacke>;
impl NackeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nacke {
        match self.bits {
            false => Nacke::_0,
            true => Nacke::_1,
        }
    }
    ///Do not suspend transfer operation during NACK reception (disable transfer suspension)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nacke::_0
    }
    ///Suspend transfer operation during NACK reception (enable transfer suspension)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nacke::_1
    }
}
///Field `NACKE` writer - NACK Reception Transfer Suspension Enable
pub type NackeW<'a, REG> = crate::BitWriter<'a, REG, Nacke>;
impl<'a, REG> NackeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not suspend transfer operation during NACK reception (disable transfer suspension)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nacke::_0)
    }
    ///Suspend transfer operation during NACK reception (enable transfer suspension)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nacke::_1)
    }
}
/**Digital Noise Filter Circuit Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfe {
    ///0: Do not use the digital noise filter circuit
    _0 = 0,
    ///1: Use the digital noise filter circuit
    _1 = 1,
}
impl From<Nfe> for bool {
    #[inline(always)]
    fn from(variant: Nfe) -> Self {
        variant as u8 != 0
    }
}
///Field `NFE` reader - Digital Noise Filter Circuit Enable
pub type NfeR = crate::BitReader<Nfe>;
impl NfeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nfe {
        match self.bits {
            false => Nfe::_0,
            true => Nfe::_1,
        }
    }
    ///Do not use the digital noise filter circuit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfe::_0
    }
    ///Use the digital noise filter circuit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfe::_1
    }
}
///Field `NFE` writer - Digital Noise Filter Circuit Enable
pub type NfeW<'a, REG> = crate::BitWriter<'a, REG, Nfe>;
impl<'a, REG> NfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use the digital noise filter circuit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfe::_0)
    }
    ///Use the digital noise filter circuit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfe::_1)
    }
}
/**SCL Synchronous Circuit Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scle {
    ///0: Do not use the SCL synchronous circuit
    _0 = 0,
    ///1: Use the SCL synchronous circuit
    _1 = 1,
}
impl From<Scle> for bool {
    #[inline(always)]
    fn from(variant: Scle) -> Self {
        variant as u8 != 0
    }
}
///Field `SCLE` reader - SCL Synchronous Circuit Enable
pub type ScleR = crate::BitReader<Scle>;
impl ScleR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Scle {
        match self.bits {
            false => Scle::_0,
            true => Scle::_1,
        }
    }
    ///Do not use the SCL synchronous circuit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Scle::_0
    }
    ///Use the SCL synchronous circuit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Scle::_1
    }
}
///Field `SCLE` writer - SCL Synchronous Circuit Enable
pub type ScleW<'a, REG> = crate::BitWriter<'a, REG, Scle>;
impl<'a, REG> ScleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use the SCL synchronous circuit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Scle::_0)
    }
    ///Use the SCL synchronous circuit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Scle::_1)
    }
}
/**Fast-Mode Plus Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmpe {
    ///0: Do not use the Fm+ slope control circuit for the SCLn and SDAn pins
    _0 = 0,
    ///1: Use the Fm+ slope control circuit for the SCLn and SDAn pins.
    _1 = 1,
}
impl From<Fmpe> for bool {
    #[inline(always)]
    fn from(variant: Fmpe) -> Self {
        variant as u8 != 0
    }
}
///Field `FMPE` reader - Fast-Mode Plus Enable
pub type FmpeR = crate::BitReader<Fmpe>;
impl FmpeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fmpe {
        match self.bits {
            false => Fmpe::_0,
            true => Fmpe::_1,
        }
    }
    ///Do not use the Fm+ slope control circuit for the SCLn and SDAn pins
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fmpe::_0
    }
    ///Use the Fm+ slope control circuit for the SCLn and SDAn pins.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fmpe::_1
    }
}
///Field `FMPE` writer - Fast-Mode Plus Enable
pub type FmpeW<'a, REG> = crate::BitWriter<'a, REG, Fmpe>;
impl<'a, REG> FmpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use the Fm+ slope control circuit for the SCLn and SDAn pins
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fmpe::_0)
    }
    ///Use the Fm+ slope control circuit for the SCLn and SDAn pins.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fmpe::_1)
    }
}
impl R {
    ///Bit 0 - Timeout Function Enable
    #[inline(always)]
    pub fn tmoe(&self) -> TmoeR {
        TmoeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Master Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn male(&self) -> MaleR {
        MaleR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NACK Transmission Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn nale(&self) -> NaleR {
        NaleR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Slave Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn sale(&self) -> SaleR {
        SaleR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NACK Reception Transfer Suspension Enable
    #[inline(always)]
    pub fn nacke(&self) -> NackeR {
        NackeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Digital Noise Filter Circuit Enable
    #[inline(always)]
    pub fn nfe(&self) -> NfeR {
        NfeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SCL Synchronous Circuit Enable
    #[inline(always)]
    pub fn scle(&self) -> ScleR {
        ScleR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Fast-Mode Plus Enable
    #[inline(always)]
    pub fn fmpe(&self) -> FmpeR {
        FmpeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICFER")
            .field("tmoe", &self.tmoe())
            .field("male", &self.male())
            .field("nale", &self.nale())
            .field("sale", &self.sale())
            .field("nacke", &self.nacke())
            .field("nfe", &self.nfe())
            .field("scle", &self.scle())
            .field("fmpe", &self.fmpe())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timeout Function Enable
    #[inline(always)]
    pub fn tmoe(&mut self) -> TmoeW<IcferSpec> {
        TmoeW::new(self, 0)
    }
    ///Bit 1 - Master Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn male(&mut self) -> MaleW<IcferSpec> {
        MaleW::new(self, 1)
    }
    ///Bit 2 - NACK Transmission Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn nale(&mut self) -> NaleW<IcferSpec> {
        NaleW::new(self, 2)
    }
    ///Bit 3 - Slave Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn sale(&mut self) -> SaleW<IcferSpec> {
        SaleW::new(self, 3)
    }
    ///Bit 4 - NACK Reception Transfer Suspension Enable
    #[inline(always)]
    pub fn nacke(&mut self) -> NackeW<IcferSpec> {
        NackeW::new(self, 4)
    }
    ///Bit 5 - Digital Noise Filter Circuit Enable
    #[inline(always)]
    pub fn nfe(&mut self) -> NfeW<IcferSpec> {
        NfeW::new(self, 5)
    }
    ///Bit 6 - SCL Synchronous Circuit Enable
    #[inline(always)]
    pub fn scle(&mut self) -> ScleW<IcferSpec> {
        ScleW::new(self, 6)
    }
    ///Bit 7 - Fast-Mode Plus Enable
    #[inline(always)]
    pub fn fmpe(&mut self) -> FmpeW<IcferSpec> {
        FmpeW::new(self, 7)
    }
}
/**I2C Bus Function Enable Register

You can [`read`](crate::Reg::read) this register and get [`icfer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcferSpec;
impl crate::RegisterSpec for IcferSpec {
    type Ux = u8;
}
///`read()` method returns [`icfer::R`](R) reader structure
impl crate::Readable for IcferSpec {}
///`write(|w| ..)` method takes [`icfer::W`](W) writer structure
impl crate::Writable for IcferSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICFER to value 0x72
impl crate::Resettable for IcferSpec {
    const RESET_VALUE: u8 = 0x72;
}
