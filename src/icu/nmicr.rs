///Register `NMICR` reader
pub type R = crate::R<NmicrSpec>;
///Register `NMICR` writer
pub type W = crate::W<NmicrSpec>;
/**NMI Detection Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmimd {
    ///0: Falling edge
    _0 = 0,
    ///1: Rising edge
    _1 = 1,
}
impl From<Nmimd> for bool {
    #[inline(always)]
    fn from(variant: Nmimd) -> Self {
        variant as u8 != 0
    }
}
///Field `NMIMD` reader - NMI Detection Set
pub type NmimdR = crate::BitReader<Nmimd>;
impl NmimdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nmimd {
        match self.bits {
            false => Nmimd::_0,
            true => Nmimd::_1,
        }
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nmimd::_0
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nmimd::_1
    }
}
///Field `NMIMD` writer - NMI Detection Set
pub type NmimdW<'a, REG> = crate::BitWriter<'a, REG, Nmimd>;
impl<'a, REG> NmimdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nmimd::_0)
    }
    ///Rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nmimd::_1)
    }
}
/**NMI Digital Filter Sampling Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfclksel {
    ///0: PCLKB
    _00 = 0,
    ///1: PCLKB/8
    _01 = 1,
    ///2: PCLKB/32
    _10 = 2,
    ///3: PCLKB/64
    _11 = 3,
}
impl From<Nfclksel> for u8 {
    #[inline(always)]
    fn from(variant: Nfclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nfclksel {
    type Ux = u8;
}
impl crate::IsEnum for Nfclksel {}
///Field `NFCLKSEL` reader - NMI Digital Filter Sampling Clock Select
pub type NfclkselR = crate::FieldReader<Nfclksel>;
impl NfclkselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nfclksel {
        match self.bits {
            0 => Nfclksel::_00,
            1 => Nfclksel::_01,
            2 => Nfclksel::_10,
            3 => Nfclksel::_11,
            _ => unreachable!(),
        }
    }
    ///PCLKB
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Nfclksel::_00
    }
    ///PCLKB/8
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Nfclksel::_01
    }
    ///PCLKB/32
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Nfclksel::_10
    }
    ///PCLKB/64
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Nfclksel::_11
    }
}
///Field `NFCLKSEL` writer - NMI Digital Filter Sampling Clock Select
pub type NfclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nfclksel, crate::Safe>;
impl<'a, REG> NfclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKB
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Nfclksel::_00)
    }
    ///PCLKB/8
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Nfclksel::_01)
    }
    ///PCLKB/32
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Nfclksel::_10)
    }
    ///PCLKB/64
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Nfclksel::_11)
    }
}
/**NMI Digital Filter Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nflten {
    ///0: Disabled.
    _0 = 0,
    ///1: Enabled.
    _1 = 1,
}
impl From<Nflten> for bool {
    #[inline(always)]
    fn from(variant: Nflten) -> Self {
        variant as u8 != 0
    }
}
///Field `NFLTEN` reader - NMI Digital Filter Enable
pub type NfltenR = crate::BitReader<Nflten>;
impl NfltenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nflten {
        match self.bits {
            false => Nflten::_0,
            true => Nflten::_1,
        }
    }
    ///Disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nflten::_0
    }
    ///Enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nflten::_1
    }
}
///Field `NFLTEN` writer - NMI Digital Filter Enable
pub type NfltenW<'a, REG> = crate::BitWriter<'a, REG, Nflten>;
impl<'a, REG> NfltenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nflten::_0)
    }
    ///Enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nflten::_1)
    }
}
impl R {
    ///Bit 0 - NMI Detection Set
    #[inline(always)]
    pub fn nmimd(&self) -> NmimdR {
        NmimdR::new((self.bits & 1) != 0)
    }
    ///Bits 4:5 - NMI Digital Filter Sampling Clock Select
    #[inline(always)]
    pub fn nfclksel(&self) -> NfclkselR {
        NfclkselR::new((self.bits >> 4) & 3)
    }
    ///Bit 7 - NMI Digital Filter Enable
    #[inline(always)]
    pub fn nflten(&self) -> NfltenR {
        NfltenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMICR")
            .field("nmimd", &self.nmimd())
            .field("nfclksel", &self.nfclksel())
            .field("nflten", &self.nflten())
            .finish()
    }
}
impl W {
    ///Bit 0 - NMI Detection Set
    #[inline(always)]
    pub fn nmimd(&mut self) -> NmimdW<NmicrSpec> {
        NmimdW::new(self, 0)
    }
    ///Bits 4:5 - NMI Digital Filter Sampling Clock Select
    #[inline(always)]
    pub fn nfclksel(&mut self) -> NfclkselW<NmicrSpec> {
        NfclkselW::new(self, 4)
    }
    ///Bit 7 - NMI Digital Filter Enable
    #[inline(always)]
    pub fn nflten(&mut self) -> NfltenW<NmicrSpec> {
        NfltenW::new(self, 7)
    }
}
/**NMI Pin Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`nmicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NmicrSpec;
impl crate::RegisterSpec for NmicrSpec {
    type Ux = u8;
}
///`read()` method returns [`nmicr::R`](R) reader structure
impl crate::Readable for NmicrSpec {}
///`write(|w| ..)` method takes [`nmicr::W`](W) writer structure
impl crate::Writable for NmicrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NMICR to value 0
impl crate::Resettable for NmicrSpec {}
