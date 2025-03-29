///Register `ICMR3` reader
pub type R = crate::R<Icmr3Spec>;
///Register `ICMR3` writer
pub type W = crate::W<Icmr3Spec>;
/**Noise Filter Stage Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nf {
    ///0: Filter out noise of up to 1 IIC-phi cycle (single-stage filter)
    _00 = 0,
    ///1: Filter out noise of up to 2 IIC-phi cycles (2-stage filter)
    _01 = 1,
    ///2: Filter out noise of up to 3 IIC-phi cycles (3-stage filter)
    _10 = 2,
    ///3: Filter out noise of up to 4 IIC-phi cycles (4-stage filter)
    _11 = 3,
}
impl From<Nf> for u8 {
    #[inline(always)]
    fn from(variant: Nf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nf {
    type Ux = u8;
}
impl crate::IsEnum for Nf {}
///Field `NF` reader - Noise Filter Stage Select
pub type NfR = crate::FieldReader<Nf>;
impl NfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nf {
        match self.bits {
            0 => Nf::_00,
            1 => Nf::_01,
            2 => Nf::_10,
            3 => Nf::_11,
            _ => unreachable!(),
        }
    }
    ///Filter out noise of up to 1 IIC-phi cycle (single-stage filter)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Nf::_00
    }
    ///Filter out noise of up to 2 IIC-phi cycles (2-stage filter)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Nf::_01
    }
    ///Filter out noise of up to 3 IIC-phi cycles (3-stage filter)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Nf::_10
    }
    ///Filter out noise of up to 4 IIC-phi cycles (4-stage filter)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Nf::_11
    }
}
///Field `NF` writer - Noise Filter Stage Select
pub type NfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nf, crate::Safe>;
impl<'a, REG> NfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Filter out noise of up to 1 IIC-phi cycle (single-stage filter)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::_00)
    }
    ///Filter out noise of up to 2 IIC-phi cycles (2-stage filter)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::_01)
    }
    ///Filter out noise of up to 3 IIC-phi cycles (3-stage filter)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::_10)
    }
    ///Filter out noise of up to 4 IIC-phi cycles (4-stage filter)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::_11)
    }
}
/**Receive Acknowledge

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackbr {
    ///0: 0 received as the acknowledge bit (ACK reception)
    _0 = 0,
    ///1: 1 received as the acknowledge bit (NACK reception)
    _1 = 1,
}
impl From<Ackbr> for bool {
    #[inline(always)]
    fn from(variant: Ackbr) -> Self {
        variant as u8 != 0
    }
}
///Field `ACKBR` reader - Receive Acknowledge
pub type AckbrR = crate::BitReader<Ackbr>;
impl AckbrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ackbr {
        match self.bits {
            false => Ackbr::_0,
            true => Ackbr::_1,
        }
    }
    ///0 received as the acknowledge bit (ACK reception)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ackbr::_0
    }
    ///1 received as the acknowledge bit (NACK reception)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ackbr::_1
    }
}
/**Transmit Acknowledge

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackbt {
    ///0: Send 0 as the acknowledge bit (ACK transmission)
    _0 = 0,
    ///1: Send 1 as the acknowledge bit (NACK transmission)
    _1 = 1,
}
impl From<Ackbt> for bool {
    #[inline(always)]
    fn from(variant: Ackbt) -> Self {
        variant as u8 != 0
    }
}
///Field `ACKBT` reader - Transmit Acknowledge
pub type AckbtR = crate::BitReader<Ackbt>;
impl AckbtR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ackbt {
        match self.bits {
            false => Ackbt::_0,
            true => Ackbt::_1,
        }
    }
    ///Send 0 as the acknowledge bit (ACK transmission)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ackbt::_0
    }
    ///Send 1 as the acknowledge bit (NACK transmission)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ackbt::_1
    }
}
///Field `ACKBT` writer - Transmit Acknowledge
pub type AckbtW<'a, REG> = crate::BitWriter<'a, REG, Ackbt>;
impl<'a, REG> AckbtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Send 0 as the acknowledge bit (ACK transmission)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ackbt::_0)
    }
    ///Send 1 as the acknowledge bit (NACK transmission)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ackbt::_1)
    }
}
/**ACKBT Write Protect

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackwp {
    ///0: Write protect ACKBT bit
    _0 = 0,
    ///1: Write enable ACKBT bit
    _1 = 1,
}
impl From<Ackwp> for bool {
    #[inline(always)]
    fn from(variant: Ackwp) -> Self {
        variant as u8 != 0
    }
}
///Field `ACKWP` reader - ACKBT Write Protect
pub type AckwpR = crate::BitReader<Ackwp>;
impl AckwpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ackwp {
        match self.bits {
            false => Ackwp::_0,
            true => Ackwp::_1,
        }
    }
    ///Write protect ACKBT bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ackwp::_0
    }
    ///Write enable ACKBT bit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ackwp::_1
    }
}
///Field `ACKWP` writer - ACKBT Write Protect
pub type AckwpW<'a, REG> = crate::BitWriter<'a, REG, Ackwp>;
impl<'a, REG> AckwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write protect ACKBT bit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ackwp::_0)
    }
    ///Write enable ACKBT bit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ackwp::_1)
    }
}
/**RDRF Flag Set Timing Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdrfs {
    ///0: Set the RDRF flag on the rising edge of the 9th SCL clock cycle. The SCLn line is not held low on the falling edge of the 8th clock cycle.
    _0 = 0,
    ///1: Set the RDRF flag on the rising edge of the 8th SCL clock cycle. The SCLn line is held low on the falling edge of the 8th clock cycle.
    _1 = 1,
}
impl From<Rdrfs> for bool {
    #[inline(always)]
    fn from(variant: Rdrfs) -> Self {
        variant as u8 != 0
    }
}
///Field `RDRFS` reader - RDRF Flag Set Timing Select
pub type RdrfsR = crate::BitReader<Rdrfs>;
impl RdrfsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rdrfs {
        match self.bits {
            false => Rdrfs::_0,
            true => Rdrfs::_1,
        }
    }
    ///Set the RDRF flag on the rising edge of the 9th SCL clock cycle. The SCLn line is not held low on the falling edge of the 8th clock cycle.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdrfs::_0
    }
    ///Set the RDRF flag on the rising edge of the 8th SCL clock cycle. The SCLn line is held low on the falling edge of the 8th clock cycle.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdrfs::_1
    }
}
///Field `RDRFS` writer - RDRF Flag Set Timing Select
pub type RdrfsW<'a, REG> = crate::BitWriter<'a, REG, Rdrfs>;
impl<'a, REG> RdrfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set the RDRF flag on the rising edge of the 9th SCL clock cycle. The SCLn line is not held low on the falling edge of the 8th clock cycle.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrfs::_0)
    }
    ///Set the RDRF flag on the rising edge of the 8th SCL clock cycle. The SCLn line is held low on the falling edge of the 8th clock cycle.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrfs::_1)
    }
}
/**Low-hold is released by reading ICDRR.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wait {
    ///0: No wait (The SCLn line is not held low during the period between the 9th clock cycle and the 1st clock cycle.)
    _0 = 0,
    ///1: Wait (The SCLn line is held low during the period between the 9th clock cycle and the 1st clock cycle.)
    _1 = 1,
}
impl From<Wait> for bool {
    #[inline(always)]
    fn from(variant: Wait) -> Self {
        variant as u8 != 0
    }
}
///Field `WAIT` reader - Low-hold is released by reading ICDRR.
pub type WaitR = crate::BitReader<Wait>;
impl WaitR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wait {
        match self.bits {
            false => Wait::_0,
            true => Wait::_1,
        }
    }
    ///No wait (The SCLn line is not held low during the period between the 9th clock cycle and the 1st clock cycle.)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wait::_0
    }
    ///Wait (The SCLn line is held low during the period between the 9th clock cycle and the 1st clock cycle.)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wait::_1
    }
}
///Field `WAIT` writer - Low-hold is released by reading ICDRR.
pub type WaitW<'a, REG> = crate::BitWriter<'a, REG, Wait>;
impl<'a, REG> WaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No wait (The SCLn line is not held low during the period between the 9th clock cycle and the 1st clock cycle.)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::_0)
    }
    ///Wait (The SCLn line is held low during the period between the 9th clock cycle and the 1st clock cycle.)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::_1)
    }
}
/**SMBus/I2C Bus Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smbs {
    ///0: Select I2C Bus
    _0 = 0,
    ///1: Select SMBus
    _1 = 1,
}
impl From<Smbs> for bool {
    #[inline(always)]
    fn from(variant: Smbs) -> Self {
        variant as u8 != 0
    }
}
///Field `SMBS` reader - SMBus/I2C Bus Select
pub type SmbsR = crate::BitReader<Smbs>;
impl SmbsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Smbs {
        match self.bits {
            false => Smbs::_0,
            true => Smbs::_1,
        }
    }
    ///Select I2C Bus
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Smbs::_0
    }
    ///Select SMBus
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Smbs::_1
    }
}
///Field `SMBS` writer - SMBus/I2C Bus Select
pub type SmbsW<'a, REG> = crate::BitWriter<'a, REG, Smbs>;
impl<'a, REG> SmbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select I2C Bus
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Smbs::_0)
    }
    ///Select SMBus
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Smbs::_1)
    }
}
impl R {
    ///Bits 0:1 - Noise Filter Stage Select
    #[inline(always)]
    pub fn nf(&self) -> NfR {
        NfR::new(self.bits & 3)
    }
    ///Bit 2 - Receive Acknowledge
    #[inline(always)]
    pub fn ackbr(&self) -> AckbrR {
        AckbrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit Acknowledge
    #[inline(always)]
    pub fn ackbt(&self) -> AckbtR {
        AckbtR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ACKBT Write Protect
    #[inline(always)]
    pub fn ackwp(&self) -> AckwpR {
        AckwpR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RDRF Flag Set Timing Select
    #[inline(always)]
    pub fn rdrfs(&self) -> RdrfsR {
        RdrfsR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Low-hold is released by reading ICDRR.
    #[inline(always)]
    pub fn wait(&self) -> WaitR {
        WaitR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SMBus/I2C Bus Select
    #[inline(always)]
    pub fn smbs(&self) -> SmbsR {
        SmbsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICMR3")
            .field("nf", &self.nf())
            .field("ackbr", &self.ackbr())
            .field("ackbt", &self.ackbt())
            .field("ackwp", &self.ackwp())
            .field("rdrfs", &self.rdrfs())
            .field("wait", &self.wait())
            .field("smbs", &self.smbs())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Noise Filter Stage Select
    #[inline(always)]
    pub fn nf(&mut self) -> NfW<Icmr3Spec> {
        NfW::new(self, 0)
    }
    ///Bit 3 - Transmit Acknowledge
    #[inline(always)]
    pub fn ackbt(&mut self) -> AckbtW<Icmr3Spec> {
        AckbtW::new(self, 3)
    }
    ///Bit 4 - ACKBT Write Protect
    #[inline(always)]
    pub fn ackwp(&mut self) -> AckwpW<Icmr3Spec> {
        AckwpW::new(self, 4)
    }
    ///Bit 5 - RDRF Flag Set Timing Select
    #[inline(always)]
    pub fn rdrfs(&mut self) -> RdrfsW<Icmr3Spec> {
        RdrfsW::new(self, 5)
    }
    ///Bit 6 - Low-hold is released by reading ICDRR.
    #[inline(always)]
    pub fn wait(&mut self) -> WaitW<Icmr3Spec> {
        WaitW::new(self, 6)
    }
    ///Bit 7 - SMBus/I2C Bus Select
    #[inline(always)]
    pub fn smbs(&mut self) -> SmbsW<Icmr3Spec> {
        SmbsW::new(self, 7)
    }
}
/**I2C Bus Mode Register 3

You can [`read`](crate::Reg::read) this register and get [`icmr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Icmr3Spec;
impl crate::RegisterSpec for Icmr3Spec {
    type Ux = u8;
}
///`read()` method returns [`icmr3::R`](R) reader structure
impl crate::Readable for Icmr3Spec {}
///`write(|w| ..)` method takes [`icmr3::W`](W) writer structure
impl crate::Writable for Icmr3Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICMR3 to value 0
impl crate::Resettable for Icmr3Spec {}
