///Register `CECCTL0` reader
pub type R = crate::R<Cecctl0Spec>;
///Register `CECCTL0` writer
pub type W = crate::W<Cecctl0Spec>;
/**EOM Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eom {
    ///0: Continues transmission.
    _0 = 0,
    ///1: Last frame
    _1 = 1,
}
impl From<Eom> for bool {
    #[inline(always)]
    fn from(variant: Eom) -> Self {
        variant as u8 != 0
    }
}
///Field `EOM` reader - EOM Setting
pub type EomR = crate::BitReader<Eom>;
impl EomR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eom {
        match self.bits {
            false => Eom::_0,
            true => Eom::_1,
        }
    }
    ///Continues transmission.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eom::_0
    }
    ///Last frame
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eom::_1
    }
}
///Field `EOM` writer - EOM Setting
pub type EomW<'a, REG> = crate::BitWriter<'a, REG, Eom>;
impl<'a, REG> EomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Continues transmission.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eom::_0)
    }
    ///Last frame
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eom::_1)
    }
}
/**Reception Enable Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cecrxen {
    ///0: Disables continuing reception or reports abnormal reception.
    _0 = 0,
    ///1: Enables continuing reception or reports normal reception. lists the reception status and ACK/NACK timing output.
    _1 = 1,
}
impl From<Cecrxen> for bool {
    #[inline(always)]
    fn from(variant: Cecrxen) -> Self {
        variant as u8 != 0
    }
}
///Field `CECRXEN` reader - Reception Enable Control
pub type CecrxenR = crate::BitReader<Cecrxen>;
impl CecrxenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cecrxen {
        match self.bits {
            false => Cecrxen::_0,
            true => Cecrxen::_1,
        }
    }
    ///Disables continuing reception or reports abnormal reception.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cecrxen::_0
    }
    ///Enables continuing reception or reports normal reception. lists the reception status and ACK/NACK timing output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cecrxen::_1
    }
}
///Field `CECRXEN` writer - Reception Enable Control
pub type CecrxenW<'a, REG> = crate::BitWriter<'a, REG, Cecrxen>;
impl<'a, REG> CecrxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables continuing reception or reports abnormal reception.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cecrxen::_0)
    }
    ///Enables continuing reception or reports normal reception. lists the reception status and ACK/NACK timing output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cecrxen::_1)
    }
}
/**Transmission Start Trigger

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txtrg {
    ///0: No effect
    _0 = 0,
    ///1: Starts CEC transmission.
    _1 = 1,
}
impl From<Txtrg> for bool {
    #[inline(always)]
    fn from(variant: Txtrg) -> Self {
        variant as u8 != 0
    }
}
///Field `TXTRG` writer - Transmission Start Trigger
pub type TxtrgW<'a, REG> = crate::BitWriter<'a, REG, Txtrg>;
impl<'a, REG> TxtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrg::_0)
    }
    ///Starts CEC transmission.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrg::_1)
    }
}
/**CEC Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccl {
    ///0: PCLKB/25
    _000 = 0,
    ///1: PCLKB/26
    _001 = 1,
    ///2: PCLKB/27
    _010 = 2,
    ///3: PCLKB/28
    _011 = 3,
    ///4: PCLKB/29
    _100 = 4,
    ///5: PCLKB/210
    _101 = 5,
    ///6: CECCLK (when using SOSC)
    _110 = 6,
    ///7: CECCLK/28 (when using MOSC)
    _111 = 7,
}
impl From<Ccl> for u8 {
    #[inline(always)]
    fn from(variant: Ccl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccl {
    type Ux = u8;
}
impl crate::IsEnum for Ccl {}
///Field `CCL` reader - CEC Clock Select
pub type CclR = crate::FieldReader<Ccl>;
impl CclR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ccl {
        match self.bits {
            0 => Ccl::_000,
            1 => Ccl::_001,
            2 => Ccl::_010,
            3 => Ccl::_011,
            4 => Ccl::_100,
            5 => Ccl::_101,
            6 => Ccl::_110,
            7 => Ccl::_111,
            _ => unreachable!(),
        }
    }
    ///PCLKB/25
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ccl::_000
    }
    ///PCLKB/26
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ccl::_001
    }
    ///PCLKB/27
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ccl::_010
    }
    ///PCLKB/28
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ccl::_011
    }
    ///PCLKB/29
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ccl::_100
    }
    ///PCLKB/210
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ccl::_101
    }
    ///CECCLK (when using SOSC)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Ccl::_110
    }
    ///CECCLK/28 (when using MOSC)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Ccl::_111
    }
}
///Field `CCL` writer - CEC Clock Select
pub type CclW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccl, crate::Safe>;
impl<'a, REG> CclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKB/25
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ccl::_000)
    }
    ///PCLKB/26
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ccl::_001)
    }
    ///PCLKB/27
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ccl::_010)
    }
    ///PCLKB/28
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ccl::_011)
    }
    ///PCLKB/29
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ccl::_100)
    }
    ///PCLKB/210
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Ccl::_101)
    }
    ///CECCLK (when using SOSC)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Ccl::_110)
    }
    ///CECCLK/28 (when using MOSC)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Ccl::_111)
    }
}
/**ACK Bit Timing Error (Bit Width) Check Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackten {
    ///0: Does not detect ACK bit timing errors.
    _0 = 0,
    ///1: Detects ACK bit timing errors.
    _1 = 1,
}
impl From<Ackten> for bool {
    #[inline(always)]
    fn from(variant: Ackten) -> Self {
        variant as u8 != 0
    }
}
///Field `ACKTEN` reader - ACK Bit Timing Error (Bit Width) Check Enable
pub type AcktenR = crate::BitReader<Ackten>;
impl AcktenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ackten {
        match self.bits {
            false => Ackten::_0,
            true => Ackten::_1,
        }
    }
    ///Does not detect ACK bit timing errors.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ackten::_0
    }
    ///Detects ACK bit timing errors.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ackten::_1
    }
}
///Field `ACKTEN` writer - ACK Bit Timing Error (Bit Width) Check Enable
pub type AcktenW<'a, REG> = crate::BitWriter<'a, REG, Ackten>;
impl<'a, REG> AcktenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not detect ACK bit timing errors.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ackten::_0)
    }
    ///Detects ACK bit timing errors.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ackten::_1)
    }
}
/**CEC Operation Enable Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cece {
    ///0: Disables CEC operation.
    _0 = 0,
    ///1: Enables CEC operation.
    _1 = 1,
}
impl From<Cece> for bool {
    #[inline(always)]
    fn from(variant: Cece) -> Self {
        variant as u8 != 0
    }
}
///Field `CECE` reader - CEC Operation Enable Flag
pub type CeceR = crate::BitReader<Cece>;
impl CeceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cece {
        match self.bits {
            false => Cece::_0,
            true => Cece::_1,
        }
    }
    ///Disables CEC operation.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cece::_0
    }
    ///Enables CEC operation.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cece::_1
    }
}
///Field `CECE` writer - CEC Operation Enable Flag
pub type CeceW<'a, REG> = crate::BitWriter<'a, REG, Cece>;
impl<'a, REG> CeceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables CEC operation.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cece::_0)
    }
    ///Enables CEC operation.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cece::_1)
    }
}
impl R {
    ///Bit 0 - EOM Setting
    #[inline(always)]
    pub fn eom(&self) -> EomR {
        EomR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Reception Enable Control
    #[inline(always)]
    pub fn cecrxen(&self) -> CecrxenR {
        CecrxenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 3:5 - CEC Clock Select
    #[inline(always)]
    pub fn ccl(&self) -> CclR {
        CclR::new((self.bits >> 3) & 7)
    }
    ///Bit 6 - ACK Bit Timing Error (Bit Width) Check Enable
    #[inline(always)]
    pub fn ackten(&self) -> AcktenR {
        AcktenR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CEC Operation Enable Flag
    #[inline(always)]
    pub fn cece(&self) -> CeceR {
        CeceR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CECCTL0")
            .field("eom", &self.eom())
            .field("cecrxen", &self.cecrxen())
            .field("ccl", &self.ccl())
            .field("ackten", &self.ackten())
            .field("cece", &self.cece())
            .finish()
    }
}
impl W {
    ///Bit 0 - EOM Setting
    #[inline(always)]
    pub fn eom(&mut self) -> EomW<Cecctl0Spec> {
        EomW::new(self, 0)
    }
    ///Bit 1 - Reception Enable Control
    #[inline(always)]
    pub fn cecrxen(&mut self) -> CecrxenW<Cecctl0Spec> {
        CecrxenW::new(self, 1)
    }
    ///Bit 2 - Transmission Start Trigger
    #[inline(always)]
    pub fn txtrg(&mut self) -> TxtrgW<Cecctl0Spec> {
        TxtrgW::new(self, 2)
    }
    ///Bits 3:5 - CEC Clock Select
    #[inline(always)]
    pub fn ccl(&mut self) -> CclW<Cecctl0Spec> {
        CclW::new(self, 3)
    }
    ///Bit 6 - ACK Bit Timing Error (Bit Width) Check Enable
    #[inline(always)]
    pub fn ackten(&mut self) -> AcktenW<Cecctl0Spec> {
        AcktenW::new(self, 6)
    }
    ///Bit 7 - CEC Operation Enable Flag
    #[inline(always)]
    pub fn cece(&mut self) -> CeceW<Cecctl0Spec> {
        CeceW::new(self, 7)
    }
}
/**CEC Control Register 0

You can [`read`](crate::Reg::read) this register and get [`cecctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cecctl0Spec;
impl crate::RegisterSpec for Cecctl0Spec {
    type Ux = u8;
}
///`read()` method returns [`cecctl0::R`](R) reader structure
impl crate::Readable for Cecctl0Spec {}
///`write(|w| ..)` method takes [`cecctl0::W`](W) writer structure
impl crate::Writable for Cecctl0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CECCTL0 to value 0
impl crate::Resettable for Cecctl0Spec {}
