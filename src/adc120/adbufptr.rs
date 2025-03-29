///Register `ADBUFPTR` reader
pub type R = crate::R<AdbufptrSpec>;
///Register `ADBUFPTR` writer
pub type W = crate::W<AdbufptrSpec>;
///Field `BUFPTR` reader - Data Buffer Pointer
pub type BufptrR = crate::FieldReader;
///Field `BUFPTR` writer - Data Buffer Pointer
pub type BufptrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Pointer Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptrovf {
    ///0: The data buffer pointer has not overflowed.
    _0 = 0,
    ///1: The data buffer pointer has overflowed.
    _1 = 1,
}
impl From<Ptrovf> for bool {
    #[inline(always)]
    fn from(variant: Ptrovf) -> Self {
        variant as u8 != 0
    }
}
///Field `PTROVF` reader - Pointer Overflow Flag
pub type PtrovfR = crate::BitReader<Ptrovf>;
impl PtrovfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ptrovf {
        match self.bits {
            false => Ptrovf::_0,
            true => Ptrovf::_1,
        }
    }
    ///The data buffer pointer has not overflowed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ptrovf::_0
    }
    ///The data buffer pointer has overflowed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ptrovf::_1
    }
}
///Field `PTROVF` writer - Pointer Overflow Flag
pub type PtrovfW<'a, REG> = crate::BitWriter<'a, REG, Ptrovf>;
impl<'a, REG> PtrovfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The data buffer pointer has not overflowed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ptrovf::_0)
    }
    ///The data buffer pointer has overflowed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ptrovf::_1)
    }
}
impl R {
    ///Bits 0:3 - Data Buffer Pointer
    #[inline(always)]
    pub fn bufptr(&self) -> BufptrR {
        BufptrR::new(self.bits & 0x0f)
    }
    ///Bit 4 - Pointer Overflow Flag
    #[inline(always)]
    pub fn ptrovf(&self) -> PtrovfR {
        PtrovfR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADBUFPTR")
            .field("bufptr", &self.bufptr())
            .field("ptrovf", &self.ptrovf())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Data Buffer Pointer
    #[inline(always)]
    pub fn bufptr(&mut self) -> BufptrW<AdbufptrSpec> {
        BufptrW::new(self, 0)
    }
    ///Bit 4 - Pointer Overflow Flag
    #[inline(always)]
    pub fn ptrovf(&mut self) -> PtrovfW<AdbufptrSpec> {
        PtrovfW::new(self, 4)
    }
}
/**A/D Data Buffer Pointer Register

You can [`read`](crate::Reg::read) this register and get [`adbufptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adbufptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdbufptrSpec;
impl crate::RegisterSpec for AdbufptrSpec {
    type Ux = u8;
}
///`read()` method returns [`adbufptr::R`](R) reader structure
impl crate::Readable for AdbufptrSpec {}
///`write(|w| ..)` method takes [`adbufptr::W`](W) writer structure
impl crate::Writable for AdbufptrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADBUFPTR to value 0
impl crate::Resettable for AdbufptrSpec {}
