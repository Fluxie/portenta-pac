///Register `SOFCFG` reader
pub type R = crate::R<SofcfgSpec>;
///Register `SOFCFG` writer
pub type W = crate::W<SofcfgSpec>;
///Field `EDGESTS` reader - Edge Interrupt Output Status Monitor
pub type EdgestsR = crate::BitReader;
/**BRDY Interrupt Status Clear Timing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brdym {
    ///0: Clear BRDY flag by software
    _0 = 0,
    ///1: Clear BRDY flag by the USBFS through a data read from the FIFO buffer or data write to the FIFO buffer
    _1 = 1,
}
impl From<Brdym> for bool {
    #[inline(always)]
    fn from(variant: Brdym) -> Self {
        variant as u8 != 0
    }
}
///Field `BRDYM` reader - BRDY Interrupt Status Clear Timing
pub type BrdymR = crate::BitReader<Brdym>;
impl BrdymR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Brdym {
        match self.bits {
            false => Brdym::_0,
            true => Brdym::_1,
        }
    }
    ///Clear BRDY flag by software
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Brdym::_0
    }
    ///Clear BRDY flag by the USBFS through a data read from the FIFO buffer or data write to the FIFO buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Brdym::_1
    }
}
///Field `BRDYM` writer - BRDY Interrupt Status Clear Timing
pub type BrdymW<'a, REG> = crate::BitWriter<'a, REG, Brdym>;
impl<'a, REG> BrdymW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear BRDY flag by software
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Brdym::_0)
    }
    ///Clear BRDY flag by the USBFS through a data read from the FIFO buffer or data write to the FIFO buffer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Brdym::_1)
    }
}
/**Transaction-Enabled Time Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trnensel {
    ///0: Not low-speed communication
    _0 = 0,
    ///1: Low-speed communication
    _1 = 1,
}
impl From<Trnensel> for bool {
    #[inline(always)]
    fn from(variant: Trnensel) -> Self {
        variant as u8 != 0
    }
}
///Field `TRNENSEL` reader - Transaction-Enabled Time Select
pub type TrnenselR = crate::BitReader<Trnensel>;
impl TrnenselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Trnensel {
        match self.bits {
            false => Trnensel::_0,
            true => Trnensel::_1,
        }
    }
    ///Not low-speed communication
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trnensel::_0
    }
    ///Low-speed communication
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trnensel::_1
    }
}
///Field `TRNENSEL` writer - Transaction-Enabled Time Select
pub type TrnenselW<'a, REG> = crate::BitWriter<'a, REG, Trnensel>;
impl<'a, REG> TrnenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not low-speed communication
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trnensel::_0)
    }
    ///Low-speed communication
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trnensel::_1)
    }
}
impl R {
    ///Bit 4 - Edge Interrupt Output Status Monitor
    #[inline(always)]
    pub fn edgests(&self) -> EdgestsR {
        EdgestsR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - BRDY Interrupt Status Clear Timing
    #[inline(always)]
    pub fn brdym(&self) -> BrdymR {
        BrdymR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Transaction-Enabled Time Select
    #[inline(always)]
    pub fn trnensel(&self) -> TrnenselR {
        TrnenselR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOFCFG")
            .field("edgests", &self.edgests())
            .field("brdym", &self.brdym())
            .field("trnensel", &self.trnensel())
            .finish()
    }
}
impl W {
    ///Bit 6 - BRDY Interrupt Status Clear Timing
    #[inline(always)]
    pub fn brdym(&mut self) -> BrdymW<SofcfgSpec> {
        BrdymW::new(self, 6)
    }
    ///Bit 8 - Transaction-Enabled Time Select
    #[inline(always)]
    pub fn trnensel(&mut self) -> TrnenselW<SofcfgSpec> {
        TrnenselW::new(self, 8)
    }
}
/**SOF Output Configuration Register

You can [`read`](crate::Reg::read) this register and get [`sofcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sofcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SofcfgSpec;
impl crate::RegisterSpec for SofcfgSpec {
    type Ux = u16;
}
///`read()` method returns [`sofcfg::R`](R) reader structure
impl crate::Readable for SofcfgSpec {}
///`write(|w| ..)` method takes [`sofcfg::W`](W) writer structure
impl crate::Writable for SofcfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOFCFG to value 0
impl crate::Resettable for SofcfgSpec {}
