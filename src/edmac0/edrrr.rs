///Register `EDRRR` reader
pub type R = crate::R<EdrrrSpec>;
///Register `EDRRR` writer
pub type W = crate::W<EdrrrSpec>;
/**Receive Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr {
    ///0: Disable the receive function
    _0 = 0,
    ///1: Read receive descriptor and enable the receive function.
    _1 = 1,
}
impl From<Rr> for bool {
    #[inline(always)]
    fn from(variant: Rr) -> Self {
        variant as u8 != 0
    }
}
///Field `RR` reader - Receive Request
pub type RrR = crate::BitReader<Rr>;
impl RrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rr {
        match self.bits {
            false => Rr::_0,
            true => Rr::_1,
        }
    }
    ///Disable the receive function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rr::_0
    }
    ///Read receive descriptor and enable the receive function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rr::_1
    }
}
///Field `RR` writer - Receive Request
pub type RrW<'a, REG> = crate::BitWriter<'a, REG, Rr>;
impl<'a, REG> RrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the receive function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rr::_0)
    }
    ///Read receive descriptor and enable the receive function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rr::_1)
    }
}
impl R {
    ///Bit 0 - Receive Request
    #[inline(always)]
    pub fn rr(&self) -> RrR {
        RrR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDRRR").field("rr", &self.rr()).finish()
    }
}
impl W {
    ///Bit 0 - Receive Request
    #[inline(always)]
    pub fn rr(&mut self) -> RrW<EdrrrSpec> {
        RrW::new(self, 0)
    }
}
/**EDMAC Receive Request Register

You can [`read`](crate::Reg::read) this register and get [`edrrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edrrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EdrrrSpec;
impl crate::RegisterSpec for EdrrrSpec {
    type Ux = u32;
}
///`read()` method returns [`edrrr::R`](R) reader structure
impl crate::Readable for EdrrrSpec {}
///`write(|w| ..)` method takes [`edrrr::W`](W) writer structure
impl crate::Writable for EdrrrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EDRRR to value 0
impl crate::Resettable for EdrrrSpec {}
