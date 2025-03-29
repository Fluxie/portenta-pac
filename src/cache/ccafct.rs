///Register `CCAFCT` reader
pub type R = crate::R<CcafctSpec>;
///Register `CCAFCT` writer
pub type W = crate::W<CcafctSpec>;
/**C-Cache Flush

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc {
    ///0: No action
    _0 = 0,
    ///1: C-cache line flush (all lines invalidated)
    _1 = 1,
}
impl From<Fc> for bool {
    #[inline(always)]
    fn from(variant: Fc) -> Self {
        variant as u8 != 0
    }
}
///Field `FC` reader - C-Cache Flush
pub type FcR = crate::BitReader<Fc>;
impl FcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fc {
        match self.bits {
            false => Fc::_0,
            true => Fc::_1,
        }
    }
    ///No action
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fc::_0
    }
    ///C-cache line flush (all lines invalidated)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fc::_1
    }
}
///Field `FC` writer - C-Cache Flush
pub type FcW<'a, REG> = crate::BitWriter<'a, REG, Fc>;
impl<'a, REG> FcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fc::_0)
    }
    ///C-cache line flush (all lines invalidated)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fc::_1)
    }
}
impl R {
    ///Bit 0 - C-Cache Flush
    #[inline(always)]
    pub fn fc(&self) -> FcR {
        FcR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCAFCT").field("fc", &self.fc()).finish()
    }
}
impl W {
    ///Bit 0 - C-Cache Flush
    #[inline(always)]
    pub fn fc(&mut self) -> FcW<CcafctSpec> {
        FcW::new(self, 0)
    }
}
/**C-Cache Flush Control Register

You can [`read`](crate::Reg::read) this register and get [`ccafct::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccafct::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CcafctSpec;
impl crate::RegisterSpec for CcafctSpec {
    type Ux = u32;
}
///`read()` method returns [`ccafct::R`](R) reader structure
impl crate::Readable for CcafctSpec {}
///`write(|w| ..)` method takes [`ccafct::W`](W) writer structure
impl crate::Writable for CcafctSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCAFCT to value 0
impl crate::Resettable for CcafctSpec {}
