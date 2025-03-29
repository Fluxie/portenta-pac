///Register `CECES` reader
pub type R = crate::R<CecesSpec>;
///Register `CECES` writer
pub type W = crate::W<CecesSpec>;
/**Overrun Error Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oerr {
    ///0: No overrun error has occurred.
    _0 = 0,
    ///1: An overrun error has occurred.
    _1 = 1,
}
impl From<Oerr> for bool {
    #[inline(always)]
    fn from(variant: Oerr) -> Self {
        variant as u8 != 0
    }
}
///Field `OERR` reader - Overrun Error Detection Flag
pub type OerrR = crate::BitReader<Oerr>;
impl OerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oerr {
        match self.bits {
            false => Oerr::_0,
            true => Oerr::_1,
        }
    }
    ///No overrun error has occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oerr::_0
    }
    ///An overrun error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oerr::_1
    }
}
/**Underrun Error Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uerr {
    ///0: No underrun error has occurred.
    _0 = 0,
    ///1: An underrun error has occurred.
    _1 = 1,
}
impl From<Uerr> for bool {
    #[inline(always)]
    fn from(variant: Uerr) -> Self {
        variant as u8 != 0
    }
}
///Field `UERR` reader - Underrun Error Detection Flag
pub type UerrR = crate::BitReader<Uerr>;
impl UerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uerr {
        match self.bits {
            false => Uerr::_0,
            true => Uerr::_1,
        }
    }
    ///No underrun error has occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uerr::_0
    }
    ///An underrun error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uerr::_1
    }
}
/**ACK Error Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackerr {
    ///0: No ACK error has occurred.
    _0 = 0,
    ///1: An ACK error has occurred.
    _1 = 1,
}
impl From<Ackerr> for bool {
    #[inline(always)]
    fn from(variant: Ackerr) -> Self {
        variant as u8 != 0
    }
}
///Field `ACKERR` reader - ACK Error Detection Flag
pub type AckerrR = crate::BitReader<Ackerr>;
impl AckerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ackerr {
        match self.bits {
            false => Ackerr::_0,
            true => Ackerr::_1,
        }
    }
    ///No ACK error has occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ackerr::_0
    }
    ///An ACK error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ackerr::_1
    }
}
/**Timing Error Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terr {
    ///0: No timing error has occurred.
    _0 = 0,
    ///1: A timing error has occurred.
    _1 = 1,
}
impl From<Terr> for bool {
    #[inline(always)]
    fn from(variant: Terr) -> Self {
        variant as u8 != 0
    }
}
///Field `TERR` reader - Timing Error Detection Flag
pub type TerrR = crate::BitReader<Terr>;
impl TerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Terr {
        match self.bits {
            false => Terr::_0,
            true => Terr::_1,
        }
    }
    ///No timing error has occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Terr::_0
    }
    ///A timing error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Terr::_1
    }
}
/**Transmission Error Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txerr {
    ///0: No transmission error has occurred.
    _0 = 0,
    ///1: A transmission error has occurred.
    _1 = 1,
}
impl From<Txerr> for bool {
    #[inline(always)]
    fn from(variant: Txerr) -> Self {
        variant as u8 != 0
    }
}
///Field `TXERR` reader - Transmission Error Detection Flag
pub type TxerrR = crate::BitReader<Txerr>;
impl TxerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txerr {
        match self.bits {
            false => Txerr::_0,
            true => Txerr::_1,
        }
    }
    ///No transmission error has occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txerr::_0
    }
    ///A transmission error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txerr::_1
    }
}
/**Arbitration Loss Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aerr {
    ///0: No arbitration loss has occurred.
    _0 = 0,
    ///1: An arbitration loss has occurred.
    _1 = 1,
}
impl From<Aerr> for bool {
    #[inline(always)]
    fn from(variant: Aerr) -> Self {
        variant as u8 != 0
    }
}
///Field `AERR` reader - Arbitration Loss Detection Flag
pub type AerrR = crate::BitReader<Aerr>;
impl AerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aerr {
        match self.bits {
            false => Aerr::_0,
            true => Aerr::_1,
        }
    }
    ///No arbitration loss has occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aerr::_0
    }
    ///An arbitration loss has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aerr::_1
    }
}
/**Bus Lock Error Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blerr {
    ///0: No bus lock error has occurred.
    _0 = 0,
    ///1: A bus lock error has occurred.
    _1 = 1,
}
impl From<Blerr> for bool {
    #[inline(always)]
    fn from(variant: Blerr) -> Self {
        variant as u8 != 0
    }
}
///Field `BLERR` reader - Bus Lock Error Detection Flag
pub type BlerrR = crate::BitReader<Blerr>;
impl BlerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Blerr {
        match self.bits {
            false => Blerr::_0,
            true => Blerr::_1,
        }
    }
    ///No bus lock error has occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Blerr::_0
    }
    ///A bus lock error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Blerr::_1
    }
}
impl R {
    ///Bit 0 - Overrun Error Detection Flag
    #[inline(always)]
    pub fn oerr(&self) -> OerrR {
        OerrR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Underrun Error Detection Flag
    #[inline(always)]
    pub fn uerr(&self) -> UerrR {
        UerrR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ACK Error Detection Flag
    #[inline(always)]
    pub fn ackerr(&self) -> AckerrR {
        AckerrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timing Error Detection Flag
    #[inline(always)]
    pub fn terr(&self) -> TerrR {
        TerrR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmission Error Detection Flag
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Arbitration Loss Detection Flag
    #[inline(always)]
    pub fn aerr(&self) -> AerrR {
        AerrR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Bus Lock Error Detection Flag
    #[inline(always)]
    pub fn blerr(&self) -> BlerrR {
        BlerrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CECES")
            .field("oerr", &self.oerr())
            .field("uerr", &self.uerr())
            .field("ackerr", &self.ackerr())
            .field("terr", &self.terr())
            .field("txerr", &self.txerr())
            .field("aerr", &self.aerr())
            .field("blerr", &self.blerr())
            .finish()
    }
}
impl W {}
/**CEC Communication Error Status Register

You can [`read`](crate::Reg::read) this register and get [`ceces::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceces::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CecesSpec;
impl crate::RegisterSpec for CecesSpec {
    type Ux = u8;
}
///`read()` method returns [`ceces::R`](R) reader structure
impl crate::Readable for CecesSpec {}
///`write(|w| ..)` method takes [`ceces::W`](W) writer structure
impl crate::Writable for CecesSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CECES to value 0
impl crate::Resettable for CecesSpec {}
