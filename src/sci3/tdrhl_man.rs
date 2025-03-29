///Register `TDRHL_MAN` reader
pub type R = crate::R<TdrhlManSpec>;
///Register `TDRHL_MAN` writer
pub type W = crate::W<TdrhlManSpec>;
///Field `TDAT` reader - Serial transmit data
pub type TdatR = crate::FieldReader<u16>;
///Field `TDAT` writer - Serial transmit data
pub type TdatW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
/**Multi-processor transfer bit flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpbt {
    ///0: Data transmission cycles
    _0 = 0,
    ///1: ID transmission cycles
    _1 = 1,
}
impl From<Mpbt> for bool {
    #[inline(always)]
    fn from(variant: Mpbt) -> Self {
        variant as u8 != 0
    }
}
///Field `MPBT` reader - Multi-processor transfer bit flag
pub type MpbtR = crate::BitReader<Mpbt>;
impl MpbtR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mpbt {
        match self.bits {
            false => Mpbt::_0,
            true => Mpbt::_1,
        }
    }
    ///Data transmission cycles
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpbt::_0
    }
    ///ID transmission cycles
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mpbt::_1
    }
}
///Field `MPBT` writer - Multi-processor transfer bit flag
pub type MpbtW<'a, REG> = crate::BitWriter<'a, REG, Mpbt>;
impl<'a, REG> MpbtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data transmission cycles
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpbt::_0)
    }
    ///ID transmission cycles
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpbt::_1)
    }
}
/**Transmit SYNC data bit

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsync {
    ///0: The Start Bit is transmitted as DATA SYNC.
    _0 = 0,
    ///1: The Start Bit is transmitted as COMMAND SYNC.
    _1 = 1,
}
impl From<Tsync> for bool {
    #[inline(always)]
    fn from(variant: Tsync) -> Self {
        variant as u8 != 0
    }
}
///Field `TSYNC` reader - Transmit SYNC data bit
pub type TsyncR = crate::BitReader<Tsync>;
impl TsyncR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tsync {
        match self.bits {
            false => Tsync::_0,
            true => Tsync::_1,
        }
    }
    ///The Start Bit is transmitted as DATA SYNC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsync::_0
    }
    ///The Start Bit is transmitted as COMMAND SYNC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsync::_1
    }
}
///Field `TSYNC` writer - Transmit SYNC data bit
pub type TsyncW<'a, REG> = crate::BitWriter<'a, REG, Tsync>;
impl<'a, REG> TsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The Start Bit is transmitted as DATA SYNC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsync::_0)
    }
    ///The Start Bit is transmitted as COMMAND SYNC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsync::_1)
    }
}
impl R {
    ///Bits 0:8 - Serial transmit data
    #[inline(always)]
    pub fn tdat(&self) -> TdatR {
        TdatR::new(self.bits & 0x01ff)
    }
    ///Bit 9 - Multi-processor transfer bit flag
    #[inline(always)]
    pub fn mpbt(&self) -> MpbtR {
        MpbtR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - Transmit SYNC data bit
    #[inline(always)]
    pub fn tsync(&self) -> TsyncR {
        TsyncR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDRHL_MAN")
            .field("tdat", &self.tdat())
            .field("mpbt", &self.mpbt())
            .field("tsync", &self.tsync())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Serial transmit data
    #[inline(always)]
    pub fn tdat(&mut self) -> TdatW<TdrhlManSpec> {
        TdatW::new(self, 0)
    }
    ///Bit 9 - Multi-processor transfer bit flag
    #[inline(always)]
    pub fn mpbt(&mut self) -> MpbtW<TdrhlManSpec> {
        MpbtW::new(self, 9)
    }
    ///Bit 12 - Transmit SYNC data bit
    #[inline(always)]
    pub fn tsync(&mut self) -> TsyncW<TdrhlManSpec> {
        TsyncW::new(self, 12)
    }
}
/**Transmit Data Register for Manchester mode (MMR.MANEN = 1)

You can [`read`](crate::Reg::read) this register and get [`tdrhl_man::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdrhl_man::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TdrhlManSpec;
impl crate::RegisterSpec for TdrhlManSpec {
    type Ux = u16;
}
///`read()` method returns [`tdrhl_man::R`](R) reader structure
impl crate::Readable for TdrhlManSpec {}
///`write(|w| ..)` method takes [`tdrhl_man::W`](W) writer structure
impl crate::Writable for TdrhlManSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDRHL_MAN to value 0xffff
impl crate::Resettable for TdrhlManSpec {
    const RESET_VALUE: u16 = 0xffff;
}
