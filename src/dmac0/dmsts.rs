///Register `DMSTS` reader
pub type R = crate::R<DmstsSpec>;
///Register `DMSTS` writer
pub type W = crate::W<DmstsSpec>;
/**Transfer Escape End Interrupt Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esif {
    ///0: A transfer escape end interrupt has not been generated.
    _0 = 0,
    ///1: A transfer escape end interrupt has been generated.
    _1 = 1,
}
impl From<Esif> for bool {
    #[inline(always)]
    fn from(variant: Esif) -> Self {
        variant as u8 != 0
    }
}
///Field `ESIF` reader - Transfer Escape End Interrupt Flag
pub type EsifR = crate::BitReader<Esif>;
impl EsifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Esif {
        match self.bits {
            false => Esif::_0,
            true => Esif::_1,
        }
    }
    ///A transfer escape end interrupt has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Esif::_0
    }
    ///A transfer escape end interrupt has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Esif::_1
    }
}
///Field `ESIF` writer - Transfer Escape End Interrupt Flag
pub type EsifW<'a, REG> = crate::BitWriter<'a, REG, Esif>;
impl<'a, REG> EsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A transfer escape end interrupt has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Esif::_0)
    }
    ///A transfer escape end interrupt has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Esif::_1)
    }
}
/**Transfer End Interrupt Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtif {
    ///0: A transfer end interrupt has not been generated.
    _0 = 0,
    ///1: A transfer end interrupt has been generated.
    _1 = 1,
}
impl From<Dtif> for bool {
    #[inline(always)]
    fn from(variant: Dtif) -> Self {
        variant as u8 != 0
    }
}
///Field `DTIF` reader - Transfer End Interrupt Flag
pub type DtifR = crate::BitReader<Dtif>;
impl DtifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dtif {
        match self.bits {
            false => Dtif::_0,
            true => Dtif::_1,
        }
    }
    ///A transfer end interrupt has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtif::_0
    }
    ///A transfer end interrupt has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtif::_1
    }
}
///Field `DTIF` writer - Transfer End Interrupt Flag
pub type DtifW<'a, REG> = crate::BitWriter<'a, REG, Dtif>;
impl<'a, REG> DtifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A transfer end interrupt has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtif::_0)
    }
    ///A transfer end interrupt has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtif::_1)
    }
}
/**DMAC Active Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    ///0: DMAC is in the idle state.
    _0 = 0,
    ///1: DMAC is operating.
    _1 = 1,
}
impl From<Act> for bool {
    #[inline(always)]
    fn from(variant: Act) -> Self {
        variant as u8 != 0
    }
}
///Field `ACT` reader - DMAC Active Flag
pub type ActR = crate::BitReader<Act>;
impl ActR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Act {
        match self.bits {
            false => Act::_0,
            true => Act::_1,
        }
    }
    ///DMAC is in the idle state.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Act::_0
    }
    ///DMAC is operating.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Act::_1
    }
}
impl R {
    ///Bit 0 - Transfer Escape End Interrupt Flag
    #[inline(always)]
    pub fn esif(&self) -> EsifR {
        EsifR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Transfer End Interrupt Flag
    #[inline(always)]
    pub fn dtif(&self) -> DtifR {
        DtifR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - DMAC Active Flag
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMSTS")
            .field("esif", &self.esif())
            .field("dtif", &self.dtif())
            .field("act", &self.act())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer Escape End Interrupt Flag
    #[inline(always)]
    pub fn esif(&mut self) -> EsifW<DmstsSpec> {
        EsifW::new(self, 0)
    }
    ///Bit 4 - Transfer End Interrupt Flag
    #[inline(always)]
    pub fn dtif(&mut self) -> DtifW<DmstsSpec> {
        DtifW::new(self, 4)
    }
}
/**DMA Status Register

You can [`read`](crate::Reg::read) this register and get [`dmsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmstsSpec;
impl crate::RegisterSpec for DmstsSpec {
    type Ux = u8;
}
///`read()` method returns [`dmsts::R`](R) reader structure
impl crate::Readable for DmstsSpec {}
///`write(|w| ..)` method takes [`dmsts::W`](W) writer structure
impl crate::Writable for DmstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMSTS to value 0
impl crate::Resettable for DmstsSpec {}
