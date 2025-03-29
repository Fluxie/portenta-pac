///Register `DMREQ` reader
pub type R = crate::R<DmreqSpec>;
///Register `DMREQ` writer
pub type W = crate::W<DmreqSpec>;
/**DMA Software Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swreq {
    ///0: DMA transfer is not requested.
    _0 = 0,
    ///1: DMA transfer is requested.
    _1 = 1,
}
impl From<Swreq> for bool {
    #[inline(always)]
    fn from(variant: Swreq) -> Self {
        variant as u8 != 0
    }
}
///Field `SWREQ` reader - DMA Software Start
pub type SwreqR = crate::BitReader<Swreq>;
impl SwreqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Swreq {
        match self.bits {
            false => Swreq::_0,
            true => Swreq::_1,
        }
    }
    ///DMA transfer is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Swreq::_0
    }
    ///DMA transfer is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Swreq::_1
    }
}
///Field `SWREQ` writer - DMA Software Start
pub type SwreqW<'a, REG> = crate::BitWriter<'a, REG, Swreq>;
impl<'a, REG> SwreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfer is not requested.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Swreq::_0)
    }
    ///DMA transfer is requested.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Swreq::_1)
    }
}
/**DMA Software Start Bit Auto Clear Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clrs {
    ///0: SWREQ bit is cleared after DMA transfer is started by software.
    _0 = 0,
    ///1: SWREQ bit is not cleared after DMA transfer is started by software.
    _1 = 1,
}
impl From<Clrs> for bool {
    #[inline(always)]
    fn from(variant: Clrs) -> Self {
        variant as u8 != 0
    }
}
///Field `CLRS` reader - DMA Software Start Bit Auto Clear Select
pub type ClrsR = crate::BitReader<Clrs>;
impl ClrsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Clrs {
        match self.bits {
            false => Clrs::_0,
            true => Clrs::_1,
        }
    }
    ///SWREQ bit is cleared after DMA transfer is started by software.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Clrs::_0
    }
    ///SWREQ bit is not cleared after DMA transfer is started by software.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Clrs::_1
    }
}
///Field `CLRS` writer - DMA Software Start Bit Auto Clear Select
pub type ClrsW<'a, REG> = crate::BitWriter<'a, REG, Clrs>;
impl<'a, REG> ClrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SWREQ bit is cleared after DMA transfer is started by software.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Clrs::_0)
    }
    ///SWREQ bit is not cleared after DMA transfer is started by software.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Clrs::_1)
    }
}
impl R {
    ///Bit 0 - DMA Software Start
    #[inline(always)]
    pub fn swreq(&self) -> SwreqR {
        SwreqR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DMA Software Start Bit Auto Clear Select
    #[inline(always)]
    pub fn clrs(&self) -> ClrsR {
        ClrsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMREQ")
            .field("swreq", &self.swreq())
            .field("clrs", &self.clrs())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA Software Start
    #[inline(always)]
    pub fn swreq(&mut self) -> SwreqW<DmreqSpec> {
        SwreqW::new(self, 0)
    }
    ///Bit 4 - DMA Software Start Bit Auto Clear Select
    #[inline(always)]
    pub fn clrs(&mut self) -> ClrsW<DmreqSpec> {
        ClrsW::new(self, 4)
    }
}
/**DMA Software Start Register

You can [`read`](crate::Reg::read) this register and get [`dmreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmreqSpec;
impl crate::RegisterSpec for DmreqSpec {
    type Ux = u8;
}
///`read()` method returns [`dmreq::R`](R) reader structure
impl crate::Readable for DmreqSpec {}
///`write(|w| ..)` method takes [`dmreq::W`](W) writer structure
impl crate::Writable for DmreqSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMREQ to value 0
impl crate::Resettable for DmreqSpec {}
