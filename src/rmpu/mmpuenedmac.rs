///Register `MMPUENEDMAC` reader
pub type R = crate::R<MmpuenedmacSpec>;
///Register `MMPUENEDMAC` writer
pub type W = crate::W<MmpuenedmacSpec>;
/**Bus Master MPU of EDMAC enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    ///0: Bus Master MPU of EDMAC is disabled.
    _0 = 0,
    ///1: Bus Master MPU of EDMAC is enabled.
    _1 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - Bus Master MPU of EDMAC enable
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
    ///Bus Master MPU of EDMAC is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enable::_0
    }
    ///Bus Master MPU of EDMAC is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enable::_1
    }
}
///Field `ENABLE` writer - Bus Master MPU of EDMAC enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus Master MPU of EDMAC is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_0)
    }
    ///Bus Master MPU of EDMAC is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_1)
    }
}
///Field `KEY` writer - These bits enable or disable writes to the ENABLE bit.
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Bus Master MPU of EDMAC enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUENEDMAC").field("enable", &self.enable()).finish()
    }
}
impl W {
    ///Bit 0 - Bus Master MPU of EDMAC enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<MmpuenedmacSpec> {
        EnableW::new(self, 0)
    }
    ///Bits 8:15 - These bits enable or disable writes to the ENABLE bit.
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<MmpuenedmacSpec> {
        KeyW::new(self, 8)
    }
}
/**MMPU Enable Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuenedmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuenedmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmpuenedmacSpec;
impl crate::RegisterSpec for MmpuenedmacSpec {
    type Ux = u16;
}
///`read()` method returns [`mmpuenedmac::R`](R) reader structure
impl crate::Readable for MmpuenedmacSpec {}
///`write(|w| ..)` method takes [`mmpuenedmac::W`](W) writer structure
impl crate::Writable for MmpuenedmacSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUENEDMAC to value 0
impl crate::Resettable for MmpuenedmacSpec {}
