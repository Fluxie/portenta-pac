///Register `ECSIPR` reader
pub type R = crate::R<EcsiprSpec>;
///Register `ECSIPR` writer
pub type W = crate::W<EcsiprSpec>;
/**False Carrier Detect Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icdip {
    ///0: Disable interrupt notification
    _0 = 0,
    ///1: Enable interrupt notification.
    _1 = 1,
}
impl From<Icdip> for bool {
    #[inline(always)]
    fn from(variant: Icdip) -> Self {
        variant as u8 != 0
    }
}
///Field `ICDIP` reader - False Carrier Detect Interrupt Enable
pub type IcdipR = crate::BitReader<Icdip>;
impl IcdipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Icdip {
        match self.bits {
            false => Icdip::_0,
            true => Icdip::_1,
        }
    }
    ///Disable interrupt notification
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Icdip::_0
    }
    ///Enable interrupt notification.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Icdip::_1
    }
}
///Field `ICDIP` writer - False Carrier Detect Interrupt Enable
pub type IcdipW<'a, REG> = crate::BitWriter<'a, REG, Icdip>;
impl<'a, REG> IcdipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt notification
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Icdip::_0)
    }
    ///Enable interrupt notification.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Icdip::_1)
    }
}
/**Magic Packet Detect Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpdip {
    ///0: Disable interrupt notification
    _0 = 0,
    ///1: Enable interrupt notification.
    _1 = 1,
}
impl From<Mpdip> for bool {
    #[inline(always)]
    fn from(variant: Mpdip) -> Self {
        variant as u8 != 0
    }
}
///Field `MPDIP` reader - Magic Packet Detect Interrupt Enable
pub type MpdipR = crate::BitReader<Mpdip>;
impl MpdipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mpdip {
        match self.bits {
            false => Mpdip::_0,
            true => Mpdip::_1,
        }
    }
    ///Disable interrupt notification
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpdip::_0
    }
    ///Enable interrupt notification.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mpdip::_1
    }
}
///Field `MPDIP` writer - Magic Packet Detect Interrupt Enable
pub type MpdipW<'a, REG> = crate::BitWriter<'a, REG, Mpdip>;
impl<'a, REG> MpdipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt notification
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpdip::_0)
    }
    ///Enable interrupt notification.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpdip::_1)
    }
}
/**LINK Signal Change Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lchngip {
    ///0: Disable interrupt notification
    _0 = 0,
    ///1: Enable interrupt notification.
    _1 = 1,
}
impl From<Lchngip> for bool {
    #[inline(always)]
    fn from(variant: Lchngip) -> Self {
        variant as u8 != 0
    }
}
///Field `LCHNGIP` reader - LINK Signal Change Interrupt Enable
pub type LchngipR = crate::BitReader<Lchngip>;
impl LchngipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lchngip {
        match self.bits {
            false => Lchngip::_0,
            true => Lchngip::_1,
        }
    }
    ///Disable interrupt notification
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lchngip::_0
    }
    ///Enable interrupt notification.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lchngip::_1
    }
}
///Field `LCHNGIP` writer - LINK Signal Change Interrupt Enable
pub type LchngipW<'a, REG> = crate::BitWriter<'a, REG, Lchngip>;
impl<'a, REG> LchngipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt notification
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lchngip::_0)
    }
    ///Enable interrupt notification.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lchngip::_1)
    }
}
/**PAUSE Frame Retransmit Over Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psrtoip {
    ///0: Disable interrupt notification
    _0 = 0,
    ///1: Enable interrupt notification.
    _1 = 1,
}
impl From<Psrtoip> for bool {
    #[inline(always)]
    fn from(variant: Psrtoip) -> Self {
        variant as u8 != 0
    }
}
///Field `PSRTOIP` reader - PAUSE Frame Retransmit Over Interrupt Enable
pub type PsrtoipR = crate::BitReader<Psrtoip>;
impl PsrtoipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psrtoip {
        match self.bits {
            false => Psrtoip::_0,
            true => Psrtoip::_1,
        }
    }
    ///Disable interrupt notification
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psrtoip::_0
    }
    ///Enable interrupt notification.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psrtoip::_1
    }
}
///Field `PSRTOIP` writer - PAUSE Frame Retransmit Over Interrupt Enable
pub type PsrtoipW<'a, REG> = crate::BitWriter<'a, REG, Psrtoip>;
impl<'a, REG> PsrtoipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt notification
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psrtoip::_0)
    }
    ///Enable interrupt notification.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psrtoip::_1)
    }
}
/**Continuous Broadcast Frame Reception Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfsipr {
    ///0: Disable interrupt notification
    _0 = 0,
    ///1: Enable interrupt notification.
    _1 = 1,
}
impl From<Bfsipr> for bool {
    #[inline(always)]
    fn from(variant: Bfsipr) -> Self {
        variant as u8 != 0
    }
}
///Field `BFSIPR` reader - Continuous Broadcast Frame Reception Interrupt Enable
pub type BfsiprR = crate::BitReader<Bfsipr>;
impl BfsiprR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bfsipr {
        match self.bits {
            false => Bfsipr::_0,
            true => Bfsipr::_1,
        }
    }
    ///Disable interrupt notification
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bfsipr::_0
    }
    ///Enable interrupt notification.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bfsipr::_1
    }
}
///Field `BFSIPR` writer - Continuous Broadcast Frame Reception Interrupt Enable
pub type BfsiprW<'a, REG> = crate::BitWriter<'a, REG, Bfsipr>;
impl<'a, REG> BfsiprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt notification
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bfsipr::_0)
    }
    ///Enable interrupt notification.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfsipr::_1)
    }
}
impl R {
    ///Bit 0 - False Carrier Detect Interrupt Enable
    #[inline(always)]
    pub fn icdip(&self) -> IcdipR {
        IcdipR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Magic Packet Detect Interrupt Enable
    #[inline(always)]
    pub fn mpdip(&self) -> MpdipR {
        MpdipR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LINK Signal Change Interrupt Enable
    #[inline(always)]
    pub fn lchngip(&self) -> LchngipR {
        LchngipR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - PAUSE Frame Retransmit Over Interrupt Enable
    #[inline(always)]
    pub fn psrtoip(&self) -> PsrtoipR {
        PsrtoipR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Continuous Broadcast Frame Reception Interrupt Enable
    #[inline(always)]
    pub fn bfsipr(&self) -> BfsiprR {
        BfsiprR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECSIPR")
            .field("icdip", &self.icdip())
            .field("mpdip", &self.mpdip())
            .field("lchngip", &self.lchngip())
            .field("psrtoip", &self.psrtoip())
            .field("bfsipr", &self.bfsipr())
            .finish()
    }
}
impl W {
    ///Bit 0 - False Carrier Detect Interrupt Enable
    #[inline(always)]
    pub fn icdip(&mut self) -> IcdipW<EcsiprSpec> {
        IcdipW::new(self, 0)
    }
    ///Bit 1 - Magic Packet Detect Interrupt Enable
    #[inline(always)]
    pub fn mpdip(&mut self) -> MpdipW<EcsiprSpec> {
        MpdipW::new(self, 1)
    }
    ///Bit 2 - LINK Signal Change Interrupt Enable
    #[inline(always)]
    pub fn lchngip(&mut self) -> LchngipW<EcsiprSpec> {
        LchngipW::new(self, 2)
    }
    ///Bit 4 - PAUSE Frame Retransmit Over Interrupt Enable
    #[inline(always)]
    pub fn psrtoip(&mut self) -> PsrtoipW<EcsiprSpec> {
        PsrtoipW::new(self, 4)
    }
    ///Bit 5 - Continuous Broadcast Frame Reception Interrupt Enable
    #[inline(always)]
    pub fn bfsipr(&mut self) -> BfsiprW<EcsiprSpec> {
        BfsiprW::new(self, 5)
    }
}
/**ETHERC Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ecsipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EcsiprSpec;
impl crate::RegisterSpec for EcsiprSpec {
    type Ux = u32;
}
///`read()` method returns [`ecsipr::R`](R) reader structure
impl crate::Readable for EcsiprSpec {}
///`write(|w| ..)` method takes [`ecsipr::W`](W) writer structure
impl crate::Writable for EcsiprSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECSIPR to value 0
impl crate::Resettable for EcsiprSpec {}
