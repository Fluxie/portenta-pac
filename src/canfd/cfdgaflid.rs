///Register `CFDGAFLID%s` reader
pub type R = crate::R<CfdgaflidSpec>;
///Register `CFDGAFLID%s` writer
pub type W = crate::W<CfdgaflidSpec>;
///Field `GAFLID` reader - Global Acceptance Filter List Entry ID Field
pub type GaflidR = crate::FieldReader<u32>;
///Field `GAFLID` writer - Global Acceptance Filter List Entry ID Field
pub type GaflidW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
/**Global Acceptance Filter List Entry Loopback Configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gafllb {
    ///0: Global Acceptance Filter List entry ID for acceptance filtering with attribute RX
    _0 = 0,
    ///1: Global Acceptance Filter List entry ID for acceptance filtering with attribute TX
    _1 = 1,
}
impl From<Gafllb> for bool {
    #[inline(always)]
    fn from(variant: Gafllb) -> Self {
        variant as u8 != 0
    }
}
///Field `GAFLLB` reader - Global Acceptance Filter List Entry Loopback Configuration
pub type GafllbR = crate::BitReader<Gafllb>;
impl GafllbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gafllb {
        match self.bits {
            false => Gafllb::_0,
            true => Gafllb::_1,
        }
    }
    ///Global Acceptance Filter List entry ID for acceptance filtering with attribute RX
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gafllb::_0
    }
    ///Global Acceptance Filter List entry ID for acceptance filtering with attribute TX
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gafllb::_1
    }
}
///Field `GAFLLB` writer - Global Acceptance Filter List Entry Loopback Configuration
pub type GafllbW<'a, REG> = crate::BitWriter<'a, REG, Gafllb>;
impl<'a, REG> GafllbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Global Acceptance Filter List entry ID for acceptance filtering with attribute RX
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gafllb::_0)
    }
    ///Global Acceptance Filter List entry ID for acceptance filtering with attribute TX
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gafllb::_1)
    }
}
/**Global Acceptance Filter List Entry RTR Field

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gaflrtr {
    ///0: Data frame
    _0 = 0,
    ///1: Remote frame
    _1 = 1,
}
impl From<Gaflrtr> for bool {
    #[inline(always)]
    fn from(variant: Gaflrtr) -> Self {
        variant as u8 != 0
    }
}
///Field `GAFLRTR` reader - Global Acceptance Filter List Entry RTR Field
pub type GaflrtrR = crate::BitReader<Gaflrtr>;
impl GaflrtrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gaflrtr {
        match self.bits {
            false => Gaflrtr::_0,
            true => Gaflrtr::_1,
        }
    }
    ///Data frame
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gaflrtr::_0
    }
    ///Remote frame
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gaflrtr::_1
    }
}
///Field `GAFLRTR` writer - Global Acceptance Filter List Entry RTR Field
pub type GaflrtrW<'a, REG> = crate::BitWriter<'a, REG, Gaflrtr>;
impl<'a, REG> GaflrtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data frame
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflrtr::_0)
    }
    ///Remote frame
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflrtr::_1)
    }
}
/**Global Acceptance Filter List Entry IDE Field

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gaflide {
    ///0: Standard identifier of rule entry ID is valid for acceptance filtering
    _0 = 0,
    ///1: Extended identifier of rule entry ID is valid for acceptance filtering
    _1 = 1,
}
impl From<Gaflide> for bool {
    #[inline(always)]
    fn from(variant: Gaflide) -> Self {
        variant as u8 != 0
    }
}
///Field `GAFLIDE` reader - Global Acceptance Filter List Entry IDE Field
pub type GaflideR = crate::BitReader<Gaflide>;
impl GaflideR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gaflide {
        match self.bits {
            false => Gaflide::_0,
            true => Gaflide::_1,
        }
    }
    ///Standard identifier of rule entry ID is valid for acceptance filtering
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gaflide::_0
    }
    ///Extended identifier of rule entry ID is valid for acceptance filtering
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gaflide::_1
    }
}
///Field `GAFLIDE` writer - Global Acceptance Filter List Entry IDE Field
pub type GaflideW<'a, REG> = crate::BitWriter<'a, REG, Gaflide>;
impl<'a, REG> GaflideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Standard identifier of rule entry ID is valid for acceptance filtering
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflide::_0)
    }
    ///Extended identifier of rule entry ID is valid for acceptance filtering
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflide::_1)
    }
}
impl R {
    ///Bits 0:28 - Global Acceptance Filter List Entry ID Field
    #[inline(always)]
    pub fn gaflid(&self) -> GaflidR {
        GaflidR::new(self.bits & 0x1fff_ffff)
    }
    ///Bit 29 - Global Acceptance Filter List Entry Loopback Configuration
    #[inline(always)]
    pub fn gafllb(&self) -> GafllbR {
        GafllbR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Global Acceptance Filter List Entry RTR Field
    #[inline(always)]
    pub fn gaflrtr(&self) -> GaflrtrR {
        GaflrtrR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Global Acceptance Filter List Entry IDE Field
    #[inline(always)]
    pub fn gaflide(&self) -> GaflideR {
        GaflideR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGAFLID")
            .field("gaflid", &self.gaflid())
            .field("gafllb", &self.gafllb())
            .field("gaflrtr", &self.gaflrtr())
            .field("gaflide", &self.gaflide())
            .finish()
    }
}
impl W {
    ///Bits 0:28 - Global Acceptance Filter List Entry ID Field
    #[inline(always)]
    pub fn gaflid(&mut self) -> GaflidW<CfdgaflidSpec> {
        GaflidW::new(self, 0)
    }
    ///Bit 29 - Global Acceptance Filter List Entry Loopback Configuration
    #[inline(always)]
    pub fn gafllb(&mut self) -> GafllbW<CfdgaflidSpec> {
        GafllbW::new(self, 29)
    }
    ///Bit 30 - Global Acceptance Filter List Entry RTR Field
    #[inline(always)]
    pub fn gaflrtr(&mut self) -> GaflrtrW<CfdgaflidSpec> {
        GaflrtrW::new(self, 30)
    }
    ///Bit 31 - Global Acceptance Filter List Entry IDE Field
    #[inline(always)]
    pub fn gaflide(&mut self) -> GaflideW<CfdgaflidSpec> {
        GaflideW::new(self, 31)
    }
}
/**Global Acceptance Filter List ID Registers

You can [`read`](crate::Reg::read) this register and get [`cfdgaflid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgaflidSpec;
impl crate::RegisterSpec for CfdgaflidSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgaflid::R`](R) reader structure
impl crate::Readable for CfdgaflidSpec {}
///`write(|w| ..)` method takes [`cfdgaflid::W`](W) writer structure
impl crate::Writable for CfdgaflidSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGAFLID%s to value 0
impl crate::Resettable for CfdgaflidSpec {}
