///Register `AGTISR` reader
pub type R = crate::R<AgtisrSpec>;
///Register `AGTISR` writer
pub type W = crate::W<AgtisrSpec>;
/**AGTEEn Polarity Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eeps {
    ///0: An event is counted during the low-level period
    _0 = 0,
    ///1: An event is counted during the high-level period
    _1 = 1,
}
impl From<Eeps> for bool {
    #[inline(always)]
    fn from(variant: Eeps) -> Self {
        variant as u8 != 0
    }
}
///Field `EEPS` reader - AGTEEn Polarity Selection
pub type EepsR = crate::BitReader<Eeps>;
impl EepsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eeps {
        match self.bits {
            false => Eeps::_0,
            true => Eeps::_1,
        }
    }
    ///An event is counted during the low-level period
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eeps::_0
    }
    ///An event is counted during the high-level period
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eeps::_1
    }
}
///Field `EEPS` writer - AGTEEn Polarity Selection
pub type EepsW<'a, REG> = crate::BitWriter<'a, REG, Eeps>;
impl<'a, REG> EepsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///An event is counted during the low-level period
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eeps::_0)
    }
    ///An event is counted during the high-level period
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eeps::_1)
    }
}
impl R {
    ///Bit 2 - AGTEEn Polarity Selection
    #[inline(always)]
    pub fn eeps(&self) -> EepsR {
        EepsR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTISR").field("eeps", &self.eeps()).finish()
    }
}
impl W {
    ///Bit 2 - AGTEEn Polarity Selection
    #[inline(always)]
    pub fn eeps(&mut self) -> EepsW<AgtisrSpec> {
        EepsW::new(self, 2)
    }
}
/**AGT Event Pin Select Register

You can [`read`](crate::Reg::read) this register and get [`agtisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AgtisrSpec;
impl crate::RegisterSpec for AgtisrSpec {
    type Ux = u8;
}
///`read()` method returns [`agtisr::R`](R) reader structure
impl crate::Readable for AgtisrSpec {}
///`write(|w| ..)` method takes [`agtisr::W`](W) writer structure
impl crate::Writable for AgtisrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTISR to value 0
impl crate::Resettable for AgtisrSpec {}
