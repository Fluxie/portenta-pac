///Register `DTCSAR` reader
pub type R = crate::R<DtcsarSpec>;
///Register `DTCSAR` writer
pub type W = crate::W<DtcsarSpec>;
/**DTC Security Attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtcstsa {
    ///0: Secure.
    _0 = 0,
    ///1: Non-Secure.
    _1 = 1,
}
impl From<Dtcstsa> for bool {
    #[inline(always)]
    fn from(variant: Dtcstsa) -> Self {
        variant as u8 != 0
    }
}
///Field `DTCSTSA` reader - DTC Security Attribution
pub type DtcstsaR = crate::BitReader<Dtcstsa>;
impl DtcstsaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dtcstsa {
        match self.bits {
            false => Dtcstsa::_0,
            true => Dtcstsa::_1,
        }
    }
    ///Secure.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtcstsa::_0
    }
    ///Non-Secure.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtcstsa::_1
    }
}
///Field `DTCSTSA` writer - DTC Security Attribution
pub type DtcstsaW<'a, REG> = crate::BitWriter<'a, REG, Dtcstsa>;
impl<'a, REG> DtcstsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcstsa::_0)
    }
    ///Non-Secure.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcstsa::_1)
    }
}
impl R {
    ///Bit 0 - DTC Security Attribution
    #[inline(always)]
    pub fn dtcstsa(&self) -> DtcstsaR {
        DtcstsaR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCSAR").field("dtcstsa", &self.dtcstsa()).finish()
    }
}
impl W {
    ///Bit 0 - DTC Security Attribution
    #[inline(always)]
    pub fn dtcstsa(&mut self) -> DtcstsaW<DtcsarSpec> {
        DtcstsaW::new(self, 0)
    }
}
/**DTC Controller Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`dtcsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DtcsarSpec;
impl crate::RegisterSpec for DtcsarSpec {
    type Ux = u32;
}
///`read()` method returns [`dtcsar::R`](R) reader structure
impl crate::Readable for DtcsarSpec {}
///`write(|w| ..)` method takes [`dtcsar::W`](W) writer structure
impl crate::Writable for DtcsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTCSAR to value 0xffff_ffff
impl crate::Resettable for DtcsarSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
