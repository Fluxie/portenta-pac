///Register `PWPRS` reader
pub type R = crate::R<PwprsSpec>;
///Register `PWPRS` writer
pub type W = crate::W<PwprsSpec>;
/**PmnPFS Register Write Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfswe {
    ///0: Disable writes to the PmnPFS register
    _0 = 0,
    ///1: Enable writes to the PmnPFS register
    _1 = 1,
}
impl From<Pfswe> for bool {
    #[inline(always)]
    fn from(variant: Pfswe) -> Self {
        variant as u8 != 0
    }
}
///Field `PFSWE` reader - PmnPFS Register Write Enable
pub type PfsweR = crate::BitReader<Pfswe>;
impl PfsweR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pfswe {
        match self.bits {
            false => Pfswe::_0,
            true => Pfswe::_1,
        }
    }
    ///Disable writes to the PmnPFS register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pfswe::_0
    }
    ///Enable writes to the PmnPFS register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pfswe::_1
    }
}
///Field `PFSWE` writer - PmnPFS Register Write Enable
pub type PfsweW<'a, REG> = crate::BitWriter<'a, REG, Pfswe>;
impl<'a, REG> PfsweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable writes to the PmnPFS register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pfswe::_0)
    }
    ///Enable writes to the PmnPFS register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pfswe::_1)
    }
}
/**PFSWE Bit Write Disable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0wi {
    ///0: Enable writes the PFSWE bit
    _0 = 0,
    ///1: Disable writes to the PFSWE bit
    _1 = 1,
}
impl From<B0wi> for bool {
    #[inline(always)]
    fn from(variant: B0wi) -> Self {
        variant as u8 != 0
    }
}
///Field `B0WI` reader - PFSWE Bit Write Disable
pub type B0wiR = crate::BitReader<B0wi>;
impl B0wiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> B0wi {
        match self.bits {
            false => B0wi::_0,
            true => B0wi::_1,
        }
    }
    ///Enable writes the PFSWE bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0wi::_0
    }
    ///Disable writes to the PFSWE bit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0wi::_1
    }
}
///Field `B0WI` writer - PFSWE Bit Write Disable
pub type B0wiW<'a, REG> = crate::BitWriter<'a, REG, B0wi>;
impl<'a, REG> B0wiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable writes the PFSWE bit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B0wi::_0)
    }
    ///Disable writes to the PFSWE bit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B0wi::_1)
    }
}
impl R {
    ///Bit 6 - PmnPFS Register Write Enable
    #[inline(always)]
    pub fn pfswe(&self) -> PfsweR {
        PfsweR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PFSWE Bit Write Disable
    #[inline(always)]
    pub fn b0wi(&self) -> B0wiR {
        B0wiR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWPRS")
            .field("pfswe", &self.pfswe())
            .field("b0wi", &self.b0wi())
            .finish()
    }
}
impl W {
    ///Bit 6 - PmnPFS Register Write Enable
    #[inline(always)]
    pub fn pfswe(&mut self) -> PfsweW<PwprsSpec> {
        PfsweW::new(self, 6)
    }
    ///Bit 7 - PFSWE Bit Write Disable
    #[inline(always)]
    pub fn b0wi(&mut self) -> B0wiW<PwprsSpec> {
        B0wiW::new(self, 7)
    }
}
/**Write-Protect Register for Secure

You can [`read`](crate::Reg::read) this register and get [`pwprs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwprs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PwprsSpec;
impl crate::RegisterSpec for PwprsSpec {
    type Ux = u8;
}
///`read()` method returns [`pwprs::R`](R) reader structure
impl crate::Readable for PwprsSpec {}
///`write(|w| ..)` method takes [`pwprs::W`](W) writer structure
impl crate::Writable for PwprsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWPRS to value 0x80
impl crate::Resettable for PwprsSpec {
    const RESET_VALUE: u8 = 0x80;
}
