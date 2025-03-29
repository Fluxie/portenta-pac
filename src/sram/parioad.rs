///Register `PARIOAD` reader
pub type R = crate::R<ParioadSpec>;
///Register `PARIOAD` writer
pub type W = crate::W<ParioadSpec>;
/**Operation After Detection

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
///Field `OAD` reader - Operation After Detection
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
///Field `OAD` writer - Operation After Detection
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
impl R {
    ///Bit 0 - Operation After Detection
    #[inline(always)]
    pub fn oad(&self) -> OadR {
        OadR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARIOAD").field("oad", &self.oad()).finish()
    }
}
impl W {
    ///Bit 0 - Operation After Detection
    #[inline(always)]
    pub fn oad(&mut self) -> OadW<ParioadSpec> {
        OadW::new(self, 0)
    }
}
/**SRAM Parity Error Operation After Detection Register

You can [`read`](crate::Reg::read) this register and get [`parioad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parioad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ParioadSpec;
impl crate::RegisterSpec for ParioadSpec {
    type Ux = u8;
}
///`read()` method returns [`parioad::R`](R) reader structure
impl crate::Readable for ParioadSpec {}
///`write(|w| ..)` method takes [`parioad::W`](W) writer structure
impl crate::Writable for ParioadSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PARIOAD to value 0
impl crate::Resettable for ParioadSpec {}
