///Register `ACTR` reader
pub type R = crate::R<ActrSpec>;
///Register `ACTR` writer
pub type W = crate::W<ActrSpec>;
///Field `AST` reader - Adjustment value for receive Sampling Timing
pub type AstR = crate::FieldReader;
///Field `AST` writer - Adjustment value for receive Sampling Timing
pub type AstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Adjustment Direction for receive sampling timing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ajd {
    ///0: The sampling timing is adjusted backward to the middle of bit.
    _0 = 0,
    ///1: The sampling timing is adjusted forward to the middle of bit.
    _1 = 1,
}
impl From<Ajd> for bool {
    #[inline(always)]
    fn from(variant: Ajd) -> Self {
        variant as u8 != 0
    }
}
///Field `AJD` reader - Adjustment Direction for receive sampling timing
pub type AjdR = crate::BitReader<Ajd>;
impl AjdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ajd {
        match self.bits {
            false => Ajd::_0,
            true => Ajd::_1,
        }
    }
    ///The sampling timing is adjusted backward to the middle of bit.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ajd::_0
    }
    ///The sampling timing is adjusted forward to the middle of bit.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ajd::_1
    }
}
///Field `AJD` writer - Adjustment Direction for receive sampling timing
pub type AjdW<'a, REG> = crate::BitWriter<'a, REG, Ajd>;
impl<'a, REG> AjdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The sampling timing is adjusted backward to the middle of bit.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ajd::_0)
    }
    ///The sampling timing is adjusted forward to the middle of bit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ajd::_1)
    }
}
///Field `ATT` reader - Adjustment value for Transmit timing
pub type AttR = crate::FieldReader;
///Field `ATT` writer - Adjustment value for Transmit timing
pub type AttW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Adjustment edge for transmit timing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aet {
    ///0: Adjust the rising edge timing.
    _0 = 0,
    ///1: Adjust the falling edge timing.
    _1 = 1,
}
impl From<Aet> for bool {
    #[inline(always)]
    fn from(variant: Aet) -> Self {
        variant as u8 != 0
    }
}
///Field `AET` reader - Adjustment edge for transmit timing
pub type AetR = crate::BitReader<Aet>;
impl AetR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aet {
        match self.bits {
            false => Aet::_0,
            true => Aet::_1,
        }
    }
    ///Adjust the rising edge timing.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aet::_0
    }
    ///Adjust the falling edge timing.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aet::_1
    }
}
///Field `AET` writer - Adjustment edge for transmit timing
pub type AetW<'a, REG> = crate::BitWriter<'a, REG, Aet>;
impl<'a, REG> AetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Adjust the rising edge timing.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aet::_0)
    }
    ///Adjust the falling edge timing.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aet::_1)
    }
}
impl R {
    ///Bits 0:2 - Adjustment value for receive Sampling Timing
    #[inline(always)]
    pub fn ast(&self) -> AstR {
        AstR::new(self.bits & 7)
    }
    ///Bit 3 - Adjustment Direction for receive sampling timing
    #[inline(always)]
    pub fn ajd(&self) -> AjdR {
        AjdR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Adjustment value for Transmit timing
    #[inline(always)]
    pub fn att(&self) -> AttR {
        AttR::new((self.bits >> 4) & 7)
    }
    ///Bit 7 - Adjustment edge for transmit timing
    #[inline(always)]
    pub fn aet(&self) -> AetR {
        AetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTR")
            .field("ast", &self.ast())
            .field("ajd", &self.ajd())
            .field("att", &self.att())
            .field("aet", &self.aet())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Adjustment value for receive Sampling Timing
    #[inline(always)]
    pub fn ast(&mut self) -> AstW<ActrSpec> {
        AstW::new(self, 0)
    }
    ///Bit 3 - Adjustment Direction for receive sampling timing
    #[inline(always)]
    pub fn ajd(&mut self) -> AjdW<ActrSpec> {
        AjdW::new(self, 3)
    }
    ///Bits 4:6 - Adjustment value for Transmit timing
    #[inline(always)]
    pub fn att(&mut self) -> AttW<ActrSpec> {
        AttW::new(self, 4)
    }
    ///Bit 7 - Adjustment edge for transmit timing
    #[inline(always)]
    pub fn aet(&mut self) -> AetW<ActrSpec> {
        AetW::new(self, 7)
    }
}
/**Adjustment Communication Timing Register

You can [`read`](crate::Reg::read) this register and get [`actr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ActrSpec;
impl crate::RegisterSpec for ActrSpec {
    type Ux = u8;
}
///`read()` method returns [`actr::R`](R) reader structure
impl crate::Readable for ActrSpec {}
///`write(|w| ..)` method takes [`actr::W`](W) writer structure
impl crate::Writable for ActrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACTR to value 0
impl crate::Resettable for ActrSpec {}
