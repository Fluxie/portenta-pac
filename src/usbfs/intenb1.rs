///Register `INTENB1` reader
pub type R = crate::R<Intenb1Spec>;
///Register `INTENB1` writer
pub type W = crate::W<Intenb1Spec>;
/**PDDETINT Detection Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pddetinte {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pddetinte> for bool {
    #[inline(always)]
    fn from(variant: Pddetinte) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDETINTE` reader - PDDETINT Detection Interrupt Request Enable
pub type PddetinteR = crate::BitReader<Pddetinte>;
impl PddetinteR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pddetinte {
        match self.bits {
            false => Pddetinte::_0,
            true => Pddetinte::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pddetinte::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pddetinte::_1
    }
}
///Field `PDDETINTE` writer - PDDETINT Detection Interrupt Request Enable
pub type PddetinteW<'a, REG> = crate::BitWriter<'a, REG, Pddetinte>;
impl<'a, REG> PddetinteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pddetinte::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pddetinte::_1)
    }
}
/**Setup Transaction Normal Response Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sacke {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Sacke> for bool {
    #[inline(always)]
    fn from(variant: Sacke) -> Self {
        variant as u8 != 0
    }
}
///Field `SACKE` reader - Setup Transaction Normal Response Interrupt Enable
pub type SackeR = crate::BitReader<Sacke>;
impl SackeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sacke {
        match self.bits {
            false => Sacke::_0,
            true => Sacke::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sacke::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sacke::_1
    }
}
///Field `SACKE` writer - Setup Transaction Normal Response Interrupt Enable
pub type SackeW<'a, REG> = crate::BitWriter<'a, REG, Sacke>;
impl<'a, REG> SackeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sacke::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sacke::_1)
    }
}
/**Setup Transaction Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Signe> for bool {
    #[inline(always)]
    fn from(variant: Signe) -> Self {
        variant as u8 != 0
    }
}
///Field `SIGNE` reader - Setup Transaction Error Interrupt Enable
pub type SigneR = crate::BitReader<Signe>;
impl SigneR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Signe {
        match self.bits {
            false => Signe::_0,
            true => Signe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Signe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Signe::_1
    }
}
///Field `SIGNE` writer - Setup Transaction Error Interrupt Enable
pub type SigneW<'a, REG> = crate::BitWriter<'a, REG, Signe>;
impl<'a, REG> SigneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Signe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Signe::_1)
    }
}
/**EOF Error Detection Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoferre {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Eoferre> for bool {
    #[inline(always)]
    fn from(variant: Eoferre) -> Self {
        variant as u8 != 0
    }
}
///Field `EOFERRE` reader - EOF Error Detection Interrupt Enable
pub type EoferreR = crate::BitReader<Eoferre>;
impl EoferreR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eoferre {
        match self.bits {
            false => Eoferre::_0,
            true => Eoferre::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eoferre::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eoferre::_1
    }
}
///Field `EOFERRE` writer - EOF Error Detection Interrupt Enable
pub type EoferreW<'a, REG> = crate::BitWriter<'a, REG, Eoferre>;
impl<'a, REG> EoferreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eoferre::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eoferre::_1)
    }
}
/**Connection Detection Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attche {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Attche> for bool {
    #[inline(always)]
    fn from(variant: Attche) -> Self {
        variant as u8 != 0
    }
}
///Field `ATTCHE` reader - Connection Detection Interrupt Enable
pub type AttcheR = crate::BitReader<Attche>;
impl AttcheR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Attche {
        match self.bits {
            false => Attche::_0,
            true => Attche::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Attche::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Attche::_1
    }
}
///Field `ATTCHE` writer - Connection Detection Interrupt Enable
pub type AttcheW<'a, REG> = crate::BitWriter<'a, REG, Attche>;
impl<'a, REG> AttcheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Attche::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Attche::_1)
    }
}
/**Disconnection Detection Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtche {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Dtche> for bool {
    #[inline(always)]
    fn from(variant: Dtche) -> Self {
        variant as u8 != 0
    }
}
///Field `DTCHE` reader - Disconnection Detection Interrupt Enable
pub type DtcheR = crate::BitReader<Dtche>;
impl DtcheR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dtche {
        match self.bits {
            false => Dtche::_0,
            true => Dtche::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtche::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtche::_1
    }
}
///Field `DTCHE` writer - Disconnection Detection Interrupt Enable
pub type DtcheW<'a, REG> = crate::BitWriter<'a, REG, Dtche>;
impl<'a, REG> DtcheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtche::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtche::_1)
    }
}
/**USB Bus Change Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bchge {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Bchge> for bool {
    #[inline(always)]
    fn from(variant: Bchge) -> Self {
        variant as u8 != 0
    }
}
///Field `BCHGE` reader - USB Bus Change Interrupt Enable
pub type BchgeR = crate::BitReader<Bchge>;
impl BchgeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bchge {
        match self.bits {
            false => Bchge::_0,
            true => Bchge::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bchge::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bchge::_1
    }
}
///Field `BCHGE` writer - USB Bus Change Interrupt Enable
pub type BchgeW<'a, REG> = crate::BitWriter<'a, REG, Bchge>;
impl<'a, REG> BchgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bchge::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bchge::_1)
    }
}
/**Overcurrent Input Change Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrcre {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Ovrcre> for bool {
    #[inline(always)]
    fn from(variant: Ovrcre) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRCRE` reader - Overcurrent Input Change Interrupt Enable
pub type OvrcreR = crate::BitReader<Ovrcre>;
impl OvrcreR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ovrcre {
        match self.bits {
            false => Ovrcre::_0,
            true => Ovrcre::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovrcre::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovrcre::_1
    }
}
///Field `OVRCRE` writer - Overcurrent Input Change Interrupt Enable
pub type OvrcreW<'a, REG> = crate::BitWriter<'a, REG, Ovrcre>;
impl<'a, REG> OvrcreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrcre::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrcre::_1)
    }
}
impl R {
    ///Bit 0 - PDDETINT Detection Interrupt Request Enable
    #[inline(always)]
    pub fn pddetinte(&self) -> PddetinteR {
        PddetinteR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Setup Transaction Normal Response Interrupt Enable
    #[inline(always)]
    pub fn sacke(&self) -> SackeR {
        SackeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Setup Transaction Error Interrupt Enable
    #[inline(always)]
    pub fn signe(&self) -> SigneR {
        SigneR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EOF Error Detection Interrupt Enable
    #[inline(always)]
    pub fn eoferre(&self) -> EoferreR {
        EoferreR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 11 - Connection Detection Interrupt Enable
    #[inline(always)]
    pub fn attche(&self) -> AttcheR {
        AttcheR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Disconnection Detection Interrupt Enable
    #[inline(always)]
    pub fn dtche(&self) -> DtcheR {
        DtcheR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USB Bus Change Interrupt Enable
    #[inline(always)]
    pub fn bchge(&self) -> BchgeR {
        BchgeR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Overcurrent Input Change Interrupt Enable
    #[inline(always)]
    pub fn ovrcre(&self) -> OvrcreR {
        OvrcreR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENB1")
            .field("pddetinte", &self.pddetinte())
            .field("sacke", &self.sacke())
            .field("signe", &self.signe())
            .field("eoferre", &self.eoferre())
            .field("attche", &self.attche())
            .field("dtche", &self.dtche())
            .field("bchge", &self.bchge())
            .field("ovrcre", &self.ovrcre())
            .finish()
    }
}
impl W {
    ///Bit 0 - PDDETINT Detection Interrupt Request Enable
    #[inline(always)]
    pub fn pddetinte(&mut self) -> PddetinteW<Intenb1Spec> {
        PddetinteW::new(self, 0)
    }
    ///Bit 4 - Setup Transaction Normal Response Interrupt Enable
    #[inline(always)]
    pub fn sacke(&mut self) -> SackeW<Intenb1Spec> {
        SackeW::new(self, 4)
    }
    ///Bit 5 - Setup Transaction Error Interrupt Enable
    #[inline(always)]
    pub fn signe(&mut self) -> SigneW<Intenb1Spec> {
        SigneW::new(self, 5)
    }
    ///Bit 6 - EOF Error Detection Interrupt Enable
    #[inline(always)]
    pub fn eoferre(&mut self) -> EoferreW<Intenb1Spec> {
        EoferreW::new(self, 6)
    }
    ///Bit 11 - Connection Detection Interrupt Enable
    #[inline(always)]
    pub fn attche(&mut self) -> AttcheW<Intenb1Spec> {
        AttcheW::new(self, 11)
    }
    ///Bit 12 - Disconnection Detection Interrupt Enable
    #[inline(always)]
    pub fn dtche(&mut self) -> DtcheW<Intenb1Spec> {
        DtcheW::new(self, 12)
    }
    ///Bit 14 - USB Bus Change Interrupt Enable
    #[inline(always)]
    pub fn bchge(&mut self) -> BchgeW<Intenb1Spec> {
        BchgeW::new(self, 14)
    }
    ///Bit 15 - Overcurrent Input Change Interrupt Enable
    #[inline(always)]
    pub fn ovrcre(&mut self) -> OvrcreW<Intenb1Spec> {
        OvrcreW::new(self, 15)
    }
}
/**Interrupt Enable Register 1

You can [`read`](crate::Reg::read) this register and get [`intenb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Intenb1Spec;
impl crate::RegisterSpec for Intenb1Spec {
    type Ux = u16;
}
///`read()` method returns [`intenb1::R`](R) reader structure
impl crate::Readable for Intenb1Spec {}
///`write(|w| ..)` method takes [`intenb1::W`](W) writer structure
impl crate::Writable for Intenb1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTENB1 to value 0
impl crate::Resettable for Intenb1Spec {}
