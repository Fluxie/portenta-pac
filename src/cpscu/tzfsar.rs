///Register `TZFSAR` reader
pub type R = crate::R<TzfsarSpec>;
///Register `TZFSAR` writer
pub type W = crate::W<TzfsarSpec>;
/**Security attributes of registers for TrustZone Filter

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tzfsa0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Tzfsa0> for bool {
    #[inline(always)]
    fn from(variant: Tzfsa0) -> Self {
        variant as u8 != 0
    }
}
///Field `TZFSA0` reader - Security attributes of registers for TrustZone Filter
pub type Tzfsa0R = crate::BitReader<Tzfsa0>;
impl Tzfsa0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tzfsa0 {
        match self.bits {
            false => Tzfsa0::_0,
            true => Tzfsa0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tzfsa0::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tzfsa0::_1
    }
}
///Field `TZFSA0` writer - Security attributes of registers for TrustZone Filter
pub type Tzfsa0W<'a, REG> = crate::BitWriter<'a, REG, Tzfsa0>;
impl<'a, REG> Tzfsa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tzfsa0::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tzfsa0::_1)
    }
}
impl R {
    ///Bit 0 - Security attributes of registers for TrustZone Filter
    #[inline(always)]
    pub fn tzfsa0(&self) -> Tzfsa0R {
        Tzfsa0R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZFSAR").field("tzfsa0", &self.tzfsa0()).finish()
    }
}
impl W {
    ///Bit 0 - Security attributes of registers for TrustZone Filter
    #[inline(always)]
    pub fn tzfsa0(&mut self) -> Tzfsa0W<TzfsarSpec> {
        Tzfsa0W::new(self, 0)
    }
}
/**TrustZone Filter Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`tzfsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzfsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TzfsarSpec;
impl crate::RegisterSpec for TzfsarSpec {
    type Ux = u32;
}
///`read()` method returns [`tzfsar::R`](R) reader structure
impl crate::Readable for TzfsarSpec {}
///`write(|w| ..)` method takes [`tzfsar::W`](W) writer structure
impl crate::Writable for TzfsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZFSAR to value 0xffff_fffe
impl crate::Resettable for TzfsarSpec {
    const RESET_VALUE: u32 = 0xffff_fffe;
}
