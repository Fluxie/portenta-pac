///Register `SSIOFR` reader
pub type R = crate::R<SsiofrSpec>;
///Register `SSIOFR` writer
pub type W = crate::W<SsiofrSpec>;
/**Audio Format Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Omod {
    ///0: I2S format
    _00 = 0,
    ///1: TDM format
    _01 = 1,
    ///2: Monaural format
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Omod> for u8 {
    #[inline(always)]
    fn from(variant: Omod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Omod {
    type Ux = u8;
}
impl crate::IsEnum for Omod {}
///Field `OMOD` reader - Audio Format Select
pub type OmodR = crate::FieldReader<Omod>;
impl OmodR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Omod {
        match self.bits {
            0 => Omod::_00,
            1 => Omod::_01,
            2 => Omod::_10,
            3 => Omod::_11,
            _ => unreachable!(),
        }
    }
    ///I2S format
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Omod::_00
    }
    ///TDM format
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Omod::_01
    }
    ///Monaural format
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Omod::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Omod::_11
    }
}
///Field `OMOD` writer - Audio Format Select
pub type OmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Omod, crate::Safe>;
impl<'a, REG> OmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///I2S format
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Omod::_00)
    }
    ///TDM format
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Omod::_01)
    }
    ///Monaural format
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Omod::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Omod::_11)
    }
}
/**Whether to Enable LRCK/FS Continuation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrcont {
    ///0: Disables LRCK/FS continuation
    _0 = 0,
    ///1: Enables LRCK/FS continuation
    _1 = 1,
}
impl From<Lrcont> for bool {
    #[inline(always)]
    fn from(variant: Lrcont) -> Self {
        variant as u8 != 0
    }
}
///Field `LRCONT` reader - Whether to Enable LRCK/FS Continuation
pub type LrcontR = crate::BitReader<Lrcont>;
impl LrcontR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lrcont {
        match self.bits {
            false => Lrcont::_0,
            true => Lrcont::_1,
        }
    }
    ///Disables LRCK/FS continuation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lrcont::_0
    }
    ///Enables LRCK/FS continuation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lrcont::_1
    }
}
///Field `LRCONT` writer - Whether to Enable LRCK/FS Continuation
pub type LrcontW<'a, REG> = crate::BitWriter<'a, REG, Lrcont>;
impl<'a, REG> LrcontW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables LRCK/FS continuation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lrcont::_0)
    }
    ///Enables LRCK/FS continuation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lrcont::_1)
    }
}
/**Whether to Enable Stopping BCK Output When SSIE is in Idle Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bckastp {
    ///0: Always outputs BCK to the SSIBCK0 pin
    _0 = 0,
    ///1: Automatically controls output of BCK to the SSIBCK0 pin
    _1 = 1,
}
impl From<Bckastp> for bool {
    #[inline(always)]
    fn from(variant: Bckastp) -> Self {
        variant as u8 != 0
    }
}
///Field `BCKASTP` reader - Whether to Enable Stopping BCK Output When SSIE is in Idle Status
pub type BckastpR = crate::BitReader<Bckastp>;
impl BckastpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bckastp {
        match self.bits {
            false => Bckastp::_0,
            true => Bckastp::_1,
        }
    }
    ///Always outputs BCK to the SSIBCK0 pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bckastp::_0
    }
    ///Automatically controls output of BCK to the SSIBCK0 pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bckastp::_1
    }
}
///Field `BCKASTP` writer - Whether to Enable Stopping BCK Output When SSIE is in Idle Status
pub type BckastpW<'a, REG> = crate::BitWriter<'a, REG, Bckastp>;
impl<'a, REG> BckastpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Always outputs BCK to the SSIBCK0 pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bckastp::_0)
    }
    ///Automatically controls output of BCK to the SSIBCK0 pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bckastp::_1)
    }
}
impl R {
    ///Bits 0:1 - Audio Format Select
    #[inline(always)]
    pub fn omod(&self) -> OmodR {
        OmodR::new((self.bits & 3) as u8)
    }
    ///Bit 8 - Whether to Enable LRCK/FS Continuation
    #[inline(always)]
    pub fn lrcont(&self) -> LrcontR {
        LrcontR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Whether to Enable Stopping BCK Output When SSIE is in Idle Status
    #[inline(always)]
    pub fn bckastp(&self) -> BckastpR {
        BckastpR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSIOFR")
            .field("omod", &self.omod())
            .field("lrcont", &self.lrcont())
            .field("bckastp", &self.bckastp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Audio Format Select
    #[inline(always)]
    pub fn omod(&mut self) -> OmodW<SsiofrSpec> {
        OmodW::new(self, 0)
    }
    ///Bit 8 - Whether to Enable LRCK/FS Continuation
    #[inline(always)]
    pub fn lrcont(&mut self) -> LrcontW<SsiofrSpec> {
        LrcontW::new(self, 8)
    }
    ///Bit 9 - Whether to Enable Stopping BCK Output When SSIE is in Idle Status
    #[inline(always)]
    pub fn bckastp(&mut self) -> BckastpW<SsiofrSpec> {
        BckastpW::new(self, 9)
    }
}
/**Audio Format Register

You can [`read`](crate::Reg::read) this register and get [`ssiofr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiofr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsiofrSpec;
impl crate::RegisterSpec for SsiofrSpec {
    type Ux = u32;
}
///`read()` method returns [`ssiofr::R`](R) reader structure
impl crate::Readable for SsiofrSpec {}
///`write(|w| ..)` method takes [`ssiofr::W`](W) writer structure
impl crate::Writable for SsiofrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSIOFR to value 0
impl crate::Resettable for SsiofrSpec {}
