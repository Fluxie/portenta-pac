///Register `CAICR` reader
pub type R = crate::R<CaicrSpec>;
///Register `CAICR` writer
pub type W = crate::W<CaicrSpec>;
/**Frequency Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ferrie {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Ferrie> for bool {
    #[inline(always)]
    fn from(variant: Ferrie) -> Self {
        variant as u8 != 0
    }
}
///Field `FERRIE` reader - Frequency Error Interrupt Request Enable
pub type FerrieR = crate::BitReader<Ferrie>;
impl FerrieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ferrie {
        match self.bits {
            false => Ferrie::_0,
            true => Ferrie::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ferrie::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ferrie::_1
    }
}
///Field `FERRIE` writer - Frequency Error Interrupt Request Enable
pub type FerrieW<'a, REG> = crate::BitWriter<'a, REG, Ferrie>;
impl<'a, REG> FerrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ferrie::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ferrie::_1)
    }
}
/**Measurement End Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mendie {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Mendie> for bool {
    #[inline(always)]
    fn from(variant: Mendie) -> Self {
        variant as u8 != 0
    }
}
///Field `MENDIE` reader - Measurement End Interrupt Request Enable
pub type MendieR = crate::BitReader<Mendie>;
impl MendieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mendie {
        match self.bits {
            false => Mendie::_0,
            true => Mendie::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mendie::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mendie::_1
    }
}
///Field `MENDIE` writer - Measurement End Interrupt Request Enable
pub type MendieW<'a, REG> = crate::BitWriter<'a, REG, Mendie>;
impl<'a, REG> MendieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mendie::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mendie::_1)
    }
}
/**Overflow Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovfie {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Ovfie> for bool {
    #[inline(always)]
    fn from(variant: Ovfie) -> Self {
        variant as u8 != 0
    }
}
///Field `OVFIE` reader - Overflow Interrupt Request Enable
pub type OvfieR = crate::BitReader<Ovfie>;
impl OvfieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ovfie {
        match self.bits {
            false => Ovfie::_0,
            true => Ovfie::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovfie::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovfie::_1
    }
}
///Field `OVFIE` writer - Overflow Interrupt Request Enable
pub type OvfieW<'a, REG> = crate::BitWriter<'a, REG, Ovfie>;
impl<'a, REG> OvfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovfie::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovfie::_1)
    }
}
/**FERRF Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ferrfcl {
    ///0: No effect
    _0 = 0,
    ///1: The CASTR.FERRF flag is cleared
    _1 = 1,
}
impl From<Ferrfcl> for bool {
    #[inline(always)]
    fn from(variant: Ferrfcl) -> Self {
        variant as u8 != 0
    }
}
///Field `FERRFCL` writer - FERRF Clear
pub type FerrfclW<'a, REG> = crate::BitWriter<'a, REG, Ferrfcl>;
impl<'a, REG> FerrfclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ferrfcl::_0)
    }
    ///The CASTR.FERRF flag is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ferrfcl::_1)
    }
}
/**MENDF Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mendfcl {
    ///0: No effect
    _0 = 0,
    ///1: The CASTR.MENDF flag is cleared
    _1 = 1,
}
impl From<Mendfcl> for bool {
    #[inline(always)]
    fn from(variant: Mendfcl) -> Self {
        variant as u8 != 0
    }
}
///Field `MENDFCL` writer - MENDF Clear
pub type MendfclW<'a, REG> = crate::BitWriter<'a, REG, Mendfcl>;
impl<'a, REG> MendfclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mendfcl::_0)
    }
    ///The CASTR.MENDF flag is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mendfcl::_1)
    }
}
/**OVFF Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovffcl {
    ///0: No effect
    _0 = 0,
    ///1: The CASTR.OVFF flag is cleared.
    _1 = 1,
}
impl From<Ovffcl> for bool {
    #[inline(always)]
    fn from(variant: Ovffcl) -> Self {
        variant as u8 != 0
    }
}
///Field `OVFFCL` writer - OVFF Clear
pub type OvffclW<'a, REG> = crate::BitWriter<'a, REG, Ovffcl>;
impl<'a, REG> OvffclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovffcl::_0)
    }
    ///The CASTR.OVFF flag is cleared.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovffcl::_1)
    }
}
impl R {
    ///Bit 0 - Frequency Error Interrupt Request Enable
    #[inline(always)]
    pub fn ferrie(&self) -> FerrieR {
        FerrieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Measurement End Interrupt Request Enable
    #[inline(always)]
    pub fn mendie(&self) -> MendieR {
        MendieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn ovfie(&self) -> OvfieR {
        OvfieR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAICR")
            .field("ferrie", &self.ferrie())
            .field("mendie", &self.mendie())
            .field("ovfie", &self.ovfie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Frequency Error Interrupt Request Enable
    #[inline(always)]
    pub fn ferrie(&mut self) -> FerrieW<CaicrSpec> {
        FerrieW::new(self, 0)
    }
    ///Bit 1 - Measurement End Interrupt Request Enable
    #[inline(always)]
    pub fn mendie(&mut self) -> MendieW<CaicrSpec> {
        MendieW::new(self, 1)
    }
    ///Bit 2 - Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn ovfie(&mut self) -> OvfieW<CaicrSpec> {
        OvfieW::new(self, 2)
    }
    ///Bit 4 - FERRF Clear
    #[inline(always)]
    pub fn ferrfcl(&mut self) -> FerrfclW<CaicrSpec> {
        FerrfclW::new(self, 4)
    }
    ///Bit 5 - MENDF Clear
    #[inline(always)]
    pub fn mendfcl(&mut self) -> MendfclW<CaicrSpec> {
        MendfclW::new(self, 5)
    }
    ///Bit 6 - OVFF Clear
    #[inline(always)]
    pub fn ovffcl(&mut self) -> OvffclW<CaicrSpec> {
        OvffclW::new(self, 6)
    }
}
/**CAC Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`caicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CaicrSpec;
impl crate::RegisterSpec for CaicrSpec {
    type Ux = u8;
}
///`read()` method returns [`caicr::R`](R) reader structure
impl crate::Readable for CaicrSpec {}
///`write(|w| ..)` method takes [`caicr::W`](W) writer structure
impl crate::Writable for CaicrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CAICR to value 0
impl crate::Resettable for CaicrSpec {}
