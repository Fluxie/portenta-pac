///Register `MMPUENDMAC` reader
pub type R = crate::R<MmpuendmacSpec>;
///Register `MMPUENDMAC` writer
pub type W = crate::W<MmpuendmacSpec>;
/**Bus Master MPU of DMAC enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    ///0: Bus Master MPU of DMAC is disabled.
    _0 = 0,
    ///1: Bus Master MPU of DMAC is enabled.
    _1 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - Bus Master MPU of DMAC enable
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::_0,
            true => Enable::_1,
        }
    }
    ///Bus Master MPU of DMAC is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enable::_0
    }
    ///Bus Master MPU of DMAC is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enable::_1
    }
}
///Field `ENABLE` writer - Bus Master MPU of DMAC enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus Master MPU of DMAC is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_0)
    }
    ///Bus Master MPU of DMAC is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_1)
    }
}
///Field `KEY` writer - These bits enable or disable writes to the ENABLE bit.
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Bus Master MPU of DMAC enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUENDMAC").field("enable", &self.enable()).finish()
    }
}
impl W {
    ///Bit 0 - Bus Master MPU of DMAC enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<MmpuendmacSpec> {
        EnableW::new(self, 0)
    }
    ///Bits 8:15 - These bits enable or disable writes to the ENABLE bit.
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<MmpuendmacSpec> {
        KeyW::new(self, 8)
    }
}
/**MMPU Enable Register for DMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuendmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuendmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmpuendmacSpec;
impl crate::RegisterSpec for MmpuendmacSpec {
    type Ux = u16;
}
///`read()` method returns [`mmpuendmac::R`](R) reader structure
impl crate::Readable for MmpuendmacSpec {}
///`write(|w| ..)` method takes [`mmpuendmac::W`](W) writer structure
impl crate::Writable for MmpuendmacSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUENDMAC to value 0
impl crate::Resettable for MmpuendmacSpec {}
