///Register `SMR_SMCI` reader
pub type R = crate::R<SmrSmciSpec>;
///Register `SMR_SMCI` writer
pub type W = crate::W<SmrSmciSpec>;
/**Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cks {
    ///0: PCLK clock (n = 0)
    _00 = 0,
    ///1: PCLK/4 clock (n = 1)
    _01 = 1,
    ///2: PCLK/16 clock (n = 2)
    _10 = 2,
    ///3: PCLK/64 clock (n = 3)
    _11 = 3,
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cks {
    type Ux = u8;
}
impl crate::IsEnum for Cks {}
///Field `CKS` reader - Clock Select
pub type CksR = crate::FieldReader<Cks>;
impl CksR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            0 => Cks::_00,
            1 => Cks::_01,
            2 => Cks::_10,
            3 => Cks::_11,
            _ => unreachable!(),
        }
    }
    ///PCLK clock (n = 0)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cks::_00
    }
    ///PCLK/4 clock (n = 1)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cks::_01
    }
    ///PCLK/16 clock (n = 2)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cks::_10
    }
    ///PCLK/64 clock (n = 3)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cks::_11
    }
}
///Field `CKS` writer - Clock Select
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cks, crate::Safe>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK clock (n = 0)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_00)
    }
    ///PCLK/4 clock (n = 1)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_01)
    }
    ///PCLK/16 clock (n = 2)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_10)
    }
    ///PCLK/64 clock (n = 3)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_11)
    }
}
///Field `BCP` reader - Base Clock Pulse
pub type BcpR = crate::FieldReader;
///Field `BCP` writer - Base Clock Pulse
pub type BcpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Parity Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pm {
    ///0: Even parity
    _0 = 0,
    ///1: Odd parity
    _1 = 1,
}
impl From<Pm> for bool {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as u8 != 0
    }
}
///Field `PM` reader - Parity Mode
pub type PmR = crate::BitReader<Pm>;
impl PmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pm {
        match self.bits {
            false => Pm::_0,
            true => Pm::_1,
        }
    }
    ///Even parity
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pm::_0
    }
    ///Odd parity
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pm::_1
    }
}
///Field `PM` writer - Parity Mode
pub type PmW<'a, REG> = crate::BitWriter<'a, REG, Pm>;
impl<'a, REG> PmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Even parity
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_0)
    }
    ///Odd parity
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_1)
    }
}
///Field `PE` reader - Parity Enable
pub type PeR = crate::BitReader;
///Field `PE` writer - Parity Enable
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
/**Block Transfer Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blk {
    ///0: Normal mode operation
    _0 = 0,
    ///1: Block transfer mode operation
    _1 = 1,
}
impl From<Blk> for bool {
    #[inline(always)]
    fn from(variant: Blk) -> Self {
        variant as u8 != 0
    }
}
///Field `BLK` reader - Block Transfer Mode
pub type BlkR = crate::BitReader<Blk>;
impl BlkR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Blk {
        match self.bits {
            false => Blk::_0,
            true => Blk::_1,
        }
    }
    ///Normal mode operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Blk::_0
    }
    ///Block transfer mode operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Blk::_1
    }
}
///Field `BLK` writer - Block Transfer Mode
pub type BlkW<'a, REG> = crate::BitWriter<'a, REG, Blk>;
impl<'a, REG> BlkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Blk::_0)
    }
    ///Block transfer mode operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Blk::_1)
    }
}
/**GSM Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gm {
    ///0: Normal mode operation
    _0 = 0,
    ///1: GSM mode operation
    _1 = 1,
}
impl From<Gm> for bool {
    #[inline(always)]
    fn from(variant: Gm) -> Self {
        variant as u8 != 0
    }
}
///Field `GM` reader - GSM Mode
pub type GmR = crate::BitReader<Gm>;
impl GmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gm {
        match self.bits {
            false => Gm::_0,
            true => Gm::_1,
        }
    }
    ///Normal mode operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gm::_0
    }
    ///GSM mode operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gm::_1
    }
}
///Field `GM` writer - GSM Mode
pub type GmW<'a, REG> = crate::BitWriter<'a, REG, Gm>;
impl<'a, REG> GmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gm::_0)
    }
    ///GSM mode operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gm::_1)
    }
}
impl R {
    ///Bits 0:1 - Clock Select
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(self.bits & 3)
    }
    ///Bits 2:3 - Base Clock Pulse
    #[inline(always)]
    pub fn bcp(&self) -> BcpR {
        BcpR::new((self.bits >> 2) & 3)
    }
    ///Bit 4 - Parity Mode
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Parity Enable
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Block Transfer Mode
    #[inline(always)]
    pub fn blk(&self) -> BlkR {
        BlkR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GSM Mode
    #[inline(always)]
    pub fn gm(&self) -> GmR {
        GmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMR_SMCI")
            .field("cks", &self.cks())
            .field("bcp", &self.bcp())
            .field("pm", &self.pm())
            .field("pe", &self.pe())
            .field("blk", &self.blk())
            .field("gm", &self.gm())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Clock Select
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<SmrSmciSpec> {
        CksW::new(self, 0)
    }
    ///Bits 2:3 - Base Clock Pulse
    #[inline(always)]
    pub fn bcp(&mut self) -> BcpW<SmrSmciSpec> {
        BcpW::new(self, 2)
    }
    ///Bit 4 - Parity Mode
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<SmrSmciSpec> {
        PmW::new(self, 4)
    }
    ///Bit 5 - Parity Enable
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<SmrSmciSpec> {
        PeW::new(self, 5)
    }
    ///Bit 6 - Block Transfer Mode
    #[inline(always)]
    pub fn blk(&mut self) -> BlkW<SmrSmciSpec> {
        BlkW::new(self, 6)
    }
    ///Bit 7 - GSM Mode
    #[inline(always)]
    pub fn gm(&mut self) -> GmW<SmrSmciSpec> {
        GmW::new(self, 7)
    }
}
/**Serial Mode Register for Smart Card Interface Mode (SCMR.SMIF = 1)

You can [`read`](crate::Reg::read) this register and get [`smr_smci::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr_smci::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SmrSmciSpec;
impl crate::RegisterSpec for SmrSmciSpec {
    type Ux = u8;
}
///`read()` method returns [`smr_smci::R`](R) reader structure
impl crate::Readable for SmrSmciSpec {}
///`write(|w| ..)` method takes [`smr_smci::W`](W) writer structure
impl crate::Writable for SmrSmciSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMR_SMCI to value 0
impl crate::Resettable for SmrSmciSpec {}
