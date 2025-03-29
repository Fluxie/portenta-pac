///Register `MMPUSARA` reader
pub type R = crate::R<MmpusaraSpec>;
///Register `MMPUSARA` writer
pub type W = crate::W<MmpusaraSpec>;
/**MMPUA Security Attribution (n = 0 to 7)

Value on reset: 255*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mmpuasan {
    ///0: Secure
    _0 = 0,
    ///1: Non-Secure
    _1 = 1,
}
impl From<Mmpuasan> for u8 {
    #[inline(always)]
    fn from(variant: Mmpuasan) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mmpuasan {
    type Ux = u8;
}
impl crate::IsEnum for Mmpuasan {}
///Field `MMPUASAn` reader - MMPUA Security Attribution (n = 0 to 7)
pub type MmpuasanR = crate::FieldReader<Mmpuasan>;
impl MmpuasanR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mmpuasan> {
        match self.bits {
            0 => Some(Mmpuasan::_0),
            1 => Some(Mmpuasan::_1),
            _ => None,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mmpuasan::_0
    }
    ///Non-Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mmpuasan::_1
    }
}
///Field `MMPUASAn` writer - MMPUA Security Attribution (n = 0 to 7)
pub type MmpuasanW<'a, REG> = crate::FieldWriter<'a, REG, 8, Mmpuasan>;
impl<'a, REG> MmpuasanW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mmpuasan::_0)
    }
    ///Non-Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmpuasan::_1)
    }
}
impl R {
    ///Bits 0:7 - MMPUA Security Attribution (n = 0 to 7)
    #[inline(always)]
    pub fn mmpuasan(&self) -> MmpuasanR {
        MmpuasanR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUSARA").field("mmpuasan", &self.mmpuasan()).finish()
    }
}
impl W {
    ///Bits 0:7 - MMPUA Security Attribution (n = 0 to 7)
    #[inline(always)]
    pub fn mmpuasan(&mut self) -> MmpuasanW<MmpusaraSpec> {
        MmpuasanW::new(self, 0)
    }
}
/**Master Memory Protection Unit Security Attribution Register A

You can [`read`](crate::Reg::read) this register and get [`mmpusara::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusara::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmpusaraSpec;
impl crate::RegisterSpec for MmpusaraSpec {
    type Ux = u32;
}
///`read()` method returns [`mmpusara::R`](R) reader structure
impl crate::Readable for MmpusaraSpec {}
///`write(|w| ..)` method takes [`mmpusara::W`](W) writer structure
impl crate::Writable for MmpusaraSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUSARA to value 0xffff_ffff
impl crate::Resettable for MmpusaraSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
