///Register `CFDTHLSTS%s` reader
pub type R = crate::R<CfdthlstsSpec>;
///Register `CFDTHLSTS%s` writer
pub type W = crate::W<CfdthlstsSpec>;
/**TX History List Empty

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thlemp {
    ///0: TX History List not empty
    _0 = 0,
    ///1: TX History List empty
    _1 = 1,
}
impl From<Thlemp> for bool {
    #[inline(always)]
    fn from(variant: Thlemp) -> Self {
        variant as u8 != 0
    }
}
///Field `THLEMP` reader - TX History List Empty
pub type ThlempR = crate::BitReader<Thlemp>;
impl ThlempR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thlemp {
        match self.bits {
            false => Thlemp::_0,
            true => Thlemp::_1,
        }
    }
    ///TX History List not empty
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thlemp::_0
    }
    ///TX History List empty
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thlemp::_1
    }
}
/**TX History List Full

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thlfll {
    ///0: TX History List not full
    _0 = 0,
    ///1: TX History List full
    _1 = 1,
}
impl From<Thlfll> for bool {
    #[inline(always)]
    fn from(variant: Thlfll) -> Self {
        variant as u8 != 0
    }
}
///Field `THLFLL` reader - TX History List Full
pub type ThlfllR = crate::BitReader<Thlfll>;
impl ThlfllR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thlfll {
        match self.bits {
            false => Thlfll::_0,
            true => Thlfll::_1,
        }
    }
    ///TX History List not full
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thlfll::_0
    }
    ///TX History List full
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thlfll::_1
    }
}
/**TX History List Entry Lost

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thlelt {
    ///0: No entry lost in TX History List
    _0 = 0,
    ///1: TX History List entry Lost
    _1 = 1,
}
impl From<Thlelt> for bool {
    #[inline(always)]
    fn from(variant: Thlelt) -> Self {
        variant as u8 != 0
    }
}
///Field `THLELT` reader - TX History List Entry Lost
pub type ThleltR = crate::BitReader<Thlelt>;
impl ThleltR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thlelt {
        match self.bits {
            false => Thlelt::_0,
            true => Thlelt::_1,
        }
    }
    ///No entry lost in TX History List
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thlelt::_0
    }
    ///TX History List entry Lost
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thlelt::_1
    }
}
///Field `THLELT` writer - TX History List Entry Lost
pub type ThleltW<'a, REG> = crate::BitWriter<'a, REG, Thlelt>;
impl<'a, REG> ThleltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No entry lost in TX History List
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Thlelt::_0)
    }
    ///TX History List entry Lost
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Thlelt::_1)
    }
}
/**TX History List Interrupt Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thlif {
    ///0: TX History List interrupt condition not satisfied
    _0 = 0,
    ///1: TX History List interrupt condition satisfied
    _1 = 1,
}
impl From<Thlif> for bool {
    #[inline(always)]
    fn from(variant: Thlif) -> Self {
        variant as u8 != 0
    }
}
///Field `THLIF` reader - TX History List Interrupt Flag
pub type ThlifR = crate::BitReader<Thlif>;
impl ThlifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thlif {
        match self.bits {
            false => Thlif::_0,
            true => Thlif::_1,
        }
    }
    ///TX History List interrupt condition not satisfied
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thlif::_0
    }
    ///TX History List interrupt condition satisfied
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thlif::_1
    }
}
///Field `THLIF` writer - TX History List Interrupt Flag
pub type ThlifW<'a, REG> = crate::BitWriter<'a, REG, Thlif>;
impl<'a, REG> ThlifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX History List interrupt condition not satisfied
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Thlif::_0)
    }
    ///TX History List interrupt condition satisfied
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Thlif::_1)
    }
}
///Field `THLMC` reader - TX History List Message Count
pub type ThlmcR = crate::FieldReader;
impl R {
    ///Bit 0 - TX History List Empty
    #[inline(always)]
    pub fn thlemp(&self) -> ThlempR {
        ThlempR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX History List Full
    #[inline(always)]
    pub fn thlfll(&self) -> ThlfllR {
        ThlfllR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TX History List Entry Lost
    #[inline(always)]
    pub fn thlelt(&self) -> ThleltR {
        ThleltR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TX History List Interrupt Flag
    #[inline(always)]
    pub fn thlif(&self) -> ThlifR {
        ThlifR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:13 - TX History List Message Count
    #[inline(always)]
    pub fn thlmc(&self) -> ThlmcR {
        ThlmcR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTHLSTS")
            .field("thlemp", &self.thlemp())
            .field("thlfll", &self.thlfll())
            .field("thlelt", &self.thlelt())
            .field("thlif", &self.thlif())
            .field("thlmc", &self.thlmc())
            .finish()
    }
}
impl W {
    ///Bit 2 - TX History List Entry Lost
    #[inline(always)]
    pub fn thlelt(&mut self) -> ThleltW<CfdthlstsSpec> {
        ThleltW::new(self, 2)
    }
    ///Bit 3 - TX History List Interrupt Flag
    #[inline(always)]
    pub fn thlif(&mut self) -> ThlifW<CfdthlstsSpec> {
        ThlifW::new(self, 3)
    }
}
/**TX History List Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdthlsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdthlsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdthlstsSpec;
impl crate::RegisterSpec for CfdthlstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdthlsts::R`](R) reader structure
impl crate::Readable for CfdthlstsSpec {}
///`write(|w| ..)` method takes [`cfdthlsts::W`](W) writer structure
impl crate::Writable for CfdthlstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTHLSTS%s to value 0x01
impl crate::Resettable for CfdthlstsSpec {
    const RESET_VALUE: u32 = 0x01;
}
