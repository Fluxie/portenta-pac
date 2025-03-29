///Register `SNZEDCR1` reader
pub type R = crate::R<Snzedcr1Spec>;
///Register `SNZEDCR1` writer
pub type W = crate::W<Snzedcr1Spec>;
/**AGT3 underflow Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agt3unfed {
    ///0: Disable the Snooze End request
    _0 = 0,
    ///1: Enable the Snooze End request
    _1 = 1,
}
impl From<Agt3unfed> for bool {
    #[inline(always)]
    fn from(variant: Agt3unfed) -> Self {
        variant as u8 != 0
    }
}
///Field `AGT3UNFED` reader - AGT3 underflow Snooze End Enable
pub type Agt3unfedR = crate::BitReader<Agt3unfed>;
impl Agt3unfedR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Agt3unfed {
        match self.bits {
            false => Agt3unfed::_0,
            true => Agt3unfed::_1,
        }
    }
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agt3unfed::_0
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agt3unfed::_1
    }
}
///Field `AGT3UNFED` writer - AGT3 underflow Snooze End Enable
pub type Agt3unfedW<'a, REG> = crate::BitWriter<'a, REG, Agt3unfed>;
impl<'a, REG> Agt3unfedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agt3unfed::_0)
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agt3unfed::_1)
    }
}
impl R {
    ///Bit 0 - AGT3 underflow Snooze End Enable
    #[inline(always)]
    pub fn agt3unfed(&self) -> Agt3unfedR {
        Agt3unfedR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNZEDCR1").field("agt3unfed", &self.agt3unfed()).finish()
    }
}
impl W {
    ///Bit 0 - AGT3 underflow Snooze End Enable
    #[inline(always)]
    pub fn agt3unfed(&mut self) -> Agt3unfedW<Snzedcr1Spec> {
        Agt3unfedW::new(self, 0)
    }
}
/**Snooze End Control Register 1

You can [`read`](crate::Reg::read) this register and get [`snzedcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Snzedcr1Spec;
impl crate::RegisterSpec for Snzedcr1Spec {
    type Ux = u8;
}
///`read()` method returns [`snzedcr1::R`](R) reader structure
impl crate::Readable for Snzedcr1Spec {}
///`write(|w| ..)` method takes [`snzedcr1::W`](W) writer structure
impl crate::Writable for Snzedcr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SNZEDCR1 to value 0
impl crate::Resettable for Snzedcr1Spec {}
