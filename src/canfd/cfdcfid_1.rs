///Register `CFDCFID%s_1` reader
pub type R = crate::R<Cfdcfid1Spec>;
///Register `CFDCFID%s_1` writer
pub type W = crate::W<Cfdcfid1Spec>;
///Field `CFID` reader - Common FIFO Buffer ID Field
pub type CfidR = crate::FieldReader<u32>;
///Field `CFID` writer - Common FIFO Buffer ID Field
pub type CfidW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
/**THL Entry Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thlen {
    ///0: Entry is not to be stored in THL after successful TX
    _0 = 0,
    ///1: Entry is to be stored in THL after successful TX
    _1 = 1,
}
impl From<Thlen> for bool {
    #[inline(always)]
    fn from(variant: Thlen) -> Self {
        variant as u8 != 0
    }
}
///Field `THLEN` reader - THL Entry Enable
pub type ThlenR = crate::BitReader<Thlen>;
impl ThlenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thlen {
        match self.bits {
            false => Thlen::_0,
            true => Thlen::_1,
        }
    }
    ///Entry is not to be stored in THL after successful TX
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thlen::_0
    }
    ///Entry is to be stored in THL after successful TX
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thlen::_1
    }
}
///Field `THLEN` writer - THL Entry Enable
pub type ThlenW<'a, REG> = crate::BitWriter<'a, REG, Thlen>;
impl<'a, REG> ThlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Entry is not to be stored in THL after successful TX
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Thlen::_0)
    }
    ///Entry is to be stored in THL after successful TX
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Thlen::_1)
    }
}
/**Common FIFO Buffer RTR bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfrtr {
    ///0: Data frame
    _0 = 0,
    ///1: Remote frame
    _1 = 1,
}
impl From<Cfrtr> for bool {
    #[inline(always)]
    fn from(variant: Cfrtr) -> Self {
        variant as u8 != 0
    }
}
///Field `CFRTR` reader - Common FIFO Buffer RTR bit
pub type CfrtrR = crate::BitReader<Cfrtr>;
impl CfrtrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfrtr {
        match self.bits {
            false => Cfrtr::_0,
            true => Cfrtr::_1,
        }
    }
    ///Data frame
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfrtr::_0
    }
    ///Remote frame
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfrtr::_1
    }
}
///Field `CFRTR` writer - Common FIFO Buffer RTR bit
pub type CfrtrW<'a, REG> = crate::BitWriter<'a, REG, Cfrtr>;
impl<'a, REG> CfrtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data frame
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfrtr::_0)
    }
    ///Remote frame
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfrtr::_1)
    }
}
/**Common FIFO Buffer IDE bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfide {
    ///0: STD-ID is to be transmitted or has been received
    _0 = 0,
    ///1: EXT-ID is to be transmitted or has been received
    _1 = 1,
}
impl From<Cfide> for bool {
    #[inline(always)]
    fn from(variant: Cfide) -> Self {
        variant as u8 != 0
    }
}
///Field `CFIDE` reader - Common FIFO Buffer IDE bit
pub type CfideR = crate::BitReader<Cfide>;
impl CfideR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfide {
        match self.bits {
            false => Cfide::_0,
            true => Cfide::_1,
        }
    }
    ///STD-ID is to be transmitted or has been received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfide::_0
    }
    ///EXT-ID is to be transmitted or has been received
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfide::_1
    }
}
///Field `CFIDE` writer - Common FIFO Buffer IDE bit
pub type CfideW<'a, REG> = crate::BitWriter<'a, REG, Cfide>;
impl<'a, REG> CfideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///STD-ID is to be transmitted or has been received
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfide::_0)
    }
    ///EXT-ID is to be transmitted or has been received
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfide::_1)
    }
}
impl R {
    ///Bits 0:28 - Common FIFO Buffer ID Field
    #[inline(always)]
    pub fn cfid(&self) -> CfidR {
        CfidR::new(self.bits & 0x1fff_ffff)
    }
    ///Bit 29 - THL Entry Enable
    #[inline(always)]
    pub fn thlen(&self) -> ThlenR {
        ThlenR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Common FIFO Buffer RTR bit
    #[inline(always)]
    pub fn cfrtr(&self) -> CfrtrR {
        CfrtrR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Common FIFO Buffer IDE bit
    #[inline(always)]
    pub fn cfide(&self) -> CfideR {
        CfideR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFID_1")
            .field("cfid", &self.cfid())
            .field("thlen", &self.thlen())
            .field("cfrtr", &self.cfrtr())
            .field("cfide", &self.cfide())
            .finish()
    }
}
impl W {
    ///Bits 0:28 - Common FIFO Buffer ID Field
    #[inline(always)]
    pub fn cfid(&mut self) -> CfidW<Cfdcfid1Spec> {
        CfidW::new(self, 0)
    }
    ///Bit 29 - THL Entry Enable
    #[inline(always)]
    pub fn thlen(&mut self) -> ThlenW<Cfdcfid1Spec> {
        ThlenW::new(self, 29)
    }
    ///Bit 30 - Common FIFO Buffer RTR bit
    #[inline(always)]
    pub fn cfrtr(&mut self) -> CfrtrW<Cfdcfid1Spec> {
        CfrtrW::new(self, 30)
    }
    ///Bit 31 - Common FIFO Buffer IDE bit
    #[inline(always)]
    pub fn cfide(&mut self) -> CfideW<Cfdcfid1Spec> {
        CfideW::new(self, 31)
    }
}
/**Common FIFO Access ID Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcfid_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfid_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdcfid1Spec;
impl crate::RegisterSpec for Cfdcfid1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfid_1::R`](R) reader structure
impl crate::Readable for Cfdcfid1Spec {}
///`write(|w| ..)` method takes [`cfdcfid_1::W`](W) writer structure
impl crate::Writable for Cfdcfid1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCFID%s_1 to value 0
impl crate::Resettable for Cfdcfid1Spec {}
