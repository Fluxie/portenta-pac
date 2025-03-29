///Register `CR3` reader
pub type R = crate::R<Cr3Spec>;
///Register `CR3` writer
pub type W = crate::W<Cr3Spec>;
/**Start Frame Detection Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdst {
    ///0: Detection of Start Frame is not performed.
    _0 = 0,
    ///1: Detection of Start Frame is performed.
    _1 = 1,
}
impl From<Sdst> for bool {
    #[inline(always)]
    fn from(variant: Sdst) -> Self {
        variant as u8 != 0
    }
}
///Field `SDST` reader - Start Frame Detection Start
pub type SdstR = crate::BitReader<Sdst>;
impl SdstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdst {
        match self.bits {
            false => Sdst::_0,
            true => Sdst::_1,
        }
    }
    ///Detection of Start Frame is not performed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdst::_0
    }
    ///Detection of Start Frame is performed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdst::_1
    }
}
///Field `SDST` writer - Start Frame Detection Start
pub type SdstW<'a, REG> = crate::BitWriter<'a, REG, Sdst>;
impl<'a, REG> SdstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detection of Start Frame is not performed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdst::_0)
    }
    ///Detection of Start Frame is performed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdst::_1)
    }
}
impl R {
    ///Bit 0 - Start Frame Detection Start
    #[inline(always)]
    pub fn sdst(&self) -> SdstR {
        SdstR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3").field("sdst", &self.sdst()).finish()
    }
}
impl W {
    ///Bit 0 - Start Frame Detection Start
    #[inline(always)]
    pub fn sdst(&mut self) -> SdstW<Cr3Spec> {
        SdstW::new(self, 0)
    }
}
/**Control Register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u8;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for Cr3Spec {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for Cr3Spec {}
