///Register `NRDYSTS` reader
pub type R = crate::R<NrdystsSpec>;
///Register `NRDYSTS` writer
pub type W = crate::W<NrdystsSpec>;
/**NRDY Interrupt Status Flag for Pipe\[9:0\]

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pipenrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred.
    _1 = 1,
}
impl From<Pipenrdy> for u16 {
    #[inline(always)]
    fn from(variant: Pipenrdy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pipenrdy {
    type Ux = u16;
}
impl crate::IsEnum for Pipenrdy {}
///Field `PIPENRDY` reader - NRDY Interrupt Status Flag for Pipe\[9:0\]
pub type PipenrdyR = crate::FieldReader<Pipenrdy>;
impl PipenrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pipenrdy> {
        match self.bits {
            0 => Some(Pipenrdy::_0),
            1 => Some(Pipenrdy::_1),
            _ => None,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipenrdy::_0
    }
    ///NRDY interrupt occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipenrdy::_1
    }
}
///Field `PIPENRDY` writer - NRDY Interrupt Status Flag for Pipe\[9:0\]
pub type PipenrdyW<'a, REG> = crate::FieldWriter<'a, REG, 10, Pipenrdy>;
impl<'a, REG> PipenrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipenrdy::_0)
    }
    ///NRDY interrupt occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipenrdy::_1)
    }
}
impl R {
    ///Bits 0:9 - NRDY Interrupt Status Flag for Pipe\[9:0\]
    #[inline(always)]
    pub fn pipenrdy(&self) -> PipenrdyR {
        PipenrdyR::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NRDYSTS").field("pipenrdy", &self.pipenrdy()).finish()
    }
}
impl W {
    ///Bits 0:9 - NRDY Interrupt Status Flag for Pipe\[9:0\]
    #[inline(always)]
    pub fn pipenrdy(&mut self) -> PipenrdyW<NrdystsSpec> {
        PipenrdyW::new(self, 0)
    }
}
/**NRDY Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`nrdysts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdysts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NrdystsSpec;
impl crate::RegisterSpec for NrdystsSpec {
    type Ux = u16;
}
///`read()` method returns [`nrdysts::R`](R) reader structure
impl crate::Readable for NrdystsSpec {}
///`write(|w| ..)` method takes [`nrdysts::W`](W) writer structure
impl crate::Writable for NrdystsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NRDYSTS to value 0
impl crate::Resettable for NrdystsSpec {}
