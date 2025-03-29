///Register `INTSTS1` reader
pub type R = crate::R<Intsts1Spec>;
///Register `INTSTS1` writer
pub type W = crate::W<Intsts1Spec>;
/**PDDET Detection Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pddetint {
    ///0: No PDDET interrupt occurred
    _0 = 0,
    ///1: PDDET interrupt occurred
    _1 = 1,
}
impl From<Pddetint> for bool {
    #[inline(always)]
    fn from(variant: Pddetint) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDETINT` reader - PDDET Detection Interrupt Status Flag
pub type PddetintR = crate::BitReader<Pddetint>;
impl PddetintR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pddetint {
        match self.bits {
            false => Pddetint::_0,
            true => Pddetint::_1,
        }
    }
    ///No PDDET interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pddetint::_0
    }
    ///PDDET interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pddetint::_1
    }
}
///Field `PDDETINT` writer - PDDET Detection Interrupt Status Flag
pub type PddetintW<'a, REG> = crate::BitWriter<'a, REG, Pddetint>;
impl<'a, REG> PddetintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No PDDET interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pddetint::_0)
    }
    ///PDDET interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pddetint::_1)
    }
}
/**Setup Transaction Normal Response Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sack {
    ///0: No SACK interrupt occurred
    _0 = 0,
    ///1: SACK interrupt occurred
    _1 = 1,
}
impl From<Sack> for bool {
    #[inline(always)]
    fn from(variant: Sack) -> Self {
        variant as u8 != 0
    }
}
///Field `SACK` reader - Setup Transaction Normal Response Interrupt Status Flag
pub type SackR = crate::BitReader<Sack>;
impl SackR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sack {
        match self.bits {
            false => Sack::_0,
            true => Sack::_1,
        }
    }
    ///No SACK interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sack::_0
    }
    ///SACK interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sack::_1
    }
}
///Field `SACK` writer - Setup Transaction Normal Response Interrupt Status Flag
pub type SackW<'a, REG> = crate::BitWriter<'a, REG, Sack>;
impl<'a, REG> SackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No SACK interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sack::_0)
    }
    ///SACK interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sack::_1)
    }
}
/**Setup Transaction Error Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    ///0: No SIGN interrupt occurred
    _0 = 0,
    ///1: SIGN interrupt occurred
    _1 = 1,
}
impl From<Sign> for bool {
    #[inline(always)]
    fn from(variant: Sign) -> Self {
        variant as u8 != 0
    }
}
///Field `SIGN` reader - Setup Transaction Error Interrupt Status Flag
pub type SignR = crate::BitReader<Sign>;
impl SignR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sign {
        match self.bits {
            false => Sign::_0,
            true => Sign::_1,
        }
    }
    ///No SIGN interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sign::_0
    }
    ///SIGN interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sign::_1
    }
}
///Field `SIGN` writer - Setup Transaction Error Interrupt Status Flag
pub type SignW<'a, REG> = crate::BitWriter<'a, REG, Sign>;
impl<'a, REG> SignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No SIGN interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sign::_0)
    }
    ///SIGN interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sign::_1)
    }
}
/**EOF Error Detection Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoferr {
    ///0: No EOFERR interrupt occurred
    _0 = 0,
    ///1: EOFERR interrupt occurred
    _1 = 1,
}
impl From<Eoferr> for bool {
    #[inline(always)]
    fn from(variant: Eoferr) -> Self {
        variant as u8 != 0
    }
}
///Field `EOFERR` reader - EOF Error Detection Interrupt Status Flag
pub type EoferrR = crate::BitReader<Eoferr>;
impl EoferrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eoferr {
        match self.bits {
            false => Eoferr::_0,
            true => Eoferr::_1,
        }
    }
    ///No EOFERR interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eoferr::_0
    }
    ///EOFERR interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eoferr::_1
    }
}
///Field `EOFERR` writer - EOF Error Detection Interrupt Status Flag
pub type EoferrW<'a, REG> = crate::BitWriter<'a, REG, Eoferr>;
impl<'a, REG> EoferrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No EOFERR interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eoferr::_0)
    }
    ///EOFERR interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eoferr::_1)
    }
}
/**LPM Transaction End Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpmend {
    ///0: No LPMEND interrupt occurred
    _0 = 0,
    ///1: LPMEND interrupt occurred
    _1 = 1,
}
impl From<Lpmend> for bool {
    #[inline(always)]
    fn from(variant: Lpmend) -> Self {
        variant as u8 != 0
    }
}
///Field `LPMEND` reader - LPM Transaction End Interrupt Status Flag
pub type LpmendR = crate::BitReader<Lpmend>;
impl LpmendR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lpmend {
        match self.bits {
            false => Lpmend::_0,
            true => Lpmend::_1,
        }
    }
    ///No LPMEND interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lpmend::_0
    }
    ///LPMEND interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lpmend::_1
    }
}
///Field `LPMEND` writer - LPM Transaction End Interrupt Status Flag
pub type LpmendW<'a, REG> = crate::BitWriter<'a, REG, Lpmend>;
impl<'a, REG> LpmendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No LPMEND interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmend::_0)
    }
    ///LPMEND interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmend::_1)
    }
}
/**L1 Resume End Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1rsmend {
    ///0: No L1RSMEND interrupt occurred
    _0 = 0,
    ///1: L1RSMEND interrupt occurred
    _1 = 1,
}
impl From<L1rsmend> for bool {
    #[inline(always)]
    fn from(variant: L1rsmend) -> Self {
        variant as u8 != 0
    }
}
///Field `L1RSMEND` reader - L1 Resume End Interrupt Status Flag
pub type L1rsmendR = crate::BitReader<L1rsmend>;
impl L1rsmendR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1rsmend {
        match self.bits {
            false => L1rsmend::_0,
            true => L1rsmend::_1,
        }
    }
    ///No L1RSMEND interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1rsmend::_0
    }
    ///L1RSMEND interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1rsmend::_1
    }
}
///Field `L1RSMEND` writer - L1 Resume End Interrupt Status Flag
pub type L1rsmendW<'a, REG> = crate::BitWriter<'a, REG, L1rsmend>;
impl<'a, REG> L1rsmendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No L1RSMEND interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1rsmend::_0)
    }
    ///L1RSMEND interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1rsmend::_1)
    }
}
/**USB Connection Detection Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attch {
    ///0: No ATTCH interrupt occurred
    _0 = 0,
    ///1: ATTCH interrupt occurred
    _1 = 1,
}
impl From<Attch> for bool {
    #[inline(always)]
    fn from(variant: Attch) -> Self {
        variant as u8 != 0
    }
}
///Field `ATTCH` reader - USB Connection Detection Interrupt Status Flag
pub type AttchR = crate::BitReader<Attch>;
impl AttchR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Attch {
        match self.bits {
            false => Attch::_0,
            true => Attch::_1,
        }
    }
    ///No ATTCH interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Attch::_0
    }
    ///ATTCH interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Attch::_1
    }
}
///Field `ATTCH` writer - USB Connection Detection Interrupt Status Flag
pub type AttchW<'a, REG> = crate::BitWriter<'a, REG, Attch>;
impl<'a, REG> AttchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No ATTCH interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Attch::_0)
    }
    ///ATTCH interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Attch::_1)
    }
}
/**USB Disconnection Detection Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtch {
    ///0: No DTCH interrupt occurred
    _0 = 0,
    ///1: DTCH interrupt occurred
    _1 = 1,
}
impl From<Dtch> for bool {
    #[inline(always)]
    fn from(variant: Dtch) -> Self {
        variant as u8 != 0
    }
}
///Field `DTCH` reader - USB Disconnection Detection Interrupt Status Flag
pub type DtchR = crate::BitReader<Dtch>;
impl DtchR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dtch {
        match self.bits {
            false => Dtch::_0,
            true => Dtch::_1,
        }
    }
    ///No DTCH interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtch::_0
    }
    ///DTCH interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtch::_1
    }
}
///Field `DTCH` writer - USB Disconnection Detection Interrupt Status Flag
pub type DtchW<'a, REG> = crate::BitWriter<'a, REG, Dtch>;
impl<'a, REG> DtchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No DTCH interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtch::_0)
    }
    ///DTCH interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtch::_1)
    }
}
/**USB Bus Change Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bchg {
    ///0: No BCHG interrupt occurred
    _0 = 0,
    ///1: BCHG interrupt occurred
    _1 = 1,
}
impl From<Bchg> for bool {
    #[inline(always)]
    fn from(variant: Bchg) -> Self {
        variant as u8 != 0
    }
}
///Field `BCHG` reader - USB Bus Change Interrupt Status Flag
pub type BchgR = crate::BitReader<Bchg>;
impl BchgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bchg {
        match self.bits {
            false => Bchg::_0,
            true => Bchg::_1,
        }
    }
    ///No BCHG interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bchg::_0
    }
    ///BCHG interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bchg::_1
    }
}
///Field `BCHG` writer - USB Bus Change Interrupt Status Flag
pub type BchgW<'a, REG> = crate::BitWriter<'a, REG, Bchg>;
impl<'a, REG> BchgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No BCHG interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bchg::_0)
    }
    ///BCHG interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bchg::_1)
    }
}
/**OVRCR Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrcr {
    ///0: No OVRCR interrupt occurred
    _0 = 0,
    ///1: OVRCR interrupt occurred
    _1 = 1,
}
impl From<Ovrcr> for bool {
    #[inline(always)]
    fn from(variant: Ovrcr) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRCR` reader - OVRCR Interrupt Status Flag
pub type OvrcrR = crate::BitReader<Ovrcr>;
impl OvrcrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ovrcr {
        match self.bits {
            false => Ovrcr::_0,
            true => Ovrcr::_1,
        }
    }
    ///No OVRCR interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovrcr::_0
    }
    ///OVRCR interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovrcr::_1
    }
}
///Field `OVRCR` writer - OVRCR Interrupt Status Flag
pub type OvrcrW<'a, REG> = crate::BitWriter<'a, REG, Ovrcr>;
impl<'a, REG> OvrcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No OVRCR interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrcr::_0)
    }
    ///OVRCR interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrcr::_1)
    }
}
impl R {
    ///Bit 0 - PDDET Detection Interrupt Status Flag
    #[inline(always)]
    pub fn pddetint(&self) -> PddetintR {
        PddetintR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Setup Transaction Normal Response Interrupt Status Flag
    #[inline(always)]
    pub fn sack(&self) -> SackR {
        SackR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Setup Transaction Error Interrupt Status Flag
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EOF Error Detection Interrupt Status Flag
    #[inline(always)]
    pub fn eoferr(&self) -> EoferrR {
        EoferrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - LPM Transaction End Interrupt Status Flag
    #[inline(always)]
    pub fn lpmend(&self) -> LpmendR {
        LpmendR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - L1 Resume End Interrupt Status Flag
    #[inline(always)]
    pub fn l1rsmend(&self) -> L1rsmendR {
        L1rsmendR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - USB Connection Detection Interrupt Status Flag
    #[inline(always)]
    pub fn attch(&self) -> AttchR {
        AttchR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - USB Disconnection Detection Interrupt Status Flag
    #[inline(always)]
    pub fn dtch(&self) -> DtchR {
        DtchR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USB Bus Change Interrupt Status Flag
    #[inline(always)]
    pub fn bchg(&self) -> BchgR {
        BchgR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - OVRCR Interrupt Status Flag
    #[inline(always)]
    pub fn ovrcr(&self) -> OvrcrR {
        OvrcrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTS1")
            .field("pddetint", &self.pddetint())
            .field("sack", &self.sack())
            .field("sign", &self.sign())
            .field("eoferr", &self.eoferr())
            .field("lpmend", &self.lpmend())
            .field("l1rsmend", &self.l1rsmend())
            .field("attch", &self.attch())
            .field("dtch", &self.dtch())
            .field("bchg", &self.bchg())
            .field("ovrcr", &self.ovrcr())
            .finish()
    }
}
impl W {
    ///Bit 0 - PDDET Detection Interrupt Status Flag
    #[inline(always)]
    pub fn pddetint(&mut self) -> PddetintW<Intsts1Spec> {
        PddetintW::new(self, 0)
    }
    ///Bit 4 - Setup Transaction Normal Response Interrupt Status Flag
    #[inline(always)]
    pub fn sack(&mut self) -> SackW<Intsts1Spec> {
        SackW::new(self, 4)
    }
    ///Bit 5 - Setup Transaction Error Interrupt Status Flag
    #[inline(always)]
    pub fn sign(&mut self) -> SignW<Intsts1Spec> {
        SignW::new(self, 5)
    }
    ///Bit 6 - EOF Error Detection Interrupt Status Flag
    #[inline(always)]
    pub fn eoferr(&mut self) -> EoferrW<Intsts1Spec> {
        EoferrW::new(self, 6)
    }
    ///Bit 8 - LPM Transaction End Interrupt Status Flag
    #[inline(always)]
    pub fn lpmend(&mut self) -> LpmendW<Intsts1Spec> {
        LpmendW::new(self, 8)
    }
    ///Bit 9 - L1 Resume End Interrupt Status Flag
    #[inline(always)]
    pub fn l1rsmend(&mut self) -> L1rsmendW<Intsts1Spec> {
        L1rsmendW::new(self, 9)
    }
    ///Bit 11 - USB Connection Detection Interrupt Status Flag
    #[inline(always)]
    pub fn attch(&mut self) -> AttchW<Intsts1Spec> {
        AttchW::new(self, 11)
    }
    ///Bit 12 - USB Disconnection Detection Interrupt Status Flag
    #[inline(always)]
    pub fn dtch(&mut self) -> DtchW<Intsts1Spec> {
        DtchW::new(self, 12)
    }
    ///Bit 14 - USB Bus Change Interrupt Status Flag
    #[inline(always)]
    pub fn bchg(&mut self) -> BchgW<Intsts1Spec> {
        BchgW::new(self, 14)
    }
    ///Bit 15 - OVRCR Interrupt Status Flag
    #[inline(always)]
    pub fn ovrcr(&mut self) -> OvrcrW<Intsts1Spec> {
        OvrcrW::new(self, 15)
    }
}
/**Interrupt Status Register 1

You can [`read`](crate::Reg::read) this register and get [`intsts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Intsts1Spec;
impl crate::RegisterSpec for Intsts1Spec {
    type Ux = u16;
}
///`read()` method returns [`intsts1::R`](R) reader structure
impl crate::Readable for Intsts1Spec {}
///`write(|w| ..)` method takes [`intsts1::W`](W) writer structure
impl crate::Writable for Intsts1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTSTS1 to value 0
impl crate::Resettable for Intsts1Spec {}
