///Register `SARU%s` reader
pub type R = crate::R<SaruSpec>;
///Register `SARU%s` writer
pub type W = crate::W<SaruSpec>;
/**7-bit/10-bit Address Format Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fs {
    ///0: Select 7-bit address format
    _0 = 0,
    ///1: Select 10-bit address format
    _1 = 1,
}
impl From<Fs> for bool {
    #[inline(always)]
    fn from(variant: Fs) -> Self {
        variant as u8 != 0
    }
}
///Field `FS` reader - 7-bit/10-bit Address Format Select
pub type FsR = crate::BitReader<Fs>;
impl FsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fs {
        match self.bits {
            false => Fs::_0,
            true => Fs::_1,
        }
    }
    ///Select 7-bit address format
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fs::_0
    }
    ///Select 10-bit address format
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fs::_1
    }
}
///Field `FS` writer - 7-bit/10-bit Address Format Select
pub type FsW<'a, REG> = crate::BitWriter<'a, REG, Fs>;
impl<'a, REG> FsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select 7-bit address format
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fs::_0)
    }
    ///Select 10-bit address format
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fs::_1)
    }
}
///Field `SVA` reader - 10-bit Address Upper Bits
pub type SvaR = crate::FieldReader;
///Field `SVA` writer - 10-bit Address Upper Bits
pub type SvaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - 7-bit/10-bit Address Format Select
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - 10-bit Address Upper Bits
    #[inline(always)]
    pub fn sva(&self) -> SvaR {
        SvaR::new((self.bits >> 1) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SARU").field("fs", &self.fs()).field("sva", &self.sva()).finish()
    }
}
impl W {
    ///Bit 0 - 7-bit/10-bit Address Format Select
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<SaruSpec> {
        FsW::new(self, 0)
    }
    ///Bits 1:2 - 10-bit Address Upper Bits
    #[inline(always)]
    pub fn sva(&mut self) -> SvaW<SaruSpec> {
        SvaW::new(self, 1)
    }
}
/**Slave Address Register Uy

You can [`read`](crate::Reg::read) this register and get [`saru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SaruSpec;
impl crate::RegisterSpec for SaruSpec {
    type Ux = u8;
}
///`read()` method returns [`saru::R`](R) reader structure
impl crate::Readable for SaruSpec {}
///`write(|w| ..)` method takes [`saru::W`](W) writer structure
impl crate::Writable for SaruSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SARU%s to value 0
impl crate::Resettable for SaruSpec {}
