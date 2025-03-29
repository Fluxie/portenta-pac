///Register `NRDYENB` reader
pub type R = crate::R<NrdyenbSpec>;
///Register `NRDYENB` writer
pub type W = crate::W<NrdyenbSpec>;
/**NRDY Interrupt Enable for Pipes \[9:0\]

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pipenrdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipenrdye> for u16 {
    #[inline(always)]
    fn from(variant: Pipenrdye) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pipenrdye {
    type Ux = u16;
}
impl crate::IsEnum for Pipenrdye {}
///Field `PIPENRDYE` reader - NRDY Interrupt Enable for Pipes \[9:0\]
pub type PipenrdyeR = crate::FieldReader<Pipenrdye>;
impl PipenrdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pipenrdye> {
        match self.bits {
            0 => Some(Pipenrdye::_0),
            1 => Some(Pipenrdye::_1),
            _ => None,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipenrdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipenrdye::_1
    }
}
///Field `PIPENRDYE` writer - NRDY Interrupt Enable for Pipes \[9:0\]
pub type PipenrdyeW<'a, REG> = crate::FieldWriter<'a, REG, 10, Pipenrdye>;
impl<'a, REG> PipenrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipenrdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipenrdye::_1)
    }
}
impl R {
    ///Bits 0:9 - NRDY Interrupt Enable for Pipes \[9:0\]
    #[inline(always)]
    pub fn pipenrdye(&self) -> PipenrdyeR {
        PipenrdyeR::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NRDYENB").field("pipenrdye", &self.pipenrdye()).finish()
    }
}
impl W {
    ///Bits 0:9 - NRDY Interrupt Enable for Pipes \[9:0\]
    #[inline(always)]
    pub fn pipenrdye(&mut self) -> PipenrdyeW<NrdyenbSpec> {
        PipenrdyeW::new(self, 0)
    }
}
/**NRDY Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`nrdyenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdyenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NrdyenbSpec;
impl crate::RegisterSpec for NrdyenbSpec {
    type Ux = u16;
}
///`read()` method returns [`nrdyenb::R`](R) reader structure
impl crate::Readable for NrdyenbSpec {}
///`write(|w| ..)` method takes [`nrdyenb::W`](W) writer structure
impl crate::Writable for NrdyenbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NRDYENB to value 0
impl crate::Resettable for NrdyenbSpec {}
