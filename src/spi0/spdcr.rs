///Register `SPDCR` reader
pub type R = crate::R<SpdcrSpec>;
///Register `SPDCR` writer
pub type W = crate::W<SpdcrSpec>;
/**Number of Frames Specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spfc {
    ///0: 1 frame
    _00 = 0,
    ///1: 2 frames
    _01 = 1,
    ///2: 3 frames
    _10 = 2,
    ///3: 4 frames
    _11 = 3,
}
impl From<Spfc> for u8 {
    #[inline(always)]
    fn from(variant: Spfc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spfc {
    type Ux = u8;
}
impl crate::IsEnum for Spfc {}
///Field `SPFC` reader - Number of Frames Specification
pub type SpfcR = crate::FieldReader<Spfc>;
impl SpfcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spfc {
        match self.bits {
            0 => Spfc::_00,
            1 => Spfc::_01,
            2 => Spfc::_10,
            3 => Spfc::_11,
            _ => unreachable!(),
        }
    }
    ///1 frame
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Spfc::_00
    }
    ///2 frames
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Spfc::_01
    }
    ///3 frames
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Spfc::_10
    }
    ///4 frames
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Spfc::_11
    }
}
///Field `SPFC` writer - Number of Frames Specification
pub type SpfcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Spfc, crate::Safe>;
impl<'a, REG> SpfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 frame
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Spfc::_00)
    }
    ///2 frames
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Spfc::_01)
    }
    ///3 frames
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Spfc::_10)
    }
    ///4 frames
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Spfc::_11)
    }
}
/**SPI Receive/Transmit Data Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprdtd {
    ///0: Read SPDR/SPDR_HA values from receive buffer
    _0 = 0,
    ///1: Read SPDR/SPDR_HA values from transmit buffer, but only if the transmit buffer is empty
    _1 = 1,
}
impl From<Sprdtd> for bool {
    #[inline(always)]
    fn from(variant: Sprdtd) -> Self {
        variant as u8 != 0
    }
}
///Field `SPRDTD` reader - SPI Receive/Transmit Data Select
pub type SprdtdR = crate::BitReader<Sprdtd>;
impl SprdtdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sprdtd {
        match self.bits {
            false => Sprdtd::_0,
            true => Sprdtd::_1,
        }
    }
    ///Read SPDR/SPDR_HA values from receive buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sprdtd::_0
    }
    ///Read SPDR/SPDR_HA values from transmit buffer, but only if the transmit buffer is empty
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sprdtd::_1
    }
}
///Field `SPRDTD` writer - SPI Receive/Transmit Data Select
pub type SprdtdW<'a, REG> = crate::BitWriter<'a, REG, Sprdtd>;
impl<'a, REG> SprdtdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read SPDR/SPDR_HA values from receive buffer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sprdtd::_0)
    }
    ///Read SPDR/SPDR_HA values from transmit buffer, but only if the transmit buffer is empty
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sprdtd::_1)
    }
}
/**SPI Word Access/Halfword Access Specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Splw {
    ///0: Set SPDR_HA to valid for halfword access
    _0 = 0,
    ///1: Set SPDR to valid for word access
    _1 = 1,
}
impl From<Splw> for bool {
    #[inline(always)]
    fn from(variant: Splw) -> Self {
        variant as u8 != 0
    }
}
///Field `SPLW` reader - SPI Word Access/Halfword Access Specification
pub type SplwR = crate::BitReader<Splw>;
impl SplwR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Splw {
        match self.bits {
            false => Splw::_0,
            true => Splw::_1,
        }
    }
    ///Set SPDR_HA to valid for halfword access
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Splw::_0
    }
    ///Set SPDR to valid for word access
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Splw::_1
    }
}
///Field `SPLW` writer - SPI Word Access/Halfword Access Specification
pub type SplwW<'a, REG> = crate::BitWriter<'a, REG, Splw>;
impl<'a, REG> SplwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set SPDR_HA to valid for halfword access
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Splw::_0)
    }
    ///Set SPDR to valid for word access
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Splw::_1)
    }
}
/**SPI Byte Access Specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spbyt {
    ///0: SPDR/SPDR_HA is accessed in halfword or word (SPLW is valid)
    _0 = 0,
    ///1: SPDR_BY is accessed in byte (SPLW is invalid)
    _1 = 1,
}
impl From<Spbyt> for bool {
    #[inline(always)]
    fn from(variant: Spbyt) -> Self {
        variant as u8 != 0
    }
}
///Field `SPBYT` reader - SPI Byte Access Specification
pub type SpbytR = crate::BitReader<Spbyt>;
impl SpbytR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spbyt {
        match self.bits {
            false => Spbyt::_0,
            true => Spbyt::_1,
        }
    }
    ///SPDR/SPDR_HA is accessed in halfword or word (SPLW is valid)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spbyt::_0
    }
    ///SPDR_BY is accessed in byte (SPLW is invalid)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spbyt::_1
    }
}
///Field `SPBYT` writer - SPI Byte Access Specification
pub type SpbytW<'a, REG> = crate::BitWriter<'a, REG, Spbyt>;
impl<'a, REG> SpbytW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPDR/SPDR_HA is accessed in halfword or word (SPLW is valid)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spbyt::_0)
    }
    ///SPDR_BY is accessed in byte (SPLW is invalid)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spbyt::_1)
    }
}
impl R {
    ///Bits 0:1 - Number of Frames Specification
    #[inline(always)]
    pub fn spfc(&self) -> SpfcR {
        SpfcR::new(self.bits & 3)
    }
    ///Bit 4 - SPI Receive/Transmit Data Select
    #[inline(always)]
    pub fn sprdtd(&self) -> SprdtdR {
        SprdtdR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SPI Word Access/Halfword Access Specification
    #[inline(always)]
    pub fn splw(&self) -> SplwR {
        SplwR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SPI Byte Access Specification
    #[inline(always)]
    pub fn spbyt(&self) -> SpbytR {
        SpbytR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPDCR")
            .field("spfc", &self.spfc())
            .field("sprdtd", &self.sprdtd())
            .field("splw", &self.splw())
            .field("spbyt", &self.spbyt())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Number of Frames Specification
    #[inline(always)]
    pub fn spfc(&mut self) -> SpfcW<SpdcrSpec> {
        SpfcW::new(self, 0)
    }
    ///Bit 4 - SPI Receive/Transmit Data Select
    #[inline(always)]
    pub fn sprdtd(&mut self) -> SprdtdW<SpdcrSpec> {
        SprdtdW::new(self, 4)
    }
    ///Bit 5 - SPI Word Access/Halfword Access Specification
    #[inline(always)]
    pub fn splw(&mut self) -> SplwW<SpdcrSpec> {
        SplwW::new(self, 5)
    }
    ///Bit 6 - SPI Byte Access Specification
    #[inline(always)]
    pub fn spbyt(&mut self) -> SpbytW<SpdcrSpec> {
        SpbytW::new(self, 6)
    }
}
/**SPI Data Control Register

You can [`read`](crate::Reg::read) this register and get [`spdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpdcrSpec;
impl crate::RegisterSpec for SpdcrSpec {
    type Ux = u8;
}
///`read()` method returns [`spdcr::R`](R) reader structure
impl crate::Readable for SpdcrSpec {}
///`write(|w| ..)` method takes [`spdcr::W`](W) writer structure
impl crate::Writable for SpdcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPDCR to value 0
impl crate::Resettable for SpdcrSpec {}
