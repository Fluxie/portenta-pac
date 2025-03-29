///Register `RMPR` reader
pub type R = crate::R<RmprSpec>;
///Register `RMPR` writer
pub type W = crate::W<RmprSpec>;
/**Receive Preface Length

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rplen {
    ///0: Disables the receive preface generation
    _0 = 0,
    ///1: Receive preface length (bit length)
    Others = 1,
}
impl From<Rplen> for u8 {
    #[inline(always)]
    fn from(variant: Rplen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rplen {
    type Ux = u8;
}
impl crate::IsEnum for Rplen {}
///Field `RPLEN` reader - Receive Preface Length
pub type RplenR = crate::FieldReader<Rplen>;
impl RplenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rplen {
        match self.bits {
            0 => Rplen::_0,
            _ => Rplen::Others,
        }
    }
    ///Disables the receive preface generation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rplen::_0
    }
    ///Receive preface length (bit length)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Rplen::Others)
    }
}
///Field `RPLEN` writer - Receive Preface Length
pub type RplenW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rplen, crate::Safe>;
impl<'a, REG> RplenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disables the receive preface generation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rplen::_0)
    }
    ///Receive preface length (bit length)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Rplen::Others)
    }
}
/**Receive Preface Pattern

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rppat {
    ///0: ALL ZERO
    _00 = 0,
    ///1: ZERO ONE
    _01 = 1,
    ///2: ONE ZERO
    _10 = 2,
    ///3: ALL ONE
    _11 = 3,
}
impl From<Rppat> for u8 {
    #[inline(always)]
    fn from(variant: Rppat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rppat {
    type Ux = u8;
}
impl crate::IsEnum for Rppat {}
///Field `RPPAT` reader - Receive Preface Pattern
pub type RppatR = crate::FieldReader<Rppat>;
impl RppatR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rppat {
        match self.bits {
            0 => Rppat::_00,
            1 => Rppat::_01,
            2 => Rppat::_10,
            3 => Rppat::_11,
            _ => unreachable!(),
        }
    }
    ///ALL ZERO
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rppat::_00
    }
    ///ZERO ONE
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rppat::_01
    }
    ///ONE ZERO
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rppat::_10
    }
    ///ALL ONE
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rppat::_11
    }
}
///Field `RPPAT` writer - Receive Preface Pattern
pub type RppatW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rppat, crate::Safe>;
impl<'a, REG> RppatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ALL ZERO
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rppat::_00)
    }
    ///ZERO ONE
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rppat::_01)
    }
    ///ONE ZERO
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rppat::_10)
    }
    ///ALL ONE
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rppat::_11)
    }
}
impl R {
    ///Bits 0:3 - Receive Preface Length
    #[inline(always)]
    pub fn rplen(&self) -> RplenR {
        RplenR::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - Receive Preface Pattern
    #[inline(always)]
    pub fn rppat(&self) -> RppatR {
        RppatR::new((self.bits >> 4) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMPR")
            .field("rplen", &self.rplen())
            .field("rppat", &self.rppat())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Receive Preface Length
    #[inline(always)]
    pub fn rplen(&mut self) -> RplenW<RmprSpec> {
        RplenW::new(self, 0)
    }
    ///Bits 4:5 - Receive Preface Pattern
    #[inline(always)]
    pub fn rppat(&mut self) -> RppatW<RmprSpec> {
        RppatW::new(self, 4)
    }
}
/**Receive Manchester Preface Setting Register

You can [`read`](crate::Reg::read) this register and get [`rmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RmprSpec;
impl crate::RegisterSpec for RmprSpec {
    type Ux = u8;
}
///`read()` method returns [`rmpr::R`](R) reader structure
impl crate::Readable for RmprSpec {}
///`write(|w| ..)` method takes [`rmpr::W`](W) writer structure
impl crate::Writable for RmprSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RMPR to value 0
impl crate::Resettable for RmprSpec {}
