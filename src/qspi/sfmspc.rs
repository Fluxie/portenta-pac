///Register `SFMSPC` reader
pub type R = crate::R<SfmspcSpec>;
///Register `SFMSPC` writer
pub type W = crate::W<SfmspcSpec>;
/**SPI protocol select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sfmspi {
    ///0: Single SPI Protocol, Extended SPI protocol
    _00 = 0,
    ///1: Dual SPI protocol
    _01 = 1,
    ///2: Quad SPI protocol
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Sfmspi> for u8 {
    #[inline(always)]
    fn from(variant: Sfmspi) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sfmspi {
    type Ux = u8;
}
impl crate::IsEnum for Sfmspi {}
///Field `SFMSPI` reader - SPI protocol select
pub type SfmspiR = crate::FieldReader<Sfmspi>;
impl SfmspiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmspi {
        match self.bits {
            0 => Sfmspi::_00,
            1 => Sfmspi::_01,
            2 => Sfmspi::_10,
            3 => Sfmspi::_11,
            _ => unreachable!(),
        }
    }
    ///Single SPI Protocol, Extended SPI protocol
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sfmspi::_00
    }
    ///Dual SPI protocol
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sfmspi::_01
    }
    ///Quad SPI protocol
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sfmspi::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sfmspi::_11
    }
}
///Field `SFMSPI` writer - SPI protocol select
pub type SfmspiW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sfmspi, crate::Safe>;
impl<'a, REG> SfmspiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Single SPI Protocol, Extended SPI protocol
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmspi::_00)
    }
    ///Dual SPI protocol
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmspi::_01)
    }
    ///Quad SPI protocol
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmspi::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmspi::_11)
    }
}
/**QSPCLK extended selection bit when switching I/O of QIOn pin

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmsde {
    ///0: No QSPCLK extension
    _0 = 0,
    ///1: QSPCLK expansion when switching I/O direction of QIOn pin
    _1 = 1,
}
impl From<Sfmsde> for bool {
    #[inline(always)]
    fn from(variant: Sfmsde) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMSDE` reader - QSPCLK extended selection bit when switching I/O of QIOn pin
pub type SfmsdeR = crate::BitReader<Sfmsde>;
impl SfmsdeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmsde {
        match self.bits {
            false => Sfmsde::_0,
            true => Sfmsde::_1,
        }
    }
    ///No QSPCLK extension
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmsde::_0
    }
    ///QSPCLK expansion when switching I/O direction of QIOn pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmsde::_1
    }
}
///Field `SFMSDE` writer - QSPCLK extended selection bit when switching I/O of QIOn pin
pub type SfmsdeW<'a, REG> = crate::BitWriter<'a, REG, Sfmsde>;
impl<'a, REG> SfmsdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No QSPCLK extension
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsde::_0)
    }
    ///QSPCLK expansion when switching I/O direction of QIOn pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsde::_1)
    }
}
impl R {
    ///Bits 0:1 - SPI protocol select
    #[inline(always)]
    pub fn sfmspi(&self) -> SfmspiR {
        SfmspiR::new((self.bits & 3) as u8)
    }
    ///Bit 4 - QSPCLK extended selection bit when switching I/O of QIOn pin
    #[inline(always)]
    pub fn sfmsde(&self) -> SfmsdeR {
        SfmsdeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMSPC")
            .field("sfmspi", &self.sfmspi())
            .field("sfmsde", &self.sfmsde())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SPI protocol select
    #[inline(always)]
    pub fn sfmspi(&mut self) -> SfmspiW<SfmspcSpec> {
        SfmspiW::new(self, 0)
    }
    ///Bit 4 - QSPCLK extended selection bit when switching I/O of QIOn pin
    #[inline(always)]
    pub fn sfmsde(&mut self) -> SfmsdeW<SfmspcSpec> {
        SfmsdeW::new(self, 4)
    }
}
/**SPI Protocol Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmspc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmspc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmspcSpec;
impl crate::RegisterSpec for SfmspcSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmspc::R`](R) reader structure
impl crate::Readable for SfmspcSpec {}
///`write(|w| ..)` method takes [`sfmspc::W`](W) writer structure
impl crate::Writable for SfmspcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSPC to value 0x10
impl crate::Resettable for SfmspcSpec {
    const RESET_VALUE: u32 = 0x10;
}
