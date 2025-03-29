///Register `CR0` reader
pub type R = crate::R<Cr0Spec>;
///Register `CR0` writer
pub type W = crate::W<Cr0Spec>;
/**Start Frame Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfsf {
    ///0: Start Frame detection function is disabled.
    _0 = 0,
    ///1: Start Frame detection function is enabled.
    _1 = 1,
}
impl From<Sfsf> for bool {
    #[inline(always)]
    fn from(variant: Sfsf) -> Self {
        variant as u8 != 0
    }
}
///Field `SFSF` reader - Start Frame Status Flag
pub type SfsfR = crate::BitReader<Sfsf>;
impl SfsfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfsf {
        match self.bits {
            false => Sfsf::_0,
            true => Sfsf::_1,
        }
    }
    ///Start Frame detection function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfsf::_0
    }
    ///Start Frame detection function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfsf::_1
    }
}
/**RXDXn Input Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdsf {
    ///0: RXDXn input is enabled.
    _0 = 0,
    ///1: RXDXn input is disabled.
    _1 = 1,
}
impl From<Rxdsf> for bool {
    #[inline(always)]
    fn from(variant: Rxdsf) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDSF` reader - RXDXn Input Status Flag
pub type RxdsfR = crate::BitReader<Rxdsf>;
impl RxdsfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rxdsf {
        match self.bits {
            false => Rxdsf::_0,
            true => Rxdsf::_1,
        }
    }
    ///RXDXn input is enabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxdsf::_0
    }
    ///RXDXn input is disabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxdsf::_1
    }
}
/**Bit Rate Measurement Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brme {
    ///0: Measurement of bit rate is disabled.
    _0 = 0,
    ///1: Measurement of bit rate is enabled.
    _1 = 1,
}
impl From<Brme> for bool {
    #[inline(always)]
    fn from(variant: Brme) -> Self {
        variant as u8 != 0
    }
}
///Field `BRME` reader - Bit Rate Measurement Enable
pub type BrmeR = crate::BitReader<Brme>;
impl BrmeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Brme {
        match self.bits {
            false => Brme::_0,
            true => Brme::_1,
        }
    }
    ///Measurement of bit rate is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Brme::_0
    }
    ///Measurement of bit rate is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Brme::_1
    }
}
///Field `BRME` writer - Bit Rate Measurement Enable
pub type BrmeW<'a, REG> = crate::BitWriter<'a, REG, Brme>;
impl<'a, REG> BrmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Measurement of bit rate is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Brme::_0)
    }
    ///Measurement of bit rate is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Brme::_1)
    }
}
impl R {
    ///Bit 1 - Start Frame Status Flag
    #[inline(always)]
    pub fn sfsf(&self) -> SfsfR {
        SfsfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXDXn Input Status Flag
    #[inline(always)]
    pub fn rxdsf(&self) -> RxdsfR {
        RxdsfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bit Rate Measurement Enable
    #[inline(always)]
    pub fn brme(&self) -> BrmeR {
        BrmeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR0")
            .field("sfsf", &self.sfsf())
            .field("rxdsf", &self.rxdsf())
            .field("brme", &self.brme())
            .finish()
    }
}
impl W {
    ///Bit 3 - Bit Rate Measurement Enable
    #[inline(always)]
    pub fn brme(&mut self) -> BrmeW<Cr0Spec> {
        BrmeW::new(self, 3)
    }
}
/**Control Register 0

You can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u8;
}
///`read()` method returns [`cr0::R`](R) reader structure
impl crate::Readable for Cr0Spec {}
///`write(|w| ..)` method takes [`cr0::W`](W) writer structure
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR0 to value 0
impl crate::Resettable for Cr0Spec {}
