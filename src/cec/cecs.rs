///Register `CECS` reader
pub type R = crate::R<CecsSpec>;
///Register `CECS` writer
pub type W = crate::W<CecsSpec>;
/**Address Match Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adrf {
    ///0: During communication between other stations, while communication is stopped, or while the local station is transmitting
    _0 = 0,
    ///1: During local reception
    _1 = 1,
}
impl From<Adrf> for bool {
    #[inline(always)]
    fn from(variant: Adrf) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRF` reader - Address Match Detection Flag
pub type AdrfR = crate::BitReader<Adrf>;
impl AdrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adrf {
        match self.bits {
            false => Adrf::_0,
            true => Adrf::_1,
        }
    }
    ///During communication between other stations, while communication is stopped, or while the local station is transmitting
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adrf::_0
    }
    ///During local reception
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adrf::_1
    }
}
/**Bus Busy Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busst {
    ///0: Bus-free state
    _0 = 0,
    ///1: Bus-busy state
    _1 = 1,
}
impl From<Busst> for bool {
    #[inline(always)]
    fn from(variant: Busst) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSST` reader - Bus Busy Detection Flag
pub type BusstR = crate::BitReader<Busst>;
impl BusstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Busst {
        match self.bits {
            false => Busst::_0,
            true => Busst::_1,
        }
    }
    ///Bus-free state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busst::_0
    }
    ///Bus-busy state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busst::_1
    }
}
/**Transmission Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txst {
    ///0: During communication standby state or reception (a follower is operating.)
    _0 = 0,
    ///1: During transmission (an initiator is operating.)
    _1 = 1,
}
impl From<Txst> for bool {
    #[inline(always)]
    fn from(variant: Txst) -> Self {
        variant as u8 != 0
    }
}
///Field `TXST` reader - Transmission Status Flag
pub type TxstR = crate::BitReader<Txst>;
impl TxstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txst {
        match self.bits {
            false => Txst::_0,
            true => Txst::_1,
        }
    }
    ///During communication standby state or reception (a follower is operating.)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txst::_0
    }
    ///During transmission (an initiator is operating.)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txst::_1
    }
}
/**EOM Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eomf {
    ///0: The EOM flag received immediately before is logically 0.
    _0 = 0,
    ///1: The EOM flag received immediately before is logically 1.
    _1 = 1,
}
impl From<Eomf> for bool {
    #[inline(always)]
    fn from(variant: Eomf) -> Self {
        variant as u8 != 0
    }
}
///Field `EOMF` reader - EOM Flag
pub type EomfR = crate::BitReader<Eomf>;
impl EomfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eomf {
        match self.bits {
            false => Eomf::_0,
            true => Eomf::_1,
        }
    }
    ///The EOM flag received immediately before is logically 0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eomf::_0
    }
    ///The EOM flag received immediately before is logically 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eomf::_1
    }
}
/**INTCE Generation Source Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itcef {
    ///0: Generates a communication complete interrupt (INTCE) if the signal-free time is counted.
    _0 = 0,
    ///1: Generates INTCE if communication is complete or an error is detected.
    _1 = 1,
}
impl From<Itcef> for bool {
    #[inline(always)]
    fn from(variant: Itcef) -> Self {
        variant as u8 != 0
    }
}
///Field `ITCEF` reader - INTCE Generation Source Flag
pub type ItcefR = crate::BitReader<Itcef>;
impl ItcefR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Itcef {
        match self.bits {
            false => Itcef::_0,
            true => Itcef::_1,
        }
    }
    ///Generates a communication complete interrupt (INTCE) if the signal-free time is counted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Itcef::_0
    }
    ///Generates INTCE if communication is complete or an error is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Itcef::_1
    }
}
/**Signal-Free Time Rewrite Disable Report Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sftst {
    ///0: Enables rewriting CECCTL1.SFT\[1:0\].
    _0 = 0,
    ///1: Disables rewriting CECCTL1.SFT\[1:0\].
    _1 = 1,
}
impl From<Sftst> for bool {
    #[inline(always)]
    fn from(variant: Sftst) -> Self {
        variant as u8 != 0
    }
}
///Field `SFTST` reader - Signal-Free Time Rewrite Disable Report Flag
pub type SftstR = crate::BitReader<Sftst>;
impl SftstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sftst {
        match self.bits {
            false => Sftst::_0,
            true => Sftst::_1,
        }
    }
    ///Enables rewriting CECCTL1.SFT\[1:0\].
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sftst::_0
    }
    ///Disables rewriting CECCTL1.SFT\[1:0\].
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sftst::_1
    }
}
impl R {
    ///Bit 0 - Address Match Detection Flag
    #[inline(always)]
    pub fn adrf(&self) -> AdrfR {
        AdrfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bus Busy Detection Flag
    #[inline(always)]
    pub fn busst(&self) -> BusstR {
        BusstR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmission Status Flag
    #[inline(always)]
    pub fn txst(&self) -> TxstR {
        TxstR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOM Flag
    #[inline(always)]
    pub fn eomf(&self) -> EomfR {
        EomfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - INTCE Generation Source Flag
    #[inline(always)]
    pub fn itcef(&self) -> ItcefR {
        ItcefR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Signal-Free Time Rewrite Disable Report Flag
    #[inline(always)]
    pub fn sftst(&self) -> SftstR {
        SftstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CECS")
            .field("adrf", &self.adrf())
            .field("busst", &self.busst())
            .field("txst", &self.txst())
            .field("eomf", &self.eomf())
            .field("itcef", &self.itcef())
            .field("sftst", &self.sftst())
            .finish()
    }
}
impl W {}
/**CEC Communication Status Register

You can [`read`](crate::Reg::read) this register and get [`cecs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CecsSpec;
impl crate::RegisterSpec for CecsSpec {
    type Ux = u8;
}
///`read()` method returns [`cecs::R`](R) reader structure
impl crate::Readable for CecsSpec {}
///`write(|w| ..)` method takes [`cecs::W`](W) writer structure
impl crate::Writable for CecsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CECS to value 0
impl crate::Resettable for CecsSpec {}
