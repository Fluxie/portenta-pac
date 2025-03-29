///Register `BRDYSTS` reader
pub type R = crate::R<BrdystsSpec>;
///Register `BRDYSTS` writer
pub type W = crate::W<BrdystsSpec>;
/**BRDY Interrupt Status Flag for Pipe\[9:0\]

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pipebrdy {
    ///0: No BRDY interrupt occurred
    _0 = 0,
    ///1: BRDY interrupt occurred
    _1 = 1,
}
impl From<Pipebrdy> for u16 {
    #[inline(always)]
    fn from(variant: Pipebrdy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pipebrdy {
    type Ux = u16;
}
impl crate::IsEnum for Pipebrdy {}
///Field `PIPEBRDY` reader - BRDY Interrupt Status Flag for Pipe\[9:0\]
pub type PipebrdyR = crate::FieldReader<Pipebrdy>;
impl PipebrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pipebrdy> {
        match self.bits {
            0 => Some(Pipebrdy::_0),
            1 => Some(Pipebrdy::_1),
            _ => None,
        }
    }
    ///No BRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipebrdy::_0
    }
    ///BRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipebrdy::_1
    }
}
///Field `PIPEBRDY` writer - BRDY Interrupt Status Flag for Pipe\[9:0\]
pub type PipebrdyW<'a, REG> = crate::FieldWriter<'a, REG, 10, Pipebrdy>;
impl<'a, REG> PipebrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///No BRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipebrdy::_0)
    }
    ///BRDY interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipebrdy::_1)
    }
}
impl R {
    ///Bits 0:9 - BRDY Interrupt Status Flag for Pipe\[9:0\]
    #[inline(always)]
    pub fn pipebrdy(&self) -> PipebrdyR {
        PipebrdyR::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRDYSTS").field("pipebrdy", &self.pipebrdy()).finish()
    }
}
impl W {
    ///Bits 0:9 - BRDY Interrupt Status Flag for Pipe\[9:0\]
    #[inline(always)]
    pub fn pipebrdy(&mut self) -> PipebrdyW<BrdystsSpec> {
        PipebrdyW::new(self, 0)
    }
}
/**BRDY Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`brdysts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdysts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BrdystsSpec;
impl crate::RegisterSpec for BrdystsSpec {
    type Ux = u16;
}
///`read()` method returns [`brdysts::R`](R) reader structure
impl crate::Readable for BrdystsSpec {}
///`write(|w| ..)` method takes [`brdysts::W`](W) writer structure
impl crate::Writable for BrdystsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRDYSTS to value 0
impl crate::Resettable for BrdystsSpec {}
