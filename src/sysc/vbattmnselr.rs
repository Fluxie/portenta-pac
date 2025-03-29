///Register `VBATTMNSELR` reader
pub type R = crate::R<VbattmnselrSpec>;
///Register `VBATTMNSELR` writer
pub type W = crate::W<VbattmnselrSpec>;
/**VBATT Low Voltage Detect Function Select Bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbattmnsel {
    ///0: Disables VBATT low voltage detect function
    _0 = 0,
    ///1: Enables VBATT low voltage detect function
    _1 = 1,
}
impl From<Vbattmnsel> for bool {
    #[inline(always)]
    fn from(variant: Vbattmnsel) -> Self {
        variant as u8 != 0
    }
}
///Field `VBATTMNSEL` reader - VBATT Low Voltage Detect Function Select Bit
pub type VbattmnselR = crate::BitReader<Vbattmnsel>;
impl VbattmnselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vbattmnsel {
        match self.bits {
            false => Vbattmnsel::_0,
            true => Vbattmnsel::_1,
        }
    }
    ///Disables VBATT low voltage detect function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbattmnsel::_0
    }
    ///Enables VBATT low voltage detect function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbattmnsel::_1
    }
}
///Field `VBATTMNSEL` writer - VBATT Low Voltage Detect Function Select Bit
pub type VbattmnselW<'a, REG> = crate::BitWriter<'a, REG, Vbattmnsel>;
impl<'a, REG> VbattmnselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables VBATT low voltage detect function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbattmnsel::_0)
    }
    ///Enables VBATT low voltage detect function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbattmnsel::_1)
    }
}
impl R {
    ///Bit 0 - VBATT Low Voltage Detect Function Select Bit
    #[inline(always)]
    pub fn vbattmnsel(&self) -> VbattmnselR {
        VbattmnselR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBATTMNSELR").field("vbattmnsel", &self.vbattmnsel()).finish()
    }
}
impl W {
    ///Bit 0 - VBATT Low Voltage Detect Function Select Bit
    #[inline(always)]
    pub fn vbattmnsel(&mut self) -> VbattmnselW<VbattmnselrSpec> {
        VbattmnselW::new(self, 0)
    }
}
/**Battery Backup Voltage Monitor Function Select Register

You can [`read`](crate::Reg::read) this register and get [`vbattmnselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbattmnselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VbattmnselrSpec;
impl crate::RegisterSpec for VbattmnselrSpec {
    type Ux = u8;
}
///`read()` method returns [`vbattmnselr::R`](R) reader structure
impl crate::Readable for VbattmnselrSpec {}
///`write(|w| ..)` method takes [`vbattmnselr::W`](W) writer structure
impl crate::Writable for VbattmnselrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBATTMNSELR to value 0
impl crate::Resettable for VbattmnselrSpec {}
