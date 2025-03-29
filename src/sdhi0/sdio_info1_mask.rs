///Register `SDIO_INFO1_MASK` reader
pub type R = crate::R<SdioInfo1MaskSpec>;
///Register `SDIO_INFO1_MASK` writer
pub type W = crate::W<SdioInfo1MaskSpec>;
/**IOIRQ Interrupt Mask Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ioirqm {
    ///0: Do not mask IOIRQ interrupts
    _0 = 0,
    ///1: Mask IOIRQ interrupts
    _1 = 1,
}
impl From<Ioirqm> for bool {
    #[inline(always)]
    fn from(variant: Ioirqm) -> Self {
        variant as u8 != 0
    }
}
///Field `IOIRQM` reader - IOIRQ Interrupt Mask Control
pub type IoirqmR = crate::BitReader<Ioirqm>;
impl IoirqmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ioirqm {
        match self.bits {
            false => Ioirqm::_0,
            true => Ioirqm::_1,
        }
    }
    ///Do not mask IOIRQ interrupts
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ioirqm::_0
    }
    ///Mask IOIRQ interrupts
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ioirqm::_1
    }
}
///Field `IOIRQM` writer - IOIRQ Interrupt Mask Control
pub type IoirqmW<'a, REG> = crate::BitWriter<'a, REG, Ioirqm>;
impl<'a, REG> IoirqmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask IOIRQ interrupts
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ioirqm::_0)
    }
    ///Mask IOIRQ interrupts
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ioirqm::_1)
    }
}
/**EXPUB52 Interrupt Request Mask Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Expub52m {
    ///0: Do not mask EXPUB52 interrupt requests
    _0 = 0,
    ///1: Mask EXPUB52 interrupt requests
    _1 = 1,
}
impl From<Expub52m> for bool {
    #[inline(always)]
    fn from(variant: Expub52m) -> Self {
        variant as u8 != 0
    }
}
///Field `EXPUB52M` reader - EXPUB52 Interrupt Request Mask Control
pub type Expub52mR = crate::BitReader<Expub52m>;
impl Expub52mR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Expub52m {
        match self.bits {
            false => Expub52m::_0,
            true => Expub52m::_1,
        }
    }
    ///Do not mask EXPUB52 interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Expub52m::_0
    }
    ///Mask EXPUB52 interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Expub52m::_1
    }
}
///Field `EXPUB52M` writer - EXPUB52 Interrupt Request Mask Control
pub type Expub52mW<'a, REG> = crate::BitWriter<'a, REG, Expub52m>;
impl<'a, REG> Expub52mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask EXPUB52 interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Expub52m::_0)
    }
    ///Mask EXPUB52 interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Expub52m::_1)
    }
}
/**EXWT Interrupt Request Mask Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exwtm {
    ///0: Do not mask EXWT interrupt requests
    _0 = 0,
    ///1: Mask EXWT interrupt requests
    _1 = 1,
}
impl From<Exwtm> for bool {
    #[inline(always)]
    fn from(variant: Exwtm) -> Self {
        variant as u8 != 0
    }
}
///Field `EXWTM` reader - EXWT Interrupt Request Mask Control
pub type ExwtmR = crate::BitReader<Exwtm>;
impl ExwtmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Exwtm {
        match self.bits {
            false => Exwtm::_0,
            true => Exwtm::_1,
        }
    }
    ///Do not mask EXWT interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Exwtm::_0
    }
    ///Mask EXWT interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Exwtm::_1
    }
}
///Field `EXWTM` writer - EXWT Interrupt Request Mask Control
pub type ExwtmW<'a, REG> = crate::BitWriter<'a, REG, Exwtm>;
impl<'a, REG> ExwtmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask EXWT interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Exwtm::_0)
    }
    ///Mask EXWT interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Exwtm::_1)
    }
}
impl R {
    ///Bit 0 - IOIRQ Interrupt Mask Control
    #[inline(always)]
    pub fn ioirqm(&self) -> IoirqmR {
        IoirqmR::new((self.bits & 1) != 0)
    }
    ///Bit 14 - EXPUB52 Interrupt Request Mask Control
    #[inline(always)]
    pub fn expub52m(&self) -> Expub52mR {
        Expub52mR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EXWT Interrupt Request Mask Control
    #[inline(always)]
    pub fn exwtm(&self) -> ExwtmR {
        ExwtmR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_INFO1_MASK")
            .field("ioirqm", &self.ioirqm())
            .field("expub52m", &self.expub52m())
            .field("exwtm", &self.exwtm())
            .finish()
    }
}
impl W {
    ///Bit 0 - IOIRQ Interrupt Mask Control
    #[inline(always)]
    pub fn ioirqm(&mut self) -> IoirqmW<SdioInfo1MaskSpec> {
        IoirqmW::new(self, 0)
    }
    ///Bit 14 - EXPUB52 Interrupt Request Mask Control
    #[inline(always)]
    pub fn expub52m(&mut self) -> Expub52mW<SdioInfo1MaskSpec> {
        Expub52mW::new(self, 14)
    }
    ///Bit 15 - EXWT Interrupt Request Mask Control
    #[inline(always)]
    pub fn exwtm(&mut self) -> ExwtmW<SdioInfo1MaskSpec> {
        ExwtmW::new(self, 15)
    }
}
/**SDIO INFO1 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sdio_info1_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_info1_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdioInfo1MaskSpec;
impl crate::RegisterSpec for SdioInfo1MaskSpec {
    type Ux = u32;
}
///`read()` method returns [`sdio_info1_mask::R`](R) reader structure
impl crate::Readable for SdioInfo1MaskSpec {}
///`write(|w| ..)` method takes [`sdio_info1_mask::W`](W) writer structure
impl crate::Writable for SdioInfo1MaskSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDIO_INFO1_MASK to value 0xc007
impl crate::Resettable for SdioInfo1MaskSpec {
    const RESET_VALUE: u32 = 0xc007;
}
