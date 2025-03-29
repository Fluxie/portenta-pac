///Register `ADCSR` reader
pub type R = crate::R<AdcsrSpec>;
///Register `ADCSR` writer
pub type W = crate::W<AdcsrSpec>;
///Field `DBLANS` reader - Double Trigger Channel Select
pub type DblansR = crate::FieldReader;
///Field `DBLANS` writer - Double Trigger Channel Select
pub type DblansW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Group B Scan End Interrupt and ELC Event Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gbadie {
    ///0: Disable ADC12i_GBADI (i = 0, 1) interrupt generation on group B scan completion.
    _0 = 0,
    ///1: Enable ADC12i_GBADI (i = 0, 1) interrupt generation on group B scan completion.
    _1 = 1,
}
impl From<Gbadie> for bool {
    #[inline(always)]
    fn from(variant: Gbadie) -> Self {
        variant as u8 != 0
    }
}
///Field `GBADIE` reader - Group B Scan End Interrupt and ELC Event Enable
pub type GbadieR = crate::BitReader<Gbadie>;
impl GbadieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gbadie {
        match self.bits {
            false => Gbadie::_0,
            true => Gbadie::_1,
        }
    }
    ///Disable ADC12i_GBADI (i = 0, 1) interrupt generation on group B scan completion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gbadie::_0
    }
    ///Enable ADC12i_GBADI (i = 0, 1) interrupt generation on group B scan completion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gbadie::_1
    }
}
///Field `GBADIE` writer - Group B Scan End Interrupt and ELC Event Enable
pub type GbadieW<'a, REG> = crate::BitWriter<'a, REG, Gbadie>;
impl<'a, REG> GbadieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable ADC12i_GBADI (i = 0, 1) interrupt generation on group B scan completion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gbadie::_0)
    }
    ///Enable ADC12i_GBADI (i = 0, 1) interrupt generation on group B scan completion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gbadie::_1)
    }
}
/**Double Trigger Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dble {
    ///0: Deselect double-trigger mode.
    _0 = 0,
    ///1: Select double-trigger mode.
    _1 = 1,
}
impl From<Dble> for bool {
    #[inline(always)]
    fn from(variant: Dble) -> Self {
        variant as u8 != 0
    }
}
///Field `DBLE` reader - Double Trigger Mode Select
pub type DbleR = crate::BitReader<Dble>;
impl DbleR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dble {
        match self.bits {
            false => Dble::_0,
            true => Dble::_1,
        }
    }
    ///Deselect double-trigger mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dble::_0
    }
    ///Select double-trigger mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dble::_1
    }
}
///Field `DBLE` writer - Double Trigger Mode Select
pub type DbleW<'a, REG> = crate::BitWriter<'a, REG, Dble>;
impl<'a, REG> DbleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deselect double-trigger mode.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dble::_0)
    }
    ///Select double-trigger mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dble::_1)
    }
}
/**Trigger Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extrg {
    ///0: Start A/D conversion by the synchronous trigger (ELC).
    _0 = 0,
    ///1: Start A/D conversion by the asynchronous trigger (ADTRG0).
    _1 = 1,
}
impl From<Extrg> for bool {
    #[inline(always)]
    fn from(variant: Extrg) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTRG` reader - Trigger Select
pub type ExtrgR = crate::BitReader<Extrg>;
impl ExtrgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Extrg {
        match self.bits {
            false => Extrg::_0,
            true => Extrg::_1,
        }
    }
    ///Start A/D conversion by the synchronous trigger (ELC).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Extrg::_0
    }
    ///Start A/D conversion by the asynchronous trigger (ADTRG0).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Extrg::_1
    }
}
///Field `EXTRG` writer - Trigger Select
pub type ExtrgW<'a, REG> = crate::BitWriter<'a, REG, Extrg>;
impl<'a, REG> ExtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start A/D conversion by the synchronous trigger (ELC).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Extrg::_0)
    }
    ///Start A/D conversion by the asynchronous trigger (ADTRG0).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Extrg::_1)
    }
}
/**Trigger Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trge {
    ///0: Disable A/D conversion to be started by the synchronous or asynchronous trigger
    _0 = 0,
    ///1: Enable A/D conversion to be started by the synchronous or asynchronous trigger
    _1 = 1,
}
impl From<Trge> for bool {
    #[inline(always)]
    fn from(variant: Trge) -> Self {
        variant as u8 != 0
    }
}
///Field `TRGE` reader - Trigger Start Enable
pub type TrgeR = crate::BitReader<Trge>;
impl TrgeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Trge {
        match self.bits {
            false => Trge::_0,
            true => Trge::_1,
        }
    }
    ///Disable A/D conversion to be started by the synchronous or asynchronous trigger
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trge::_0
    }
    ///Enable A/D conversion to be started by the synchronous or asynchronous trigger
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trge::_1
    }
}
///Field `TRGE` writer - Trigger Start Enable
pub type TrgeW<'a, REG> = crate::BitWriter<'a, REG, Trge>;
impl<'a, REG> TrgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable A/D conversion to be started by the synchronous or asynchronous trigger
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trge::_0)
    }
    ///Enable A/D conversion to be started by the synchronous or asynchronous trigger
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trge::_1)
    }
}
/**Scan Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcs {
    ///0: Single scan mode
    _00 = 0,
    ///1: Group scan mode
    _01 = 1,
    ///2: Continuous scan mode
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Adcs> for u8 {
    #[inline(always)]
    fn from(variant: Adcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcs {
    type Ux = u8;
}
impl crate::IsEnum for Adcs {}
///Field `ADCS` reader - Scan Mode Select
pub type AdcsR = crate::FieldReader<Adcs>;
impl AdcsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adcs {
        match self.bits {
            0 => Adcs::_00,
            1 => Adcs::_01,
            2 => Adcs::_10,
            3 => Adcs::_11,
            _ => unreachable!(),
        }
    }
    ///Single scan mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Adcs::_00
    }
    ///Group scan mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Adcs::_01
    }
    ///Continuous scan mode
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Adcs::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Adcs::_11
    }
}
///Field `ADCS` writer - Scan Mode Select
pub type AdcsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcs, crate::Safe>;
impl<'a, REG> AdcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Single scan mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Adcs::_00)
    }
    ///Group scan mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Adcs::_01)
    }
    ///Continuous scan mode
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Adcs::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Adcs::_11)
    }
}
/**A/D Conversion Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adst {
    ///0: Stop A/D conversion process.
    _0 = 0,
    ///1: Start A/D conversion process.
    _1 = 1,
}
impl From<Adst> for bool {
    #[inline(always)]
    fn from(variant: Adst) -> Self {
        variant as u8 != 0
    }
}
///Field `ADST` reader - A/D Conversion Start
pub type AdstR = crate::BitReader<Adst>;
impl AdstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adst {
        match self.bits {
            false => Adst::_0,
            true => Adst::_1,
        }
    }
    ///Stop A/D conversion process.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adst::_0
    }
    ///Start A/D conversion process.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adst::_1
    }
}
///Field `ADST` writer - A/D Conversion Start
pub type AdstW<'a, REG> = crate::BitWriter<'a, REG, Adst>;
impl<'a, REG> AdstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop A/D conversion process.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adst::_0)
    }
    ///Start A/D conversion process.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adst::_1)
    }
}
impl R {
    ///Bits 0:4 - Double Trigger Channel Select
    #[inline(always)]
    pub fn dblans(&self) -> DblansR {
        DblansR::new((self.bits & 0x1f) as u8)
    }
    ///Bit 6 - Group B Scan End Interrupt and ELC Event Enable
    #[inline(always)]
    pub fn gbadie(&self) -> GbadieR {
        GbadieR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Double Trigger Mode Select
    #[inline(always)]
    pub fn dble(&self) -> DbleR {
        DbleR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Trigger Select
    #[inline(always)]
    pub fn extrg(&self) -> ExtrgR {
        ExtrgR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Trigger Start Enable
    #[inline(always)]
    pub fn trge(&self) -> TrgeR {
        TrgeR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 13:14 - Scan Mode Select
    #[inline(always)]
    pub fn adcs(&self) -> AdcsR {
        AdcsR::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - A/D Conversion Start
    #[inline(always)]
    pub fn adst(&self) -> AdstR {
        AdstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSR")
            .field("dblans", &self.dblans())
            .field("gbadie", &self.gbadie())
            .field("dble", &self.dble())
            .field("extrg", &self.extrg())
            .field("trge", &self.trge())
            .field("adcs", &self.adcs())
            .field("adst", &self.adst())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Double Trigger Channel Select
    #[inline(always)]
    pub fn dblans(&mut self) -> DblansW<AdcsrSpec> {
        DblansW::new(self, 0)
    }
    ///Bit 6 - Group B Scan End Interrupt and ELC Event Enable
    #[inline(always)]
    pub fn gbadie(&mut self) -> GbadieW<AdcsrSpec> {
        GbadieW::new(self, 6)
    }
    ///Bit 7 - Double Trigger Mode Select
    #[inline(always)]
    pub fn dble(&mut self) -> DbleW<AdcsrSpec> {
        DbleW::new(self, 7)
    }
    ///Bit 8 - Trigger Select
    #[inline(always)]
    pub fn extrg(&mut self) -> ExtrgW<AdcsrSpec> {
        ExtrgW::new(self, 8)
    }
    ///Bit 9 - Trigger Start Enable
    #[inline(always)]
    pub fn trge(&mut self) -> TrgeW<AdcsrSpec> {
        TrgeW::new(self, 9)
    }
    ///Bits 13:14 - Scan Mode Select
    #[inline(always)]
    pub fn adcs(&mut self) -> AdcsW<AdcsrSpec> {
        AdcsW::new(self, 13)
    }
    ///Bit 15 - A/D Conversion Start
    #[inline(always)]
    pub fn adst(&mut self) -> AdstW<AdcsrSpec> {
        AdstW::new(self, 15)
    }
}
/**A/D Control Register

You can [`read`](crate::Reg::read) this register and get [`adcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcsrSpec;
impl crate::RegisterSpec for AdcsrSpec {
    type Ux = u16;
}
///`read()` method returns [`adcsr::R`](R) reader structure
impl crate::Readable for AdcsrSpec {}
///`write(|w| ..)` method takes [`adcsr::W`](W) writer structure
impl crate::Writable for AdcsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCSR to value 0
impl crate::Resettable for AdcsrSpec {}
