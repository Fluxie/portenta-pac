///Register `ADDISCR` reader
pub type R = crate::R<AddiscrSpec>;
///Register `ADDISCR` writer
pub type W = crate::W<AddiscrSpec>;
/**Disconnection Detection Assist Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adndis {
    ///0: The disconnection detection assist function is disabled
    _0x0 = 0,
    ///1: Setting prohibited
    _0x1 = 1,
    ///2: The number of states for the discharge or precharge period.
    Others = 2,
}
impl From<Adndis> for u8 {
    #[inline(always)]
    fn from(variant: Adndis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adndis {
    type Ux = u8;
}
impl crate::IsEnum for Adndis {}
///Field `ADNDIS` reader - Disconnection Detection Assist Setting
pub type AdndisR = crate::FieldReader<Adndis>;
impl AdndisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adndis {
        match self.bits {
            0 => Adndis::_0x0,
            1 => Adndis::_0x1,
            _ => Adndis::Others,
        }
    }
    ///The disconnection detection assist function is disabled
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Adndis::_0x0
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Adndis::_0x1
    }
    ///The number of states for the discharge or precharge period.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Adndis::Others)
    }
}
///Field `ADNDIS` writer - Disconnection Detection Assist Setting
pub type AdndisW<'a, REG> = crate::FieldWriter<'a, REG, 4, Adndis, crate::Safe>;
impl<'a, REG> AdndisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The disconnection detection assist function is disabled
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adndis::_0x0)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adndis::_0x1)
    }
    ///The number of states for the discharge or precharge period.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Adndis::Others)
    }
}
/**Precharge/discharge select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pchg {
    ///0: Discharge
    _0 = 0,
    ///1: Precharge
    _1 = 1,
}
impl From<Pchg> for bool {
    #[inline(always)]
    fn from(variant: Pchg) -> Self {
        variant as u8 != 0
    }
}
///Field `PCHG` reader - Precharge/discharge select
pub type PchgR = crate::BitReader<Pchg>;
impl PchgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pchg {
        match self.bits {
            false => Pchg::_0,
            true => Pchg::_1,
        }
    }
    ///Discharge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pchg::_0
    }
    ///Precharge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pchg::_1
    }
}
///Field `PCHG` writer - Precharge/discharge select
pub type PchgW<'a, REG> = crate::BitWriter<'a, REG, Pchg>;
impl<'a, REG> PchgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discharge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pchg::_0)
    }
    ///Precharge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pchg::_1)
    }
}
impl R {
    ///Bits 0:3 - Disconnection Detection Assist Setting
    #[inline(always)]
    pub fn adndis(&self) -> AdndisR {
        AdndisR::new(self.bits & 0x0f)
    }
    ///Bit 4 - Precharge/discharge select
    #[inline(always)]
    pub fn pchg(&self) -> PchgR {
        PchgR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDISCR")
            .field("adndis", &self.adndis())
            .field("pchg", &self.pchg())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Disconnection Detection Assist Setting
    #[inline(always)]
    pub fn adndis(&mut self) -> AdndisW<AddiscrSpec> {
        AdndisW::new(self, 0)
    }
    ///Bit 4 - Precharge/discharge select
    #[inline(always)]
    pub fn pchg(&mut self) -> PchgW<AddiscrSpec> {
        PchgW::new(self, 4)
    }
}
/**A/D Disconnection Detection Control Register

You can [`read`](crate::Reg::read) this register and get [`addiscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addiscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AddiscrSpec;
impl crate::RegisterSpec for AddiscrSpec {
    type Ux = u8;
}
///`read()` method returns [`addiscr::R`](R) reader structure
impl crate::Readable for AddiscrSpec {}
///`write(|w| ..)` method takes [`addiscr::W`](W) writer structure
impl crate::Writable for AddiscrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDISCR to value 0
impl crate::Resettable for AddiscrSpec {}
