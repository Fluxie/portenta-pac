///Register `SDIO_INFO1` reader
pub type R = crate::R<SdioInfo1Spec>;
///Register `SDIO_INFO1` writer
pub type W = crate::W<SdioInfo1Spec>;
/**SDIO Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ioirq {
    ///0: No SDIO interrupt detected
    _0 = 0,
    ///1: SDIO interrupt detected
    _1 = 1,
}
impl From<Ioirq> for bool {
    #[inline(always)]
    fn from(variant: Ioirq) -> Self {
        variant as u8 != 0
    }
}
///Field `IOIRQ` reader - SDIO Interrupt Status Flag
pub type IoirqR = crate::BitReader<Ioirq>;
impl IoirqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ioirq {
        match self.bits {
            false => Ioirq::_0,
            true => Ioirq::_1,
        }
    }
    ///No SDIO interrupt detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ioirq::_0
    }
    ///SDIO interrupt detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ioirq::_1
    }
}
///Field `IOIRQ` writer - SDIO Interrupt Status Flag
pub type IoirqW<'a, REG> = crate::BitWriter<'a, REG, Ioirq>;
impl<'a, REG> IoirqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No SDIO interrupt detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ioirq::_0)
    }
    ///SDIO interrupt detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ioirq::_1)
    }
}
///Field `EXPUB52` reader - EXPUB52 Status Flag
pub type Expub52R = crate::BitReader;
///Field `EXPUB52` writer - EXPUB52 Status Flag
pub type Expub52W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXWT` reader - EXWT Status Flag
pub type ExwtR = crate::BitReader;
///Field `EXWT` writer - EXWT Status Flag
pub type ExwtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SDIO Interrupt Status Flag
    #[inline(always)]
    pub fn ioirq(&self) -> IoirqR {
        IoirqR::new((self.bits & 1) != 0)
    }
    ///Bit 14 - EXPUB52 Status Flag
    #[inline(always)]
    pub fn expub52(&self) -> Expub52R {
        Expub52R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EXWT Status Flag
    #[inline(always)]
    pub fn exwt(&self) -> ExwtR {
        ExwtR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_INFO1")
            .field("ioirq", &self.ioirq())
            .field("expub52", &self.expub52())
            .field("exwt", &self.exwt())
            .finish()
    }
}
impl W {
    ///Bit 0 - SDIO Interrupt Status Flag
    #[inline(always)]
    pub fn ioirq(&mut self) -> IoirqW<SdioInfo1Spec> {
        IoirqW::new(self, 0)
    }
    ///Bit 14 - EXPUB52 Status Flag
    #[inline(always)]
    pub fn expub52(&mut self) -> Expub52W<SdioInfo1Spec> {
        Expub52W::new(self, 14)
    }
    ///Bit 15 - EXWT Status Flag
    #[inline(always)]
    pub fn exwt(&mut self) -> ExwtW<SdioInfo1Spec> {
        ExwtW::new(self, 15)
    }
}
/**SDIO Interrupt Flag Register

You can [`read`](crate::Reg::read) this register and get [`sdio_info1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_info1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdioInfo1Spec;
impl crate::RegisterSpec for SdioInfo1Spec {
    type Ux = u32;
}
///`read()` method returns [`sdio_info1::R`](R) reader structure
impl crate::Readable for SdioInfo1Spec {}
///`write(|w| ..)` method takes [`sdio_info1::W`](W) writer structure
impl crate::Writable for SdioInfo1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDIO_INFO1 to value 0
impl crate::Resettable for SdioInfo1Spec {}
