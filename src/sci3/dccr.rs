///Register `DCCR` reader
pub type R = crate::R<DccrSpec>;
///Register `DCCR` writer
pub type W = crate::W<DccrSpec>;
/**Data Compare Match Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcmf {
    ///0: Not matched
    _0 = 0,
    ///1: Matched
    _1 = 1,
}
impl From<Dcmf> for bool {
    #[inline(always)]
    fn from(variant: Dcmf) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMF` reader - Data Compare Match Flag
pub type DcmfR = crate::BitReader<Dcmf>;
impl DcmfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dcmf {
        match self.bits {
            false => Dcmf::_0,
            true => Dcmf::_1,
        }
    }
    ///Not matched
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcmf::_0
    }
    ///Matched
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcmf::_1
    }
}
///Field `DCMF` writer - Data Compare Match Flag
pub type DcmfW<'a, REG> = crate::BitWriter<'a, REG, Dcmf>;
impl<'a, REG> DcmfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not matched
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmf::_0)
    }
    ///Matched
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmf::_1)
    }
}
/**Data Compare Match Parity Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dper {
    ///0: No parity error occurred
    _0 = 0,
    ///1: Parity error occurred
    _1 = 1,
}
impl From<Dper> for bool {
    #[inline(always)]
    fn from(variant: Dper) -> Self {
        variant as u8 != 0
    }
}
///Field `DPER` reader - Data Compare Match Parity Error Flag
pub type DperR = crate::BitReader<Dper>;
impl DperR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dper {
        match self.bits {
            false => Dper::_0,
            true => Dper::_1,
        }
    }
    ///No parity error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dper::_0
    }
    ///Parity error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dper::_1
    }
}
///Field `DPER` writer - Data Compare Match Parity Error Flag
pub type DperW<'a, REG> = crate::BitWriter<'a, REG, Dper>;
impl<'a, REG> DperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No parity error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dper::_0)
    }
    ///Parity error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dper::_1)
    }
}
/**Data Compare Match Framing Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfer {
    ///0: No framing error occurred
    _0 = 0,
    ///1: Framing error occurred
    _1 = 1,
}
impl From<Dfer> for bool {
    #[inline(always)]
    fn from(variant: Dfer) -> Self {
        variant as u8 != 0
    }
}
///Field `DFER` reader - Data Compare Match Framing Error Flag
pub type DferR = crate::BitReader<Dfer>;
impl DferR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dfer {
        match self.bits {
            false => Dfer::_0,
            true => Dfer::_1,
        }
    }
    ///No framing error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dfer::_0
    }
    ///Framing error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dfer::_1
    }
}
///Field `DFER` writer - Data Compare Match Framing Error Flag
pub type DferW<'a, REG> = crate::BitWriter<'a, REG, Dfer>;
impl<'a, REG> DferW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No framing error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfer::_0)
    }
    ///Framing error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfer::_1)
    }
}
/**ID Frame Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idsel {
    ///0: Always compare data regardless of the MPB bit value
    _0 = 0,
    ///1: Only compare data when MPB bit = 1 (ID frame)
    _1 = 1,
}
impl From<Idsel> for bool {
    #[inline(always)]
    fn from(variant: Idsel) -> Self {
        variant as u8 != 0
    }
}
///Field `IDSEL` reader - ID Frame Select
pub type IdselR = crate::BitReader<Idsel>;
impl IdselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Idsel {
        match self.bits {
            false => Idsel::_0,
            true => Idsel::_1,
        }
    }
    ///Always compare data regardless of the MPB bit value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idsel::_0
    }
    ///Only compare data when MPB bit = 1 (ID frame)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idsel::_1
    }
}
///Field `IDSEL` writer - ID Frame Select
pub type IdselW<'a, REG> = crate::BitWriter<'a, REG, Idsel>;
impl<'a, REG> IdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Always compare data regardless of the MPB bit value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Idsel::_0)
    }
    ///Only compare data when MPB bit = 1 (ID frame)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Idsel::_1)
    }
}
/**Data Compare Match Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcme {
    ///0: Disable address match function
    _0 = 0,
    ///1: Enable address match function
    _1 = 1,
}
impl From<Dcme> for bool {
    #[inline(always)]
    fn from(variant: Dcme) -> Self {
        variant as u8 != 0
    }
}
///Field `DCME` reader - Data Compare Match Enable
pub type DcmeR = crate::BitReader<Dcme>;
impl DcmeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dcme {
        match self.bits {
            false => Dcme::_0,
            true => Dcme::_1,
        }
    }
    ///Disable address match function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcme::_0
    }
    ///Enable address match function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcme::_1
    }
}
///Field `DCME` writer - Data Compare Match Enable
pub type DcmeW<'a, REG> = crate::BitWriter<'a, REG, Dcme>;
impl<'a, REG> DcmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable address match function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcme::_0)
    }
    ///Enable address match function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcme::_1)
    }
}
impl R {
    ///Bit 0 - Data Compare Match Flag
    #[inline(always)]
    pub fn dcmf(&self) -> DcmfR {
        DcmfR::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Data Compare Match Parity Error Flag
    #[inline(always)]
    pub fn dper(&self) -> DperR {
        DperR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data Compare Match Framing Error Flag
    #[inline(always)]
    pub fn dfer(&self) -> DferR {
        DferR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - ID Frame Select
    #[inline(always)]
    pub fn idsel(&self) -> IdselR {
        IdselR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Data Compare Match Enable
    #[inline(always)]
    pub fn dcme(&self) -> DcmeR {
        DcmeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCCR")
            .field("dcmf", &self.dcmf())
            .field("dper", &self.dper())
            .field("dfer", &self.dfer())
            .field("idsel", &self.idsel())
            .field("dcme", &self.dcme())
            .finish()
    }
}
impl W {
    ///Bit 0 - Data Compare Match Flag
    #[inline(always)]
    pub fn dcmf(&mut self) -> DcmfW<DccrSpec> {
        DcmfW::new(self, 0)
    }
    ///Bit 3 - Data Compare Match Parity Error Flag
    #[inline(always)]
    pub fn dper(&mut self) -> DperW<DccrSpec> {
        DperW::new(self, 3)
    }
    ///Bit 4 - Data Compare Match Framing Error Flag
    #[inline(always)]
    pub fn dfer(&mut self) -> DferW<DccrSpec> {
        DferW::new(self, 4)
    }
    ///Bit 6 - ID Frame Select
    #[inline(always)]
    pub fn idsel(&mut self) -> IdselW<DccrSpec> {
        IdselW::new(self, 6)
    }
    ///Bit 7 - Data Compare Match Enable
    #[inline(always)]
    pub fn dcme(&mut self) -> DcmeW<DccrSpec> {
        DcmeW::new(self, 7)
    }
}
/**Data Compare Match Control Register

You can [`read`](crate::Reg::read) this register and get [`dccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DccrSpec;
impl crate::RegisterSpec for DccrSpec {
    type Ux = u8;
}
///`read()` method returns [`dccr::R`](R) reader structure
impl crate::Readable for DccrSpec {}
///`write(|w| ..)` method takes [`dccr::W`](W) writer structure
impl crate::Writable for DccrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCCR to value 0x40
impl crate::Resettable for DccrSpec {
    const RESET_VALUE: u8 = 0x40;
}
