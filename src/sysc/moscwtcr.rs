///Register `MOSCWTCR` reader
pub type R = crate::R<MoscwtcrSpec>;
///Register `MOSCWTCR` writer
pub type W = crate::W<MoscwtcrSpec>;
/**Main Clock Oscillator Wait Time Setting

Value on reset: 5*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msts {
    ///0: Wait time = 3 cycles (11.4 us)
    _0x0 = 0,
    ///1: Wait time = 35 cycles (133.5 us)
    _0x1 = 1,
    ///2: Wait time = 67 cycles (255.6 us)
    _0x2 = 2,
    ///3: Wait time = 131 cycles (499.7 us)
    _0x3 = 3,
    ///4: Wait time = 259 cycles (988.0 us)
    _0x4 = 4,
    ///5: Wait time = 547 cycles (2086.6 us)
    _0x5 = 5,
    ///6: Wait time = 1059 cycles (4039.8 us)
    _0x6 = 6,
    ///7: Wait time = 2147 cycles (8190.2 us)
    _0x7 = 7,
    ///8: Wait time = 4291 cycles (16368.9 us)
    _0x8 = 8,
    ///9: Wait time = 8163 cycles (31139.4 us)
    _0x9 = 9,
    ///10: Setting prohibited
    Others = 10,
}
impl From<Msts> for u8 {
    #[inline(always)]
    fn from(variant: Msts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msts {
    type Ux = u8;
}
impl crate::IsEnum for Msts {}
///Field `MSTS` reader - Main Clock Oscillator Wait Time Setting
pub type MstsR = crate::FieldReader<Msts>;
impl MstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Msts {
        match self.bits {
            0 => Msts::_0x0,
            1 => Msts::_0x1,
            2 => Msts::_0x2,
            3 => Msts::_0x3,
            4 => Msts::_0x4,
            5 => Msts::_0x5,
            6 => Msts::_0x6,
            7 => Msts::_0x7,
            8 => Msts::_0x8,
            9 => Msts::_0x9,
            _ => Msts::Others,
        }
    }
    ///Wait time = 3 cycles (11.4 us)
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Msts::_0x0
    }
    ///Wait time = 35 cycles (133.5 us)
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Msts::_0x1
    }
    ///Wait time = 67 cycles (255.6 us)
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Msts::_0x2
    }
    ///Wait time = 131 cycles (499.7 us)
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Msts::_0x3
    }
    ///Wait time = 259 cycles (988.0 us)
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Msts::_0x4
    }
    ///Wait time = 547 cycles (2086.6 us)
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Msts::_0x5
    }
    ///Wait time = 1059 cycles (4039.8 us)
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Msts::_0x6
    }
    ///Wait time = 2147 cycles (8190.2 us)
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Msts::_0x7
    }
    ///Wait time = 4291 cycles (16368.9 us)
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Msts::_0x8
    }
    ///Wait time = 8163 cycles (31139.4 us)
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Msts::_0x9
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Msts::Others)
    }
}
///Field `MSTS` writer - Main Clock Oscillator Wait Time Setting
pub type MstsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Msts, crate::Safe>;
impl<'a, REG> MstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Wait time = 3 cycles (11.4 us)
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0x0)
    }
    ///Wait time = 35 cycles (133.5 us)
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0x1)
    }
    ///Wait time = 67 cycles (255.6 us)
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0x2)
    }
    ///Wait time = 131 cycles (499.7 us)
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0x3)
    }
    ///Wait time = 259 cycles (988.0 us)
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0x4)
    }
    ///Wait time = 547 cycles (2086.6 us)
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0x5)
    }
    ///Wait time = 1059 cycles (4039.8 us)
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0x6)
    }
    ///Wait time = 2147 cycles (8190.2 us)
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0x7)
    }
    ///Wait time = 4291 cycles (16368.9 us)
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0x8)
    }
    ///Wait time = 8163 cycles (31139.4 us)
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0x9)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::Others)
    }
}
impl R {
    ///Bits 0:3 - Main Clock Oscillator Wait Time Setting
    #[inline(always)]
    pub fn msts(&self) -> MstsR {
        MstsR::new(self.bits & 0x0f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOSCWTCR").field("msts", &self.msts()).finish()
    }
}
impl W {
    ///Bits 0:3 - Main Clock Oscillator Wait Time Setting
    #[inline(always)]
    pub fn msts(&mut self) -> MstsW<MoscwtcrSpec> {
        MstsW::new(self, 0)
    }
}
/**Main Clock Oscillator Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`moscwtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moscwtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MoscwtcrSpec;
impl crate::RegisterSpec for MoscwtcrSpec {
    type Ux = u8;
}
///`read()` method returns [`moscwtcr::R`](R) reader structure
impl crate::Readable for MoscwtcrSpec {}
///`write(|w| ..)` method takes [`moscwtcr::W`](W) writer structure
impl crate::Writable for MoscwtcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOSCWTCR to value 0x05
impl crate::Resettable for MoscwtcrSpec {
    const RESET_VALUE: u8 = 0x05;
}
