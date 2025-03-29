///Register `DMECHR` reader
pub type R = crate::R<DmechrSpec>;
///Register `DMECHR` writer
pub type W = crate::W<DmechrSpec>;
///Field `DMECH` reader - DMAC Error channel
pub type DmechR = crate::FieldReader;
/**DMAC Error channel Security Attribution Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmechsam {
    ///0: Secure channel
    _0 = 0,
    ///1: Non-secure channel
    _1 = 1,
}
impl From<Dmechsam> for bool {
    #[inline(always)]
    fn from(variant: Dmechsam) -> Self {
        variant as u8 != 0
    }
}
///Field `DMECHSAM` reader - DMAC Error channel Security Attribution Monitor
pub type DmechsamR = crate::BitReader<Dmechsam>;
impl DmechsamR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dmechsam {
        match self.bits {
            false => Dmechsam::_0,
            true => Dmechsam::_1,
        }
    }
    ///Secure channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmechsam::_0
    }
    ///Non-secure channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmechsam::_1
    }
}
/**DMAC Error Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmesta {
    ///0: No DMA transfer error occurred
    _0 = 0,
    ///1: DMA transfer error occurred
    _1 = 1,
}
impl From<Dmesta> for bool {
    #[inline(always)]
    fn from(variant: Dmesta) -> Self {
        variant as u8 != 0
    }
}
///Field `DMESTA` reader - DMAC Error Status
pub type DmestaR = crate::BitReader<Dmesta>;
impl DmestaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dmesta {
        match self.bits {
            false => Dmesta::_0,
            true => Dmesta::_1,
        }
    }
    ///No DMA transfer error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmesta::_0
    }
    ///DMA transfer error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmesta::_1
    }
}
///Field `DMESTA` writer - DMAC Error Status
pub type DmestaW<'a, REG> = crate::BitWriter<'a, REG, Dmesta>;
impl<'a, REG> DmestaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No DMA transfer error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmesta::_0)
    }
    ///DMA transfer error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmesta::_1)
    }
}
impl R {
    ///Bits 0:2 - DMAC Error channel
    #[inline(always)]
    pub fn dmech(&self) -> DmechR {
        DmechR::new((self.bits & 7) as u8)
    }
    ///Bit 8 - DMAC Error channel Security Attribution Monitor
    #[inline(always)]
    pub fn dmechsam(&self) -> DmechsamR {
        DmechsamR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - DMAC Error Status
    #[inline(always)]
    pub fn dmesta(&self) -> DmestaR {
        DmestaR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMECHR")
            .field("dmech", &self.dmech())
            .field("dmechsam", &self.dmechsam())
            .field("dmesta", &self.dmesta())
            .finish()
    }
}
impl W {
    ///Bit 16 - DMAC Error Status
    #[inline(always)]
    pub fn dmesta(&mut self) -> DmestaW<DmechrSpec> {
        DmestaW::new(self, 16)
    }
}
/**DMAC Error Channel Register

You can [`read`](crate::Reg::read) this register and get [`dmechr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmechr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmechrSpec;
impl crate::RegisterSpec for DmechrSpec {
    type Ux = u32;
}
///`read()` method returns [`dmechr::R`](R) reader structure
impl crate::Readable for DmechrSpec {}
///`write(|w| ..)` method takes [`dmechr::W`](W) writer structure
impl crate::Writable for DmechrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMECHR to value 0
impl crate::Resettable for DmechrSpec {}
