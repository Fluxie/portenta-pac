///Register `DMINT` reader
pub type R = crate::R<DmintSpec>;
///Register `DMINT` writer
pub type W = crate::W<DmintSpec>;
/**Destination Address Extended Repeat Area Overflow Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Darie {
    ///0: Disables an interrupt request for an extended repeat area overflow on the destination address.
    _0 = 0,
    ///1: Enables an interrupt request for an extended repeat area overflow on the destination address.
    _1 = 1,
}
impl From<Darie> for bool {
    #[inline(always)]
    fn from(variant: Darie) -> Self {
        variant as u8 != 0
    }
}
///Field `DARIE` reader - Destination Address Extended Repeat Area Overflow Interrupt Enable
pub type DarieR = crate::BitReader<Darie>;
impl DarieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Darie {
        match self.bits {
            false => Darie::_0,
            true => Darie::_1,
        }
    }
    ///Disables an interrupt request for an extended repeat area overflow on the destination address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Darie::_0
    }
    ///Enables an interrupt request for an extended repeat area overflow on the destination address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Darie::_1
    }
}
///Field `DARIE` writer - Destination Address Extended Repeat Area Overflow Interrupt Enable
pub type DarieW<'a, REG> = crate::BitWriter<'a, REG, Darie>;
impl<'a, REG> DarieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an interrupt request for an extended repeat area overflow on the destination address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Darie::_0)
    }
    ///Enables an interrupt request for an extended repeat area overflow on the destination address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Darie::_1)
    }
}
/**Source Address Extended Repeat Area Overflow Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sarie {
    ///0: Disables an interrupt request for an extended repeat area overflow on the source address.
    _0 = 0,
    ///1: Enables an interrupt request for an extended repeat area overflow on the source address.
    _1 = 1,
}
impl From<Sarie> for bool {
    #[inline(always)]
    fn from(variant: Sarie) -> Self {
        variant as u8 != 0
    }
}
///Field `SARIE` reader - Source Address Extended Repeat Area Overflow Interrupt Enable
pub type SarieR = crate::BitReader<Sarie>;
impl SarieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sarie {
        match self.bits {
            false => Sarie::_0,
            true => Sarie::_1,
        }
    }
    ///Disables an interrupt request for an extended repeat area overflow on the source address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sarie::_0
    }
    ///Enables an interrupt request for an extended repeat area overflow on the source address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sarie::_1
    }
}
///Field `SARIE` writer - Source Address Extended Repeat Area Overflow Interrupt Enable
pub type SarieW<'a, REG> = crate::BitWriter<'a, REG, Sarie>;
impl<'a, REG> SarieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an interrupt request for an extended repeat area overflow on the source address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sarie::_0)
    }
    ///Enables an interrupt request for an extended repeat area overflow on the source address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sarie::_1)
    }
}
/**Repeat Size End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rptie {
    ///0: Disables the repeat size end interrupt request.
    _0 = 0,
    ///1: Enables the repeat size end interrupt request.
    _1 = 1,
}
impl From<Rptie> for bool {
    #[inline(always)]
    fn from(variant: Rptie) -> Self {
        variant as u8 != 0
    }
}
///Field `RPTIE` reader - Repeat Size End Interrupt Enable
pub type RptieR = crate::BitReader<Rptie>;
impl RptieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rptie {
        match self.bits {
            false => Rptie::_0,
            true => Rptie::_1,
        }
    }
    ///Disables the repeat size end interrupt request.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rptie::_0
    }
    ///Enables the repeat size end interrupt request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rptie::_1
    }
}
///Field `RPTIE` writer - Repeat Size End Interrupt Enable
pub type RptieW<'a, REG> = crate::BitWriter<'a, REG, Rptie>;
impl<'a, REG> RptieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the repeat size end interrupt request.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rptie::_0)
    }
    ///Enables the repeat size end interrupt request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rptie::_1)
    }
}
/**Transfer Escape End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esie {
    ///0: Disables the transfer escape end interrupt request.
    _0 = 0,
    ///1: Enables the transfer escape end interrupt request.
    _1 = 1,
}
impl From<Esie> for bool {
    #[inline(always)]
    fn from(variant: Esie) -> Self {
        variant as u8 != 0
    }
}
///Field `ESIE` reader - Transfer Escape End Interrupt Enable
pub type EsieR = crate::BitReader<Esie>;
impl EsieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Esie {
        match self.bits {
            false => Esie::_0,
            true => Esie::_1,
        }
    }
    ///Disables the transfer escape end interrupt request.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Esie::_0
    }
    ///Enables the transfer escape end interrupt request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Esie::_1
    }
}
///Field `ESIE` writer - Transfer Escape End Interrupt Enable
pub type EsieW<'a, REG> = crate::BitWriter<'a, REG, Esie>;
impl<'a, REG> EsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the transfer escape end interrupt request.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Esie::_0)
    }
    ///Enables the transfer escape end interrupt request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Esie::_1)
    }
}
/**Transfer End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtie {
    ///0: Disables the transfer end interrupt request.
    _0 = 0,
    ///1: Enables the transfer end interrupt request.
    _1 = 1,
}
impl From<Dtie> for bool {
    #[inline(always)]
    fn from(variant: Dtie) -> Self {
        variant as u8 != 0
    }
}
///Field `DTIE` reader - Transfer End Interrupt Enable
pub type DtieR = crate::BitReader<Dtie>;
impl DtieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dtie {
        match self.bits {
            false => Dtie::_0,
            true => Dtie::_1,
        }
    }
    ///Disables the transfer end interrupt request.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtie::_0
    }
    ///Enables the transfer end interrupt request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtie::_1
    }
}
///Field `DTIE` writer - Transfer End Interrupt Enable
pub type DtieW<'a, REG> = crate::BitWriter<'a, REG, Dtie>;
impl<'a, REG> DtieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the transfer end interrupt request.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtie::_0)
    }
    ///Enables the transfer end interrupt request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtie::_1)
    }
}
impl R {
    ///Bit 0 - Destination Address Extended Repeat Area Overflow Interrupt Enable
    #[inline(always)]
    pub fn darie(&self) -> DarieR {
        DarieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Source Address Extended Repeat Area Overflow Interrupt Enable
    #[inline(always)]
    pub fn sarie(&self) -> SarieR {
        SarieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Repeat Size End Interrupt Enable
    #[inline(always)]
    pub fn rptie(&self) -> RptieR {
        RptieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transfer Escape End Interrupt Enable
    #[inline(always)]
    pub fn esie(&self) -> EsieR {
        EsieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transfer End Interrupt Enable
    #[inline(always)]
    pub fn dtie(&self) -> DtieR {
        DtieR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMINT")
            .field("darie", &self.darie())
            .field("sarie", &self.sarie())
            .field("rptie", &self.rptie())
            .field("esie", &self.esie())
            .field("dtie", &self.dtie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Destination Address Extended Repeat Area Overflow Interrupt Enable
    #[inline(always)]
    pub fn darie(&mut self) -> DarieW<DmintSpec> {
        DarieW::new(self, 0)
    }
    ///Bit 1 - Source Address Extended Repeat Area Overflow Interrupt Enable
    #[inline(always)]
    pub fn sarie(&mut self) -> SarieW<DmintSpec> {
        SarieW::new(self, 1)
    }
    ///Bit 2 - Repeat Size End Interrupt Enable
    #[inline(always)]
    pub fn rptie(&mut self) -> RptieW<DmintSpec> {
        RptieW::new(self, 2)
    }
    ///Bit 3 - Transfer Escape End Interrupt Enable
    #[inline(always)]
    pub fn esie(&mut self) -> EsieW<DmintSpec> {
        EsieW::new(self, 3)
    }
    ///Bit 4 - Transfer End Interrupt Enable
    #[inline(always)]
    pub fn dtie(&mut self) -> DtieW<DmintSpec> {
        DtieW::new(self, 4)
    }
}
/**DMA Interrupt Setting Register

You can [`read`](crate::Reg::read) this register and get [`dmint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmintSpec;
impl crate::RegisterSpec for DmintSpec {
    type Ux = u8;
}
///`read()` method returns [`dmint::R`](R) reader structure
impl crate::Readable for DmintSpec {}
///`write(|w| ..)` method takes [`dmint::W`](W) writer structure
impl crate::Writable for DmintSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMINT to value 0
impl crate::Resettable for DmintSpec {}
