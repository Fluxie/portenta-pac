///Register `RDRHL_MAN` reader
pub type R = crate::R<RdrhlManSpec>;
///Field `RDAT` reader - Serial receive data
pub type RdatR = crate::FieldReader<u16>;
/**Multi-processor bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpb {
    ///0: Data transmission cycles
    _0 = 0,
    ///1: ID transmission cycles
    _1 = 1,
}
impl From<Mpb> for bool {
    #[inline(always)]
    fn from(variant: Mpb) -> Self {
        variant as u8 != 0
    }
}
///Field `MPB` reader - Multi-processor bit
pub type MpbR = crate::BitReader<Mpb>;
impl MpbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mpb {
        match self.bits {
            false => Mpb::_0,
            true => Mpb::_1,
        }
    }
    ///Data transmission cycles
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpb::_0
    }
    ///ID transmission cycles
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mpb::_1
    }
}
/**Receive SYNC data bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsync {
    ///0: The received the Start Bit is DATA SYNC
    _0 = 0,
    ///1: The received the Start Bit is COMMAND SYNC
    _1 = 1,
}
impl From<Rsync> for bool {
    #[inline(always)]
    fn from(variant: Rsync) -> Self {
        variant as u8 != 0
    }
}
///Field `RSYNC` reader - Receive SYNC data bit
pub type RsyncR = crate::BitReader<Rsync>;
impl RsyncR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rsync {
        match self.bits {
            false => Rsync::_0,
            true => Rsync::_1,
        }
    }
    ///The received the Start Bit is DATA SYNC
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rsync::_0
    }
    ///The received the Start Bit is COMMAND SYNC
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rsync::_1
    }
}
impl R {
    ///Bits 0:8 - Serial receive data
    #[inline(always)]
    pub fn rdat(&self) -> RdatR {
        RdatR::new(self.bits & 0x01ff)
    }
    ///Bit 9 - Multi-processor bit
    #[inline(always)]
    pub fn mpb(&self) -> MpbR {
        MpbR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - Receive SYNC data bit
    #[inline(always)]
    pub fn rsync(&self) -> RsyncR {
        RsyncR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDRHL_MAN")
            .field("rdat", &self.rdat())
            .field("mpb", &self.mpb())
            .field("rsync", &self.rsync())
            .finish()
    }
}
/**Receive Data Register for Manchester mode (MMR.MANEN = 1)

You can [`read`](crate::Reg::read) this register and get [`rdrhl_man::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdrhlManSpec;
impl crate::RegisterSpec for RdrhlManSpec {
    type Ux = u16;
}
///`read()` method returns [`rdrhl_man::R`](R) reader structure
impl crate::Readable for RdrhlManSpec {}
///`reset()` method sets RDRHL_MAN to value 0
impl crate::Resettable for RdrhlManSpec {}
