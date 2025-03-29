///Register `MOMCR` reader
pub type R = crate::R<MomcrSpec>;
///Register `MOMCR` writer
pub type W = crate::W<MomcrSpec>;
/**Main Clock Oscillator Drive Capability 0 Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modrv {
    ///0: 20 MHz to 24 MHz
    _00 = 0,
    ///1: 16 MHz to 20 MHz
    _01 = 1,
    ///2: 8 MHz to 16 MHz
    _10 = 2,
    ///3: 8 MHz
    _11 = 3,
}
impl From<Modrv> for u8 {
    #[inline(always)]
    fn from(variant: Modrv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modrv {
    type Ux = u8;
}
impl crate::IsEnum for Modrv {}
///Field `MODRV` reader - Main Clock Oscillator Drive Capability 0 Switching
pub type ModrvR = crate::FieldReader<Modrv>;
impl ModrvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Modrv {
        match self.bits {
            0 => Modrv::_00,
            1 => Modrv::_01,
            2 => Modrv::_10,
            3 => Modrv::_11,
            _ => unreachable!(),
        }
    }
    ///20 MHz to 24 MHz
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Modrv::_00
    }
    ///16 MHz to 20 MHz
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Modrv::_01
    }
    ///8 MHz to 16 MHz
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Modrv::_10
    }
    ///8 MHz
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Modrv::_11
    }
}
///Field `MODRV` writer - Main Clock Oscillator Drive Capability 0 Switching
pub type ModrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Modrv, crate::Safe>;
impl<'a, REG> ModrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///20 MHz to 24 MHz
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Modrv::_00)
    }
    ///16 MHz to 20 MHz
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Modrv::_01)
    }
    ///8 MHz to 16 MHz
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Modrv::_10)
    }
    ///8 MHz
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Modrv::_11)
    }
}
/**Main Clock Oscillator Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mosel {
    ///0: Resonator
    _0 = 0,
    ///1: External clock input
    _1 = 1,
}
impl From<Mosel> for bool {
    #[inline(always)]
    fn from(variant: Mosel) -> Self {
        variant as u8 != 0
    }
}
///Field `MOSEL` reader - Main Clock Oscillator Switching
pub type MoselR = crate::BitReader<Mosel>;
impl MoselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mosel {
        match self.bits {
            false => Mosel::_0,
            true => Mosel::_1,
        }
    }
    ///Resonator
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mosel::_0
    }
    ///External clock input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mosel::_1
    }
}
///Field `MOSEL` writer - Main Clock Oscillator Switching
pub type MoselW<'a, REG> = crate::BitWriter<'a, REG, Mosel>;
impl<'a, REG> MoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resonator
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mosel::_0)
    }
    ///External clock input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mosel::_1)
    }
}
impl R {
    ///Bits 4:5 - Main Clock Oscillator Drive Capability 0 Switching
    #[inline(always)]
    pub fn modrv(&self) -> ModrvR {
        ModrvR::new((self.bits >> 4) & 3)
    }
    ///Bit 6 - Main Clock Oscillator Switching
    #[inline(always)]
    pub fn mosel(&self) -> MoselR {
        MoselR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOMCR")
            .field("modrv", &self.modrv())
            .field("mosel", &self.mosel())
            .finish()
    }
}
impl W {
    ///Bits 4:5 - Main Clock Oscillator Drive Capability 0 Switching
    #[inline(always)]
    pub fn modrv(&mut self) -> ModrvW<MomcrSpec> {
        ModrvW::new(self, 4)
    }
    ///Bit 6 - Main Clock Oscillator Switching
    #[inline(always)]
    pub fn mosel(&mut self) -> MoselW<MomcrSpec> {
        MoselW::new(self, 6)
    }
}
/**Main Clock Oscillator Mode Oscillation Control Register

You can [`read`](crate::Reg::read) this register and get [`momcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`momcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MomcrSpec;
impl crate::RegisterSpec for MomcrSpec {
    type Ux = u8;
}
///`read()` method returns [`momcr::R`](R) reader structure
impl crate::Readable for MomcrSpec {}
///`write(|w| ..)` method takes [`momcr::W`](W) writer structure
impl crate::Writable for MomcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOMCR to value 0
impl crate::Resettable for MomcrSpec {}
