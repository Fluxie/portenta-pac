///Register `CFIFOCTR` reader
pub type R = crate::R<CfifoctrSpec>;
///Register `CFIFOCTR` writer
pub type W = crate::W<CfifoctrSpec>;
///Field `DTLN` reader - Receive Data Length Flag
pub type DtlnR = crate::FieldReader<u16>;
/**FIFO Port Ready Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frdy {
    ///0: FIFO port access disabled
    _0 = 0,
    ///1: FIFO port access enabled
    _1 = 1,
}
impl From<Frdy> for bool {
    #[inline(always)]
    fn from(variant: Frdy) -> Self {
        variant as u8 != 0
    }
}
///Field `FRDY` reader - FIFO Port Ready Flag
pub type FrdyR = crate::BitReader<Frdy>;
impl FrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Frdy {
        match self.bits {
            false => Frdy::_0,
            true => Frdy::_1,
        }
    }
    ///FIFO port access disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Frdy::_0
    }
    ///FIFO port access enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Frdy::_1
    }
}
/**CPU Buffer Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bclr {
    ///0: No operation (writing 0 has no effect)
    _0 = 0,
    ///1: Clear FIFO buffer on the CPU side
    _1 = 1,
}
impl From<Bclr> for bool {
    #[inline(always)]
    fn from(variant: Bclr) -> Self {
        variant as u8 != 0
    }
}
///Field `BCLR` writer - CPU Buffer Clear
pub type BclrW<'a, REG> = crate::BitWriter<'a, REG, Bclr>;
impl<'a, REG> BclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No operation (writing 0 has no effect)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bclr::_0)
    }
    ///Clear FIFO buffer on the CPU side
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bclr::_1)
    }
}
/**FIFO Buffer Valid Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bval {
    ///0: Invalid (writing 0 has no effect)
    _0 = 0,
    ///1: Writing ended
    _1 = 1,
}
impl From<Bval> for bool {
    #[inline(always)]
    fn from(variant: Bval) -> Self {
        variant as u8 != 0
    }
}
///Field `BVAL` reader - FIFO Buffer Valid Flag
pub type BvalR = crate::BitReader<Bval>;
impl BvalR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bval {
        match self.bits {
            false => Bval::_0,
            true => Bval::_1,
        }
    }
    ///Invalid (writing 0 has no effect)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bval::_0
    }
    ///Writing ended
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bval::_1
    }
}
///Field `BVAL` writer - FIFO Buffer Valid Flag
pub type BvalW<'a, REG> = crate::BitWriter<'a, REG, Bval>;
impl<'a, REG> BvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid (writing 0 has no effect)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bval::_0)
    }
    ///Writing ended
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bval::_1)
    }
}
impl R {
    ///Bits 0:11 - Receive Data Length Flag
    #[inline(always)]
    pub fn dtln(&self) -> DtlnR {
        DtlnR::new(self.bits & 0x0fff)
    }
    ///Bit 13 - FIFO Port Ready Flag
    #[inline(always)]
    pub fn frdy(&self) -> FrdyR {
        FrdyR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - FIFO Buffer Valid Flag
    #[inline(always)]
    pub fn bval(&self) -> BvalR {
        BvalR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFIFOCTR")
            .field("dtln", &self.dtln())
            .field("frdy", &self.frdy())
            .field("bval", &self.bval())
            .finish()
    }
}
impl W {
    ///Bit 14 - CPU Buffer Clear
    #[inline(always)]
    pub fn bclr(&mut self) -> BclrW<CfifoctrSpec> {
        BclrW::new(self, 14)
    }
    ///Bit 15 - FIFO Buffer Valid Flag
    #[inline(always)]
    pub fn bval(&mut self) -> BvalW<CfifoctrSpec> {
        BvalW::new(self, 15)
    }
}
/**FIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`cfifoctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfifoctrSpec;
impl crate::RegisterSpec for CfifoctrSpec {
    type Ux = u16;
}
///`read()` method returns [`cfifoctr::R`](R) reader structure
impl crate::Readable for CfifoctrSpec {}
///`write(|w| ..)` method takes [`cfifoctr::W`](W) writer structure
impl crate::Writable for CfifoctrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFOCTR to value 0
impl crate::Resettable for CfifoctrSpec {}
