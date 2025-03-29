///Register `VBTBER` reader
pub type R = crate::R<VbtberSpec>;
///Register `VBTBER` writer
pub type W = crate::W<VbtberSpec>;
/**VBATT backup register access enable bit

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbae {
    ///0: Disable to access VBTBKR
    _0 = 0,
    ///1: Enable to access VBTBKR
    _1 = 1,
}
impl From<Vbae> for bool {
    #[inline(always)]
    fn from(variant: Vbae) -> Self {
        variant as u8 != 0
    }
}
///Field `VBAE` reader - VBATT backup register access enable bit
pub type VbaeR = crate::BitReader<Vbae>;
impl VbaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vbae {
        match self.bits {
            false => Vbae::_0,
            true => Vbae::_1,
        }
    }
    ///Disable to access VBTBKR
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbae::_0
    }
    ///Enable to access VBTBKR
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbae::_1
    }
}
///Field `VBAE` writer - VBATT backup register access enable bit
pub type VbaeW<'a, REG> = crate::BitWriter<'a, REG, Vbae>;
impl<'a, REG> VbaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable to access VBTBKR
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbae::_0)
    }
    ///Enable to access VBTBKR
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbae::_1)
    }
}
impl R {
    ///Bit 3 - VBATT backup register access enable bit
    #[inline(always)]
    pub fn vbae(&self) -> VbaeR {
        VbaeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTBER").field("vbae", &self.vbae()).finish()
    }
}
impl W {
    ///Bit 3 - VBATT backup register access enable bit
    #[inline(always)]
    pub fn vbae(&mut self) -> VbaeW<VbtberSpec> {
        VbaeW::new(self, 3)
    }
}
/**VBATT Backup Enable Register

You can [`read`](crate::Reg::read) this register and get [`vbtber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VbtberSpec;
impl crate::RegisterSpec for VbtberSpec {
    type Ux = u8;
}
///`read()` method returns [`vbtber::R`](R) reader structure
impl crate::Readable for VbtberSpec {}
///`write(|w| ..)` method takes [`vbtber::W`](W) writer structure
impl crate::Writable for VbtberSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTBER to value 0x08
impl crate::Resettable for VbtberSpec {
    const RESET_VALUE: u8 = 0x08;
}
