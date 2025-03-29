///Register `EDMR` reader
pub type R = crate::R<EdmrSpec>;
///Register `EDMR` writer
pub type W = crate::W<EdmrSpec>;
///Field `SWR` reader - Software Reset
pub type SwrR = crate::BitReader;
///Field `SWR` writer - Software Reset
pub type SwrW<'a, REG> = crate::BitWriter<'a, REG>;
/**Transmit/Receive Descriptor Length

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dl {
    ///0: 16 bytes
    _00 = 0,
    ///1: 32 bytes
    _01 = 1,
    ///2: 64 bytes
    _10 = 2,
    ///3: 16 bytes.
    _11 = 3,
}
impl From<Dl> for u8 {
    #[inline(always)]
    fn from(variant: Dl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dl {
    type Ux = u8;
}
impl crate::IsEnum for Dl {}
///Field `DL` reader - Transmit/Receive Descriptor Length
pub type DlR = crate::FieldReader<Dl>;
impl DlR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dl {
        match self.bits {
            0 => Dl::_00,
            1 => Dl::_01,
            2 => Dl::_10,
            3 => Dl::_11,
            _ => unreachable!(),
        }
    }
    ///16 bytes
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dl::_00
    }
    ///32 bytes
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dl::_01
    }
    ///64 bytes
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dl::_10
    }
    ///16 bytes.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dl::_11
    }
}
///Field `DL` writer - Transmit/Receive Descriptor Length
pub type DlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dl, crate::Safe>;
impl<'a, REG> DlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///16 bytes
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dl::_00)
    }
    ///32 bytes
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dl::_01)
    }
    ///64 bytes
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dl::_10)
    }
    ///16 bytes.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dl::_11)
    }
}
/**Big Endian Mode/Little Endian Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum De {
    ///0: Big endian mode
    _0 = 0,
    ///1: Little endian mode.
    _1 = 1,
}
impl From<De> for bool {
    #[inline(always)]
    fn from(variant: De) -> Self {
        variant as u8 != 0
    }
}
///Field `DE` reader - Big Endian Mode/Little Endian Mode
pub type DeR = crate::BitReader<De>;
impl DeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> De {
        match self.bits {
            false => De::_0,
            true => De::_1,
        }
    }
    ///Big endian mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == De::_0
    }
    ///Little endian mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == De::_1
    }
}
///Field `DE` writer - Big Endian Mode/Little Endian Mode
pub type DeW<'a, REG> = crate::BitWriter<'a, REG, De>;
impl<'a, REG> DeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Big endian mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(De::_0)
    }
    ///Little endian mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(De::_1)
    }
}
impl R {
    ///Bit 0 - Software Reset
    #[inline(always)]
    pub fn swr(&self) -> SwrR {
        SwrR::new((self.bits & 1) != 0)
    }
    ///Bits 4:5 - Transmit/Receive Descriptor Length
    #[inline(always)]
    pub fn dl(&self) -> DlR {
        DlR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Big Endian Mode/Little Endian Mode
    #[inline(always)]
    pub fn de(&self) -> DeR {
        DeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDMR")
            .field("swr", &self.swr())
            .field("dl", &self.dl())
            .field("de", &self.de())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software Reset
    #[inline(always)]
    pub fn swr(&mut self) -> SwrW<EdmrSpec> {
        SwrW::new(self, 0)
    }
    ///Bits 4:5 - Transmit/Receive Descriptor Length
    #[inline(always)]
    pub fn dl(&mut self) -> DlW<EdmrSpec> {
        DlW::new(self, 4)
    }
    ///Bit 6 - Big Endian Mode/Little Endian Mode
    #[inline(always)]
    pub fn de(&mut self) -> DeW<EdmrSpec> {
        DeW::new(self, 6)
    }
}
/**EDMAC Mode Register

You can [`read`](crate::Reg::read) this register and get [`edmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EdmrSpec;
impl crate::RegisterSpec for EdmrSpec {
    type Ux = u32;
}
///`read()` method returns [`edmr::R`](R) reader structure
impl crate::Readable for EdmrSpec {}
///`write(|w| ..)` method takes [`edmr::W`](W) writer structure
impl crate::Writable for EdmrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EDMR to value 0
impl crate::Resettable for EdmrSpec {}
