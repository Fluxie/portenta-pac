///Register `OCTACKCR` reader
pub type R = crate::R<OctackcrSpec>;
///Register `OCTACKCR` writer
pub type W = crate::W<OctackcrSpec>;
/**Octal-SPI Clock (OCTACLK) Source Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Octacksel {
    ///0: HOCO
    _000 = 0,
    ///1: MOCO (value after reset)
    _001 = 1,
    ///2: LOCO
    _010 = 2,
    ///3: Main clock oscillator
    _011 = 3,
    ///4: Sub-clock oscillator
    _100 = 4,
    ///5: PLL
    _101 = 5,
    ///6: PLL2
    _110 = 6,
    ///7: Setting prohibited.
    Others = 7,
}
impl From<Octacksel> for u8 {
    #[inline(always)]
    fn from(variant: Octacksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Octacksel {
    type Ux = u8;
}
impl crate::IsEnum for Octacksel {}
///Field `OCTACKSEL` reader - Octal-SPI Clock (OCTACLK) Source Select
pub type OctackselR = crate::FieldReader<Octacksel>;
impl OctackselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Octacksel {
        match self.bits {
            0 => Octacksel::_000,
            1 => Octacksel::_001,
            2 => Octacksel::_010,
            3 => Octacksel::_011,
            4 => Octacksel::_100,
            5 => Octacksel::_101,
            6 => Octacksel::_110,
            7 => Octacksel::Others,
            _ => unreachable!(),
        }
    }
    ///HOCO
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Octacksel::_000
    }
    ///MOCO (value after reset)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Octacksel::_001
    }
    ///LOCO
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Octacksel::_010
    }
    ///Main clock oscillator
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Octacksel::_011
    }
    ///Sub-clock oscillator
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Octacksel::_100
    }
    ///PLL
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Octacksel::_101
    }
    ///PLL2
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Octacksel::_110
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Octacksel::Others
    }
}
///Field `OCTACKSEL` writer - Octal-SPI Clock (OCTACLK) Source Select
pub type OctackselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Octacksel, crate::Safe>;
impl<'a, REG> OctackselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HOCO
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Octacksel::_000)
    }
    ///MOCO (value after reset)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Octacksel::_001)
    }
    ///LOCO
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Octacksel::_010)
    }
    ///Main clock oscillator
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Octacksel::_011)
    }
    ///Sub-clock oscillator
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Octacksel::_100)
    }
    ///PLL
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Octacksel::_101)
    }
    ///PLL2
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Octacksel::_110)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Octacksel::Others)
    }
}
/**Octal-SPI Clock (OCTACLK) Switching Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Octacksreq {
    ///0: No request
    _0 = 0,
    ///1: Request switching.
    _1 = 1,
}
impl From<Octacksreq> for bool {
    #[inline(always)]
    fn from(variant: Octacksreq) -> Self {
        variant as u8 != 0
    }
}
///Field `OCTACKSREQ` reader - Octal-SPI Clock (OCTACLK) Switching Request
pub type OctacksreqR = crate::BitReader<Octacksreq>;
impl OctacksreqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Octacksreq {
        match self.bits {
            false => Octacksreq::_0,
            true => Octacksreq::_1,
        }
    }
    ///No request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Octacksreq::_0
    }
    ///Request switching.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Octacksreq::_1
    }
}
///Field `OCTACKSREQ` writer - Octal-SPI Clock (OCTACLK) Switching Request
pub type OctacksreqW<'a, REG> = crate::BitWriter<'a, REG, Octacksreq>;
impl<'a, REG> OctacksreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Octacksreq::_0)
    }
    ///Request switching.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Octacksreq::_1)
    }
}
/**Octal-SPI Clock (OCTACLK) Switching Ready state flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Octacksrdy {
    ///0: Switching not possible
    _0 = 0,
    ///1: Switching possible.
    _1 = 1,
}
impl From<Octacksrdy> for bool {
    #[inline(always)]
    fn from(variant: Octacksrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `OCTACKSRDY` reader - Octal-SPI Clock (OCTACLK) Switching Ready state flag
pub type OctacksrdyR = crate::BitReader<Octacksrdy>;
impl OctacksrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Octacksrdy {
        match self.bits {
            false => Octacksrdy::_0,
            true => Octacksrdy::_1,
        }
    }
    ///Switching not possible
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Octacksrdy::_0
    }
    ///Switching possible.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Octacksrdy::_1
    }
}
impl R {
    ///Bits 0:2 - Octal-SPI Clock (OCTACLK) Source Select
    #[inline(always)]
    pub fn octacksel(&self) -> OctackselR {
        OctackselR::new(self.bits & 7)
    }
    ///Bit 6 - Octal-SPI Clock (OCTACLK) Switching Request
    #[inline(always)]
    pub fn octacksreq(&self) -> OctacksreqR {
        OctacksreqR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Octal-SPI Clock (OCTACLK) Switching Ready state flag
    #[inline(always)]
    pub fn octacksrdy(&self) -> OctacksrdyR {
        OctacksrdyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCTACKCR")
            .field("octacksel", &self.octacksel())
            .field("octacksreq", &self.octacksreq())
            .field("octacksrdy", &self.octacksrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Octal-SPI Clock (OCTACLK) Source Select
    #[inline(always)]
    pub fn octacksel(&mut self) -> OctackselW<OctackcrSpec> {
        OctackselW::new(self, 0)
    }
    ///Bit 6 - Octal-SPI Clock (OCTACLK) Switching Request
    #[inline(always)]
    pub fn octacksreq(&mut self) -> OctacksreqW<OctackcrSpec> {
        OctacksreqW::new(self, 6)
    }
}
/**Octal-SPI Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`octackcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octackcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OctackcrSpec;
impl crate::RegisterSpec for OctackcrSpec {
    type Ux = u8;
}
///`read()` method returns [`octackcr::R`](R) reader structure
impl crate::Readable for OctackcrSpec {}
///`write(|w| ..)` method takes [`octackcr::W`](W) writer structure
impl crate::Writable for OctackcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OCTACKCR to value 0x01
impl crate::Resettable for OctackcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
