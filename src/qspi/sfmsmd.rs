///Register `SFMSMD` reader
pub type R = crate::R<SfmsmdSpec>;
///Register `SFMSMD` writer
pub type W = crate::W<SfmsmdSpec>;
/**Serial interface read mode select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sfmrm {
    ///0: Standard Read
    _000 = 0,
    ///1: Fast Read
    _001 = 1,
    ///2: Fast Read Dual Output
    _010 = 2,
    ///3: Fast Read Dual I/O
    _011 = 3,
    ///4: Fast Read Quad Output
    _100 = 4,
    ///5: Fast Read Quad I/O
    _101 = 5,
    ///6: Setting prohibited
    Others = 6,
}
impl From<Sfmrm> for u8 {
    #[inline(always)]
    fn from(variant: Sfmrm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sfmrm {
    type Ux = u8;
}
impl crate::IsEnum for Sfmrm {}
///Field `SFMRM` reader - Serial interface read mode select
pub type SfmrmR = crate::FieldReader<Sfmrm>;
impl SfmrmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmrm {
        match self.bits {
            0 => Sfmrm::_000,
            1 => Sfmrm::_001,
            2 => Sfmrm::_010,
            3 => Sfmrm::_011,
            4 => Sfmrm::_100,
            5 => Sfmrm::_101,
            _ => Sfmrm::Others,
        }
    }
    ///Standard Read
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Sfmrm::_000
    }
    ///Fast Read
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Sfmrm::_001
    }
    ///Fast Read Dual Output
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Sfmrm::_010
    }
    ///Fast Read Dual I/O
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Sfmrm::_011
    }
    ///Fast Read Quad Output
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Sfmrm::_100
    }
    ///Fast Read Quad I/O
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Sfmrm::_101
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Sfmrm::Others)
    }
}
///Field `SFMRM` writer - Serial interface read mode select
pub type SfmrmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sfmrm, crate::Safe>;
impl<'a, REG> SfmrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Standard Read
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmrm::_000)
    }
    ///Fast Read
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmrm::_001)
    }
    ///Fast Read Dual Output
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmrm::_010)
    }
    ///Fast Read Dual I/O
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmrm::_011)
    }
    ///Fast Read Quad Output
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmrm::_100)
    }
    ///Fast Read Quad I/O
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmrm::_101)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmrm::Others)
    }
}
/**QSSL extension function select after SPI bus access

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sfmse {
    ///0: Do not extend QSSL
    _00 = 0,
    ///1: Extend QSSL by 33 QSPCLK
    _01 = 1,
    ///2: Extend QSSL by 129 QSPCLK
    _10 = 2,
    ///3: Extend QSSL infinitely
    _11 = 3,
}
impl From<Sfmse> for u8 {
    #[inline(always)]
    fn from(variant: Sfmse) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sfmse {
    type Ux = u8;
}
impl crate::IsEnum for Sfmse {}
///Field `SFMSE` reader - QSSL extension function select after SPI bus access
pub type SfmseR = crate::FieldReader<Sfmse>;
impl SfmseR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmse {
        match self.bits {
            0 => Sfmse::_00,
            1 => Sfmse::_01,
            2 => Sfmse::_10,
            3 => Sfmse::_11,
            _ => unreachable!(),
        }
    }
    ///Do not extend QSSL
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sfmse::_00
    }
    ///Extend QSSL by 33 QSPCLK
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sfmse::_01
    }
    ///Extend QSSL by 129 QSPCLK
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sfmse::_10
    }
    ///Extend QSSL infinitely
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sfmse::_11
    }
}
///Field `SFMSE` writer - QSSL extension function select after SPI bus access
pub type SfmseW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sfmse, crate::Safe>;
impl<'a, REG> SfmseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Do not extend QSSL
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmse::_00)
    }
    ///Extend QSSL by 33 QSPCLK
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmse::_01)
    }
    ///Extend QSSL by 129 QSPCLK
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmse::_10)
    }
    ///Extend QSSL infinitely
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmse::_11)
    }
}
/**Prefetch function select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmpfe {
    ///0: Disable function
    _0 = 0,
    ///1: Enable function
    _1 = 1,
}
impl From<Sfmpfe> for bool {
    #[inline(always)]
    fn from(variant: Sfmpfe) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMPFE` reader - Prefetch function select
pub type SfmpfeR = crate::BitReader<Sfmpfe>;
impl SfmpfeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmpfe {
        match self.bits {
            false => Sfmpfe::_0,
            true => Sfmpfe::_1,
        }
    }
    ///Disable function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmpfe::_0
    }
    ///Enable function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmpfe::_1
    }
}
///Field `SFMPFE` writer - Prefetch function select
pub type SfmpfeW<'a, REG> = crate::BitWriter<'a, REG, Sfmpfe>;
impl<'a, REG> SfmpfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmpfe::_0)
    }
    ///Enable function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmpfe::_1)
    }
}
/**Function select for stopping prefetch at locations other than on byte boundaries

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmpae {
    ///0: Disable function
    _0 = 0,
    ///1: Enable function
    _1 = 1,
}
impl From<Sfmpae> for bool {
    #[inline(always)]
    fn from(variant: Sfmpae) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMPAE` reader - Function select for stopping prefetch at locations other than on byte boundaries
pub type SfmpaeR = crate::BitReader<Sfmpae>;
impl SfmpaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmpae {
        match self.bits {
            false => Sfmpae::_0,
            true => Sfmpae::_1,
        }
    }
    ///Disable function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmpae::_0
    }
    ///Enable function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmpae::_1
    }
}
///Field `SFMPAE` writer - Function select for stopping prefetch at locations other than on byte boundaries
pub type SfmpaeW<'a, REG> = crate::BitWriter<'a, REG, Sfmpae>;
impl<'a, REG> SfmpaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmpae::_0)
    }
    ///Enable function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmpae::_1)
    }
}
/**SPI mode select.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmmd3 {
    ///0: SPI mode 0
    _0 = 0,
    ///1: SPI mode 3
    _1 = 1,
}
impl From<Sfmmd3> for bool {
    #[inline(always)]
    fn from(variant: Sfmmd3) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMMD3` reader - SPI mode select.
pub type Sfmmd3R = crate::BitReader<Sfmmd3>;
impl Sfmmd3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmmd3 {
        match self.bits {
            false => Sfmmd3::_0,
            true => Sfmmd3::_1,
        }
    }
    ///SPI mode 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmmd3::_0
    }
    ///SPI mode 3
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmmd3::_1
    }
}
///Field `SFMMD3` writer - SPI mode select.
pub type Sfmmd3W<'a, REG> = crate::BitWriter<'a, REG, Sfmmd3>;
impl<'a, REG> Sfmmd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI mode 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmmd3::_0)
    }
    ///SPI mode 3
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmmd3::_1)
    }
}
/**Extension select for the I/O buffer output enable signal for the serial interface

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmoex {
    ///0: Do not extend
    _0 = 0,
    ///1: Extend by 1 QSPCLK
    _1 = 1,
}
impl From<Sfmoex> for bool {
    #[inline(always)]
    fn from(variant: Sfmoex) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMOEX` reader - Extension select for the I/O buffer output enable signal for the serial interface
pub type SfmoexR = crate::BitReader<Sfmoex>;
impl SfmoexR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmoex {
        match self.bits {
            false => Sfmoex::_0,
            true => Sfmoex::_1,
        }
    }
    ///Do not extend
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmoex::_0
    }
    ///Extend by 1 QSPCLK
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmoex::_1
    }
}
///Field `SFMOEX` writer - Extension select for the I/O buffer output enable signal for the serial interface
pub type SfmoexW<'a, REG> = crate::BitWriter<'a, REG, Sfmoex>;
impl<'a, REG> SfmoexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not extend
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmoex::_0)
    }
    ///Extend by 1 QSPCLK
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmoex::_1)
    }
}
/**Hold time adjustment for serial transmission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmohw {
    ///0: Do not extend high-level width of QSPCLK during transmission
    _0 = 0,
    ///1: Extend high-level width of QSPCLK by 1 PCLKA during transmission
    _1 = 1,
}
impl From<Sfmohw> for bool {
    #[inline(always)]
    fn from(variant: Sfmohw) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMOHW` reader - Hold time adjustment for serial transmission
pub type SfmohwR = crate::BitReader<Sfmohw>;
impl SfmohwR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmohw {
        match self.bits {
            false => Sfmohw::_0,
            true => Sfmohw::_1,
        }
    }
    ///Do not extend high-level width of QSPCLK during transmission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmohw::_0
    }
    ///Extend high-level width of QSPCLK by 1 PCLKA during transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmohw::_1
    }
}
///Field `SFMOHW` writer - Hold time adjustment for serial transmission
pub type SfmohwW<'a, REG> = crate::BitWriter<'a, REG, Sfmohw>;
impl<'a, REG> SfmohwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not extend high-level width of QSPCLK during transmission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmohw::_0)
    }
    ///Extend high-level width of QSPCLK by 1 PCLKA during transmission
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmohw::_1)
    }
}
/**Setup time adjustment for serial transmission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmosw {
    ///0: Do not extend low-level width of QSPCLK during transmission
    _0 = 0,
    ///1: Extend low-level width of QSPCLK by 1 PCLKA during transmission
    _1 = 1,
}
impl From<Sfmosw> for bool {
    #[inline(always)]
    fn from(variant: Sfmosw) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMOSW` reader - Setup time adjustment for serial transmission
pub type SfmoswR = crate::BitReader<Sfmosw>;
impl SfmoswR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmosw {
        match self.bits {
            false => Sfmosw::_0,
            true => Sfmosw::_1,
        }
    }
    ///Do not extend low-level width of QSPCLK during transmission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmosw::_0
    }
    ///Extend low-level width of QSPCLK by 1 PCLKA during transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmosw::_1
    }
}
///Field `SFMOSW` writer - Setup time adjustment for serial transmission
pub type SfmoswW<'a, REG> = crate::BitWriter<'a, REG, Sfmosw>;
impl<'a, REG> SfmoswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not extend low-level width of QSPCLK during transmission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmosw::_0)
    }
    ///Extend low-level width of QSPCLK by 1 PCLKA during transmission
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmosw::_1)
    }
}
/**Read instruction code select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmcce {
    ///0: Uses automatically generated SPI instruction code
    _0 = 0,
    ///1: Use instruction code in the SFMSIC register
    _1 = 1,
}
impl From<Sfmcce> for bool {
    #[inline(always)]
    fn from(variant: Sfmcce) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMCCE` reader - Read instruction code select
pub type SfmcceR = crate::BitReader<Sfmcce>;
impl SfmcceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmcce {
        match self.bits {
            false => Sfmcce::_0,
            true => Sfmcce::_1,
        }
    }
    ///Uses automatically generated SPI instruction code
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmcce::_0
    }
    ///Use instruction code in the SFMSIC register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmcce::_1
    }
}
///Field `SFMCCE` writer - Read instruction code select
pub type SfmcceW<'a, REG> = crate::BitWriter<'a, REG, Sfmcce>;
impl<'a, REG> SfmcceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Uses automatically generated SPI instruction code
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmcce::_0)
    }
    ///Use instruction code in the SFMSIC register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmcce::_1)
    }
}
impl R {
    ///Bits 0:2 - Serial interface read mode select
    #[inline(always)]
    pub fn sfmrm(&self) -> SfmrmR {
        SfmrmR::new((self.bits & 7) as u8)
    }
    ///Bits 4:5 - QSSL extension function select after SPI bus access
    #[inline(always)]
    pub fn sfmse(&self) -> SfmseR {
        SfmseR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Prefetch function select
    #[inline(always)]
    pub fn sfmpfe(&self) -> SfmpfeR {
        SfmpfeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Function select for stopping prefetch at locations other than on byte boundaries
    #[inline(always)]
    pub fn sfmpae(&self) -> SfmpaeR {
        SfmpaeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SPI mode select.
    #[inline(always)]
    pub fn sfmmd3(&self) -> Sfmmd3R {
        Sfmmd3R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Extension select for the I/O buffer output enable signal for the serial interface
    #[inline(always)]
    pub fn sfmoex(&self) -> SfmoexR {
        SfmoexR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Hold time adjustment for serial transmission
    #[inline(always)]
    pub fn sfmohw(&self) -> SfmohwR {
        SfmohwR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Setup time adjustment for serial transmission
    #[inline(always)]
    pub fn sfmosw(&self) -> SfmoswR {
        SfmoswR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - Read instruction code select
    #[inline(always)]
    pub fn sfmcce(&self) -> SfmcceR {
        SfmcceR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMSMD")
            .field("sfmrm", &self.sfmrm())
            .field("sfmse", &self.sfmse())
            .field("sfmpfe", &self.sfmpfe())
            .field("sfmpae", &self.sfmpae())
            .field("sfmmd3", &self.sfmmd3())
            .field("sfmoex", &self.sfmoex())
            .field("sfmohw", &self.sfmohw())
            .field("sfmosw", &self.sfmosw())
            .field("sfmcce", &self.sfmcce())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Serial interface read mode select
    #[inline(always)]
    pub fn sfmrm(&mut self) -> SfmrmW<SfmsmdSpec> {
        SfmrmW::new(self, 0)
    }
    ///Bits 4:5 - QSSL extension function select after SPI bus access
    #[inline(always)]
    pub fn sfmse(&mut self) -> SfmseW<SfmsmdSpec> {
        SfmseW::new(self, 4)
    }
    ///Bit 6 - Prefetch function select
    #[inline(always)]
    pub fn sfmpfe(&mut self) -> SfmpfeW<SfmsmdSpec> {
        SfmpfeW::new(self, 6)
    }
    ///Bit 7 - Function select for stopping prefetch at locations other than on byte boundaries
    #[inline(always)]
    pub fn sfmpae(&mut self) -> SfmpaeW<SfmsmdSpec> {
        SfmpaeW::new(self, 7)
    }
    ///Bit 8 - SPI mode select.
    #[inline(always)]
    pub fn sfmmd3(&mut self) -> Sfmmd3W<SfmsmdSpec> {
        Sfmmd3W::new(self, 8)
    }
    ///Bit 9 - Extension select for the I/O buffer output enable signal for the serial interface
    #[inline(always)]
    pub fn sfmoex(&mut self) -> SfmoexW<SfmsmdSpec> {
        SfmoexW::new(self, 9)
    }
    ///Bit 10 - Hold time adjustment for serial transmission
    #[inline(always)]
    pub fn sfmohw(&mut self) -> SfmohwW<SfmsmdSpec> {
        SfmohwW::new(self, 10)
    }
    ///Bit 11 - Setup time adjustment for serial transmission
    #[inline(always)]
    pub fn sfmosw(&mut self) -> SfmoswW<SfmsmdSpec> {
        SfmoswW::new(self, 11)
    }
    ///Bit 15 - Read instruction code select
    #[inline(always)]
    pub fn sfmcce(&mut self) -> SfmcceW<SfmsmdSpec> {
        SfmcceW::new(self, 15)
    }
}
/**Transfer Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmsmdSpec;
impl crate::RegisterSpec for SfmsmdSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmsmd::R`](R) reader structure
impl crate::Readable for SfmsmdSpec {}
///`write(|w| ..)` method takes [`sfmsmd::W`](W) writer structure
impl crate::Writable for SfmsmdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSMD to value 0
impl crate::Resettable for SfmsmdSpec {}
