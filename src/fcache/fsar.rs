///Register `FSAR` reader
pub type R = crate::R<FsarSpec>;
///Register `FSAR` writer
pub type W = crate::W<FsarSpec>;
/**FLWT Security Attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flwtsa {
    ///0: Secure
    _0 = 0,
    ///1: Non-Secure
    _1 = 1,
}
impl From<Flwtsa> for bool {
    #[inline(always)]
    fn from(variant: Flwtsa) -> Self {
        variant as u8 != 0
    }
}
///Field `FLWTSA` reader - FLWT Security Attribution
pub type FlwtsaR = crate::BitReader<Flwtsa>;
impl FlwtsaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Flwtsa {
        match self.bits {
            false => Flwtsa::_0,
            true => Flwtsa::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Flwtsa::_0
    }
    ///Non-Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Flwtsa::_1
    }
}
///Field `FLWTSA` writer - FLWT Security Attribution
pub type FlwtsaW<'a, REG> = crate::BitWriter<'a, REG, Flwtsa>;
impl<'a, REG> FlwtsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Flwtsa::_0)
    }
    ///Non-Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Flwtsa::_1)
    }
}
/**FCKMHZ Security Attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fckmhzsa {
    ///0: Secure
    _0 = 0,
    ///1: Non-Secure
    _1 = 1,
}
impl From<Fckmhzsa> for bool {
    #[inline(always)]
    fn from(variant: Fckmhzsa) -> Self {
        variant as u8 != 0
    }
}
///Field `FCKMHZSA` reader - FCKMHZ Security Attribution
pub type FckmhzsaR = crate::BitReader<Fckmhzsa>;
impl FckmhzsaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fckmhzsa {
        match self.bits {
            false => Fckmhzsa::_0,
            true => Fckmhzsa::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fckmhzsa::_0
    }
    ///Non-Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fckmhzsa::_1
    }
}
///Field `FCKMHZSA` writer - FCKMHZ Security Attribution
pub type FckmhzsaW<'a, REG> = crate::BitWriter<'a, REG, Fckmhzsa>;
impl<'a, REG> FckmhzsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fckmhzsa::_0)
    }
    ///Non-Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fckmhzsa::_1)
    }
}
impl R {
    ///Bit 0 - FLWT Security Attribution
    #[inline(always)]
    pub fn flwtsa(&self) -> FlwtsaR {
        FlwtsaR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - FCKMHZ Security Attribution
    #[inline(always)]
    pub fn fckmhzsa(&self) -> FckmhzsaR {
        FckmhzsaR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSAR")
            .field("flwtsa", &self.flwtsa())
            .field("fckmhzsa", &self.fckmhzsa())
            .finish()
    }
}
impl W {
    ///Bit 0 - FLWT Security Attribution
    #[inline(always)]
    pub fn flwtsa(&mut self) -> FlwtsaW<FsarSpec> {
        FlwtsaW::new(self, 0)
    }
    ///Bit 8 - FCKMHZ Security Attribution
    #[inline(always)]
    pub fn fckmhzsa(&mut self) -> FckmhzsaW<FsarSpec> {
        FckmhzsaW::new(self, 8)
    }
}
/**Flash Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`fsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FsarSpec;
impl crate::RegisterSpec for FsarSpec {
    type Ux = u16;
}
///`read()` method returns [`fsar::R`](R) reader structure
impl crate::Readable for FsarSpec {}
///`write(|w| ..)` method takes [`fsar::W`](W) writer structure
impl crate::Writable for FsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FSAR to value 0xffff
impl crate::Resettable for FsarSpec {
    const RESET_VALUE: u16 = 0xffff;
}
