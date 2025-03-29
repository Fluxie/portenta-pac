///Register `CS%sMOD` reader
pub type R = crate::R<CsmodSpec>;
///Register `CS%sMOD` writer
pub type W = crate::W<CsmodSpec>;
/**Write Access Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrmod {
    ///0: Byte strobe mode
    _0 = 0,
    ///1: Single write strobe mode
    _1 = 1,
}
impl From<Wrmod> for bool {
    #[inline(always)]
    fn from(variant: Wrmod) -> Self {
        variant as u8 != 0
    }
}
///Field `WRMOD` reader - Write Access Mode Select
pub type WrmodR = crate::BitReader<Wrmod>;
impl WrmodR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wrmod {
        match self.bits {
            false => Wrmod::_0,
            true => Wrmod::_1,
        }
    }
    ///Byte strobe mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wrmod::_0
    }
    ///Single write strobe mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wrmod::_1
    }
}
///Field `WRMOD` writer - Write Access Mode Select
pub type WrmodW<'a, REG> = crate::BitWriter<'a, REG, Wrmod>;
impl<'a, REG> WrmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Byte strobe mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wrmod::_0)
    }
    ///Single write strobe mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wrmod::_1)
    }
}
/**External Wait Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewenb {
    ///0: External wait is disabled.
    _0 = 0,
    ///1: External wait is enabled.
    _1 = 1,
}
impl From<Ewenb> for bool {
    #[inline(always)]
    fn from(variant: Ewenb) -> Self {
        variant as u8 != 0
    }
}
///Field `EWENB` reader - External Wait Enable
pub type EwenbR = crate::BitReader<Ewenb>;
impl EwenbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ewenb {
        match self.bits {
            false => Ewenb::_0,
            true => Ewenb::_1,
        }
    }
    ///External wait is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ewenb::_0
    }
    ///External wait is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ewenb::_1
    }
}
///Field `EWENB` writer - External Wait Enable
pub type EwenbW<'a, REG> = crate::BitWriter<'a, REG, Ewenb>;
impl<'a, REG> EwenbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External wait is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ewenb::_0)
    }
    ///External wait is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewenb::_1)
    }
}
/**Page Read Access Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prenb {
    ///0: Page read access is disabled.
    _0 = 0,
    ///1: Page read access is enabled.
    _1 = 1,
}
impl From<Prenb> for bool {
    #[inline(always)]
    fn from(variant: Prenb) -> Self {
        variant as u8 != 0
    }
}
///Field `PRENB` reader - Page Read Access Enable
pub type PrenbR = crate::BitReader<Prenb>;
impl PrenbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Prenb {
        match self.bits {
            false => Prenb::_0,
            true => Prenb::_1,
        }
    }
    ///Page read access is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prenb::_0
    }
    ///Page read access is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prenb::_1
    }
}
///Field `PRENB` writer - Page Read Access Enable
pub type PrenbW<'a, REG> = crate::BitWriter<'a, REG, Prenb>;
impl<'a, REG> PrenbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Page read access is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Prenb::_0)
    }
    ///Page read access is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Prenb::_1)
    }
}
/**Page Write Access Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwenb {
    ///0: Page write access is disabled.
    _0 = 0,
    ///1: Page write access is enabled.
    _1 = 1,
}
impl From<Pwenb> for bool {
    #[inline(always)]
    fn from(variant: Pwenb) -> Self {
        variant as u8 != 0
    }
}
///Field `PWENB` reader - Page Write Access Enable
pub type PwenbR = crate::BitReader<Pwenb>;
impl PwenbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pwenb {
        match self.bits {
            false => Pwenb::_0,
            true => Pwenb::_1,
        }
    }
    ///Page write access is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pwenb::_0
    }
    ///Page write access is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pwenb::_1
    }
}
///Field `PWENB` writer - Page Write Access Enable
pub type PwenbW<'a, REG> = crate::BitWriter<'a, REG, Pwenb>;
impl<'a, REG> PwenbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Page write access is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwenb::_0)
    }
    ///Page write access is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwenb::_1)
    }
}
/**Page Read Access Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prmod {
    ///0: Normal access compatible mode
    _0 = 0,
    ///1: External data read continuous assertion mode
    _1 = 1,
}
impl From<Prmod> for bool {
    #[inline(always)]
    fn from(variant: Prmod) -> Self {
        variant as u8 != 0
    }
}
///Field `PRMOD` reader - Page Read Access Mode Select
pub type PrmodR = crate::BitReader<Prmod>;
impl PrmodR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Prmod {
        match self.bits {
            false => Prmod::_0,
            true => Prmod::_1,
        }
    }
    ///Normal access compatible mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prmod::_0
    }
    ///External data read continuous assertion mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prmod::_1
    }
}
///Field `PRMOD` writer - Page Read Access Mode Select
pub type PrmodW<'a, REG> = crate::BitWriter<'a, REG, Prmod>;
impl<'a, REG> PrmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal access compatible mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Prmod::_0)
    }
    ///External data read continuous assertion mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Prmod::_1)
    }
}
impl R {
    ///Bit 0 - Write Access Mode Select
    #[inline(always)]
    pub fn wrmod(&self) -> WrmodR {
        WrmodR::new((self.bits & 1) != 0)
    }
    ///Bit 3 - External Wait Enable
    #[inline(always)]
    pub fn ewenb(&self) -> EwenbR {
        EwenbR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Page Read Access Enable
    #[inline(always)]
    pub fn prenb(&self) -> PrenbR {
        PrenbR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Page Write Access Enable
    #[inline(always)]
    pub fn pwenb(&self) -> PwenbR {
        PwenbR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - Page Read Access Mode Select
    #[inline(always)]
    pub fn prmod(&self) -> PrmodR {
        PrmodR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSMOD")
            .field("wrmod", &self.wrmod())
            .field("ewenb", &self.ewenb())
            .field("prenb", &self.prenb())
            .field("pwenb", &self.pwenb())
            .field("prmod", &self.prmod())
            .finish()
    }
}
impl W {
    ///Bit 0 - Write Access Mode Select
    #[inline(always)]
    pub fn wrmod(&mut self) -> WrmodW<CsmodSpec> {
        WrmodW::new(self, 0)
    }
    ///Bit 3 - External Wait Enable
    #[inline(always)]
    pub fn ewenb(&mut self) -> EwenbW<CsmodSpec> {
        EwenbW::new(self, 3)
    }
    ///Bit 8 - Page Read Access Enable
    #[inline(always)]
    pub fn prenb(&mut self) -> PrenbW<CsmodSpec> {
        PrenbW::new(self, 8)
    }
    ///Bit 9 - Page Write Access Enable
    #[inline(always)]
    pub fn pwenb(&mut self) -> PwenbW<CsmodSpec> {
        PwenbW::new(self, 9)
    }
    ///Bit 15 - Page Read Access Mode Select
    #[inline(always)]
    pub fn prmod(&mut self) -> PrmodW<CsmodSpec> {
        PrmodW::new(self, 15)
    }
}
/**CS%s Mode Register

You can [`read`](crate::Reg::read) this register and get [`csmod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csmod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CsmodSpec;
impl crate::RegisterSpec for CsmodSpec {
    type Ux = u16;
}
///`read()` method returns [`csmod::R`](R) reader structure
impl crate::Readable for CsmodSpec {}
///`write(|w| ..)` method takes [`csmod::W`](W) writer structure
impl crate::Writable for CsmodSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CS%sMOD to value 0
impl crate::Resettable for CsmodSpec {}
