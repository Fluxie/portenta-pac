///Register `ICR` reader
pub type R = crate::R<IcrSpec>;
///Register `ICR` writer
pub type W = crate::W<IcrSpec>;
/**Break Field Low Width Detected Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfdie {
    ///0: Interrupts on detection of the low width for a Break Field are disabled.
    _0 = 0,
    ///1: Interrupts on detection of the low width for a Break Field are enabled.
    _1 = 1,
}
impl From<Bfdie> for bool {
    #[inline(always)]
    fn from(variant: Bfdie) -> Self {
        variant as u8 != 0
    }
}
///Field `BFDIE` reader - Break Field Low Width Detected Interrupt Enable
pub type BfdieR = crate::BitReader<Bfdie>;
impl BfdieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bfdie {
        match self.bits {
            false => Bfdie::_0,
            true => Bfdie::_1,
        }
    }
    ///Interrupts on detection of the low width for a Break Field are disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bfdie::_0
    }
    ///Interrupts on detection of the low width for a Break Field are enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bfdie::_1
    }
}
///Field `BFDIE` writer - Break Field Low Width Detected Interrupt Enable
pub type BfdieW<'a, REG> = crate::BitWriter<'a, REG, Bfdie>;
impl<'a, REG> BfdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts on detection of the low width for a Break Field are disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bfdie::_0)
    }
    ///Interrupts on detection of the low width for a Break Field are enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfdie::_1)
    }
}
/**Control Field 0 Match Detected Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf0mie {
    ///0: Interrupts on detection of a match with Control Field 0 are disabled.
    _0 = 0,
    ///1: Interrupts on detection of a match with Control Field 0 are enabled.
    _1 = 1,
}
impl From<Cf0mie> for bool {
    #[inline(always)]
    fn from(variant: Cf0mie) -> Self {
        variant as u8 != 0
    }
}
///Field `CF0MIE` reader - Control Field 0 Match Detected Interrupt Enable
pub type Cf0mieR = crate::BitReader<Cf0mie>;
impl Cf0mieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf0mie {
        match self.bits {
            false => Cf0mie::_0,
            true => Cf0mie::_1,
        }
    }
    ///Interrupts on detection of a match with Control Field 0 are disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf0mie::_0
    }
    ///Interrupts on detection of a match with Control Field 0 are enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf0mie::_1
    }
}
///Field `CF0MIE` writer - Control Field 0 Match Detected Interrupt Enable
pub type Cf0mieW<'a, REG> = crate::BitWriter<'a, REG, Cf0mie>;
impl<'a, REG> Cf0mieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts on detection of a match with Control Field 0 are disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0mie::_0)
    }
    ///Interrupts on detection of a match with Control Field 0 are enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0mie::_1)
    }
}
/**Control Field 1 Match Detected Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf1mie {
    ///0: Interrupts on detection of a match with Control Field 1 are disabled.
    _0 = 0,
    ///1: Interrupts on detection of a match with Control Field 1 are enabled.
    _1 = 1,
}
impl From<Cf1mie> for bool {
    #[inline(always)]
    fn from(variant: Cf1mie) -> Self {
        variant as u8 != 0
    }
}
///Field `CF1MIE` reader - Control Field 1 Match Detected Interrupt Enable
pub type Cf1mieR = crate::BitReader<Cf1mie>;
impl Cf1mieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf1mie {
        match self.bits {
            false => Cf1mie::_0,
            true => Cf1mie::_1,
        }
    }
    ///Interrupts on detection of a match with Control Field 1 are disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf1mie::_0
    }
    ///Interrupts on detection of a match with Control Field 1 are enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf1mie::_1
    }
}
///Field `CF1MIE` writer - Control Field 1 Match Detected Interrupt Enable
pub type Cf1mieW<'a, REG> = crate::BitWriter<'a, REG, Cf1mie>;
impl<'a, REG> Cf1mieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts on detection of a match with Control Field 1 are disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1mie::_0)
    }
    ///Interrupts on detection of a match with Control Field 1 are enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1mie::_1)
    }
}
/**Priority Interrupt Bit Detected Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pibdie {
    ///0: Interrupts on detection of the priority interrupt bit are disabled.
    _0 = 0,
    ///1: Interrupts on detection of the priority interrupt bit are enabled.
    _1 = 1,
}
impl From<Pibdie> for bool {
    #[inline(always)]
    fn from(variant: Pibdie) -> Self {
        variant as u8 != 0
    }
}
///Field `PIBDIE` reader - Priority Interrupt Bit Detected Interrupt Enable
pub type PibdieR = crate::BitReader<Pibdie>;
impl PibdieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pibdie {
        match self.bits {
            false => Pibdie::_0,
            true => Pibdie::_1,
        }
    }
    ///Interrupts on detection of the priority interrupt bit are disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pibdie::_0
    }
    ///Interrupts on detection of the priority interrupt bit are enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pibdie::_1
    }
}
///Field `PIBDIE` writer - Priority Interrupt Bit Detected Interrupt Enable
pub type PibdieW<'a, REG> = crate::BitWriter<'a, REG, Pibdie>;
impl<'a, REG> PibdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts on detection of the priority interrupt bit are disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pibdie::_0)
    }
    ///Interrupts on detection of the priority interrupt bit are enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pibdie::_1)
    }
}
/**Bus Collision Detected Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcdie {
    ///0: Interrupts on detection of a bus collision are disabled.
    _0 = 0,
    ///1: Interrupts on detection of a bus collision are enabled.
    _1 = 1,
}
impl From<Bcdie> for bool {
    #[inline(always)]
    fn from(variant: Bcdie) -> Self {
        variant as u8 != 0
    }
}
///Field `BCDIE` reader - Bus Collision Detected Interrupt Enable
pub type BcdieR = crate::BitReader<Bcdie>;
impl BcdieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bcdie {
        match self.bits {
            false => Bcdie::_0,
            true => Bcdie::_1,
        }
    }
    ///Interrupts on detection of a bus collision are disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bcdie::_0
    }
    ///Interrupts on detection of a bus collision are enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bcdie::_1
    }
}
///Field `BCDIE` writer - Bus Collision Detected Interrupt Enable
pub type BcdieW<'a, REG> = crate::BitWriter<'a, REG, Bcdie>;
impl<'a, REG> BcdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts on detection of a bus collision are disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdie::_0)
    }
    ///Interrupts on detection of a bus collision are enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdie::_1)
    }
}
/**Valid Edge Detected Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aedie {
    ///0: Interrupts on detection of a valid edge are disabled.
    _0 = 0,
    ///1: Interrupts on detection of a valid edge are enabled.
    _1 = 1,
}
impl From<Aedie> for bool {
    #[inline(always)]
    fn from(variant: Aedie) -> Self {
        variant as u8 != 0
    }
}
///Field `AEDIE` reader - Valid Edge Detected Interrupt Enable
pub type AedieR = crate::BitReader<Aedie>;
impl AedieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aedie {
        match self.bits {
            false => Aedie::_0,
            true => Aedie::_1,
        }
    }
    ///Interrupts on detection of a valid edge are disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aedie::_0
    }
    ///Interrupts on detection of a valid edge are enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aedie::_1
    }
}
///Field `AEDIE` writer - Valid Edge Detected Interrupt Enable
pub type AedieW<'a, REG> = crate::BitWriter<'a, REG, Aedie>;
impl<'a, REG> AedieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts on detection of a valid edge are disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aedie::_0)
    }
    ///Interrupts on detection of a valid edge are enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aedie::_1)
    }
}
impl R {
    ///Bit 0 - Break Field Low Width Detected Interrupt Enable
    #[inline(always)]
    pub fn bfdie(&self) -> BfdieR {
        BfdieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Control Field 0 Match Detected Interrupt Enable
    #[inline(always)]
    pub fn cf0mie(&self) -> Cf0mieR {
        Cf0mieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Control Field 1 Match Detected Interrupt Enable
    #[inline(always)]
    pub fn cf1mie(&self) -> Cf1mieR {
        Cf1mieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Priority Interrupt Bit Detected Interrupt Enable
    #[inline(always)]
    pub fn pibdie(&self) -> PibdieR {
        PibdieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Bus Collision Detected Interrupt Enable
    #[inline(always)]
    pub fn bcdie(&self) -> BcdieR {
        BcdieR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Valid Edge Detected Interrupt Enable
    #[inline(always)]
    pub fn aedie(&self) -> AedieR {
        AedieR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("bfdie", &self.bfdie())
            .field("cf0mie", &self.cf0mie())
            .field("cf1mie", &self.cf1mie())
            .field("pibdie", &self.pibdie())
            .field("bcdie", &self.bcdie())
            .field("aedie", &self.aedie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Break Field Low Width Detected Interrupt Enable
    #[inline(always)]
    pub fn bfdie(&mut self) -> BfdieW<IcrSpec> {
        BfdieW::new(self, 0)
    }
    ///Bit 1 - Control Field 0 Match Detected Interrupt Enable
    #[inline(always)]
    pub fn cf0mie(&mut self) -> Cf0mieW<IcrSpec> {
        Cf0mieW::new(self, 1)
    }
    ///Bit 2 - Control Field 1 Match Detected Interrupt Enable
    #[inline(always)]
    pub fn cf1mie(&mut self) -> Cf1mieW<IcrSpec> {
        Cf1mieW::new(self, 2)
    }
    ///Bit 3 - Priority Interrupt Bit Detected Interrupt Enable
    #[inline(always)]
    pub fn pibdie(&mut self) -> PibdieW<IcrSpec> {
        PibdieW::new(self, 3)
    }
    ///Bit 4 - Bus Collision Detected Interrupt Enable
    #[inline(always)]
    pub fn bcdie(&mut self) -> BcdieW<IcrSpec> {
        BcdieW::new(self, 4)
    }
    ///Bit 5 - Valid Edge Detected Interrupt Enable
    #[inline(always)]
    pub fn aedie(&mut self) -> AedieW<IcrSpec> {
        AedieW::new(self, 5)
    }
}
/**Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u8;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for IcrSpec {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for IcrSpec {}
