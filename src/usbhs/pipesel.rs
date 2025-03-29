///Register `PIPESEL` reader
pub type R = crate::R<PipeselSpec>;
///Register `PIPESEL` writer
pub type W = crate::W<PipeselSpec>;
/**Pipe Window Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pipesel {
    ///0: No pipe selected
    _0x0 = 0,
    ///1: Pipe 1
    _0x1 = 1,
    ///2: Pipe 2
    _0x2 = 2,
    ///3: Pipe 3
    _0x3 = 3,
    ///4: Pipe 4
    _0x4 = 4,
    ///5: Pipe 5
    _0x5 = 5,
    ///6: Pipe 6
    _0x6 = 6,
    ///7: Pipe 7
    _0x7 = 7,
    ///8: Pipe 8
    _0x8 = 8,
    ///9: Pipe 9
    _0x9 = 9,
    ///10: Setting prohibited
    Others = 10,
}
impl From<Pipesel> for u8 {
    #[inline(always)]
    fn from(variant: Pipesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pipesel {
    type Ux = u8;
}
impl crate::IsEnum for Pipesel {}
///Field `PIPESEL` reader - Pipe Window Select
pub type PipeselR = crate::FieldReader<Pipesel>;
impl PipeselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipesel {
        match self.bits {
            0 => Pipesel::_0x0,
            1 => Pipesel::_0x1,
            2 => Pipesel::_0x2,
            3 => Pipesel::_0x3,
            4 => Pipesel::_0x4,
            5 => Pipesel::_0x5,
            6 => Pipesel::_0x6,
            7 => Pipesel::_0x7,
            8 => Pipesel::_0x8,
            9 => Pipesel::_0x9,
            _ => Pipesel::Others,
        }
    }
    ///No pipe selected
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Pipesel::_0x0
    }
    ///Pipe 1
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Pipesel::_0x1
    }
    ///Pipe 2
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Pipesel::_0x2
    }
    ///Pipe 3
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Pipesel::_0x3
    }
    ///Pipe 4
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Pipesel::_0x4
    }
    ///Pipe 5
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Pipesel::_0x5
    }
    ///Pipe 6
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Pipesel::_0x6
    }
    ///Pipe 7
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Pipesel::_0x7
    }
    ///Pipe 8
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Pipesel::_0x8
    }
    ///Pipe 9
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Pipesel::_0x9
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Pipesel::Others)
    }
}
///Field `PIPESEL` writer - Pipe Window Select
pub type PipeselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pipesel, crate::Safe>;
impl<'a, REG> PipeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pipe selected
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0x0)
    }
    ///Pipe 1
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0x1)
    }
    ///Pipe 2
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0x2)
    }
    ///Pipe 3
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0x3)
    }
    ///Pipe 4
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0x4)
    }
    ///Pipe 5
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0x5)
    }
    ///Pipe 6
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0x6)
    }
    ///Pipe 7
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0x7)
    }
    ///Pipe 8
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0x8)
    }
    ///Pipe 9
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::_0x9)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pipesel::Others)
    }
}
impl R {
    ///Bits 0:3 - Pipe Window Select
    #[inline(always)]
    pub fn pipesel(&self) -> PipeselR {
        PipeselR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPESEL").field("pipesel", &self.pipesel()).finish()
    }
}
impl W {
    ///Bits 0:3 - Pipe Window Select
    #[inline(always)]
    pub fn pipesel(&mut self) -> PipeselW<PipeselSpec> {
        PipeselW::new(self, 0)
    }
}
/**Pipe Window Select Register

You can [`read`](crate::Reg::read) this register and get [`pipesel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipesel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PipeselSpec;
impl crate::RegisterSpec for PipeselSpec {
    type Ux = u16;
}
///`read()` method returns [`pipesel::R`](R) reader structure
impl crate::Readable for PipeselSpec {}
///`write(|w| ..)` method takes [`pipesel::W`](W) writer structure
impl crate::Writable for PipeselSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPESEL to value 0
impl crate::Resettable for PipeselSpec {}
