///Register `CFDGAFLM%s` reader
pub type R = crate::R<CfdgaflmSpec>;
///Register `CFDGAFLM%s` writer
pub type W = crate::W<CfdgaflmSpec>;
///Field `GAFLIDM` reader - Global Acceptance Filter List ID Mask Field
pub type GaflidmR = crate::FieldReader<u32>;
///Field `GAFLIDM` writer - Global Acceptance Filter List ID Mask Field
pub type GaflidmW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
///Field `GAFLIFL1` reader - Global Acceptance Filter List Information Label 1
pub type Gaflifl1R = crate::BitReader;
///Field `GAFLIFL1` writer - Global Acceptance Filter List Information Label 1
pub type Gaflifl1W<'a, REG> = crate::BitWriter<'a, REG>;
/**Global Acceptance Filter List Entry RTR Mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gaflrtrm {
    ///0: RTR bit is not used for ID matching
    _0 = 0,
    ///1: RTR bit is used for ID matching
    _1 = 1,
}
impl From<Gaflrtrm> for bool {
    #[inline(always)]
    fn from(variant: Gaflrtrm) -> Self {
        variant as u8 != 0
    }
}
///Field `GAFLRTRM` reader - Global Acceptance Filter List Entry RTR Mask
pub type GaflrtrmR = crate::BitReader<Gaflrtrm>;
impl GaflrtrmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gaflrtrm {
        match self.bits {
            false => Gaflrtrm::_0,
            true => Gaflrtrm::_1,
        }
    }
    ///RTR bit is not used for ID matching
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gaflrtrm::_0
    }
    ///RTR bit is used for ID matching
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gaflrtrm::_1
    }
}
///Field `GAFLRTRM` writer - Global Acceptance Filter List Entry RTR Mask
pub type GaflrtrmW<'a, REG> = crate::BitWriter<'a, REG, Gaflrtrm>;
impl<'a, REG> GaflrtrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTR bit is not used for ID matching
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflrtrm::_0)
    }
    ///RTR bit is used for ID matching
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflrtrm::_1)
    }
}
/**Global Acceptance Filter List IDE Mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gaflidem {
    ///0: IDE bit is not used for ID matching
    _0 = 0,
    ///1: IDE bit is used for ID matching
    _1 = 1,
}
impl From<Gaflidem> for bool {
    #[inline(always)]
    fn from(variant: Gaflidem) -> Self {
        variant as u8 != 0
    }
}
///Field `GAFLIDEM` reader - Global Acceptance Filter List IDE Mask
pub type GaflidemR = crate::BitReader<Gaflidem>;
impl GaflidemR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gaflidem {
        match self.bits {
            false => Gaflidem::_0,
            true => Gaflidem::_1,
        }
    }
    ///IDE bit is not used for ID matching
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gaflidem::_0
    }
    ///IDE bit is used for ID matching
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gaflidem::_1
    }
}
///Field `GAFLIDEM` writer - Global Acceptance Filter List IDE Mask
pub type GaflidemW<'a, REG> = crate::BitWriter<'a, REG, Gaflidem>;
impl<'a, REG> GaflidemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IDE bit is not used for ID matching
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflidem::_0)
    }
    ///IDE bit is used for ID matching
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflidem::_1)
    }
}
impl R {
    ///Bits 0:28 - Global Acceptance Filter List ID Mask Field
    #[inline(always)]
    pub fn gaflidm(&self) -> GaflidmR {
        GaflidmR::new(self.bits & 0x1fff_ffff)
    }
    ///Bit 29 - Global Acceptance Filter List Information Label 1
    #[inline(always)]
    pub fn gaflifl1(&self) -> Gaflifl1R {
        Gaflifl1R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Global Acceptance Filter List Entry RTR Mask
    #[inline(always)]
    pub fn gaflrtrm(&self) -> GaflrtrmR {
        GaflrtrmR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Global Acceptance Filter List IDE Mask
    #[inline(always)]
    pub fn gaflidem(&self) -> GaflidemR {
        GaflidemR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGAFLM")
            .field("gaflidm", &self.gaflidm())
            .field("gaflifl1", &self.gaflifl1())
            .field("gaflrtrm", &self.gaflrtrm())
            .field("gaflidem", &self.gaflidem())
            .finish()
    }
}
impl W {
    ///Bits 0:28 - Global Acceptance Filter List ID Mask Field
    #[inline(always)]
    pub fn gaflidm(&mut self) -> GaflidmW<CfdgaflmSpec> {
        GaflidmW::new(self, 0)
    }
    ///Bit 29 - Global Acceptance Filter List Information Label 1
    #[inline(always)]
    pub fn gaflifl1(&mut self) -> Gaflifl1W<CfdgaflmSpec> {
        Gaflifl1W::new(self, 29)
    }
    ///Bit 30 - Global Acceptance Filter List Entry RTR Mask
    #[inline(always)]
    pub fn gaflrtrm(&mut self) -> GaflrtrmW<CfdgaflmSpec> {
        GaflrtrmW::new(self, 30)
    }
    ///Bit 31 - Global Acceptance Filter List IDE Mask
    #[inline(always)]
    pub fn gaflidem(&mut self) -> GaflidemW<CfdgaflmSpec> {
        GaflidemW::new(self, 31)
    }
}
/**Global Acceptance Filter List Mask Registers

You can [`read`](crate::Reg::read) this register and get [`cfdgaflm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgaflmSpec;
impl crate::RegisterSpec for CfdgaflmSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgaflm::R`](R) reader structure
impl crate::Readable for CfdgaflmSpec {}
///`write(|w| ..)` method takes [`cfdgaflm::W`](W) writer structure
impl crate::Writable for CfdgaflmSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGAFLM%s to value 0
impl crate::Resettable for CfdgaflmSpec {}
