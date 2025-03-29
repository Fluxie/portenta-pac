///Register `BEMPENB` reader
pub type R = crate::R<BempenbSpec>;
///Register `BEMPENB` writer
pub type W = crate::W<BempenbSpec>;
/**BEMP Interrupt Enable for Pipes \[9:0\]

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pipebempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipebempe> for u16 {
    #[inline(always)]
    fn from(variant: Pipebempe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pipebempe {
    type Ux = u16;
}
impl crate::IsEnum for Pipebempe {}
///Field `PIPEBEMPE` reader - BEMP Interrupt Enable for Pipes \[9:0\]
pub type PipebempeR = crate::FieldReader<Pipebempe>;
impl PipebempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pipebempe> {
        match self.bits {
            0 => Some(Pipebempe::_0),
            1 => Some(Pipebempe::_1),
            _ => None,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipebempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipebempe::_1
    }
}
///Field `PIPEBEMPE` writer - BEMP Interrupt Enable for Pipes \[9:0\]
pub type PipebempeW<'a, REG> = crate::FieldWriter<'a, REG, 10, Pipebempe>;
impl<'a, REG> PipebempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipebempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipebempe::_1)
    }
}
impl R {
    ///Bits 0:9 - BEMP Interrupt Enable for Pipes \[9:0\]
    #[inline(always)]
    pub fn pipebempe(&self) -> PipebempeR {
        PipebempeR::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BEMPENB").field("pipebempe", &self.pipebempe()).finish()
    }
}
impl W {
    ///Bits 0:9 - BEMP Interrupt Enable for Pipes \[9:0\]
    #[inline(always)]
    pub fn pipebempe(&mut self) -> PipebempeW<BempenbSpec> {
        PipebempeW::new(self, 0)
    }
}
/**BEMP Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`bempenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BempenbSpec;
impl crate::RegisterSpec for BempenbSpec {
    type Ux = u16;
}
///`read()` method returns [`bempenb::R`](R) reader structure
impl crate::Readable for BempenbSpec {}
///`write(|w| ..)` method takes [`bempenb::W`](W) writer structure
impl crate::Writable for BempenbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BEMPENB to value 0
impl crate::Resettable for BempenbSpec {}
