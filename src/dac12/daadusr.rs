///Register `DAADUSR` reader
pub type R = crate::R<DaadusrSpec>;
///Register `DAADUSR` writer
pub type W = crate::W<DaadusrSpec>;
/**A/D Unit 1 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amadsel1 {
    ///0: Do not select unit 1
    _0 = 0,
    ///1: Select unit 1
    _1 = 1,
}
impl From<Amadsel1> for bool {
    #[inline(always)]
    fn from(variant: Amadsel1) -> Self {
        variant as u8 != 0
    }
}
///Field `AMADSEL1` reader - A/D Unit 1 Select
pub type Amadsel1R = crate::BitReader<Amadsel1>;
impl Amadsel1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Amadsel1 {
        match self.bits {
            false => Amadsel1::_0,
            true => Amadsel1::_1,
        }
    }
    ///Do not select unit 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amadsel1::_0
    }
    ///Select unit 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amadsel1::_1
    }
}
///Field `AMADSEL1` writer - A/D Unit 1 Select
pub type Amadsel1W<'a, REG> = crate::BitWriter<'a, REG, Amadsel1>;
impl<'a, REG> Amadsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select unit 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amadsel1::_0)
    }
    ///Select unit 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amadsel1::_1)
    }
}
impl R {
    ///Bit 1 - A/D Unit 1 Select
    #[inline(always)]
    pub fn amadsel1(&self) -> Amadsel1R {
        Amadsel1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAADUSR").field("amadsel1", &self.amadsel1()).finish()
    }
}
impl W {
    ///Bit 1 - A/D Unit 1 Select
    #[inline(always)]
    pub fn amadsel1(&mut self) -> Amadsel1W<DaadusrSpec> {
        Amadsel1W::new(self, 1)
    }
}
/**D/A A/D Synchronous Unit Select Register

You can [`read`](crate::Reg::read) this register and get [`daadusr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadusr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DaadusrSpec;
impl crate::RegisterSpec for DaadusrSpec {
    type Ux = u8;
}
///`read()` method returns [`daadusr::R`](R) reader structure
impl crate::Readable for DaadusrSpec {}
///`write(|w| ..)` method takes [`daadusr::W`](W) writer structure
impl crate::Writable for DaadusrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAADUSR to value 0
impl crate::Resettable for DaadusrSpec {}
