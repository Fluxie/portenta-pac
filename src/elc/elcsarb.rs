///Register `ELCSARB` reader
pub type R = crate::R<ElcsarbSpec>;
///Register `ELCSARB` writer
pub type W = crate::W<ElcsarbSpec>;
/**Event Link Setting Register n Security Attribution

Value on reset: 65535*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Elsr {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Elsr> for u16 {
    #[inline(always)]
    fn from(variant: Elsr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Elsr {
    type Ux = u16;
}
impl crate::IsEnum for Elsr {}
///Field `ELSR` reader - Event Link Setting Register n Security Attribution
pub type ElsrR = crate::FieldReader<Elsr>;
impl ElsrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Elsr> {
        match self.bits {
            0 => Some(Elsr::_0),
            1 => Some(Elsr::_1),
            _ => None,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Elsr::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Elsr::_1
    }
}
///Field `ELSR` writer - Event Link Setting Register n Security Attribution
pub type ElsrW<'a, REG> = crate::FieldWriter<'a, REG, 16, Elsr>;
impl<'a, REG> ElsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Elsr::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Elsr::_1)
    }
}
impl R {
    ///Bits 0:15 - Event Link Setting Register n Security Attribution
    #[inline(always)]
    pub fn elsr(&self) -> ElsrR {
        ElsrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ELCSARB").field("elsr", &self.elsr()).finish()
    }
}
impl W {
    ///Bits 0:15 - Event Link Setting Register n Security Attribution
    #[inline(always)]
    pub fn elsr(&mut self) -> ElsrW<ElcsarbSpec> {
        ElsrW::new(self, 0)
    }
}
/**Event Link Controller Security Attribution Register B

You can [`read`](crate::Reg::read) this register and get [`elcsarb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elcsarb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ElcsarbSpec;
impl crate::RegisterSpec for ElcsarbSpec {
    type Ux = u16;
}
///`read()` method returns [`elcsarb::R`](R) reader structure
impl crate::Readable for ElcsarbSpec {}
///`write(|w| ..)` method takes [`elcsarb::W`](W) writer structure
impl crate::Writable for ElcsarbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ELCSARB to value 0xffff
impl crate::Resettable for ElcsarbSpec {
    const RESET_VALUE: u16 = 0xffff;
}
