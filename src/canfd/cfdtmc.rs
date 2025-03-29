///Register `CFDTMC%s` reader
pub type R = crate::R<CfdtmcSpec>;
///Register `CFDTMC%s` writer
pub type W = crate::W<CfdtmcSpec>;
/**TX Message Buffer Transmission Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmtr {
    ///0: TX Message buffer transmission not requested
    _0 = 0,
    ///1: TX message buffer transmission requested
    _1 = 1,
}
impl From<Tmtr> for bool {
    #[inline(always)]
    fn from(variant: Tmtr) -> Self {
        variant as u8 != 0
    }
}
///Field `TMTR` reader - TX Message Buffer Transmission Request
pub type TmtrR = crate::BitReader<Tmtr>;
impl TmtrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmtr {
        match self.bits {
            false => Tmtr::_0,
            true => Tmtr::_1,
        }
    }
    ///TX Message buffer transmission not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmtr::_0
    }
    ///TX message buffer transmission requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmtr::_1
    }
}
///Field `TMTR` writer - TX Message Buffer Transmission Request
pub type TmtrW<'a, REG> = crate::BitWriter<'a, REG, Tmtr>;
impl<'a, REG> TmtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX Message buffer transmission not requested
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmtr::_0)
    }
    ///TX message buffer transmission requested
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmtr::_1)
    }
}
/**TX Message Buffer Transmission Abort Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmtar {
    ///0: TX message buffer transmission request abort not requested
    _0 = 0,
    ///1: TX message buffer transmission request abort requested
    _1 = 1,
}
impl From<Tmtar> for bool {
    #[inline(always)]
    fn from(variant: Tmtar) -> Self {
        variant as u8 != 0
    }
}
///Field `TMTAR` reader - TX Message Buffer Transmission Abort Request
pub type TmtarR = crate::BitReader<Tmtar>;
impl TmtarR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmtar {
        match self.bits {
            false => Tmtar::_0,
            true => Tmtar::_1,
        }
    }
    ///TX message buffer transmission request abort not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmtar::_0
    }
    ///TX message buffer transmission request abort requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmtar::_1
    }
}
///Field `TMTAR` writer - TX Message Buffer Transmission Abort Request
pub type TmtarW<'a, REG> = crate::BitWriter<'a, REG, Tmtar>;
impl<'a, REG> TmtarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX message buffer transmission request abort not requested
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmtar::_0)
    }
    ///TX message buffer transmission request abort requested
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmtar::_1)
    }
}
/**TX Message Buffer One-shot Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmom {
    ///0: TX message buffer not configured in one-shot mode
    _0 = 0,
    ///1: TX message buffer configured in one-shot mode
    _1 = 1,
}
impl From<Tmom> for bool {
    #[inline(always)]
    fn from(variant: Tmom) -> Self {
        variant as u8 != 0
    }
}
///Field `TMOM` reader - TX Message Buffer One-shot Mode
pub type TmomR = crate::BitReader<Tmom>;
impl TmomR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmom {
        match self.bits {
            false => Tmom::_0,
            true => Tmom::_1,
        }
    }
    ///TX message buffer not configured in one-shot mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmom::_0
    }
    ///TX message buffer configured in one-shot mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmom::_1
    }
}
///Field `TMOM` writer - TX Message Buffer One-shot Mode
pub type TmomW<'a, REG> = crate::BitWriter<'a, REG, Tmom>;
impl<'a, REG> TmomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX message buffer not configured in one-shot mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmom::_0)
    }
    ///TX message buffer configured in one-shot mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmom::_1)
    }
}
impl R {
    ///Bit 0 - TX Message Buffer Transmission Request
    #[inline(always)]
    pub fn tmtr(&self) -> TmtrR {
        TmtrR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX Message Buffer Transmission Abort Request
    #[inline(always)]
    pub fn tmtar(&self) -> TmtarR {
        TmtarR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TX Message Buffer One-shot Mode
    #[inline(always)]
    pub fn tmom(&self) -> TmomR {
        TmomR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTMC")
            .field("tmtr", &self.tmtr())
            .field("tmtar", &self.tmtar())
            .field("tmom", &self.tmom())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX Message Buffer Transmission Request
    #[inline(always)]
    pub fn tmtr(&mut self) -> TmtrW<CfdtmcSpec> {
        TmtrW::new(self, 0)
    }
    ///Bit 1 - TX Message Buffer Transmission Abort Request
    #[inline(always)]
    pub fn tmtar(&mut self) -> TmtarW<CfdtmcSpec> {
        TmtarW::new(self, 1)
    }
    ///Bit 2 - TX Message Buffer One-shot Mode
    #[inline(always)]
    pub fn tmom(&mut self) -> TmomW<CfdtmcSpec> {
        TmomW::new(self, 2)
    }
}
/**TX Message Buffer Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtmcSpec;
impl crate::RegisterSpec for CfdtmcSpec {
    type Ux = u8;
}
///`read()` method returns [`cfdtmc::R`](R) reader structure
impl crate::Readable for CfdtmcSpec {}
///`write(|w| ..)` method takes [`cfdtmc::W`](W) writer structure
impl crate::Writable for CfdtmcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTMC%s to value 0
impl crate::Resettable for CfdtmcSpec {}
