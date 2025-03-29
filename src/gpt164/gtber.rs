///Register `GTBER` reader
pub type R = crate::R<GtberSpec>;
///Register `GTBER` writer
pub type W = crate::W<GtberSpec>;
/**GTCCR Buffer Operation Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bd0 {
    ///0: Buffer operation is enabled
    _0 = 0,
    ///1: Buffer operation is disabled
    _1 = 1,
}
impl From<Bd0> for bool {
    #[inline(always)]
    fn from(variant: Bd0) -> Self {
        variant as u8 != 0
    }
}
///Field `BD0` reader - GTCCR Buffer Operation Disable
pub type Bd0R = crate::BitReader<Bd0>;
impl Bd0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bd0 {
        match self.bits {
            false => Bd0::_0,
            true => Bd0::_1,
        }
    }
    ///Buffer operation is enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bd0::_0
    }
    ///Buffer operation is disabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bd0::_1
    }
}
///Field `BD0` writer - GTCCR Buffer Operation Disable
pub type Bd0W<'a, REG> = crate::BitWriter<'a, REG, Bd0>;
impl<'a, REG> Bd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Buffer operation is enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bd0::_0)
    }
    ///Buffer operation is disabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bd0::_1)
    }
}
/**GTPR Buffer Operation Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bd1 {
    ///0: Buffer operation is enabled
    _0 = 0,
    ///1: Buffer operation is disabled
    _1 = 1,
}
impl From<Bd1> for bool {
    #[inline(always)]
    fn from(variant: Bd1) -> Self {
        variant as u8 != 0
    }
}
///Field `BD1` reader - GTPR Buffer Operation Disable
pub type Bd1R = crate::BitReader<Bd1>;
impl Bd1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bd1 {
        match self.bits {
            false => Bd1::_0,
            true => Bd1::_1,
        }
    }
    ///Buffer operation is enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bd1::_0
    }
    ///Buffer operation is disabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bd1::_1
    }
}
///Field `BD1` writer - GTPR Buffer Operation Disable
pub type Bd1W<'a, REG> = crate::BitWriter<'a, REG, Bd1>;
impl<'a, REG> Bd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Buffer operation is enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bd1::_0)
    }
    ///Buffer operation is disabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bd1::_1)
    }
}
/**GTCCRA Buffer Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccra {
    ///0: No buffer operation
    _00 = 0,
    ///1: Single buffer operation (GTCCRA <---->GTCCRC)
    _01 = 1,
    ///2: Double buffer operation (GTCCRA <----> GTCCRC <----> GTCCRD)
    Others = 2,
}
impl From<Ccra> for u8 {
    #[inline(always)]
    fn from(variant: Ccra) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccra {
    type Ux = u8;
}
impl crate::IsEnum for Ccra {}
///Field `CCRA` reader - GTCCRA Buffer Operation
pub type CcraR = crate::FieldReader<Ccra>;
impl CcraR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ccra {
        match self.bits {
            0 => Ccra::_00,
            1 => Ccra::_01,
            _ => Ccra::Others,
        }
    }
    ///No buffer operation
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ccra::_00
    }
    ///Single buffer operation (GTCCRA <---->GTCCRC)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ccra::_01
    }
    ///Double buffer operation (GTCCRA <----> GTCCRC <----> GTCCRD)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ccra::Others)
    }
}
///Field `CCRA` writer - GTCCRA Buffer Operation
pub type CcraW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccra, crate::Safe>;
impl<'a, REG> CcraW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No buffer operation
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ccra::_00)
    }
    ///Single buffer operation (GTCCRA <---->GTCCRC)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ccra::_01)
    }
    ///Double buffer operation (GTCCRA <----> GTCCRC <----> GTCCRD)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ccra::Others)
    }
}
/**GTCCRB Buffer Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccrb {
    ///0: No buffer operation
    _00 = 0,
    ///1: Single buffer operation (GTCCRB <----> GTCCRE)
    _01 = 1,
    ///2: Double buffer operation (GTCCRB <----> GTCCRE <----> GTCCRF)
    Others = 2,
}
impl From<Ccrb> for u8 {
    #[inline(always)]
    fn from(variant: Ccrb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccrb {
    type Ux = u8;
}
impl crate::IsEnum for Ccrb {}
///Field `CCRB` reader - GTCCRB Buffer Operation
pub type CcrbR = crate::FieldReader<Ccrb>;
impl CcrbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ccrb {
        match self.bits {
            0 => Ccrb::_00,
            1 => Ccrb::_01,
            _ => Ccrb::Others,
        }
    }
    ///No buffer operation
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ccrb::_00
    }
    ///Single buffer operation (GTCCRB <----> GTCCRE)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ccrb::_01
    }
    ///Double buffer operation (GTCCRB <----> GTCCRE <----> GTCCRF)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ccrb::Others)
    }
}
///Field `CCRB` writer - GTCCRB Buffer Operation
pub type CcrbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccrb, crate::Safe>;
impl<'a, REG> CcrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No buffer operation
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrb::_00)
    }
    ///Single buffer operation (GTCCRB <----> GTCCRE)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrb::_01)
    }
    ///Double buffer operation (GTCCRB <----> GTCCRE <----> GTCCRF)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrb::Others)
    }
}
/**GTPR Buffer Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pr {
    ///0: No buffer operation
    _00 = 0,
    ///1: Single buffer operation (GTPBR --> GTPR)
    _01 = 1,
    ///2: Setting prohibited
    Others = 2,
}
impl From<Pr> for u8 {
    #[inline(always)]
    fn from(variant: Pr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pr {
    type Ux = u8;
}
impl crate::IsEnum for Pr {}
///Field `PR` reader - GTPR Buffer Operation
pub type PrR = crate::FieldReader<Pr>;
impl PrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pr {
        match self.bits {
            0 => Pr::_00,
            1 => Pr::_01,
            _ => Pr::Others,
        }
    }
    ///No buffer operation
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Pr::_00
    }
    ///Single buffer operation (GTPBR --> GTPR)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Pr::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Pr::Others)
    }
}
///Field `PR` writer - GTPR Buffer Operation
pub type PrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pr, crate::Safe>;
impl<'a, REG> PrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No buffer operation
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::_00)
    }
    ///Single buffer operation (GTPBR --> GTPR)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::Others)
    }
}
///Field `CCRSWT` writer - GTCCRA and GTCCRB Forcible Buffer Operation
pub type CcrswtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GTCCR Buffer Operation Disable
    #[inline(always)]
    pub fn bd0(&self) -> Bd0R {
        Bd0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTPR Buffer Operation Disable
    #[inline(always)]
    pub fn bd1(&self) -> Bd1R {
        Bd1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 16:17 - GTCCRA Buffer Operation
    #[inline(always)]
    pub fn ccra(&self) -> CcraR {
        CcraR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - GTCCRB Buffer Operation
    #[inline(always)]
    pub fn ccrb(&self) -> CcrbR {
        CcrbR::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - GTPR Buffer Operation
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTBER")
            .field("bd0", &self.bd0())
            .field("bd1", &self.bd1())
            .field("ccra", &self.ccra())
            .field("ccrb", &self.ccrb())
            .field("pr", &self.pr())
            .finish()
    }
}
impl W {
    ///Bit 0 - GTCCR Buffer Operation Disable
    #[inline(always)]
    pub fn bd0(&mut self) -> Bd0W<GtberSpec> {
        Bd0W::new(self, 0)
    }
    ///Bit 1 - GTPR Buffer Operation Disable
    #[inline(always)]
    pub fn bd1(&mut self) -> Bd1W<GtberSpec> {
        Bd1W::new(self, 1)
    }
    ///Bits 16:17 - GTCCRA Buffer Operation
    #[inline(always)]
    pub fn ccra(&mut self) -> CcraW<GtberSpec> {
        CcraW::new(self, 16)
    }
    ///Bits 18:19 - GTCCRB Buffer Operation
    #[inline(always)]
    pub fn ccrb(&mut self) -> CcrbW<GtberSpec> {
        CcrbW::new(self, 18)
    }
    ///Bits 20:21 - GTPR Buffer Operation
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<GtberSpec> {
        PrW::new(self, 20)
    }
    ///Bit 22 - GTCCRA and GTCCRB Forcible Buffer Operation
    #[inline(always)]
    pub fn ccrswt(&mut self) -> CcrswtW<GtberSpec> {
        CcrswtW::new(self, 22)
    }
}
/**General PWM Timer Buffer Enable Register

You can [`read`](crate::Reg::read) this register and get [`gtber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtberSpec;
impl crate::RegisterSpec for GtberSpec {
    type Ux = u32;
}
///`read()` method returns [`gtber::R`](R) reader structure
impl crate::Readable for GtberSpec {}
///`write(|w| ..)` method takes [`gtber::W`](W) writer structure
impl crate::Writable for GtberSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTBER to value 0
impl crate::Resettable for GtberSpec {}
