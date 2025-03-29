///Register `SDIO_MODE` reader
pub type R = crate::R<SdioModeSpec>;
///Register `SDIO_MODE` writer
pub type W = crate::W<SdioModeSpec>;
/**SDIO Interrupt Acceptance Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten {
    ///0: Disable SDIO interrupt acceptance
    _0 = 0,
    ///1: Enable SDIO interrupt acceptance
    _1 = 1,
}
impl From<Inten> for bool {
    #[inline(always)]
    fn from(variant: Inten) -> Self {
        variant as u8 != 0
    }
}
///Field `INTEN` reader - SDIO Interrupt Acceptance Enable
pub type IntenR = crate::BitReader<Inten>;
impl IntenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Inten {
        match self.bits {
            false => Inten::_0,
            true => Inten::_1,
        }
    }
    ///Disable SDIO interrupt acceptance
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Inten::_0
    }
    ///Enable SDIO interrupt acceptance
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Inten::_1
    }
}
///Field `INTEN` writer - SDIO Interrupt Acceptance Enable
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG, Inten>;
impl<'a, REG> IntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable SDIO interrupt acceptance
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::_0)
    }
    ///Enable SDIO interrupt acceptance
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::_1)
    }
}
/**Read Wait Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwreq {
    ///0: Allow SD/MMC to exit read wait state
    _0 = 0,
    ///1: Request for SD/MMC to enter read wait state
    _1 = 1,
}
impl From<Rwreq> for bool {
    #[inline(always)]
    fn from(variant: Rwreq) -> Self {
        variant as u8 != 0
    }
}
///Field `RWREQ` reader - Read Wait Request
pub type RwreqR = crate::BitReader<Rwreq>;
impl RwreqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rwreq {
        match self.bits {
            false => Rwreq::_0,
            true => Rwreq::_1,
        }
    }
    ///Allow SD/MMC to exit read wait state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rwreq::_0
    }
    ///Request for SD/MMC to enter read wait state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rwreq::_1
    }
}
///Field `RWREQ` writer - Read Wait Request
pub type RwreqW<'a, REG> = crate::BitWriter<'a, REG, Rwreq>;
impl<'a, REG> RwreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Allow SD/MMC to exit read wait state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rwreq::_0)
    }
    ///Request for SD/MMC to enter read wait state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rwreq::_1)
    }
}
///Field `IOABT` reader - SDIO Abort
pub type IoabtR = crate::BitReader;
///Field `IOABT` writer - SDIO Abort
pub type IoabtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `C52PUB` reader - SDIO None Abort
pub type C52pubR = crate::BitReader;
///Field `C52PUB` writer - SDIO None Abort
pub type C52pubW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SDIO Interrupt Acceptance Enable
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Read Wait Request
    #[inline(always)]
    pub fn rwreq(&self) -> RwreqR {
        RwreqR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - SDIO Abort
    #[inline(always)]
    pub fn ioabt(&self) -> IoabtR {
        IoabtR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SDIO None Abort
    #[inline(always)]
    pub fn c52pub(&self) -> C52pubR {
        C52pubR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_MODE")
            .field("inten", &self.inten())
            .field("rwreq", &self.rwreq())
            .field("ioabt", &self.ioabt())
            .field("c52pub", &self.c52pub())
            .finish()
    }
}
impl W {
    ///Bit 0 - SDIO Interrupt Acceptance Enable
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<SdioModeSpec> {
        IntenW::new(self, 0)
    }
    ///Bit 2 - Read Wait Request
    #[inline(always)]
    pub fn rwreq(&mut self) -> RwreqW<SdioModeSpec> {
        RwreqW::new(self, 2)
    }
    ///Bit 8 - SDIO Abort
    #[inline(always)]
    pub fn ioabt(&mut self) -> IoabtW<SdioModeSpec> {
        IoabtW::new(self, 8)
    }
    ///Bit 9 - SDIO None Abort
    #[inline(always)]
    pub fn c52pub(&mut self) -> C52pubW<SdioModeSpec> {
        C52pubW::new(self, 9)
    }
}
/**SDIO Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sdio_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdioModeSpec;
impl crate::RegisterSpec for SdioModeSpec {
    type Ux = u32;
}
///`read()` method returns [`sdio_mode::R`](R) reader structure
impl crate::Readable for SdioModeSpec {}
///`write(|w| ..)` method takes [`sdio_mode::W`](W) writer structure
impl crate::Writable for SdioModeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDIO_MODE to value 0
impl crate::Resettable for SdioModeSpec {}
