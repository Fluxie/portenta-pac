///Register `CPUDSAR` reader
pub type R = crate::R<CpudsarSpec>;
///Register `CPUDSAR` writer
pub type W = crate::W<CpudsarSpec>;
/**CPU Debug Security Attribution 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpudsa0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Cpudsa0> for bool {
    #[inline(always)]
    fn from(variant: Cpudsa0) -> Self {
        variant as u8 != 0
    }
}
///Field `CPUDSA0` reader - CPU Debug Security Attribution 0
pub type Cpudsa0R = crate::BitReader<Cpudsa0>;
impl Cpudsa0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cpudsa0 {
        match self.bits {
            false => Cpudsa0::_0,
            true => Cpudsa0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpudsa0::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpudsa0::_1
    }
}
///Field `CPUDSA0` writer - CPU Debug Security Attribution 0
pub type Cpudsa0W<'a, REG> = crate::BitWriter<'a, REG, Cpudsa0>;
impl<'a, REG> Cpudsa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudsa0::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudsa0::_1)
    }
}
impl R {
    ///Bit 0 - CPU Debug Security Attribution 0
    #[inline(always)]
    pub fn cpudsa0(&self) -> Cpudsa0R {
        Cpudsa0R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUDSAR").field("cpudsa0", &self.cpudsa0()).finish()
    }
}
impl W {
    ///Bit 0 - CPU Debug Security Attribution 0
    #[inline(always)]
    pub fn cpudsa0(&mut self) -> Cpudsa0W<CpudsarSpec> {
        Cpudsa0W::new(self, 0)
    }
}
/**CPU Debug Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`cpudsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpudsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CpudsarSpec;
impl crate::RegisterSpec for CpudsarSpec {
    type Ux = u32;
}
///`read()` method returns [`cpudsar::R`](R) reader structure
impl crate::Readable for CpudsarSpec {}
///`write(|w| ..)` method takes [`cpudsar::W`](W) writer structure
impl crate::Writable for CpudsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPUDSAR to value 0xffff_fffe
impl crate::Resettable for CpudsarSpec {
    const RESET_VALUE: u32 = 0xffff_fffe;
}
