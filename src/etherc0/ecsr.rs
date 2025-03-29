///Register `ECSR` reader
pub type R = crate::R<EcsrSpec>;
///Register `ECSR` writer
pub type W = crate::W<EcsrSpec>;
/**False Carrier Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icd {
    ///0: PHY-LSI has not detected a false carrier on the line
    _0 = 0,
    ///1: PHY-LSI detected a false carrier on the line.
    _1 = 1,
}
impl From<Icd> for bool {
    #[inline(always)]
    fn from(variant: Icd) -> Self {
        variant as u8 != 0
    }
}
///Field `ICD` reader - False Carrier Detect Flag
pub type IcdR = crate::BitReader<Icd>;
impl IcdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Icd {
        match self.bits {
            false => Icd::_0,
            true => Icd::_1,
        }
    }
    ///PHY-LSI has not detected a false carrier on the line
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Icd::_0
    }
    ///PHY-LSI detected a false carrier on the line.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Icd::_1
    }
}
///Field `ICD` writer - False Carrier Detect Flag
pub type IcdW<'a, REG> = crate::BitWriter<'a, REG, Icd>;
impl<'a, REG> IcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PHY-LSI has not detected a false carrier on the line
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Icd::_0)
    }
    ///PHY-LSI detected a false carrier on the line.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Icd::_1)
    }
}
/**Magic Packet Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpd {
    ///0: Magic Packet not detected
    _0 = 0,
    ///1: Magic Packet detected.
    _1 = 1,
}
impl From<Mpd> for bool {
    #[inline(always)]
    fn from(variant: Mpd) -> Self {
        variant as u8 != 0
    }
}
///Field `MPD` reader - Magic Packet Detect Flag
pub type MpdR = crate::BitReader<Mpd>;
impl MpdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mpd {
        match self.bits {
            false => Mpd::_0,
            true => Mpd::_1,
        }
    }
    ///Magic Packet not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpd::_0
    }
    ///Magic Packet detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mpd::_1
    }
}
///Field `MPD` writer - Magic Packet Detect Flag
pub type MpdW<'a, REG> = crate::BitWriter<'a, REG, Mpd>;
impl<'a, REG> MpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Magic Packet not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpd::_0)
    }
    ///Magic Packet detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpd::_1)
    }
}
/**Link Signal Change Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lchng {
    ///0: Change in the ET0_LINKSTA signal not detected
    _0 = 0,
    ///1: Change in the ET0_LINKSTA signal detected (high to low, or low to high).
    _1 = 1,
}
impl From<Lchng> for bool {
    #[inline(always)]
    fn from(variant: Lchng) -> Self {
        variant as u8 != 0
    }
}
///Field `LCHNG` reader - Link Signal Change Flag
pub type LchngR = crate::BitReader<Lchng>;
impl LchngR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lchng {
        match self.bits {
            false => Lchng::_0,
            true => Lchng::_1,
        }
    }
    ///Change in the ET0_LINKSTA signal not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lchng::_0
    }
    ///Change in the ET0_LINKSTA signal detected (high to low, or low to high).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lchng::_1
    }
}
///Field `LCHNG` writer - Link Signal Change Flag
pub type LchngW<'a, REG> = crate::BitWriter<'a, REG, Lchng>;
impl<'a, REG> LchngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Change in the ET0_LINKSTA signal not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lchng::_0)
    }
    ///Change in the ET0_LINKSTA signal detected (high to low, or low to high).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lchng::_1)
    }
}
/**PAUSE Frame Retransmit Over Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psrto {
    ///0: PAUSE frame retransmit count has not reached the upper limit
    _0 = 0,
    ///1: PAUSE frame retransmit count reached the upper limit.
    _1 = 1,
}
impl From<Psrto> for bool {
    #[inline(always)]
    fn from(variant: Psrto) -> Self {
        variant as u8 != 0
    }
}
///Field `PSRTO` reader - PAUSE Frame Retransmit Over Flag
pub type PsrtoR = crate::BitReader<Psrto>;
impl PsrtoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psrto {
        match self.bits {
            false => Psrto::_0,
            true => Psrto::_1,
        }
    }
    ///PAUSE frame retransmit count has not reached the upper limit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psrto::_0
    }
    ///PAUSE frame retransmit count reached the upper limit.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psrto::_1
    }
}
///Field `PSRTO` writer - PAUSE Frame Retransmit Over Flag
pub type PsrtoW<'a, REG> = crate::BitWriter<'a, REG, Psrto>;
impl<'a, REG> PsrtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PAUSE frame retransmit count has not reached the upper limit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psrto::_0)
    }
    ///PAUSE frame retransmit count reached the upper limit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psrto::_1)
    }
}
/**Continuous Broadcast Frame Reception Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfr {
    ///0: Continuous reception of broadcast frames not detected
    _0 = 0,
    ///1: Continuous reception of broadcast frames detected.
    _1 = 1,
}
impl From<Bfr> for bool {
    #[inline(always)]
    fn from(variant: Bfr) -> Self {
        variant as u8 != 0
    }
}
///Field `BFR` reader - Continuous Broadcast Frame Reception Flag
pub type BfrR = crate::BitReader<Bfr>;
impl BfrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bfr {
        match self.bits {
            false => Bfr::_0,
            true => Bfr::_1,
        }
    }
    ///Continuous reception of broadcast frames not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bfr::_0
    }
    ///Continuous reception of broadcast frames detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bfr::_1
    }
}
///Field `BFR` writer - Continuous Broadcast Frame Reception Flag
pub type BfrW<'a, REG> = crate::BitWriter<'a, REG, Bfr>;
impl<'a, REG> BfrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Continuous reception of broadcast frames not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bfr::_0)
    }
    ///Continuous reception of broadcast frames detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfr::_1)
    }
}
impl R {
    ///Bit 0 - False Carrier Detect Flag
    #[inline(always)]
    pub fn icd(&self) -> IcdR {
        IcdR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Magic Packet Detect Flag
    #[inline(always)]
    pub fn mpd(&self) -> MpdR {
        MpdR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Link Signal Change Flag
    #[inline(always)]
    pub fn lchng(&self) -> LchngR {
        LchngR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - PAUSE Frame Retransmit Over Flag
    #[inline(always)]
    pub fn psrto(&self) -> PsrtoR {
        PsrtoR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Continuous Broadcast Frame Reception Flag
    #[inline(always)]
    pub fn bfr(&self) -> BfrR {
        BfrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECSR")
            .field("icd", &self.icd())
            .field("mpd", &self.mpd())
            .field("lchng", &self.lchng())
            .field("psrto", &self.psrto())
            .field("bfr", &self.bfr())
            .finish()
    }
}
impl W {
    ///Bit 0 - False Carrier Detect Flag
    #[inline(always)]
    pub fn icd(&mut self) -> IcdW<EcsrSpec> {
        IcdW::new(self, 0)
    }
    ///Bit 1 - Magic Packet Detect Flag
    #[inline(always)]
    pub fn mpd(&mut self) -> MpdW<EcsrSpec> {
        MpdW::new(self, 1)
    }
    ///Bit 2 - Link Signal Change Flag
    #[inline(always)]
    pub fn lchng(&mut self) -> LchngW<EcsrSpec> {
        LchngW::new(self, 2)
    }
    ///Bit 4 - PAUSE Frame Retransmit Over Flag
    #[inline(always)]
    pub fn psrto(&mut self) -> PsrtoW<EcsrSpec> {
        PsrtoW::new(self, 4)
    }
    ///Bit 5 - Continuous Broadcast Frame Reception Flag
    #[inline(always)]
    pub fn bfr(&mut self) -> BfrW<EcsrSpec> {
        BfrW::new(self, 5)
    }
}
/**ETHERC Status Register

You can [`read`](crate::Reg::read) this register and get [`ecsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EcsrSpec;
impl crate::RegisterSpec for EcsrSpec {
    type Ux = u32;
}
///`read()` method returns [`ecsr::R`](R) reader structure
impl crate::Readable for EcsrSpec {}
///`write(|w| ..)` method takes [`ecsr::W`](W) writer structure
impl crate::Writable for EcsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECSR to value 0
impl crate::Resettable for EcsrSpec {}
