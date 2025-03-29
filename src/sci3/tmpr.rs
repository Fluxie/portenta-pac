///Register `TMPR` reader
pub type R = crate::R<TmprSpec>;
///Register `TMPR` writer
pub type W = crate::W<TmprSpec>;
/**Transmit preface length

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tplen {
    ///0: Disables the transmit preface generation
    _0x0 = 0,
    ///1: Transmit preface length (bit length)
    Others = 1,
}
impl From<Tplen> for u8 {
    #[inline(always)]
    fn from(variant: Tplen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tplen {
    type Ux = u8;
}
impl crate::IsEnum for Tplen {}
///Field `TPLEN` reader - Transmit preface length
pub type TplenR = crate::FieldReader<Tplen>;
impl TplenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tplen {
        match self.bits {
            0 => Tplen::_0x0,
            _ => Tplen::Others,
        }
    }
    ///Disables the transmit preface generation
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Tplen::_0x0
    }
    ///Transmit preface length (bit length)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Tplen::Others)
    }
}
///Field `TPLEN` writer - Transmit preface length
pub type TplenW<'a, REG> = crate::FieldWriter<'a, REG, 4, Tplen, crate::Safe>;
impl<'a, REG> TplenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disables the transmit preface generation
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tplen::_0x0)
    }
    ///Transmit preface length (bit length)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Tplen::Others)
    }
}
/**Transmit preface pattern

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tppat {
    ///0: ALL ZERO
    _00 = 0,
    ///1: ZERO ONE
    _01 = 1,
    ///2: ONE ZERO
    _10 = 2,
    ///3: ALL ONE
    _11 = 3,
}
impl From<Tppat> for u8 {
    #[inline(always)]
    fn from(variant: Tppat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tppat {
    type Ux = u8;
}
impl crate::IsEnum for Tppat {}
///Field `TPPAT` reader - Transmit preface pattern
pub type TppatR = crate::FieldReader<Tppat>;
impl TppatR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tppat {
        match self.bits {
            0 => Tppat::_00,
            1 => Tppat::_01,
            2 => Tppat::_10,
            3 => Tppat::_11,
            _ => unreachable!(),
        }
    }
    ///ALL ZERO
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tppat::_00
    }
    ///ZERO ONE
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tppat::_01
    }
    ///ONE ZERO
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tppat::_10
    }
    ///ALL ONE
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tppat::_11
    }
}
///Field `TPPAT` writer - Transmit preface pattern
pub type TppatW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tppat, crate::Safe>;
impl<'a, REG> TppatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ALL ZERO
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tppat::_00)
    }
    ///ZERO ONE
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tppat::_01)
    }
    ///ONE ZERO
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tppat::_10)
    }
    ///ALL ONE
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tppat::_11)
    }
}
impl R {
    ///Bits 0:3 - Transmit preface length
    #[inline(always)]
    pub fn tplen(&self) -> TplenR {
        TplenR::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - Transmit preface pattern
    #[inline(always)]
    pub fn tppat(&self) -> TppatR {
        TppatR::new((self.bits >> 4) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMPR")
            .field("tplen", &self.tplen())
            .field("tppat", &self.tppat())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Transmit preface length
    #[inline(always)]
    pub fn tplen(&mut self) -> TplenW<TmprSpec> {
        TplenW::new(self, 0)
    }
    ///Bits 4:5 - Transmit preface pattern
    #[inline(always)]
    pub fn tppat(&mut self) -> TppatW<TmprSpec> {
        TppatW::new(self, 4)
    }
}
/**Transmit Manchester Preface Setting Register

You can [`read`](crate::Reg::read) this register and get [`tmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TmprSpec;
impl crate::RegisterSpec for TmprSpec {
    type Ux = u8;
}
///`read()` method returns [`tmpr::R`](R) reader structure
impl crate::Readable for TmprSpec {}
///`write(|w| ..)` method takes [`tmpr::W`](W) writer structure
impl crate::Writable for TmprSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TMPR to value 0
impl crate::Resettable for TmprSpec {}
