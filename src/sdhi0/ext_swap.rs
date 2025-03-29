///Register `EXT_SWAP` reader
pub type R = crate::R<ExtSwapSpec>;
///Register `EXT_SWAP` writer
pub type W = crate::W<ExtSwapSpec>;
/**SD_BUF0 Swap Write

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwswp {
    ///0: Normal write operation
    _0 = 0,
    ///1: Swap the byte endian order before writing to SD_BUF0 register
    _1 = 1,
}
impl From<Bwswp> for bool {
    #[inline(always)]
    fn from(variant: Bwswp) -> Self {
        variant as u8 != 0
    }
}
///Field `BWSWP` reader - SD_BUF0 Swap Write
pub type BwswpR = crate::BitReader<Bwswp>;
impl BwswpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bwswp {
        match self.bits {
            false => Bwswp::_0,
            true => Bwswp::_1,
        }
    }
    ///Normal write operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bwswp::_0
    }
    ///Swap the byte endian order before writing to SD_BUF0 register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bwswp::_1
    }
}
///Field `BWSWP` writer - SD_BUF0 Swap Write
pub type BwswpW<'a, REG> = crate::BitWriter<'a, REG, Bwswp>;
impl<'a, REG> BwswpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal write operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bwswp::_0)
    }
    ///Swap the byte endian order before writing to SD_BUF0 register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwswp::_1)
    }
}
/**SD_BUF0 Swap Read

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brswp {
    ///0: Normal read operation
    _0 = 0,
    ///1: Swap the byte endian order before reading SD_BUF0 register
    _1 = 1,
}
impl From<Brswp> for bool {
    #[inline(always)]
    fn from(variant: Brswp) -> Self {
        variant as u8 != 0
    }
}
///Field `BRSWP` reader - SD_BUF0 Swap Read
pub type BrswpR = crate::BitReader<Brswp>;
impl BrswpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Brswp {
        match self.bits {
            false => Brswp::_0,
            true => Brswp::_1,
        }
    }
    ///Normal read operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Brswp::_0
    }
    ///Swap the byte endian order before reading SD_BUF0 register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Brswp::_1
    }
}
///Field `BRSWP` writer - SD_BUF0 Swap Read
pub type BrswpW<'a, REG> = crate::BitWriter<'a, REG, Brswp>;
impl<'a, REG> BrswpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal read operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Brswp::_0)
    }
    ///Swap the byte endian order before reading SD_BUF0 register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Brswp::_1)
    }
}
impl R {
    ///Bit 6 - SD_BUF0 Swap Write
    #[inline(always)]
    pub fn bwswp(&self) -> BwswpR {
        BwswpR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SD_BUF0 Swap Read
    #[inline(always)]
    pub fn brswp(&self) -> BrswpR {
        BrswpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_SWAP")
            .field("bwswp", &self.bwswp())
            .field("brswp", &self.brswp())
            .finish()
    }
}
impl W {
    ///Bit 6 - SD_BUF0 Swap Write
    #[inline(always)]
    pub fn bwswp(&mut self) -> BwswpW<ExtSwapSpec> {
        BwswpW::new(self, 6)
    }
    ///Bit 7 - SD_BUF0 Swap Read
    #[inline(always)]
    pub fn brswp(&mut self) -> BrswpW<ExtSwapSpec> {
        BrswpW::new(self, 7)
    }
}
/**Swap Control Register

You can [`read`](crate::Reg::read) this register and get [`ext_swap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_swap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ExtSwapSpec;
impl crate::RegisterSpec for ExtSwapSpec {
    type Ux = u32;
}
///`read()` method returns [`ext_swap::R`](R) reader structure
impl crate::Readable for ExtSwapSpec {}
///`write(|w| ..)` method takes [`ext_swap::W`](W) writer structure
impl crate::Writable for ExtSwapSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXT_SWAP to value 0
impl crate::Resettable for ExtSwapSpec {}
