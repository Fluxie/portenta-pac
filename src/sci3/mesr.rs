///Register `MESR` reader
pub type R = crate::R<MesrSpec>;
///Register `MESR` writer
pub type W = crate::W<MesrSpec>;
/**Preface Error flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfer {
    ///0: No preface error detected
    _0 = 0,
    ///1: Preface error detected
    _1 = 1,
}
impl From<Pfer> for bool {
    #[inline(always)]
    fn from(variant: Pfer) -> Self {
        variant as u8 != 0
    }
}
///Field `PFER` reader - Preface Error flag
pub type PferR = crate::BitReader<Pfer>;
impl PferR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pfer {
        match self.bits {
            false => Pfer::_0,
            true => Pfer::_1,
        }
    }
    ///No preface error detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pfer::_0
    }
    ///Preface error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pfer::_1
    }
}
///Field `PFER` writer - Preface Error flag
pub type PferW<'a, REG> = crate::BitWriter<'a, REG, Pfer>;
impl<'a, REG> PferW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No preface error detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pfer::_0)
    }
    ///Preface error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pfer::_1)
    }
}
/**SYNC Error flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syer {
    ///0: No receive SYNC error detected
    _0 = 0,
    ///1: Receive SYNC error detected
    _1 = 1,
}
impl From<Syer> for bool {
    #[inline(always)]
    fn from(variant: Syer) -> Self {
        variant as u8 != 0
    }
}
///Field `SYER` reader - SYNC Error flag
pub type SyerR = crate::BitReader<Syer>;
impl SyerR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Syer {
        match self.bits {
            false => Syer::_0,
            true => Syer::_1,
        }
    }
    ///No receive SYNC error detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Syer::_0
    }
    ///Receive SYNC error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Syer::_1
    }
}
///Field `SYER` writer - SYNC Error flag
pub type SyerW<'a, REG> = crate::BitWriter<'a, REG, Syer>;
impl<'a, REG> SyerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No receive SYNC error detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Syer::_0)
    }
    ///Receive SYNC error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Syer::_1)
    }
}
/**Start Bit Error flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sber {
    ///0: No start bit error detected
    _0 = 0,
    ///1: Start bit error detected
    _1 = 1,
}
impl From<Sber> for bool {
    #[inline(always)]
    fn from(variant: Sber) -> Self {
        variant as u8 != 0
    }
}
///Field `SBER` reader - Start Bit Error flag
pub type SberR = crate::BitReader<Sber>;
impl SberR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sber {
        match self.bits {
            false => Sber::_0,
            true => Sber::_1,
        }
    }
    ///No start bit error detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sber::_0
    }
    ///Start bit error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sber::_1
    }
}
///Field `SBER` writer - Start Bit Error flag
pub type SberW<'a, REG> = crate::BitWriter<'a, REG, Sber>;
impl<'a, REG> SberW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No start bit error detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sber::_0)
    }
    ///Start bit error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sber::_1)
    }
}
impl R {
    ///Bit 0 - Preface Error flag
    #[inline(always)]
    pub fn pfer(&self) -> PferR {
        PferR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SYNC Error flag
    #[inline(always)]
    pub fn syer(&self) -> SyerR {
        SyerR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Start Bit Error flag
    #[inline(always)]
    pub fn sber(&self) -> SberR {
        SberR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MESR")
            .field("pfer", &self.pfer())
            .field("syer", &self.syer())
            .field("sber", &self.sber())
            .finish()
    }
}
impl W {
    ///Bit 0 - Preface Error flag
    #[inline(always)]
    pub fn pfer(&mut self) -> PferW<MesrSpec> {
        PferW::new(self, 0)
    }
    ///Bit 1 - SYNC Error flag
    #[inline(always)]
    pub fn syer(&mut self) -> SyerW<MesrSpec> {
        SyerW::new(self, 1)
    }
    ///Bit 2 - Start Bit Error flag
    #[inline(always)]
    pub fn sber(&mut self) -> SberW<MesrSpec> {
        SberW::new(self, 2)
    }
}
/**Manchester Extended Error Status Register

You can [`read`](crate::Reg::read) this register and get [`mesr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mesr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MesrSpec;
impl crate::RegisterSpec for MesrSpec {
    type Ux = u8;
}
///`read()` method returns [`mesr::R`](R) reader structure
impl crate::Readable for MesrSpec {}
///`write(|w| ..)` method takes [`mesr::W`](W) writer structure
impl crate::Writable for MesrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MESR to value 0
impl crate::Resettable for MesrSpec {}
