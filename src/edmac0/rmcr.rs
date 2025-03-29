///Register `RMCR` reader
pub type R = crate::R<RmcrSpec>;
///Register `RMCR` writer
pub type W = crate::W<RmcrSpec>;
/**Receive Request Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rnr {
    ///0: EDRRR.RR bit (receive request bit) is cleared to 0 when one frame is received
    _0 = 0,
    ///1: EDRRR.RR bit (receive request bit) is not cleared to 0 when one frame is received.
    _1 = 1,
}
impl From<Rnr> for bool {
    #[inline(always)]
    fn from(variant: Rnr) -> Self {
        variant as u8 != 0
    }
}
///Field `RNR` reader - Receive Request Reset
pub type RnrR = crate::BitReader<Rnr>;
impl RnrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rnr {
        match self.bits {
            false => Rnr::_0,
            true => Rnr::_1,
        }
    }
    ///EDRRR.RR bit (receive request bit) is cleared to 0 when one frame is received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rnr::_0
    }
    ///EDRRR.RR bit (receive request bit) is not cleared to 0 when one frame is received.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rnr::_1
    }
}
///Field `RNR` writer - Receive Request Reset
pub type RnrW<'a, REG> = crate::BitWriter<'a, REG, Rnr>;
impl<'a, REG> RnrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EDRRR.RR bit (receive request bit) is cleared to 0 when one frame is received
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rnr::_0)
    }
    ///EDRRR.RR bit (receive request bit) is not cleared to 0 when one frame is received.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rnr::_1)
    }
}
impl R {
    ///Bit 0 - Receive Request Reset
    #[inline(always)]
    pub fn rnr(&self) -> RnrR {
        RnrR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMCR").field("rnr", &self.rnr()).finish()
    }
}
impl W {
    ///Bit 0 - Receive Request Reset
    #[inline(always)]
    pub fn rnr(&mut self) -> RnrW<RmcrSpec> {
        RnrW::new(self, 0)
    }
}
/**Receive Method Control Register

You can [`read`](crate::Reg::read) this register and get [`rmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RmcrSpec;
impl crate::RegisterSpec for RmcrSpec {
    type Ux = u32;
}
///`read()` method returns [`rmcr::R`](R) reader structure
impl crate::Readable for RmcrSpec {}
///`write(|w| ..)` method takes [`rmcr::W`](W) writer structure
impl crate::Writable for RmcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RMCR to value 0
impl crate::Resettable for RmcrSpec {}
