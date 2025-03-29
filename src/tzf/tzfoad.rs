///Register `TZFOAD` reader
pub type R = crate::R<TzfoadSpec>;
///Register `TZFOAD` writer
pub type W = crate::W<TzfoadSpec>;
/**Operation after detection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oad {
    ///0: Non-maskable interrupt
    _0 = 0,
    ///1: Reset
    _1 = 1,
}
impl From<Oad> for bool {
    #[inline(always)]
    fn from(variant: Oad) -> Self {
        variant as u8 != 0
    }
}
///Field `OAD` reader - Operation after detection
pub type OadR = crate::BitReader<Oad>;
impl OadR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oad {
        match self.bits {
            false => Oad::_0,
            true => Oad::_1,
        }
    }
    ///Non-maskable interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oad::_0
    }
    ///Reset
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oad::_1
    }
}
///Field `OAD` writer - Operation after detection
pub type OadW<'a, REG> = crate::BitWriter<'a, REG, Oad>;
impl<'a, REG> OadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non-maskable interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oad::_0)
    }
    ///Reset
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oad::_1)
    }
}
///Field `KEY` writer - KeyCode
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Operation after detection
    #[inline(always)]
    pub fn oad(&self) -> OadR {
        OadR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZFOAD").field("oad", &self.oad()).finish()
    }
}
impl W {
    ///Bit 0 - Operation after detection
    #[inline(always)]
    pub fn oad(&mut self) -> OadW<TzfoadSpec> {
        OadW::new(self, 0)
    }
    ///Bits 8:15 - KeyCode
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<TzfoadSpec> {
        KeyW::new(self, 8)
    }
}
/**TrustZone Filter Operation After Detection Register

You can [`read`](crate::Reg::read) this register and get [`tzfoad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzfoad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TzfoadSpec;
impl crate::RegisterSpec for TzfoadSpec {
    type Ux = u16;
}
///`read()` method returns [`tzfoad::R`](R) reader structure
impl crate::Readable for TzfoadSpec {}
///`write(|w| ..)` method takes [`tzfoad::W`](W) writer structure
impl crate::Writable for TzfoadSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZFOAD to value 0
impl crate::Resettable for TzfoadSpec {}
