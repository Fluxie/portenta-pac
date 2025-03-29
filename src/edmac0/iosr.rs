///Register `IOSR` reader
pub type R = crate::R<IosrSpec>;
///Register `IOSR` writer
pub type W = crate::W<IosrSpec>;
/**External Loopback Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elb {
    ///0: Output low on the ET0_EXOUT pin
    _0 = 0,
    ///1: Output high on the ET0_EXOUT pin.
    _1 = 1,
}
impl From<Elb> for bool {
    #[inline(always)]
    fn from(variant: Elb) -> Self {
        variant as u8 != 0
    }
}
///Field `ELB` reader - External Loopback Mode
pub type ElbR = crate::BitReader<Elb>;
impl ElbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Elb {
        match self.bits {
            false => Elb::_0,
            true => Elb::_1,
        }
    }
    ///Output low on the ET0_EXOUT pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Elb::_0
    }
    ///Output high on the ET0_EXOUT pin.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Elb::_1
    }
}
///Field `ELB` writer - External Loopback Mode
pub type ElbW<'a, REG> = crate::BitWriter<'a, REG, Elb>;
impl<'a, REG> ElbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output low on the ET0_EXOUT pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Elb::_0)
    }
    ///Output high on the ET0_EXOUT pin.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Elb::_1)
    }
}
impl R {
    ///Bit 0 - External Loopback Mode
    #[inline(always)]
    pub fn elb(&self) -> ElbR {
        ElbR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOSR").field("elb", &self.elb()).finish()
    }
}
impl W {
    ///Bit 0 - External Loopback Mode
    #[inline(always)]
    pub fn elb(&mut self) -> ElbW<IosrSpec> {
        ElbW::new(self, 0)
    }
}
/**Independent Output Signal Setting Register

You can [`read`](crate::Reg::read) this register and get [`iosr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iosr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IosrSpec;
impl crate::RegisterSpec for IosrSpec {
    type Ux = u32;
}
///`read()` method returns [`iosr::R`](R) reader structure
impl crate::Readable for IosrSpec {}
///`write(|w| ..)` method takes [`iosr::W`](W) writer structure
impl crate::Writable for IosrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOSR to value 0
impl crate::Resettable for IosrSpec {}
