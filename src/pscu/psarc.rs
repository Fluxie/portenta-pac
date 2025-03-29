///Register `PSARC` reader
pub type R = crate::R<PsarcSpec>;
///Register `PSARC` writer
pub type W = crate::W<PsarcSpec>;
/**CAC and the MSTPCRC.MSTPC0 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarc0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarc0> for bool {
    #[inline(always)]
    fn from(variant: Psarc0) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARC0` reader - CAC and the MSTPCRC.MSTPC0 bit security attribution
pub type Psarc0R = crate::BitReader<Psarc0>;
impl Psarc0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarc0 {
        match self.bits {
            false => Psarc0::_0,
            true => Psarc0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarc0::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarc0::_1
    }
}
///Field `PSARC0` writer - CAC and the MSTPCRC.MSTPC0 bit security attribution
pub type Psarc0W<'a, REG> = crate::BitWriter<'a, REG, Psarc0>;
impl<'a, REG> Psarc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc0::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc0::_1)
    }
}
/**CRC and the MSTPCRC.MSTPC1 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarc1 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarc1> for bool {
    #[inline(always)]
    fn from(variant: Psarc1) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARC1` reader - CRC and the MSTPCRC.MSTPC1 bit security attribution
pub type Psarc1R = crate::BitReader<Psarc1>;
impl Psarc1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarc1 {
        match self.bits {
            false => Psarc1::_0,
            true => Psarc1::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarc1::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarc1::_1
    }
}
///Field `PSARC1` writer - CRC and the MSTPCRC.MSTPC1 bit security attribution
pub type Psarc1W<'a, REG> = crate::BitWriter<'a, REG, Psarc1>;
impl<'a, REG> Psarc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc1::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc1::_1)
    }
}
/**CTSU and the MSTPCRC.MSTPC3 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarc3 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarc3> for bool {
    #[inline(always)]
    fn from(variant: Psarc3) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARC3` reader - CTSU and the MSTPCRC.MSTPC3 bit security attribution
pub type Psarc3R = crate::BitReader<Psarc3>;
impl Psarc3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarc3 {
        match self.bits {
            false => Psarc3::_0,
            true => Psarc3::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarc3::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarc3::_1
    }
}
///Field `PSARC3` writer - CTSU and the MSTPCRC.MSTPC3 bit security attribution
pub type Psarc3W<'a, REG> = crate::BitWriter<'a, REG, Psarc3>;
impl<'a, REG> Psarc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc3::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc3::_1)
    }
}
/**SSIE0 and the MSTPCRC.MSTPC8 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarc8 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarc8> for bool {
    #[inline(always)]
    fn from(variant: Psarc8) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARC8` reader - SSIE0 and the MSTPCRC.MSTPC8 bit security attribution
pub type Psarc8R = crate::BitReader<Psarc8>;
impl Psarc8R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarc8 {
        match self.bits {
            false => Psarc8::_0,
            true => Psarc8::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarc8::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarc8::_1
    }
}
///Field `PSARC8` writer - SSIE0 and the MSTPCRC.MSTPC8 bit security attribution
pub type Psarc8W<'a, REG> = crate::BitWriter<'a, REG, Psarc8>;
impl<'a, REG> Psarc8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc8::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc8::_1)
    }
}
/**SDHI0 and the MSTPCRC.MSTPC12 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarc12 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarc12> for bool {
    #[inline(always)]
    fn from(variant: Psarc12) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARC12` reader - SDHI0 and the MSTPCRC.MSTPC12 bit security attribution
pub type Psarc12R = crate::BitReader<Psarc12>;
impl Psarc12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarc12 {
        match self.bits {
            false => Psarc12::_0,
            true => Psarc12::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarc12::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarc12::_1
    }
}
///Field `PSARC12` writer - SDHI0 and the MSTPCRC.MSTPC12 bit security attribution
pub type Psarc12W<'a, REG> = crate::BitWriter<'a, REG, Psarc12>;
impl<'a, REG> Psarc12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc12::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc12::_1)
    }
}
/**DOC and the MSTPCRC.MSTPC13 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarc13 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarc13> for bool {
    #[inline(always)]
    fn from(variant: Psarc13) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARC13` reader - DOC and the MSTPCRC.MSTPC13 bit security attribution
pub type Psarc13R = crate::BitReader<Psarc13>;
impl Psarc13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarc13 {
        match self.bits {
            false => Psarc13::_0,
            true => Psarc13::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarc13::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarc13::_1
    }
}
///Field `PSARC13` writer - DOC and the MSTPCRC.MSTPC13 bit security attribution
pub type Psarc13W<'a, REG> = crate::BitWriter<'a, REG, Psarc13>;
impl<'a, REG> Psarc13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc13::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc13::_1)
    }
}
/**CANFD0 and the MSTPCRC.MSTPC27 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarc27 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarc27> for bool {
    #[inline(always)]
    fn from(variant: Psarc27) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARC27` reader - CANFD0 and the MSTPCRC.MSTPC27 bit security attribution
pub type Psarc27R = crate::BitReader<Psarc27>;
impl Psarc27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarc27 {
        match self.bits {
            false => Psarc27::_0,
            true => Psarc27::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarc27::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarc27::_1
    }
}
///Field `PSARC27` writer - CANFD0 and the MSTPCRC.MSTPC27 bit security attribution
pub type Psarc27W<'a, REG> = crate::BitWriter<'a, REG, Psarc27>;
impl<'a, REG> Psarc27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc27::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc27::_1)
    }
}
/**SCE9 and the MSTPCRC.MSTPC31 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarc31 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarc31> for bool {
    #[inline(always)]
    fn from(variant: Psarc31) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARC31` reader - SCE9 and the MSTPCRC.MSTPC31 bit security attribution
pub type Psarc31R = crate::BitReader<Psarc31>;
impl Psarc31R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarc31 {
        match self.bits {
            false => Psarc31::_0,
            true => Psarc31::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarc31::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarc31::_1
    }
}
///Field `PSARC31` writer - SCE9 and the MSTPCRC.MSTPC31 bit security attribution
pub type Psarc31W<'a, REG> = crate::BitWriter<'a, REG, Psarc31>;
impl<'a, REG> Psarc31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc31::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarc31::_1)
    }
}
impl R {
    ///Bit 0 - CAC and the MSTPCRC.MSTPC0 bit security attribution
    #[inline(always)]
    pub fn psarc0(&self) -> Psarc0R {
        Psarc0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CRC and the MSTPCRC.MSTPC1 bit security attribution
    #[inline(always)]
    pub fn psarc1(&self) -> Psarc1R {
        Psarc1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - CTSU and the MSTPCRC.MSTPC3 bit security attribution
    #[inline(always)]
    pub fn psarc3(&self) -> Psarc3R {
        Psarc3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - SSIE0 and the MSTPCRC.MSTPC8 bit security attribution
    #[inline(always)]
    pub fn psarc8(&self) -> Psarc8R {
        Psarc8R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - SDHI0 and the MSTPCRC.MSTPC12 bit security attribution
    #[inline(always)]
    pub fn psarc12(&self) -> Psarc12R {
        Psarc12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DOC and the MSTPCRC.MSTPC13 bit security attribution
    #[inline(always)]
    pub fn psarc13(&self) -> Psarc13R {
        Psarc13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 27 - CANFD0 and the MSTPCRC.MSTPC27 bit security attribution
    #[inline(always)]
    pub fn psarc27(&self) -> Psarc27R {
        Psarc27R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 31 - SCE9 and the MSTPCRC.MSTPC31 bit security attribution
    #[inline(always)]
    pub fn psarc31(&self) -> Psarc31R {
        Psarc31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSARC")
            .field("psarc0", &self.psarc0())
            .field("psarc1", &self.psarc1())
            .field("psarc3", &self.psarc3())
            .field("psarc8", &self.psarc8())
            .field("psarc12", &self.psarc12())
            .field("psarc13", &self.psarc13())
            .field("psarc27", &self.psarc27())
            .field("psarc31", &self.psarc31())
            .finish()
    }
}
impl W {
    ///Bit 0 - CAC and the MSTPCRC.MSTPC0 bit security attribution
    #[inline(always)]
    pub fn psarc0(&mut self) -> Psarc0W<PsarcSpec> {
        Psarc0W::new(self, 0)
    }
    ///Bit 1 - CRC and the MSTPCRC.MSTPC1 bit security attribution
    #[inline(always)]
    pub fn psarc1(&mut self) -> Psarc1W<PsarcSpec> {
        Psarc1W::new(self, 1)
    }
    ///Bit 3 - CTSU and the MSTPCRC.MSTPC3 bit security attribution
    #[inline(always)]
    pub fn psarc3(&mut self) -> Psarc3W<PsarcSpec> {
        Psarc3W::new(self, 3)
    }
    ///Bit 8 - SSIE0 and the MSTPCRC.MSTPC8 bit security attribution
    #[inline(always)]
    pub fn psarc8(&mut self) -> Psarc8W<PsarcSpec> {
        Psarc8W::new(self, 8)
    }
    ///Bit 12 - SDHI0 and the MSTPCRC.MSTPC12 bit security attribution
    #[inline(always)]
    pub fn psarc12(&mut self) -> Psarc12W<PsarcSpec> {
        Psarc12W::new(self, 12)
    }
    ///Bit 13 - DOC and the MSTPCRC.MSTPC13 bit security attribution
    #[inline(always)]
    pub fn psarc13(&mut self) -> Psarc13W<PsarcSpec> {
        Psarc13W::new(self, 13)
    }
    ///Bit 27 - CANFD0 and the MSTPCRC.MSTPC27 bit security attribution
    #[inline(always)]
    pub fn psarc27(&mut self) -> Psarc27W<PsarcSpec> {
        Psarc27W::new(self, 27)
    }
    ///Bit 31 - SCE9 and the MSTPCRC.MSTPC31 bit security attribution
    #[inline(always)]
    pub fn psarc31(&mut self) -> Psarc31W<PsarcSpec> {
        Psarc31W::new(self, 31)
    }
}
/**Peripheral Security Attribution Register C

You can [`read`](crate::Reg::read) this register and get [`psarc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psarc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PsarcSpec;
impl crate::RegisterSpec for PsarcSpec {
    type Ux = u32;
}
///`read()` method returns [`psarc::R`](R) reader structure
impl crate::Readable for PsarcSpec {}
///`write(|w| ..)` method takes [`psarc::W`](W) writer structure
impl crate::Writable for PsarcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PSARC to value 0xffff_ffff
impl crate::Resettable for PsarcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
