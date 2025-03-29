///Register `BEMPSTS` reader
pub type R = crate::R<BempstsSpec>;
///Register `BEMPSTS` writer
pub type W = crate::W<BempstsSpec>;
/**BEMP Interrupt Status Flag for Pipe\[9:0\]

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pipebemp {
    ///0: No BEMP interrupt occurred
    _0 = 0,
    ///1: BEMP interrupt occurred.
    _1 = 1,
}
impl From<Pipebemp> for u16 {
    #[inline(always)]
    fn from(variant: Pipebemp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pipebemp {
    type Ux = u16;
}
impl crate::IsEnum for Pipebemp {}
///Field `PIPEBEMP` reader - BEMP Interrupt Status Flag for Pipe\[9:0\]
pub type PipebempR = crate::FieldReader<Pipebemp>;
impl PipebempR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pipebemp> {
        match self.bits {
            0 => Some(Pipebemp::_0),
            1 => Some(Pipebemp::_1),
            _ => None,
        }
    }
    ///No BEMP interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipebemp::_0
    }
    ///BEMP interrupt occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipebemp::_1
    }
}
///Field `PIPEBEMP` writer - BEMP Interrupt Status Flag for Pipe\[9:0\]
pub type PipebempW<'a, REG> = crate::FieldWriter<'a, REG, 10, Pipebemp>;
impl<'a, REG> PipebempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///No BEMP interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipebemp::_0)
    }
    ///BEMP interrupt occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipebemp::_1)
    }
}
impl R {
    ///Bits 0:9 - BEMP Interrupt Status Flag for Pipe\[9:0\]
    #[inline(always)]
    pub fn pipebemp(&self) -> PipebempR {
        PipebempR::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BEMPSTS").field("pipebemp", &self.pipebemp()).finish()
    }
}
impl W {
    ///Bits 0:9 - BEMP Interrupt Status Flag for Pipe\[9:0\]
    #[inline(always)]
    pub fn pipebemp(&mut self) -> PipebempW<BempstsSpec> {
        PipebempW::new(self, 0)
    }
}
/**BEMP Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`bempsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BempstsSpec;
impl crate::RegisterSpec for BempstsSpec {
    type Ux = u16;
}
///`read()` method returns [`bempsts::R`](R) reader structure
impl crate::Readable for BempstsSpec {}
///`write(|w| ..)` method takes [`bempsts::W`](W) writer structure
impl crate::Writable for BempstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BEMPSTS to value 0
impl crate::Resettable for BempstsSpec {}
