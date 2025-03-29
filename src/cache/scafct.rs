///Register `SCAFCT` reader
pub type R = crate::R<ScafctSpec>;
///Register `SCAFCT` writer
pub type W = crate::W<ScafctSpec>;
/**S-Cache Flush

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fs {
    ///0: No action
    _0 = 0,
    ///1: S-cache line flush (all lines invalidated)
    _1 = 1,
}
impl From<Fs> for bool {
    #[inline(always)]
    fn from(variant: Fs) -> Self {
        variant as u8 != 0
    }
}
///Field `FS` reader - S-Cache Flush
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
    ///No action
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fs::_0
    }
    ///S-cache line flush (all lines invalidated)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fs::_1
    }
}
///Field `FS` writer - S-Cache Flush
pub type FsW<'a, REG> = crate::BitWriter<'a, REG, Fs>;
impl<'a, REG> FsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fs::_0)
    }
    ///S-cache line flush (all lines invalidated)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fs::_1)
    }
}
impl R {
    ///Bit 0 - S-Cache Flush
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCAFCT").field("fs", &self.fs()).finish()
    }
}
impl W {
    ///Bit 0 - S-Cache Flush
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<ScafctSpec> {
        FsW::new(self, 0)
    }
}
/**S-Cache Flush Control Register

You can [`read`](crate::Reg::read) this register and get [`scafct::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scafct::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ScafctSpec;
impl crate::RegisterSpec for ScafctSpec {
    type Ux = u32;
}
///`read()` method returns [`scafct::R`](R) reader structure
impl crate::Readable for ScafctSpec {}
///`write(|w| ..)` method takes [`scafct::W`](W) writer structure
impl crate::Writable for ScafctSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCAFCT to value 0
impl crate::Resettable for ScafctSpec {}
