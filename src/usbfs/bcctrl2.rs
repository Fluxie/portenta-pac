///Register `BCCTRL2` reader
pub type R = crate::R<Bcctrl2Spec>;
///Register `BCCTRL2` writer
pub type W = crate::W<Bcctrl2Spec>;
/**Dedicated Charging Port (DCP) Mode Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcpmode {
    ///0: Disable DCP
    _0 = 0,
    ///1: Enable DCP
    _1 = 1,
}
impl From<Dcpmode> for bool {
    #[inline(always)]
    fn from(variant: Dcpmode) -> Self {
        variant as u8 != 0
    }
}
///Field `DCPMODE` reader - Dedicated Charging Port (DCP) Mode Control
pub type DcpmodeR = crate::BitReader<Dcpmode>;
impl DcpmodeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dcpmode {
        match self.bits {
            false => Dcpmode::_0,
            true => Dcpmode::_1,
        }
    }
    ///Disable DCP
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcpmode::_0
    }
    ///Enable DCP
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcpmode::_1
    }
}
///Field `DCPMODE` writer - Dedicated Charging Port (DCP) Mode Control
pub type DcpmodeW<'a, REG> = crate::BitWriter<'a, REG, Dcpmode>;
impl<'a, REG> DcpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable DCP
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcpmode::_0)
    }
    ///Enable DCP
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcpmode::_1)
    }
}
/**Battery Charging Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Batchge {
    ///0: Disable Battery Charging
    _0 = 0,
    ///1: Enable Battery Charging
    _1 = 1,
}
impl From<Batchge> for bool {
    #[inline(always)]
    fn from(variant: Batchge) -> Self {
        variant as u8 != 0
    }
}
///Field `BATCHGE` reader - Battery Charging Enable
pub type BatchgeR = crate::BitReader<Batchge>;
impl BatchgeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Batchge {
        match self.bits {
            false => Batchge::_0,
            true => Batchge::_1,
        }
    }
    ///Disable Battery Charging
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Batchge::_0
    }
    ///Enable Battery Charging
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Batchge::_1
    }
}
///Field `BATCHGE` writer - Battery Charging Enable
pub type BatchgeW<'a, REG> = crate::BitWriter<'a, REG, Batchge>;
impl<'a, REG> BatchgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable Battery Charging
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Batchge::_0)
    }
    ///Enable Battery Charging
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Batchge::_1)
    }
}
///Field `PHYDET` reader - Detect Sensitivity Adjustment
pub type PhydetR = crate::FieldReader;
///Field `PHYDET` writer - Detect Sensitivity Adjustment
pub type PhydetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 6 - Dedicated Charging Port (DCP) Mode Control
    #[inline(always)]
    pub fn dcpmode(&self) -> DcpmodeR {
        DcpmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Battery Charging Enable
    #[inline(always)]
    pub fn batchge(&self) -> BatchgeR {
        BatchgeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 12:13 - Detect Sensitivity Adjustment
    #[inline(always)]
    pub fn phydet(&self) -> PhydetR {
        PhydetR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCCTRL2")
            .field("dcpmode", &self.dcpmode())
            .field("batchge", &self.batchge())
            .field("phydet", &self.phydet())
            .finish()
    }
}
impl W {
    ///Bit 6 - Dedicated Charging Port (DCP) Mode Control
    #[inline(always)]
    pub fn dcpmode(&mut self) -> DcpmodeW<Bcctrl2Spec> {
        DcpmodeW::new(self, 6)
    }
    ///Bit 7 - Battery Charging Enable
    #[inline(always)]
    pub fn batchge(&mut self) -> BatchgeW<Bcctrl2Spec> {
        BatchgeW::new(self, 7)
    }
    ///Bits 12:13 - Detect Sensitivity Adjustment
    #[inline(always)]
    pub fn phydet(&mut self) -> PhydetW<Bcctrl2Spec> {
        PhydetW::new(self, 12)
    }
}
/**Battery Charging Control Register 2

You can [`read`](crate::Reg::read) this register and get [`bcctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Bcctrl2Spec;
impl crate::RegisterSpec for Bcctrl2Spec {
    type Ux = u32;
}
///`read()` method returns [`bcctrl2::R`](R) reader structure
impl crate::Readable for Bcctrl2Spec {}
///`write(|w| ..)` method takes [`bcctrl2::W`](W) writer structure
impl crate::Writable for Bcctrl2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCCTRL2 to value 0x2000
impl crate::Resettable for Bcctrl2Spec {
    const RESET_VALUE: u32 = 0x2000;
}
