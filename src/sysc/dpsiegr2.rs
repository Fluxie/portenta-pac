///Register `DPSIEGR2` reader
pub type R = crate::R<Dpsiegr2Spec>;
///Register `DPSIEGR2` writer
pub type W = crate::W<Dpsiegr2Spec>;
/**LVD1 Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlvd1eg {
    ///0: A cancel request is generated when VCC < Vdet1 (fall) is detected
    _0 = 0,
    ///1: A cancel request is generated when VCC ≥ Vdet1 (rise) is detected
    _1 = 1,
}
impl From<Dlvd1eg> for bool {
    #[inline(always)]
    fn from(variant: Dlvd1eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DLVD1EG` reader - LVD1 Edge Select
pub type Dlvd1egR = crate::BitReader<Dlvd1eg>;
impl Dlvd1egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dlvd1eg {
        match self.bits {
            false => Dlvd1eg::_0,
            true => Dlvd1eg::_1,
        }
    }
    ///A cancel request is generated when VCC < Vdet1 (fall) is detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dlvd1eg::_0
    }
    ///A cancel request is generated when VCC ≥ Vdet1 (rise) is detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dlvd1eg::_1
    }
}
///Field `DLVD1EG` writer - LVD1 Edge Select
pub type Dlvd1egW<'a, REG> = crate::BitWriter<'a, REG, Dlvd1eg>;
impl<'a, REG> Dlvd1egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated when VCC < Vdet1 (fall) is detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd1eg::_0)
    }
    ///A cancel request is generated when VCC ≥ Vdet1 (rise) is detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd1eg::_1)
    }
}
/**LVD2 Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlvd2eg {
    ///0: A cancel request is generated when VCC < Vdet2 (fall) is detected
    _0 = 0,
    ///1: A cancel request is generated when VCC ≥ Vdet2 (rise) is detected
    _1 = 1,
}
impl From<Dlvd2eg> for bool {
    #[inline(always)]
    fn from(variant: Dlvd2eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DLVD2EG` reader - LVD2 Edge Select
pub type Dlvd2egR = crate::BitReader<Dlvd2eg>;
impl Dlvd2egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dlvd2eg {
        match self.bits {
            false => Dlvd2eg::_0,
            true => Dlvd2eg::_1,
        }
    }
    ///A cancel request is generated when VCC < Vdet2 (fall) is detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dlvd2eg::_0
    }
    ///A cancel request is generated when VCC ≥ Vdet2 (rise) is detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dlvd2eg::_1
    }
}
///Field `DLVD2EG` writer - LVD2 Edge Select
pub type Dlvd2egW<'a, REG> = crate::BitWriter<'a, REG, Dlvd2eg>;
impl<'a, REG> Dlvd2egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated when VCC < Vdet2 (fall) is detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd2eg::_0)
    }
    ///A cancel request is generated when VCC ≥ Vdet2 (rise) is detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd2eg::_1)
    }
}
/**NMI Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dnmieg {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<Dnmieg> for bool {
    #[inline(always)]
    fn from(variant: Dnmieg) -> Self {
        variant as u8 != 0
    }
}
///Field `DNMIEG` reader - NMI Pin Edge Select
pub type DnmiegR = crate::BitReader<Dnmieg>;
impl DnmiegR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dnmieg {
        match self.bits {
            false => Dnmieg::_0,
            true => Dnmieg::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dnmieg::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dnmieg::_1
    }
}
///Field `DNMIEG` writer - NMI Pin Edge Select
pub type DnmiegW<'a, REG> = crate::BitWriter<'a, REG, Dnmieg>;
impl<'a, REG> DnmiegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dnmieg::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dnmieg::_1)
    }
}
impl R {
    ///Bit 0 - LVD1 Edge Select
    #[inline(always)]
    pub fn dlvd1eg(&self) -> Dlvd1egR {
        Dlvd1egR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LVD2 Edge Select
    #[inline(always)]
    pub fn dlvd2eg(&self) -> Dlvd2egR {
        Dlvd2egR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - NMI Pin Edge Select
    #[inline(always)]
    pub fn dnmieg(&self) -> DnmiegR {
        DnmiegR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSIEGR2")
            .field("dlvd1eg", &self.dlvd1eg())
            .field("dlvd2eg", &self.dlvd2eg())
            .field("dnmieg", &self.dnmieg())
            .finish()
    }
}
impl W {
    ///Bit 0 - LVD1 Edge Select
    #[inline(always)]
    pub fn dlvd1eg(&mut self) -> Dlvd1egW<Dpsiegr2Spec> {
        Dlvd1egW::new(self, 0)
    }
    ///Bit 1 - LVD2 Edge Select
    #[inline(always)]
    pub fn dlvd2eg(&mut self) -> Dlvd2egW<Dpsiegr2Spec> {
        Dlvd2egW::new(self, 1)
    }
    ///Bit 4 - NMI Pin Edge Select
    #[inline(always)]
    pub fn dnmieg(&mut self) -> DnmiegW<Dpsiegr2Spec> {
        DnmiegW::new(self, 4)
    }
}
/**Deep Software Standby Interrupt Edge Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsiegr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpsiegr2Spec;
impl crate::RegisterSpec for Dpsiegr2Spec {
    type Ux = u8;
}
///`read()` method returns [`dpsiegr2::R`](R) reader structure
impl crate::Readable for Dpsiegr2Spec {}
///`write(|w| ..)` method takes [`dpsiegr2::W`](W) writer structure
impl crate::Writable for Dpsiegr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIEGR2 to value 0
impl crate::Resettable for Dpsiegr2Spec {}
