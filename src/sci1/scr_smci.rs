///Register `SCR_SMCI` reader
pub type R = crate::R<ScrSmciSpec>;
///Register `SCR_SMCI` writer
pub type W = crate::W<ScrSmciSpec>;
/**Clock Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cke {
    ///0: When SMR_SMCI.GM = 0: Disable output The SCKn pin is available for use as an I/O port if set up in the I/O port settings When SMR_SMCI.GM = 1: Fix output low
    _00 = 0,
    ///1: When SMR_SMCI.GM = 0: Output clock When SMR_SMCI.GM = 1: Output clock
    _01 = 1,
    ///2: When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Fix output high
    _10 = 2,
    ///3: When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Output clock
    _11 = 3,
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
            2 => Cke::_10,
            3 => Cke::_11,
            _ => unreachable!(),
        }
    }
    ///When SMR_SMCI.GM = 0: Disable output The SCKn pin is available for use as an I/O port if set up in the I/O port settings When SMR_SMCI.GM = 1: Fix output low
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cke::_00
    }
    ///When SMR_SMCI.GM = 0: Output clock When SMR_SMCI.GM = 1: Output clock
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cke::_01
    }
    ///When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Fix output high
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cke::_10
    }
    ///When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Output clock
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cke::_11
    }
}
///Field `CKE` writer - Clock Enable
pub type CkeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cke, crate::Safe>;
impl<'a, REG> CkeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///When SMR_SMCI.GM = 0: Disable output The SCKn pin is available for use as an I/O port if set up in the I/O port settings When SMR_SMCI.GM = 1: Fix output low
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cke::_00)
    }
    ///When SMR_SMCI.GM = 0: Output clock When SMR_SMCI.GM = 1: Output clock
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cke::_01)
    }
    ///When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Fix output high
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cke::_10)
    }
    ///When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Output clock
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cke::_11)
    }
}
///Field `TEIE` reader - Transmit End Interrupt Enable
pub type TeieR = crate::BitReader;
///Field `TEIE` writer - Transmit End Interrupt Enable
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPIE` reader - Multi-Processor Interrupt Enable
pub type MpieR = crate::BitReader;
///Field `MPIE` writer - Multi-Processor Interrupt Enable
pub type MpieW<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("SCR_SMCI")
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
    pub fn cke(&mut self) -> CkeW<ScrSmciSpec> {
        CkeW::new(self, 0)
    }
    ///Bit 2 - Transmit End Interrupt Enable
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<ScrSmciSpec> {
        TeieW::new(self, 2)
    }
    ///Bit 3 - Multi-Processor Interrupt Enable
    #[inline(always)]
    pub fn mpie(&mut self) -> MpieW<ScrSmciSpec> {
        MpieW::new(self, 3)
    }
    ///Bit 4 - Receive Enable
    #[inline(always)]
    pub fn re(&mut self) -> ReW<ScrSmciSpec> {
        ReW::new(self, 4)
    }
    ///Bit 5 - Transmit Enable
    #[inline(always)]
    pub fn te(&mut self) -> TeW<ScrSmciSpec> {
        TeW::new(self, 5)
    }
    ///Bit 6 - Receive Interrupt Enable
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<ScrSmciSpec> {
        RieW::new(self, 6)
    }
    ///Bit 7 - Transmit Interrupt Enable
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<ScrSmciSpec> {
        TieW::new(self, 7)
    }
}
/**Serial Control Register for Smart Card Interface Mode (SCMR.SMIF = 1)

You can [`read`](crate::Reg::read) this register and get [`scr_smci::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr_smci::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ScrSmciSpec;
impl crate::RegisterSpec for ScrSmciSpec {
    type Ux = u8;
}
///`read()` method returns [`scr_smci::R`](R) reader structure
impl crate::Readable for ScrSmciSpec {}
///`write(|w| ..)` method takes [`scr_smci::W`](W) writer structure
impl crate::Writable for ScrSmciSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR_SMCI to value 0
impl crate::Resettable for ScrSmciSpec {}
