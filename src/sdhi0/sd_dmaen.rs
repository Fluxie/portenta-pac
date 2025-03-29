///Register `SD_DMAEN` reader
pub type R = crate::R<SdDmaenSpec>;
///Register `SD_DMAEN` writer
pub type W = crate::W<SdDmaenSpec>;
/**DMA Transfer Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    ///0: Disable use of DMA transfer to access SD_BUF0 register
    _0 = 0,
    ///1: Enable use of DMA transfer to access SD_BUF0 register
    _1 = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - DMA Transfer Enable
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::_0,
            true => Dmaen::_1,
        }
    }
    ///Disable use of DMA transfer to access SD_BUF0 register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmaen::_0
    }
    ///Enable use of DMA transfer to access SD_BUF0 register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmaen::_1
    }
}
///Field `DMAEN` writer - DMA Transfer Enable
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable use of DMA transfer to access SD_BUF0 register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::_0)
    }
    ///Enable use of DMA transfer to access SD_BUF0 register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::_1)
    }
}
impl R {
    ///Bit 1 - DMA Transfer Enable
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_DMAEN").field("dmaen", &self.dmaen()).finish()
    }
}
impl W {
    ///Bit 1 - DMA Transfer Enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<SdDmaenSpec> {
        DmaenW::new(self, 1)
    }
}
/**DMA Mode Enable Register

You can [`read`](crate::Reg::read) this register and get [`sd_dmaen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dmaen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdDmaenSpec;
impl crate::RegisterSpec for SdDmaenSpec {
    type Ux = u32;
}
///`read()` method returns [`sd_dmaen::R`](R) reader structure
impl crate::Readable for SdDmaenSpec {}
///`write(|w| ..)` method takes [`sd_dmaen::W`](W) writer structure
impl crate::Writable for SdDmaenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_DMAEN to value 0x1010
impl crate::Resettable for SdDmaenSpec {
    const RESET_VALUE: u32 = 0x1010;
}
