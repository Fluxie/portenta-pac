///Register `ELCSARA` reader
pub type R = crate::R<ElcsaraSpec>;
///Register `ELCSARA` writer
pub type W = crate::W<ElcsaraSpec>;
/**Event Link Controller Register Security Attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elcr {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Elcr> for bool {
    #[inline(always)]
    fn from(variant: Elcr) -> Self {
        variant as u8 != 0
    }
}
///Field `ELCR` reader - Event Link Controller Register Security Attribution
pub type ElcrR = crate::BitReader<Elcr>;
impl ElcrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Elcr {
        match self.bits {
            false => Elcr::_0,
            true => Elcr::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Elcr::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Elcr::_1
    }
}
///Field `ELCR` writer - Event Link Controller Register Security Attribution
pub type ElcrW<'a, REG> = crate::BitWriter<'a, REG, Elcr>;
impl<'a, REG> ElcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Elcr::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Elcr::_1)
    }
}
/**Event Link Software Event Generation Register 0 Security Attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elsegr0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Elsegr0> for bool {
    #[inline(always)]
    fn from(variant: Elsegr0) -> Self {
        variant as u8 != 0
    }
}
///Field `ELSEGR0` reader - Event Link Software Event Generation Register 0 Security Attribution
pub type Elsegr0R = crate::BitReader<Elsegr0>;
impl Elsegr0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Elsegr0 {
        match self.bits {
            false => Elsegr0::_0,
            true => Elsegr0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Elsegr0::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Elsegr0::_1
    }
}
///Field `ELSEGR0` writer - Event Link Software Event Generation Register 0 Security Attribution
pub type Elsegr0W<'a, REG> = crate::BitWriter<'a, REG, Elsegr0>;
impl<'a, REG> Elsegr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Elsegr0::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Elsegr0::_1)
    }
}
/**Event Link Software Event Generation Register 1 Security Attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elsegr1 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Elsegr1> for bool {
    #[inline(always)]
    fn from(variant: Elsegr1) -> Self {
        variant as u8 != 0
    }
}
///Field `ELSEGR1` reader - Event Link Software Event Generation Register 1 Security Attribution
pub type Elsegr1R = crate::BitReader<Elsegr1>;
impl Elsegr1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Elsegr1 {
        match self.bits {
            false => Elsegr1::_0,
            true => Elsegr1::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Elsegr1::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Elsegr1::_1
    }
}
///Field `ELSEGR1` writer - Event Link Software Event Generation Register 1 Security Attribution
pub type Elsegr1W<'a, REG> = crate::BitWriter<'a, REG, Elsegr1>;
impl<'a, REG> Elsegr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Elsegr1::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Elsegr1::_1)
    }
}
impl R {
    ///Bit 0 - Event Link Controller Register Security Attribution
    #[inline(always)]
    pub fn elcr(&self) -> ElcrR {
        ElcrR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Event Link Software Event Generation Register 0 Security Attribution
    #[inline(always)]
    pub fn elsegr0(&self) -> Elsegr0R {
        Elsegr0R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Event Link Software Event Generation Register 1 Security Attribution
    #[inline(always)]
    pub fn elsegr1(&self) -> Elsegr1R {
        Elsegr1R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ELCSARA")
            .field("elcr", &self.elcr())
            .field("elsegr0", &self.elsegr0())
            .field("elsegr1", &self.elsegr1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Event Link Controller Register Security Attribution
    #[inline(always)]
    pub fn elcr(&mut self) -> ElcrW<ElcsaraSpec> {
        ElcrW::new(self, 0)
    }
    ///Bit 1 - Event Link Software Event Generation Register 0 Security Attribution
    #[inline(always)]
    pub fn elsegr0(&mut self) -> Elsegr0W<ElcsaraSpec> {
        Elsegr0W::new(self, 1)
    }
    ///Bit 2 - Event Link Software Event Generation Register 1 Security Attribution
    #[inline(always)]
    pub fn elsegr1(&mut self) -> Elsegr1W<ElcsaraSpec> {
        Elsegr1W::new(self, 2)
    }
}
/**Event Link Controller Security Attribution Register A

You can [`read`](crate::Reg::read) this register and get [`elcsara::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elcsara::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ElcsaraSpec;
impl crate::RegisterSpec for ElcsaraSpec {
    type Ux = u16;
}
///`read()` method returns [`elcsara::R`](R) reader structure
impl crate::Readable for ElcsaraSpec {}
///`write(|w| ..)` method takes [`elcsara::W`](W) writer structure
impl crate::Writable for ElcsaraSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ELCSARA to value 0xffff
impl crate::Resettable for ElcsaraSpec {
    const RESET_VALUE: u16 = 0xffff;
}
