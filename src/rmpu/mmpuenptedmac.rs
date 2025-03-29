///Register `MMPUENPTEDMAC` reader
pub type R = crate::R<MmpuenptedmacSpec>;
///Register `MMPUENPTEDMAC` writer
pub type W = crate::W<MmpuenptedmacSpec>;
/**Protection of register

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protect {
    ///0: MMPUENEDMAC register writes are possible.
    _0 = 0,
    ///1: MMPUENEDMAC register writes are protected. Read is possible.
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
    ///MMPUENEDMAC register writes are possible.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Protect::_0
    }
    ///MMPUENEDMAC register writes are protected. Read is possible.
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
    ///MMPUENEDMAC register writes are possible.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::_0)
    }
    ///MMPUENEDMAC register writes are protected. Read is possible.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::_1)
    }
}
///Field `KEY` writer - These bits enable or disable writes to the PROTECT bit.
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
        f.debug_struct("MMPUENPTEDMAC").field("protect", &self.protect()).finish()
    }
}
impl W {
    ///Bit 0 - Protection of register
    #[inline(always)]
    pub fn protect(&mut self) -> ProtectW<MmpuenptedmacSpec> {
        ProtectW::new(self, 0)
    }
    ///Bits 8:15 - These bits enable or disable writes to the PROTECT bit.
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<MmpuenptedmacSpec> {
        KeyW::new(self, 8)
    }
}
/**MMPU Enable Protect Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuenptedmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuenptedmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmpuenptedmacSpec;
impl crate::RegisterSpec for MmpuenptedmacSpec {
    type Ux = u16;
}
///`read()` method returns [`mmpuenptedmac::R`](R) reader structure
impl crate::Readable for MmpuenptedmacSpec {}
///`write(|w| ..)` method takes [`mmpuenptedmac::W`](W) writer structure
impl crate::Writable for MmpuenptedmacSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUENPTEDMAC to value 0
impl crate::Resettable for MmpuenptedmacSpec {}
