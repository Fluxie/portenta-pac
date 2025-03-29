///Register `CECCTL1` reader
pub type R = crate::R<Cecctl1Spec>;
///Register `CECCTL1` writer
pub type W = crate::W<Cecctl1Spec>;
/**Signal-Free Time Data Bit Width Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sft {
    ///0: 3-data bit width
    _00 = 0,
    ///1: 5-data bit width
    _01 = 1,
    ///2: 7-data bit width
    _10 = 2,
    ///3: Does not detect signal-free time.
    _11 = 3,
}
impl From<Sft> for u8 {
    #[inline(always)]
    fn from(variant: Sft) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sft {
    type Ux = u8;
}
impl crate::IsEnum for Sft {}
///Field `SFT` reader - Signal-Free Time Data Bit Width Select
pub type SftR = crate::FieldReader<Sft>;
impl SftR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sft {
        match self.bits {
            0 => Sft::_00,
            1 => Sft::_01,
            2 => Sft::_10,
            3 => Sft::_11,
            _ => unreachable!(),
        }
    }
    ///3-data bit width
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sft::_00
    }
    ///5-data bit width
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sft::_01
    }
    ///7-data bit width
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sft::_10
    }
    ///Does not detect signal-free time.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sft::_11
    }
}
///Field `SFT` writer - Signal-Free Time Data Bit Width Select
pub type SftW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sft, crate::Safe>;
impl<'a, REG> SftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///3-data bit width
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sft::_00)
    }
    ///5-data bit width
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sft::_01)
    }
    ///7-data bit width
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sft::_10)
    }
    ///Does not detect signal-free time.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sft::_11)
    }
}
/**Communication Complete Interrupt (INTCE) Generation Timing Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cesel {
    ///0: Generates communication complete interrupt once after ACK transmission (reception) of the last frame (EOM = 1) is complete and another time after signal-free time is detected.
    _00 = 0,
    ///1: Generates communication complete interrupt after ACK transmission (reception) of the last frame (EOM = 1) is completed.
    _01 = 1,
    ///2: Generates communication complete interrupt after signal-free time is detected.
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Cesel> for u8 {
    #[inline(always)]
    fn from(variant: Cesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cesel {
    type Ux = u8;
}
impl crate::IsEnum for Cesel {}
///Field `CESEL` reader - Communication Complete Interrupt (INTCE) Generation Timing Select
pub type CeselR = crate::FieldReader<Cesel>;
impl CeselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cesel {
        match self.bits {
            0 => Cesel::_00,
            1 => Cesel::_01,
            2 => Cesel::_10,
            3 => Cesel::_11,
            _ => unreachable!(),
        }
    }
    ///Generates communication complete interrupt once after ACK transmission (reception) of the last frame (EOM = 1) is complete and another time after signal-free time is detected.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cesel::_00
    }
    ///Generates communication complete interrupt after ACK transmission (reception) of the last frame (EOM = 1) is completed.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cesel::_01
    }
    ///Generates communication complete interrupt after signal-free time is detected.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cesel::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cesel::_11
    }
}
///Field `CESEL` writer - Communication Complete Interrupt (INTCE) Generation Timing Select
pub type CeselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cesel, crate::Safe>;
impl<'a, REG> CeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Generates communication complete interrupt once after ACK transmission (reception) of the last frame (EOM = 1) is complete and another time after signal-free time is detected.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cesel::_00)
    }
    ///Generates communication complete interrupt after ACK transmission (reception) of the last frame (EOM = 1) is completed.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cesel::_01)
    }
    ///Generates communication complete interrupt after signal-free time is detected.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cesel::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cesel::_11)
    }
}
/**Start Bit Error Detection Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sterrd {
    ///0: Does not detect timing errors during start bit reception.
    _0 = 0,
    ///1: Detects timing errors during start bit reception.
    _1 = 1,
}
impl From<Sterrd> for bool {
    #[inline(always)]
    fn from(variant: Sterrd) -> Self {
        variant as u8 != 0
    }
}
///Field `STERRD` reader - Start Bit Error Detection Select
pub type SterrdR = crate::BitReader<Sterrd>;
impl SterrdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sterrd {
        match self.bits {
            false => Sterrd::_0,
            true => Sterrd::_1,
        }
    }
    ///Does not detect timing errors during start bit reception.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sterrd::_0
    }
    ///Detects timing errors during start bit reception.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sterrd::_1
    }
}
///Field `STERRD` writer - Start Bit Error Detection Select
pub type SterrdW<'a, REG> = crate::BitWriter<'a, REG, Sterrd>;
impl<'a, REG> SterrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not detect timing errors during start bit reception.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sterrd::_0)
    }
    ///Detects timing errors during start bit reception.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sterrd::_1)
    }
}
/**Bus Lock Detection Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blerrd {
    ///0: Does not detect sticking of receive data to high or low
    _0 = 0,
    ///1: Detects sticking of receive data to high or low.
    _1 = 1,
}
impl From<Blerrd> for bool {
    #[inline(always)]
    fn from(variant: Blerrd) -> Self {
        variant as u8 != 0
    }
}
///Field `BLERRD` reader - Bus Lock Detection Select
pub type BlerrdR = crate::BitReader<Blerrd>;
impl BlerrdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Blerrd {
        match self.bits {
            false => Blerrd::_0,
            true => Blerrd::_1,
        }
    }
    ///Does not detect sticking of receive data to high or low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Blerrd::_0
    }
    ///Detects sticking of receive data to high or low.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Blerrd::_1
    }
}
///Field `BLERRD` writer - Bus Lock Detection Select
pub type BlerrdW<'a, REG> = crate::BitWriter<'a, REG, Blerrd>;
impl<'a, REG> BlerrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not detect sticking of receive data to high or low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Blerrd::_0)
    }
    ///Detects sticking of receive data to high or low.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Blerrd::_1)
    }
}
/**CEC Data Interrupt (INTDA) Generation Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cintmk {
    ///0: Does not generate an interrupt when the addresses do not match.
    _0 = 0,
    ///1: Generates an interrupt when the addresses do not match.
    _1 = 1,
}
impl From<Cintmk> for bool {
    #[inline(always)]
    fn from(variant: Cintmk) -> Self {
        variant as u8 != 0
    }
}
///Field `CINTMK` reader - CEC Data Interrupt (INTDA) Generation Select
pub type CintmkR = crate::BitReader<Cintmk>;
impl CintmkR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cintmk {
        match self.bits {
            false => Cintmk::_0,
            true => Cintmk::_1,
        }
    }
    ///Does not generate an interrupt when the addresses do not match.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cintmk::_0
    }
    ///Generates an interrupt when the addresses do not match.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cintmk::_1
    }
}
///Field `CINTMK` writer - CEC Data Interrupt (INTDA) Generation Select
pub type CintmkW<'a, REG> = crate::BitWriter<'a, REG, Cintmk>;
impl<'a, REG> CintmkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not generate an interrupt when the addresses do not match.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cintmk::_0)
    }
    ///Generates an interrupt when the addresses do not match.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cintmk::_1)
    }
}
/**Digital Filter Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdfc {
    ///0: Does not use a digital filter.
    _0 = 0,
    ///1: Uses a digital filter.
    _1 = 1,
}
impl From<Cdfc> for bool {
    #[inline(always)]
    fn from(variant: Cdfc) -> Self {
        variant as u8 != 0
    }
}
///Field `CDFC` reader - Digital Filter Select
pub type CdfcR = crate::BitReader<Cdfc>;
impl CdfcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cdfc {
        match self.bits {
            false => Cdfc::_0,
            true => Cdfc::_1,
        }
    }
    ///Does not use a digital filter.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cdfc::_0
    }
    ///Uses a digital filter.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cdfc::_1
    }
}
///Field `CDFC` writer - Digital Filter Select
pub type CdfcW<'a, REG> = crate::BitWriter<'a, REG, Cdfc>;
impl<'a, REG> CdfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not use a digital filter.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdfc::_0)
    }
    ///Uses a digital filter.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdfc::_1)
    }
}
impl R {
    ///Bits 0:1 - Signal-Free Time Data Bit Width Select
    #[inline(always)]
    pub fn sft(&self) -> SftR {
        SftR::new(self.bits & 3)
    }
    ///Bits 2:3 - Communication Complete Interrupt (INTCE) Generation Timing Select
    #[inline(always)]
    pub fn cesel(&self) -> CeselR {
        CeselR::new((self.bits >> 2) & 3)
    }
    ///Bit 4 - Start Bit Error Detection Select
    #[inline(always)]
    pub fn sterrd(&self) -> SterrdR {
        SterrdR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bus Lock Detection Select
    #[inline(always)]
    pub fn blerrd(&self) -> BlerrdR {
        BlerrdR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CEC Data Interrupt (INTDA) Generation Select
    #[inline(always)]
    pub fn cintmk(&self) -> CintmkR {
        CintmkR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Digital Filter Select
    #[inline(always)]
    pub fn cdfc(&self) -> CdfcR {
        CdfcR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CECCTL1")
            .field("sft", &self.sft())
            .field("cesel", &self.cesel())
            .field("sterrd", &self.sterrd())
            .field("blerrd", &self.blerrd())
            .field("cintmk", &self.cintmk())
            .field("cdfc", &self.cdfc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Signal-Free Time Data Bit Width Select
    #[inline(always)]
    pub fn sft(&mut self) -> SftW<Cecctl1Spec> {
        SftW::new(self, 0)
    }
    ///Bits 2:3 - Communication Complete Interrupt (INTCE) Generation Timing Select
    #[inline(always)]
    pub fn cesel(&mut self) -> CeselW<Cecctl1Spec> {
        CeselW::new(self, 2)
    }
    ///Bit 4 - Start Bit Error Detection Select
    #[inline(always)]
    pub fn sterrd(&mut self) -> SterrdW<Cecctl1Spec> {
        SterrdW::new(self, 4)
    }
    ///Bit 5 - Bus Lock Detection Select
    #[inline(always)]
    pub fn blerrd(&mut self) -> BlerrdW<Cecctl1Spec> {
        BlerrdW::new(self, 5)
    }
    ///Bit 6 - CEC Data Interrupt (INTDA) Generation Select
    #[inline(always)]
    pub fn cintmk(&mut self) -> CintmkW<Cecctl1Spec> {
        CintmkW::new(self, 6)
    }
    ///Bit 7 - Digital Filter Select
    #[inline(always)]
    pub fn cdfc(&mut self) -> CdfcW<Cecctl1Spec> {
        CdfcW::new(self, 7)
    }
}
/**CEC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cecctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cecctl1Spec;
impl crate::RegisterSpec for Cecctl1Spec {
    type Ux = u8;
}
///`read()` method returns [`cecctl1::R`](R) reader structure
impl crate::Readable for Cecctl1Spec {}
///`write(|w| ..)` method takes [`cecctl1::W`](W) writer structure
impl crate::Writable for Cecctl1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CECCTL1 to value 0
impl crate::Resettable for Cecctl1Spec {}
