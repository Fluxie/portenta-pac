///Register `MMPURPTEDMAC` reader
pub type R = crate::R<MmpurptedmacSpec>;
///Register `MMPURPTEDMAC` writer
pub type W = crate::W<MmpurptedmacSpec>;
/**Protection of register

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protect {
    ///0: Bus Master MPU register for EDMAC writing is possible.
    _0 = 0,
    ///1: Bus Master MPU register for EDMAC writing is protected. Read is possible.
    _1 = 1,
}
impl From<Protect> for bool {
    #[inline(always)]
    fn from(variant: Protect) -> Self {
        variant as u8 != 0
    }
}
///Field `PROTECT` reader - Protection of register
pub type ProtectR = crate::BitReader<Protect>;
impl ProtectR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Protect {
        match self.bits {
            false => Protect::_0,
            true => Protect::_1,
        }
    }
    ///Bus Master MPU register for EDMAC writing is possible.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Protect::_0
    }
    ///Bus Master MPU register for EDMAC writing is protected. Read is possible.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Protect::_1
    }
}
///Field `PROTECT` writer - Protection of register
pub type ProtectW<'a, REG> = crate::BitWriter<'a, REG, Protect>;
impl<'a, REG> ProtectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus Master MPU register for EDMAC writing is possible.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::_0)
    }
    ///Bus Master MPU register for EDMAC writing is protected. Read is possible.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::_1)
    }
}
///Field `KEY` writer - This bit is used to enable or disable writing of the PROTECT bit.
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Protection of register
    #[inline(always)]
    pub fn protect(&self) -> ProtectR {
        ProtectR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPURPTEDMAC").field("protect", &self.protect()).finish()
    }
}
impl W {
    ///Bit 0 - Protection of register
    #[inline(always)]
    pub fn protect(&mut self) -> ProtectW<MmpurptedmacSpec> {
        ProtectW::new(self, 0)
    }
    ///Bits 8:15 - This bit is used to enable or disable writing of the PROTECT bit.
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<MmpurptedmacSpec> {
        KeyW::new(self, 8)
    }
}
/**MMPU Regions Protect Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpurptedmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpurptedmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmpurptedmacSpec;
impl crate::RegisterSpec for MmpurptedmacSpec {
    type Ux = u16;
}
///`read()` method returns [`mmpurptedmac::R`](R) reader structure
impl crate::Readable for MmpurptedmacSpec {}
///`write(|w| ..)` method takes [`mmpurptedmac::W`](W) writer structure
impl crate::Writable for MmpurptedmacSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPURPTEDMAC to value 0
impl crate::Resettable for MmpurptedmacSpec {}
