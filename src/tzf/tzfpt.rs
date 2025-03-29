///Register `TZFPT` reader
pub type R = crate::R<TzfptSpec>;
///Register `TZFPT` writer
pub type W = crate::W<TzfptSpec>;
/**Protection of register

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protect {
    ///0: All Bus TrustZone Filter register writing is protected. Read is possible.
    _0 = 0,
    ///1: All Bus TrustZone Filter register writing is possible.
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
    ///All Bus TrustZone Filter register writing is protected. Read is possible.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Protect::_0
    }
    ///All Bus TrustZone Filter register writing is possible.
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
    ///All Bus TrustZone Filter register writing is protected. Read is possible.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::_0)
    }
    ///All Bus TrustZone Filter register writing is possible.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::_1)
    }
}
///Field `KEY` writer - KeyCode
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
        f.debug_struct("TZFPT").field("protect", &self.protect()).finish()
    }
}
impl W {
    ///Bit 0 - Protection of register
    #[inline(always)]
    pub fn protect(&mut self) -> ProtectW<TzfptSpec> {
        ProtectW::new(self, 0)
    }
    ///Bits 8:15 - KeyCode
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<TzfptSpec> {
        KeyW::new(self, 8)
    }
}
/**TrustZone Filter Protect Register

You can [`read`](crate::Reg::read) this register and get [`tzfpt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzfpt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TzfptSpec;
impl crate::RegisterSpec for TzfptSpec {
    type Ux = u16;
}
///`read()` method returns [`tzfpt::R`](R) reader structure
impl crate::Readable for TzfptSpec {}
///`write(|w| ..)` method takes [`tzfpt::W`](W) writer structure
impl crate::Writable for TzfptSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZFPT to value 0
impl crate::Resettable for TzfptSpec {}
