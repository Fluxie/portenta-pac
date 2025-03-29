///Register `SFMPMD` reader
pub type R = crate::R<SfmpmdSpec>;
///Register `SFMPMD` writer
pub type W = crate::W<SfmpmdSpec>;
/**WP pin level specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmwpl {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Sfmwpl> for bool {
    #[inline(always)]
    fn from(variant: Sfmwpl) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMWPL` reader - WP pin level specification
pub type SfmwplR = crate::BitReader<Sfmwpl>;
impl SfmwplR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmwpl {
        match self.bits {
            false => Sfmwpl::_0,
            true => Sfmwpl::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmwpl::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmwpl::_1
    }
}
///Field `SFMWPL` writer - WP pin level specification
pub type SfmwplW<'a, REG> = crate::BitWriter<'a, REG, Sfmwpl>;
impl<'a, REG> SfmwplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low level
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmwpl::_0)
    }
    ///High level
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmwpl::_1)
    }
}
impl R {
    ///Bit 2 - WP pin level specification
    #[inline(always)]
    pub fn sfmwpl(&self) -> SfmwplR {
        SfmwplR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMPMD").field("sfmwpl", &self.sfmwpl()).finish()
    }
}
impl W {
    ///Bit 2 - WP pin level specification
    #[inline(always)]
    pub fn sfmwpl(&mut self) -> SfmwplW<SfmpmdSpec> {
        SfmwplW::new(self, 2)
    }
}
/**Port Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmpmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmpmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmpmdSpec;
impl crate::RegisterSpec for SfmpmdSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmpmd::R`](R) reader structure
impl crate::Readable for SfmpmdSpec {}
///`write(|w| ..)` method takes [`sfmpmd::W`](W) writer structure
impl crate::Writable for SfmpmdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMPMD to value 0
impl crate::Resettable for SfmpmdSpec {}
