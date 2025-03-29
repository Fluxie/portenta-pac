///Register `FSTATR` reader
pub type R = crate::R<FstatrSpec>;
///Register `FSTATR` writer
pub type W = crate::W<FstatrSpec>;
/**Flash Write/Erase Protect Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flweerr {
    ///0: An error has not occurred
    _0 = 0,
    ///1: An error has occurred.
    _1 = 1,
}
impl From<Flweerr> for bool {
    #[inline(always)]
    fn from(variant: Flweerr) -> Self {
        variant as u8 != 0
    }
}
///Field `FLWEERR` reader - Flash Write/Erase Protect Error Flag
pub type FlweerrR = crate::BitReader<Flweerr>;
impl FlweerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Flweerr {
        match self.bits {
            false => Flweerr::_0,
            true => Flweerr::_1,
        }
    }
    ///An error has not occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Flweerr::_0
    }
    ///An error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Flweerr::_1
    }
}
/**Programming Suspend Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgspd {
    ///0: The flash sequencer is not in the programming suspension processing state or programming suspended state
    _0 = 0,
    ///1: The flash sequencer is in the programming suspension processing state or programming suspended state.
    _1 = 1,
}
impl From<Prgspd> for bool {
    #[inline(always)]
    fn from(variant: Prgspd) -> Self {
        variant as u8 != 0
    }
}
///Field `PRGSPD` reader - Programming Suspend Status Flag
pub type PrgspdR = crate::BitReader<Prgspd>;
impl PrgspdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Prgspd {
        match self.bits {
            false => Prgspd::_0,
            true => Prgspd::_1,
        }
    }
    ///The flash sequencer is not in the programming suspension processing state or programming suspended state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prgspd::_0
    }
    ///The flash sequencer is in the programming suspension processing state or programming suspended state.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prgspd::_1
    }
}
/**Erasure Suspend Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ersspd {
    ///0: The flash sequencer is not in the erasure suspension processing state or the erasure suspended state
    _0 = 0,
    ///1: The flash sequencer is in the erasure suspension processing state or the erasure suspended state.
    _1 = 1,
}
impl From<Ersspd> for bool {
    #[inline(always)]
    fn from(variant: Ersspd) -> Self {
        variant as u8 != 0
    }
}
///Field `ERSSPD` reader - Erasure Suspend Status Flag
pub type ErsspdR = crate::BitReader<Ersspd>;
impl ErsspdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ersspd {
        match self.bits {
            false => Ersspd::_0,
            true => Ersspd::_1,
        }
    }
    ///The flash sequencer is not in the erasure suspension processing state or the erasure suspended state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ersspd::_0
    }
    ///The flash sequencer is in the erasure suspension processing state or the erasure suspended state.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ersspd::_1
    }
}
/**Data Buffer Full Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbfull {
    ///0: The data buffer is empty
    _0 = 0,
    ///1: The data buffer is full.
    _1 = 1,
}
impl From<Dbfull> for bool {
    #[inline(always)]
    fn from(variant: Dbfull) -> Self {
        variant as u8 != 0
    }
}
///Field `DBFULL` reader - Data Buffer Full Flag
pub type DbfullR = crate::BitReader<Dbfull>;
impl DbfullR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dbfull {
        match self.bits {
            false => Dbfull::_0,
            true => Dbfull::_1,
        }
    }
    ///The data buffer is empty
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dbfull::_0
    }
    ///The data buffer is full.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dbfull::_1
    }
}
/**Suspend Ready Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Susrdy {
    ///0: The flash sequencer cannot receive P/E suspend commands
    _0 = 0,
    ///1: The flash sequencer can receive P/E suspend commands.
    _1 = 1,
}
impl From<Susrdy> for bool {
    #[inline(always)]
    fn from(variant: Susrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSRDY` reader - Suspend Ready Flag
pub type SusrdyR = crate::BitReader<Susrdy>;
impl SusrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Susrdy {
        match self.bits {
            false => Susrdy::_0,
            true => Susrdy::_1,
        }
    }
    ///The flash sequencer cannot receive P/E suspend commands
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Susrdy::_0
    }
    ///The flash sequencer can receive P/E suspend commands.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Susrdy::_1
    }
}
/**Programming Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgerr {
    ///0: Programming has completed successfully
    _0 = 0,
    ///1: An error has occurred during programming.
    _1 = 1,
}
impl From<Prgerr> for bool {
    #[inline(always)]
    fn from(variant: Prgerr) -> Self {
        variant as u8 != 0
    }
}
///Field `PRGERR` reader - Programming Error Flag
pub type PrgerrR = crate::BitReader<Prgerr>;
impl PrgerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Prgerr {
        match self.bits {
            false => Prgerr::_0,
            true => Prgerr::_1,
        }
    }
    ///Programming has completed successfully
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prgerr::_0
    }
    ///An error has occurred during programming.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prgerr::_1
    }
}
/**Erasure Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erserr {
    ///0: Erasure has completed successfully
    _0 = 0,
    ///1: An error has occurred during erasure.
    _1 = 1,
}
impl From<Erserr> for bool {
    #[inline(always)]
    fn from(variant: Erserr) -> Self {
        variant as u8 != 0
    }
}
///Field `ERSERR` reader - Erasure Error Flag
pub type ErserrR = crate::BitReader<Erserr>;
impl ErserrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Erserr {
        match self.bits {
            false => Erserr::_0,
            true => Erserr::_1,
        }
    }
    ///Erasure has completed successfully
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Erserr::_0
    }
    ///An error has occurred during erasure.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Erserr::_1
    }
}
/**Illegal Command Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilglerr {
    ///0: The flash sequencer has not detected an illegal FACI command or illegal flash memory access
    _0 = 0,
    ///1: The flash sequencer has detected an illegal FACI command or illegal flash memory access.
    _1 = 1,
}
impl From<Ilglerr> for bool {
    #[inline(always)]
    fn from(variant: Ilglerr) -> Self {
        variant as u8 != 0
    }
}
///Field `ILGLERR` reader - Illegal Command Error Flag
pub type IlglerrR = crate::BitReader<Ilglerr>;
impl IlglerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ilglerr {
        match self.bits {
            false => Ilglerr::_0,
            true => Ilglerr::_1,
        }
    }
    ///The flash sequencer has not detected an illegal FACI command or illegal flash memory access
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilglerr::_0
    }
    ///The flash sequencer has detected an illegal FACI command or illegal flash memory access.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilglerr::_1
    }
}
/**Flash Ready Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frdy {
    ///0: Program, Block Erase, Multi Block Erase, P/E suspend, P/E resume, Forced Stop, Blank Check, or Configuration set command processing is in progress
    _0 = 0,
    ///1: None of the above is in progress.
    _1 = 1,
}
impl From<Frdy> for bool {
    #[inline(always)]
    fn from(variant: Frdy) -> Self {
        variant as u8 != 0
    }
}
///Field `FRDY` reader - Flash Ready Flag
pub type FrdyR = crate::BitReader<Frdy>;
impl FrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Frdy {
        match self.bits {
            false => Frdy::_0,
            true => Frdy::_1,
        }
    }
    ///Program, Block Erase, Multi Block Erase, P/E suspend, P/E resume, Forced Stop, Blank Check, or Configuration set command processing is in progress
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Frdy::_0
    }
    ///None of the above is in progress.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Frdy::_1
    }
}
/**Other Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oterr {
    ///0: A status clear or forced stop command processing is complete
    _0 = 0,
    ///1: An error has occurred.
    _1 = 1,
}
impl From<Oterr> for bool {
    #[inline(always)]
    fn from(variant: Oterr) -> Self {
        variant as u8 != 0
    }
}
///Field `OTERR` reader - Other Error
pub type OterrR = crate::BitReader<Oterr>;
impl OterrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oterr {
        match self.bits {
            false => Oterr::_0,
            true => Oterr::_1,
        }
    }
    ///A status clear or forced stop command processing is complete
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oterr::_0
    }
    ///An error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oterr::_1
    }
}
/**Security Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secerr {
    ///0: A status clear or forced stop command processing is complete
    _0 = 0,
    ///1: An error has occurred.
    _1 = 1,
}
impl From<Secerr> for bool {
    #[inline(always)]
    fn from(variant: Secerr) -> Self {
        variant as u8 != 0
    }
}
///Field `SECERR` reader - Security Error
pub type SecerrR = crate::BitReader<Secerr>;
impl SecerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Secerr {
        match self.bits {
            false => Secerr::_0,
            true => Secerr::_1,
        }
    }
    ///A status clear or forced stop command processing is complete
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Secerr::_0
    }
    ///An error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Secerr::_1
    }
}
/**FENTRY Setting Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feseterr {
    ///0: A status clear or forced stop command processing is complete
    _0 = 0,
    ///1: An error has occurred.
    _1 = 1,
}
impl From<Feseterr> for bool {
    #[inline(always)]
    fn from(variant: Feseterr) -> Self {
        variant as u8 != 0
    }
}
///Field `FESETERR` reader - FENTRY Setting Error
pub type FeseterrR = crate::BitReader<Feseterr>;
impl FeseterrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Feseterr {
        match self.bits {
            false => Feseterr::_0,
            true => Feseterr::_1,
        }
    }
    ///A status clear or forced stop command processing is complete
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Feseterr::_0
    }
    ///An error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Feseterr::_1
    }
}
/**Illegal Command Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilgcomerr {
    ///0: A status clear or forced stop command processing is complete
    _0 = 0,
    ///1: An error has occurred.
    _1 = 1,
}
impl From<Ilgcomerr> for bool {
    #[inline(always)]
    fn from(variant: Ilgcomerr) -> Self {
        variant as u8 != 0
    }
}
///Field `ILGCOMERR` reader - Illegal Command Error
pub type IlgcomerrR = crate::BitReader<Ilgcomerr>;
impl IlgcomerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ilgcomerr {
        match self.bits {
            false => Ilgcomerr::_0,
            true => Ilgcomerr::_1,
        }
    }
    ///A status clear or forced stop command processing is complete
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilgcomerr::_0
    }
    ///An error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilgcomerr::_1
    }
}
impl R {
    ///Bit 6 - Flash Write/Erase Protect Error Flag
    #[inline(always)]
    pub fn flweerr(&self) -> FlweerrR {
        FlweerrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Programming Suspend Status Flag
    #[inline(always)]
    pub fn prgspd(&self) -> PrgspdR {
        PrgspdR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Erasure Suspend Status Flag
    #[inline(always)]
    pub fn ersspd(&self) -> ErsspdR {
        ErsspdR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data Buffer Full Flag
    #[inline(always)]
    pub fn dbfull(&self) -> DbfullR {
        DbfullR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Suspend Ready Flag
    #[inline(always)]
    pub fn susrdy(&self) -> SusrdyR {
        SusrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Programming Error Flag
    #[inline(always)]
    pub fn prgerr(&self) -> PrgerrR {
        PrgerrR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Erasure Error Flag
    #[inline(always)]
    pub fn erserr(&self) -> ErserrR {
        ErserrR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Illegal Command Error Flag
    #[inline(always)]
    pub fn ilglerr(&self) -> IlglerrR {
        IlglerrR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Flash Ready Flag
    #[inline(always)]
    pub fn frdy(&self) -> FrdyR {
        FrdyR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - Other Error
    #[inline(always)]
    pub fn oterr(&self) -> OterrR {
        OterrR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Security Error
    #[inline(always)]
    pub fn secerr(&self) -> SecerrR {
        SecerrR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - FENTRY Setting Error
    #[inline(always)]
    pub fn feseterr(&self) -> FeseterrR {
        FeseterrR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Illegal Command Error
    #[inline(always)]
    pub fn ilgcomerr(&self) -> IlgcomerrR {
        IlgcomerrR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSTATR")
            .field("flweerr", &self.flweerr())
            .field("prgspd", &self.prgspd())
            .field("ersspd", &self.ersspd())
            .field("dbfull", &self.dbfull())
            .field("susrdy", &self.susrdy())
            .field("prgerr", &self.prgerr())
            .field("erserr", &self.erserr())
            .field("ilglerr", &self.ilglerr())
            .field("frdy", &self.frdy())
            .field("oterr", &self.oterr())
            .field("secerr", &self.secerr())
            .field("feseterr", &self.feseterr())
            .field("ilgcomerr", &self.ilgcomerr())
            .finish()
    }
}
impl W {}
/**Flash Status Register

You can [`read`](crate::Reg::read) this register and get [`fstatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FstatrSpec;
impl crate::RegisterSpec for FstatrSpec {
    type Ux = u32;
}
///`read()` method returns [`fstatr::R`](R) reader structure
impl crate::Readable for FstatrSpec {}
///`write(|w| ..)` method takes [`fstatr::W`](W) writer structure
impl crate::Writable for FstatrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FSTATR to value 0x8000
impl crate::Resettable for FstatrSpec {
    const RESET_VALUE: u32 = 0x8000;
}
