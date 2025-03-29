///Register `BUSSCNTPHBIU` reader
pub type R = crate::R<BusscntphbiuSpec>;
///Register `BUSSCNTPHBIU` writer
pub type W = crate::W<BusscntphbiuSpec>;
/**Arbitration Select for two masters

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arbs {
    ///0: DMAC/DTC > CPU
    _0 = 0,
    ///1: DMAC/DTC ↔ CPU
    _1 = 1,
}
impl From<Arbs> for bool {
    #[inline(always)]
    fn from(variant: Arbs) -> Self {
        variant as u8 != 0
    }
}
///Field `ARBS` reader - Arbitration Select for two masters
pub type ArbsR = crate::BitReader<Arbs>;
impl ArbsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Arbs {
        match self.bits {
            false => Arbs::_0,
            true => Arbs::_1,
        }
    }
    ///DMAC/DTC > CPU
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Arbs::_0
    }
    ///DMAC/DTC ↔ CPU
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Arbs::_1
    }
}
///Field `ARBS` writer - Arbitration Select for two masters
pub type ArbsW<'a, REG> = crate::BitWriter<'a, REG, Arbs>;
impl<'a, REG> ArbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMAC/DTC > CPU
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Arbs::_0)
    }
    ///DMAC/DTC ↔ CPU
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Arbs::_1)
    }
}
impl R {
    ///Bit 0 - Arbitration Select for two masters
    #[inline(always)]
    pub fn arbs(&self) -> ArbsR {
        ArbsR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSSCNTPHBIU").field("arbs", &self.arbs()).finish()
    }
}
impl W {
    ///Bit 0 - Arbitration Select for two masters
    #[inline(always)]
    pub fn arbs(&mut self) -> ArbsW<BusscntphbiuSpec> {
        ArbsW::new(self, 0)
    }
}
/**Slave Bus Control Register

You can [`read`](crate::Reg::read) this register and get [`busscntphbiu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntphbiu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BusscntphbiuSpec;
impl crate::RegisterSpec for BusscntphbiuSpec {
    type Ux = u16;
}
///`read()` method returns [`busscntphbiu::R`](R) reader structure
impl crate::Readable for BusscntphbiuSpec {}
///`write(|w| ..)` method takes [`busscntphbiu::W`](W) writer structure
impl crate::Writable for BusscntphbiuSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSSCNTPHBIU to value 0
impl crate::Resettable for BusscntphbiuSpec {}
