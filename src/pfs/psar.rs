///Register `P%sSAR` reader
pub type R = crate::R<PsarSpec>;
///Register `P%sSAR` writer
pub type W = crate::W<PsarSpec>;
/**Pmn Security Attribution

Value on reset: 65535*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pmnsa {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Pmnsa> for u16 {
    #[inline(always)]
    fn from(variant: Pmnsa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmnsa {
    type Ux = u16;
}
impl crate::IsEnum for Pmnsa {}
///Field `PMNSA` reader - Pmn Security Attribution
pub type PmnsaR = crate::FieldReader<Pmnsa>;
impl PmnsaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pmnsa> {
        match self.bits {
            0 => Some(Pmnsa::_0),
            1 => Some(Pmnsa::_1),
            _ => None,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pmnsa::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pmnsa::_1
    }
}
///Field `PMNSA` writer - Pmn Security Attribution
pub type PmnsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, Pmnsa>;
impl<'a, REG> PmnsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmnsa::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmnsa::_1)
    }
}
impl R {
    ///Bits 0:15 - Pmn Security Attribution
    #[inline(always)]
    pub fn pmnsa(&self) -> PmnsaR {
        PmnsaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSAR").field("pmnsa", &self.pmnsa()).finish()
    }
}
impl W {
    ///Bits 0:15 - Pmn Security Attribution
    #[inline(always)]
    pub fn pmnsa(&mut self) -> PmnsaW<PsarSpec> {
        PmnsaW::new(self, 0)
    }
}
/**Port Security Attribution register

You can [`read`](crate::Reg::read) this register and get [`psar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PsarSpec;
impl crate::RegisterSpec for PsarSpec {
    type Ux = u16;
}
///`read()` method returns [`psar::R`](R) reader structure
impl crate::Readable for PsarSpec {}
///`write(|w| ..)` method takes [`psar::W`](W) writer structure
impl crate::Writable for PsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P%sSAR to value 0xffff
impl crate::Resettable for PsarSpec {
    const RESET_VALUE: u16 = 0xffff;
}
