///Register `PHYSECTRL` reader
pub type R = crate::R<PhysectrlSpec>;
///Register `PHYSECTRL` writer
pub type W = crate::W<PhysectrlSpec>;
/**Single-ended Receiver Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnen {
    ///0: Single-ended receiver operation is disabled
    _0 = 0,
    ///1: Single-ended receiver operation is enabled
    _1 = 1,
}
impl From<Cnen> for bool {
    #[inline(always)]
    fn from(variant: Cnen) -> Self {
        variant as u8 != 0
    }
}
///Field `CNEN` reader - Single-ended Receiver Enable
pub type CnenR = crate::BitReader<Cnen>;
impl CnenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cnen {
        match self.bits {
            false => Cnen::_0,
            true => Cnen::_1,
        }
    }
    ///Single-ended receiver operation is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cnen::_0
    }
    ///Single-ended receiver operation is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cnen::_1
    }
}
///Field `CNEN` writer - Single-ended Receiver Enable
pub type CnenW<'a, REG> = crate::BitWriter<'a, REG, Cnen>;
impl<'a, REG> CnenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single-ended receiver operation is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cnen::_0)
    }
    ///Single-ended receiver operation is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cnen::_1)
    }
}
impl R {
    ///Bit 4 - Single-ended Receiver Enable
    #[inline(always)]
    pub fn cnen(&self) -> CnenR {
        CnenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHYSECTRL").field("cnen", &self.cnen()).finish()
    }
}
impl W {
    ///Bit 4 - Single-ended Receiver Enable
    #[inline(always)]
    pub fn cnen(&mut self) -> CnenW<PhysectrlSpec> {
        CnenW::new(self, 4)
    }
}
/**PHY Single-ended Receiver Control Register

You can [`read`](crate::Reg::read) this register and get [`physectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`physectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PhysectrlSpec;
impl crate::RegisterSpec for PhysectrlSpec {
    type Ux = u32;
}
///`read()` method returns [`physectrl::R`](R) reader structure
impl crate::Readable for PhysectrlSpec {}
///`write(|w| ..)` method takes [`physectrl::W`](W) writer structure
impl crate::Writable for PhysectrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PHYSECTRL to value 0
impl crate::Resettable for PhysectrlSpec {}
