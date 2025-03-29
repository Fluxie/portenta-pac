///Register `BRDYENB` reader
pub type R = crate::R<BrdyenbSpec>;
///Register `BRDYENB` writer
pub type W = crate::W<BrdyenbSpec>;
/**BRDY Interrupt Request Enable for Pipes \[9:0\]

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pipebrdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipebrdye> for u16 {
    #[inline(always)]
    fn from(variant: Pipebrdye) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pipebrdye {
    type Ux = u16;
}
impl crate::IsEnum for Pipebrdye {}
///Field `PIPEBRDYE` reader - BRDY Interrupt Request Enable for Pipes \[9:0\]
pub type PipebrdyeR = crate::FieldReader<Pipebrdye>;
impl PipebrdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pipebrdye> {
        match self.bits {
            0 => Some(Pipebrdye::_0),
            1 => Some(Pipebrdye::_1),
            _ => None,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipebrdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipebrdye::_1
    }
}
///Field `PIPEBRDYE` writer - BRDY Interrupt Request Enable for Pipes \[9:0\]
pub type PipebrdyeW<'a, REG> = crate::FieldWriter<'a, REG, 10, Pipebrdye>;
impl<'a, REG> PipebrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipebrdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipebrdye::_1)
    }
}
impl R {
    ///Bits 0:9 - BRDY Interrupt Request Enable for Pipes \[9:0\]
    #[inline(always)]
    pub fn pipebrdye(&self) -> PipebrdyeR {
        PipebrdyeR::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRDYENB").field("pipebrdye", &self.pipebrdye()).finish()
    }
}
impl W {
    ///Bits 0:9 - BRDY Interrupt Request Enable for Pipes \[9:0\]
    #[inline(always)]
    pub fn pipebrdye(&mut self) -> PipebrdyeW<BrdyenbSpec> {
        PipebrdyeW::new(self, 0)
    }
}
/**BRDY Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`brdyenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdyenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BrdyenbSpec;
impl crate::RegisterSpec for BrdyenbSpec {
    type Ux = u16;
}
///`read()` method returns [`brdyenb::R`](R) reader structure
impl crate::Readable for BrdyenbSpec {}
///`write(|w| ..)` method takes [`brdyenb::W`](W) writer structure
impl crate::Writable for BrdyenbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRDYENB to value 0
impl crate::Resettable for BrdyenbSpec {}
