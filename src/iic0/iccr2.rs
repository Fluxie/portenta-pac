///Register `ICCR2` reader
pub type R = crate::R<Iccr2Spec>;
///Register `ICCR2` writer
pub type W = crate::W<Iccr2Spec>;
/**Start Condition Issuance Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum St {
    ///0: Do not issue a start condition request
    _0 = 0,
    ///1: Issue a start condition request
    _1 = 1,
}
impl From<St> for bool {
    #[inline(always)]
    fn from(variant: St) -> Self {
        variant as u8 != 0
    }
}
///Field `ST` reader - Start Condition Issuance Request
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
    ///Do not issue a start condition request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == St::_0
    }
    ///Issue a start condition request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == St::_1
    }
}
///Field `ST` writer - Start Condition Issuance Request
pub type StW<'a, REG> = crate::BitWriter<'a, REG, St>;
impl<'a, REG> StW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not issue a start condition request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(St::_0)
    }
    ///Issue a start condition request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(St::_1)
    }
}
/**Restart Condition Issuance Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rs {
    ///0: Do not issue a restart condition request
    _0 = 0,
    ///1: Issue a restart condition request
    _1 = 1,
}
impl From<Rs> for bool {
    #[inline(always)]
    fn from(variant: Rs) -> Self {
        variant as u8 != 0
    }
}
///Field `RS` reader - Restart Condition Issuance Request
pub type RsR = crate::BitReader<Rs>;
impl RsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rs {
        match self.bits {
            false => Rs::_0,
            true => Rs::_1,
        }
    }
    ///Do not issue a restart condition request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rs::_0
    }
    ///Issue a restart condition request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rs::_1
    }
}
///Field `RS` writer - Restart Condition Issuance Request
pub type RsW<'a, REG> = crate::BitWriter<'a, REG, Rs>;
impl<'a, REG> RsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not issue a restart condition request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rs::_0)
    }
    ///Issue a restart condition request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rs::_1)
    }
}
/**Stop Condition Issuance Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sp {
    ///0: Do not issue a stop condition request
    _0 = 0,
    ///1: Issue a stop condition request
    _1 = 1,
}
impl From<Sp> for bool {
    #[inline(always)]
    fn from(variant: Sp) -> Self {
        variant as u8 != 0
    }
}
///Field `SP` reader - Stop Condition Issuance Request
pub type SpR = crate::BitReader<Sp>;
impl SpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sp {
        match self.bits {
            false => Sp::_0,
            true => Sp::_1,
        }
    }
    ///Do not issue a stop condition request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sp::_0
    }
    ///Issue a stop condition request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sp::_1
    }
}
///Field `SP` writer - Stop Condition Issuance Request
pub type SpW<'a, REG> = crate::BitWriter<'a, REG, Sp>;
impl<'a, REG> SpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not issue a stop condition request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sp::_0)
    }
    ///Issue a stop condition request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sp::_1)
    }
}
/**Transmit/Receive Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trs {
    ///0: Receive mode
    _0 = 0,
    ///1: Transmit mode
    _1 = 1,
}
impl From<Trs> for bool {
    #[inline(always)]
    fn from(variant: Trs) -> Self {
        variant as u8 != 0
    }
}
///Field `TRS` reader - Transmit/Receive Mode
pub type TrsR = crate::BitReader<Trs>;
impl TrsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Trs {
        match self.bits {
            false => Trs::_0,
            true => Trs::_1,
        }
    }
    ///Receive mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trs::_0
    }
    ///Transmit mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trs::_1
    }
}
///Field `TRS` writer - Transmit/Receive Mode
pub type TrsW<'a, REG> = crate::BitWriter<'a, REG, Trs>;
impl<'a, REG> TrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trs::_0)
    }
    ///Transmit mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trs::_1)
    }
}
/**Master/Slave Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mst {
    ///0: Slave mode
    _0 = 0,
    ///1: Master mode
    _1 = 1,
}
impl From<Mst> for bool {
    #[inline(always)]
    fn from(variant: Mst) -> Self {
        variant as u8 != 0
    }
}
///Field `MST` reader - Master/Slave Mode
pub type MstR = crate::BitReader<Mst>;
impl MstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mst {
        match self.bits {
            false => Mst::_0,
            true => Mst::_1,
        }
    }
    ///Slave mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mst::_0
    }
    ///Master mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mst::_1
    }
}
///Field `MST` writer - Master/Slave Mode
pub type MstW<'a, REG> = crate::BitWriter<'a, REG, Mst>;
impl<'a, REG> MstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mst::_0)
    }
    ///Master mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mst::_1)
    }
}
/**Bus Busy Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bbsy {
    ///0: I2C bus released (bus free state)
    _0 = 0,
    ///1: I2C bus occupied (bus busy state)
    _1 = 1,
}
impl From<Bbsy> for bool {
    #[inline(always)]
    fn from(variant: Bbsy) -> Self {
        variant as u8 != 0
    }
}
///Field `BBSY` reader - Bus Busy Detection Flag
pub type BbsyR = crate::BitReader<Bbsy>;
impl BbsyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bbsy {
        match self.bits {
            false => Bbsy::_0,
            true => Bbsy::_1,
        }
    }
    ///I2C bus released (bus free state)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bbsy::_0
    }
    ///I2C bus occupied (bus busy state)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bbsy::_1
    }
}
impl R {
    ///Bit 1 - Start Condition Issuance Request
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Restart Condition Issuance Request
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Stop Condition Issuance Request
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Transmit/Receive Mode
    #[inline(always)]
    pub fn trs(&self) -> TrsR {
        TrsR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Master/Slave Mode
    #[inline(always)]
    pub fn mst(&self) -> MstR {
        MstR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Bus Busy Detection Flag
    #[inline(always)]
    pub fn bbsy(&self) -> BbsyR {
        BbsyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICCR2")
            .field("st", &self.st())
            .field("rs", &self.rs())
            .field("sp", &self.sp())
            .field("trs", &self.trs())
            .field("mst", &self.mst())
            .field("bbsy", &self.bbsy())
            .finish()
    }
}
impl W {
    ///Bit 1 - Start Condition Issuance Request
    #[inline(always)]
    pub fn st(&mut self) -> StW<Iccr2Spec> {
        StW::new(self, 1)
    }
    ///Bit 2 - Restart Condition Issuance Request
    #[inline(always)]
    pub fn rs(&mut self) -> RsW<Iccr2Spec> {
        RsW::new(self, 2)
    }
    ///Bit 3 - Stop Condition Issuance Request
    #[inline(always)]
    pub fn sp(&mut self) -> SpW<Iccr2Spec> {
        SpW::new(self, 3)
    }
    ///Bit 5 - Transmit/Receive Mode
    #[inline(always)]
    pub fn trs(&mut self) -> TrsW<Iccr2Spec> {
        TrsW::new(self, 5)
    }
    ///Bit 6 - Master/Slave Mode
    #[inline(always)]
    pub fn mst(&mut self) -> MstW<Iccr2Spec> {
        MstW::new(self, 6)
    }
}
/**I2C Bus Control Register 2

You can [`read`](crate::Reg::read) this register and get [`iccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Iccr2Spec;
impl crate::RegisterSpec for Iccr2Spec {
    type Ux = u8;
}
///`read()` method returns [`iccr2::R`](R) reader structure
impl crate::Readable for Iccr2Spec {}
///`write(|w| ..)` method takes [`iccr2::W`](W) writer structure
impl crate::Writable for Iccr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICCR2 to value 0
impl crate::Resettable for Iccr2Spec {}
