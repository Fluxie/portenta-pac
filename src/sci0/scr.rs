///Register `SCR` reader
pub type R = crate::R<ScrSpec>;
///Register `SCR` writer
pub type W = crate::W<ScrSpec>;
/**Clock Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cke {
    ///0: In asynchronous mode, the SCKn pin is available for use as an I/O port based on the I/O port settings. In clock synchronous mode, the SCKn pin functions as the clock output pin.
    _00 = 0,
    ///1: In asynchronous mode, a clock with the same frequency as the bit rate is output from the SCKn pin. In clock synchronous mode, the SCKn pin functions as the clock output pin.
    _01 = 1,
    ///2: In asynchronous mode, input a clock with a frequency 16 times the bit rate from the SCKn pin when the SEMR.ABCS bit is 0. Input a clock signal with a frequency eight times the bit rate when the SEMR.ABCS bit is 1. The SCKn pin is available for use as an I/O port based on the I/O port settings when the GPT clock is used. In clock synchronous mode, the SCKn pin functions as the clock input pin.
    Others = 2,
}
impl From<Cke> for u8 {
    #[inline(always)]
    fn from(variant: Cke) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cke {
    type Ux = u8;
}
impl crate::IsEnum for Cke {}
///Field `CKE` reader - Clock Enable
pub type CkeR = crate::FieldReader<Cke>;
impl CkeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cke {
        match self.bits {
            0 => Cke::_00,
            1 => Cke::_01,
            _ => Cke::Others,
        }
    }
    ///In asynchronous mode, the SCKn pin is available for use as an I/O port based on the I/O port settings. In clock synchronous mode, the SCKn pin functions as the clock output pin.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cke::_00
    }
    ///In asynchronous mode, a clock with the same frequency as the bit rate is output from the SCKn pin. In clock synchronous mode, the SCKn pin functions as the clock output pin.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cke::_01
    }
    ///In asynchronous mode, input a clock with a frequency 16 times the bit rate from the SCKn pin when the SEMR.ABCS bit is 0. Input a clock signal with a frequency eight times the bit rate when the SEMR.ABCS bit is 1. The SCKn pin is available for use as an I/O port based on the I/O port settings when the GPT clock is used. In clock synchronous mode, the SCKn pin functions as the clock input pin.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Cke::Others)
    }
}
///Field `CKE` writer - Clock Enable
pub type CkeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cke, crate::Safe>;
impl<'a, REG> CkeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///In asynchronous mode, the SCKn pin is available for use as an I/O port based on the I/O port settings. In clock synchronous mode, the SCKn pin functions as the clock output pin.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cke::_00)
    }
    ///In asynchronous mode, a clock with the same frequency as the bit rate is output from the SCKn pin. In clock synchronous mode, the SCKn pin functions as the clock output pin.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cke::_01)
    }
    ///In asynchronous mode, input a clock with a frequency 16 times the bit rate from the SCKn pin when the SEMR.ABCS bit is 0. Input a clock signal with a frequency eight times the bit rate when the SEMR.ABCS bit is 1. The SCKn pin is available for use as an I/O port based on the I/O port settings when the GPT clock is used. In clock synchronous mode, the SCKn pin functions as the clock input pin.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cke::Others)
    }
}
/**Transmit End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teie {
    ///0: Disable SCIn_TEI interrupt requests
    _0 = 0,
    ///1: Enable SCIn_TEI interrupt requests
    _1 = 1,
}
impl From<Teie> for bool {
    #[inline(always)]
    fn from(variant: Teie) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIE` reader - Transmit End Interrupt Enable
pub type TeieR = crate::BitReader<Teie>;
impl TeieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Teie {
        match self.bits {
            false => Teie::_0,
            true => Teie::_1,
        }
    }
    ///Disable SCIn_TEI interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Teie::_0
    }
    ///Enable SCIn_TEI interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Teie::_1
    }
}
///Field `TEIE` writer - Transmit End Interrupt Enable
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG, Teie>;
impl<'a, REG> TeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable SCIn_TEI interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::_0)
    }
    ///Enable SCIn_TEI interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::_1)
    }
}
/**Multi-Processor Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpie {
    ///0: Normal reception
    _0 = 0,
    ///1: When data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF, ORER, and FER in SSR to 1 and the status flags SYER, PFER, and SBER in MESR are disabled. When data with the multi-processor bit set to 1 is received, the MPIE bit is automatically set to 0, and normal reception is resumed.
    _1 = 1,
}
impl From<Mpie> for bool {
    #[inline(always)]
    fn from(variant: Mpie) -> Self {
        variant as u8 != 0
    }
}
///Field `MPIE` reader - Multi-Processor Interrupt Enable
pub type MpieR = crate::BitReader<Mpie>;
impl MpieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mpie {
        match self.bits {
            false => Mpie::_0,
            true => Mpie::_1,
        }
    }
    ///Normal reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpie::_0
    }
    ///When data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF, ORER, and FER in SSR to 1 and the status flags SYER, PFER, and SBER in MESR are disabled. When data with the multi-processor bit set to 1 is received, the MPIE bit is automatically set to 0, and normal reception is resumed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mpie::_1
    }
}
///Field `MPIE` writer - Multi-Processor Interrupt Enable
pub type MpieW<'a, REG> = crate::BitWriter<'a, REG, Mpie>;
impl<'a, REG> MpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpie::_0)
    }
    ///When data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF, ORER, and FER in SSR to 1 and the status flags SYER, PFER, and SBER in MESR are disabled. When data with the multi-processor bit set to 1 is received, the MPIE bit is automatically set to 0, and normal reception is resumed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpie::_1)
    }
}
/**Receive Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Re {
    ///0: Disable serial reception
    _0 = 0,
    ///1: Enable serial reception
    _1 = 1,
}
impl From<Re> for bool {
    #[inline(always)]
    fn from(variant: Re) -> Self {
        variant as u8 != 0
    }
}
///Field `RE` reader - Receive Enable
pub type ReR = crate::BitReader<Re>;
impl ReR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Re {
        match self.bits {
            false => Re::_0,
            true => Re::_1,
        }
    }
    ///Disable serial reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Re::_0
    }
    ///Enable serial reception
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Re::_1
    }
}
///Field `RE` writer - Receive Enable
pub type ReW<'a, REG> = crate::BitWriter<'a, REG, Re>;
impl<'a, REG> ReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable serial reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Re::_0)
    }
    ///Enable serial reception
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Re::_1)
    }
}
/**Transmit Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te {
    ///0: Disable serial transmission
    _0 = 0,
    ///1: Enable serial transmission
    _1 = 1,
}
impl From<Te> for bool {
    #[inline(always)]
    fn from(variant: Te) -> Self {
        variant as u8 != 0
    }
}
///Field `TE` reader - Transmit Enable
pub type TeR = crate::BitReader<Te>;
impl TeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Te {
        match self.bits {
            false => Te::_0,
            true => Te::_1,
        }
    }
    ///Disable serial transmission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Te::_0
    }
    ///Enable serial transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Te::_1
    }
}
///Field `TE` writer - Transmit Enable
pub type TeW<'a, REG> = crate::BitWriter<'a, REG, Te>;
impl<'a, REG> TeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable serial transmission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Te::_0)
    }
    ///Enable serial transmission
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Te::_1)
    }
}
/**Receive Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rie {
    ///0: Disable SCIn_RXI and SCIn_ERI interrupt requests
    _0 = 0,
    ///1: Enable SCIn_RXI and SCIn_ERI interrupt requests
    _1 = 1,
}
impl From<Rie> for bool {
    #[inline(always)]
    fn from(variant: Rie) -> Self {
        variant as u8 != 0
    }
}
///Field `RIE` reader - Receive Interrupt Enable
pub type RieR = crate::BitReader<Rie>;
impl RieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rie {
        match self.bits {
            false => Rie::_0,
            true => Rie::_1,
        }
    }
    ///Disable SCIn_RXI and SCIn_ERI interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rie::_0
    }
    ///Enable SCIn_RXI and SCIn_ERI interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rie::_1
    }
}
///Field `RIE` writer - Receive Interrupt Enable
pub type RieW<'a, REG> = crate::BitWriter<'a, REG, Rie>;
impl<'a, REG> RieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable SCIn_RXI and SCIn_ERI interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_0)
    }
    ///Enable SCIn_RXI and SCIn_ERI interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_1)
    }
}
/**Transmit Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    ///0: Disable SCIn_TXI interrupt requests
    _0 = 0,
    ///1: Enable SCIn_TXI interrupt requests
    _1 = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
///Field `TIE` reader - Transmit Interrupt Enable
pub type TieR = crate::BitReader<Tie>;
impl TieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tie {
        match self.bits {
            false => Tie::_0,
            true => Tie::_1,
        }
    }
    ///Disable SCIn_TXI interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tie::_0
    }
    ///Enable SCIn_TXI interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tie::_1
    }
}
///Field `TIE` writer - Transmit Interrupt Enable
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable SCIn_TXI interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_0)
    }
    ///Enable SCIn_TXI interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_1)
    }
}
impl R {
    ///Bits 0:1 - Clock Enable
    #[inline(always)]
    pub fn cke(&self) -> CkeR {
        CkeR::new(self.bits & 3)
    }
    ///Bit 2 - Transmit End Interrupt Enable
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Multi-Processor Interrupt Enable
    #[inline(always)]
    pub fn mpie(&self) -> MpieR {
        MpieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receive Enable
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit Enable
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Receive Interrupt Enable
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit Interrupt Enable
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("cke", &self.cke())
            .field("teie", &self.teie())
            .field("mpie", &self.mpie())
            .field("re", &self.re())
            .field("te", &self.te())
            .field("rie", &self.rie())
            .field("tie", &self.tie())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Clock Enable
    #[inline(always)]
    pub fn cke(&mut self) -> CkeW<ScrSpec> {
        CkeW::new(self, 0)
    }
    ///Bit 2 - Transmit End Interrupt Enable
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<ScrSpec> {
        TeieW::new(self, 2)
    }
    ///Bit 3 - Multi-Processor Interrupt Enable
    #[inline(always)]
    pub fn mpie(&mut self) -> MpieW<ScrSpec> {
        MpieW::new(self, 3)
    }
    ///Bit 4 - Receive Enable
    #[inline(always)]
    pub fn re(&mut self) -> ReW<ScrSpec> {
        ReW::new(self, 4)
    }
    ///Bit 5 - Transmit Enable
    #[inline(always)]
    pub fn te(&mut self) -> TeW<ScrSpec> {
        TeW::new(self, 5)
    }
    ///Bit 6 - Receive Interrupt Enable
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<ScrSpec> {
        RieW::new(self, 6)
    }
    ///Bit 7 - Transmit Interrupt Enable
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<ScrSpec> {
        TieW::new(self, 7)
    }
}
/**Serial Control Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)

You can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u8;
}
///`read()` method returns [`scr::R`](R) reader structure
impl crate::Readable for ScrSpec {}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for ScrSpec {}
