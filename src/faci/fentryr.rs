///Register `FENTRYR` reader
pub type R = crate::R<FentryrSpec>;
///Register `FENTRYR` writer
pub type W = crate::W<FentryrSpec>;
/**Code Flash P/E Mode Entry

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fentryc {
    ///0: Code flash is in read mode
    _0 = 0,
    ///1: Code flash is in P/E mode.
    _1 = 1,
}
impl From<Fentryc> for bool {
    #[inline(always)]
    fn from(variant: Fentryc) -> Self {
        variant as u8 != 0
    }
}
///Field `FENTRYC` reader - Code Flash P/E Mode Entry
pub type FentrycR = crate::BitReader<Fentryc>;
impl FentrycR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fentryc {
        match self.bits {
            false => Fentryc::_0,
            true => Fentryc::_1,
        }
    }
    ///Code flash is in read mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fentryc::_0
    }
    ///Code flash is in P/E mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fentryc::_1
    }
}
///Field `FENTRYC` writer - Code Flash P/E Mode Entry
pub type FentrycW<'a, REG> = crate::BitWriter<'a, REG, Fentryc>;
impl<'a, REG> FentrycW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Code flash is in read mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fentryc::_0)
    }
    ///Code flash is in P/E mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fentryc::_1)
    }
}
/**Data Flash P/E Mode Entry

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fentryd {
    ///0: Data flash is in read mode
    _0 = 0,
    ///1: Data flash is in P/E mode.
    _1 = 1,
}
impl From<Fentryd> for bool {
    #[inline(always)]
    fn from(variant: Fentryd) -> Self {
        variant as u8 != 0
    }
}
///Field `FENTRYD` reader - Data Flash P/E Mode Entry
pub type FentrydR = crate::BitReader<Fentryd>;
impl FentrydR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fentryd {
        match self.bits {
            false => Fentryd::_0,
            true => Fentryd::_1,
        }
    }
    ///Data flash is in read mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fentryd::_0
    }
    ///Data flash is in P/E mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fentryd::_1
    }
}
///Field `FENTRYD` writer - Data Flash P/E Mode Entry
pub type FentrydW<'a, REG> = crate::BitWriter<'a, REG, Fentryd>;
impl<'a, REG> FentrydW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data flash is in read mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fentryd::_0)
    }
    ///Data flash is in P/E mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fentryd::_1)
    }
}
///Field `KEY` writer - Key Code
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Code Flash P/E Mode Entry
    #[inline(always)]
    pub fn fentryc(&self) -> FentrycR {
        FentrycR::new((self.bits & 1) != 0)
    }
    ///Bit 7 - Data Flash P/E Mode Entry
    #[inline(always)]
    pub fn fentryd(&self) -> FentrydR {
        FentrydR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FENTRYR")
            .field("fentryc", &self.fentryc())
            .field("fentryd", &self.fentryd())
            .finish()
    }
}
impl W {
    ///Bit 0 - Code Flash P/E Mode Entry
    #[inline(always)]
    pub fn fentryc(&mut self) -> FentrycW<FentryrSpec> {
        FentrycW::new(self, 0)
    }
    ///Bit 7 - Data Flash P/E Mode Entry
    #[inline(always)]
    pub fn fentryd(&mut self) -> FentrydW<FentryrSpec> {
        FentrydW::new(self, 7)
    }
    ///Bits 8:15 - Key Code
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<FentryrSpec> {
        KeyW::new(self, 8)
    }
}
/**Flash P/E Mode Entry Register

You can [`read`](crate::Reg::read) this register and get [`fentryr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fentryr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FentryrSpec;
impl crate::RegisterSpec for FentryrSpec {
    type Ux = u16;
}
///`read()` method returns [`fentryr::R`](R) reader structure
impl crate::Readable for FentryrSpec {}
///`write(|w| ..)` method takes [`fentryr::W`](W) writer structure
impl crate::Writable for FentryrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FENTRYR to value 0
impl crate::Resettable for FentryrSpec {}
