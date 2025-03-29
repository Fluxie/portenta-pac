///Register `CFDGTSTCTR` reader
pub type R = crate::R<CfdgtstctrSpec>;
///Register `CFDGTSTCTR` writer
pub type W = crate::W<CfdgtstctrSpec>;
/**Internal CAN Bus Communication Test Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icbctme {
    ///0: Internal CAN Bus Communication test mode disabled
    _0 = 0,
    ///1: Internal CAN Bus Communication test mode enabled
    _1 = 1,
}
impl From<Icbctme> for bool {
    #[inline(always)]
    fn from(variant: Icbctme) -> Self {
        variant as u8 != 0
    }
}
///Field `ICBCTME` reader - Internal CAN Bus Communication Test Mode Enable
pub type IcbctmeR = crate::BitReader<Icbctme>;
impl IcbctmeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Icbctme {
        match self.bits {
            false => Icbctme::_0,
            true => Icbctme::_1,
        }
    }
    ///Internal CAN Bus Communication test mode disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Icbctme::_0
    }
    ///Internal CAN Bus Communication test mode enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Icbctme::_1
    }
}
///Field `ICBCTME` writer - Internal CAN Bus Communication Test Mode Enable
pub type IcbctmeW<'a, REG> = crate::BitWriter<'a, REG, Icbctme>;
impl<'a, REG> IcbctmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal CAN Bus Communication test mode disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Icbctme::_0)
    }
    ///Internal CAN Bus Communication test mode enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Icbctme::_1)
    }
}
/**RAM Test Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtme {
    ///0: RAM test mode disabled
    _0 = 0,
    ///1: RAM test mode enabled
    _1 = 1,
}
impl From<Rtme> for bool {
    #[inline(always)]
    fn from(variant: Rtme) -> Self {
        variant as u8 != 0
    }
}
///Field `RTME` reader - RAM Test Mode Enable
pub type RtmeR = crate::BitReader<Rtme>;
impl RtmeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtme {
        match self.bits {
            false => Rtme::_0,
            true => Rtme::_1,
        }
    }
    ///RAM test mode disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtme::_0
    }
    ///RAM test mode enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtme::_1
    }
}
///Field `RTME` writer - RAM Test Mode Enable
pub type RtmeW<'a, REG> = crate::BitWriter<'a, REG, Rtme>;
impl<'a, REG> RtmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RAM test mode disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtme::_0)
    }
    ///RAM test mode enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtme::_1)
    }
}
impl R {
    ///Bit 0 - Internal CAN Bus Communication Test Mode Enable
    #[inline(always)]
    pub fn icbctme(&self) -> IcbctmeR {
        IcbctmeR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - RAM Test Mode Enable
    #[inline(always)]
    pub fn rtme(&self) -> RtmeR {
        RtmeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGTSTCTR")
            .field("icbctme", &self.icbctme())
            .field("rtme", &self.rtme())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal CAN Bus Communication Test Mode Enable
    #[inline(always)]
    pub fn icbctme(&mut self) -> IcbctmeW<CfdgtstctrSpec> {
        IcbctmeW::new(self, 0)
    }
    ///Bit 2 - RAM Test Mode Enable
    #[inline(always)]
    pub fn rtme(&mut self) -> RtmeW<CfdgtstctrSpec> {
        RtmeW::new(self, 2)
    }
}
/**Global Test Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdgtstctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgtstctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgtstctrSpec;
impl crate::RegisterSpec for CfdgtstctrSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgtstctr::R`](R) reader structure
impl crate::Readable for CfdgtstctrSpec {}
///`write(|w| ..)` method takes [`cfdgtstctr::W`](W) writer structure
impl crate::Writable for CfdgtstctrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGTSTCTR to value 0
impl crate::Resettable for CfdgtstctrSpec {}
