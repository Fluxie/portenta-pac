///Register `SNZEDCR0` reader
pub type R = crate::R<Snzedcr0Spec>;
///Register `SNZEDCR0` writer
pub type W = crate::W<Snzedcr0Spec>;
/**AGT1 Underflow Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agtunfed {
    ///0: Disable the snooze end request
    _0 = 0,
    ///1: Enable the snooze end request
    _1 = 1,
}
impl From<Agtunfed> for bool {
    #[inline(always)]
    fn from(variant: Agtunfed) -> Self {
        variant as u8 != 0
    }
}
///Field `AGTUNFED` reader - AGT1 Underflow Snooze End Enable
pub type AgtunfedR = crate::BitReader<Agtunfed>;
impl AgtunfedR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Agtunfed {
        match self.bits {
            false => Agtunfed::_0,
            true => Agtunfed::_1,
        }
    }
    ///Disable the snooze end request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agtunfed::_0
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agtunfed::_1
    }
}
///Field `AGTUNFED` writer - AGT1 Underflow Snooze End Enable
pub type AgtunfedW<'a, REG> = crate::BitWriter<'a, REG, Agtunfed>;
impl<'a, REG> AgtunfedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze end request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agtunfed::_0)
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agtunfed::_1)
    }
}
/**Last DTC Transmission Completion Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtczred {
    ///0: Disable the snooze end request
    _0 = 0,
    ///1: Enable the snooze end request
    _1 = 1,
}
impl From<Dtczred> for bool {
    #[inline(always)]
    fn from(variant: Dtczred) -> Self {
        variant as u8 != 0
    }
}
///Field `DTCZRED` reader - Last DTC Transmission Completion Snooze End Enable
pub type DtczredR = crate::BitReader<Dtczred>;
impl DtczredR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dtczred {
        match self.bits {
            false => Dtczred::_0,
            true => Dtczred::_1,
        }
    }
    ///Disable the snooze end request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtczred::_0
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtczred::_1
    }
}
///Field `DTCZRED` writer - Last DTC Transmission Completion Snooze End Enable
pub type DtczredW<'a, REG> = crate::BitWriter<'a, REG, Dtczred>;
impl<'a, REG> DtczredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze end request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtczred::_0)
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtczred::_1)
    }
}
/**Not Last DTC Transmission Completion Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtcnzred {
    ///0: Disable the snooze end request
    _0 = 0,
    ///1: Enable the snooze end request
    _1 = 1,
}
impl From<Dtcnzred> for bool {
    #[inline(always)]
    fn from(variant: Dtcnzred) -> Self {
        variant as u8 != 0
    }
}
///Field `DTCNZRED` reader - Not Last DTC Transmission Completion Snooze End Enable
pub type DtcnzredR = crate::BitReader<Dtcnzred>;
impl DtcnzredR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dtcnzred {
        match self.bits {
            false => Dtcnzred::_0,
            true => Dtcnzred::_1,
        }
    }
    ///Disable the snooze end request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtcnzred::_0
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtcnzred::_1
    }
}
///Field `DTCNZRED` writer - Not Last DTC Transmission Completion Snooze End Enable
pub type DtcnzredW<'a, REG> = crate::BitWriter<'a, REG, Dtcnzred>;
impl<'a, REG> DtcnzredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze end request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcnzred::_0)
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcnzred::_1)
    }
}
/**ADC120 Compare Match Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ad0mated {
    ///0: Disable the snooze end request
    _0 = 0,
    ///1: Enable the snooze end request
    _1 = 1,
}
impl From<Ad0mated> for bool {
    #[inline(always)]
    fn from(variant: Ad0mated) -> Self {
        variant as u8 != 0
    }
}
///Field `AD0MATED` reader - ADC120 Compare Match Snooze End Enable
pub type Ad0matedR = crate::BitReader<Ad0mated>;
impl Ad0matedR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ad0mated {
        match self.bits {
            false => Ad0mated::_0,
            true => Ad0mated::_1,
        }
    }
    ///Disable the snooze end request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ad0mated::_0
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ad0mated::_1
    }
}
///Field `AD0MATED` writer - ADC120 Compare Match Snooze End Enable
pub type Ad0matedW<'a, REG> = crate::BitWriter<'a, REG, Ad0mated>;
impl<'a, REG> Ad0matedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze end request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ad0mated::_0)
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ad0mated::_1)
    }
}
/**ADC120 Compare Mismatch Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ad0umted {
    ///0: Disable the snooze end request
    _0 = 0,
    ///1: Enable the snooze end request
    _1 = 1,
}
impl From<Ad0umted> for bool {
    #[inline(always)]
    fn from(variant: Ad0umted) -> Self {
        variant as u8 != 0
    }
}
///Field `AD0UMTED` reader - ADC120 Compare Mismatch Snooze End Enable
pub type Ad0umtedR = crate::BitReader<Ad0umted>;
impl Ad0umtedR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ad0umted {
        match self.bits {
            false => Ad0umted::_0,
            true => Ad0umted::_1,
        }
    }
    ///Disable the snooze end request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ad0umted::_0
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ad0umted::_1
    }
}
///Field `AD0UMTED` writer - ADC120 Compare Mismatch Snooze End Enable
pub type Ad0umtedW<'a, REG> = crate::BitWriter<'a, REG, Ad0umted>;
impl<'a, REG> Ad0umtedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze end request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ad0umted::_0)
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ad0umted::_1)
    }
}
/**ADC121 Compare Match Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ad1mated {
    ///0: Disable the snooze end request
    _0 = 0,
    ///1: Enable the snooze end request
    _1 = 1,
}
impl From<Ad1mated> for bool {
    #[inline(always)]
    fn from(variant: Ad1mated) -> Self {
        variant as u8 != 0
    }
}
///Field `AD1MATED` reader - ADC121 Compare Match Snooze End Enable
pub type Ad1matedR = crate::BitReader<Ad1mated>;
impl Ad1matedR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ad1mated {
        match self.bits {
            false => Ad1mated::_0,
            true => Ad1mated::_1,
        }
    }
    ///Disable the snooze end request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ad1mated::_0
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ad1mated::_1
    }
}
///Field `AD1MATED` writer - ADC121 Compare Match Snooze End Enable
pub type Ad1matedW<'a, REG> = crate::BitWriter<'a, REG, Ad1mated>;
impl<'a, REG> Ad1matedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze end request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ad1mated::_0)
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ad1mated::_1)
    }
}
/**ADC121 Compare Mismatch Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ad1umted {
    ///0: Disable the snooze end request
    _0 = 0,
    ///1: Enable the snooze end request
    _1 = 1,
}
impl From<Ad1umted> for bool {
    #[inline(always)]
    fn from(variant: Ad1umted) -> Self {
        variant as u8 != 0
    }
}
///Field `AD1UMTED` reader - ADC121 Compare Mismatch Snooze End Enable
pub type Ad1umtedR = crate::BitReader<Ad1umted>;
impl Ad1umtedR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ad1umted {
        match self.bits {
            false => Ad1umted::_0,
            true => Ad1umted::_1,
        }
    }
    ///Disable the snooze end request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ad1umted::_0
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ad1umted::_1
    }
}
///Field `AD1UMTED` writer - ADC121 Compare Mismatch Snooze End Enable
pub type Ad1umtedW<'a, REG> = crate::BitWriter<'a, REG, Ad1umted>;
impl<'a, REG> Ad1umtedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze end request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ad1umted::_0)
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ad1umted::_1)
    }
}
/**SCI0 Address Mismatch Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sci0umted {
    ///0: Disable the snooze end request
    _0 = 0,
    ///1: Enable the snooze end request
    _1 = 1,
}
impl From<Sci0umted> for bool {
    #[inline(always)]
    fn from(variant: Sci0umted) -> Self {
        variant as u8 != 0
    }
}
///Field `SCI0UMTED` reader - SCI0 Address Mismatch Snooze End Enable
pub type Sci0umtedR = crate::BitReader<Sci0umted>;
impl Sci0umtedR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sci0umted {
        match self.bits {
            false => Sci0umted::_0,
            true => Sci0umted::_1,
        }
    }
    ///Disable the snooze end request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sci0umted::_0
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sci0umted::_1
    }
}
///Field `SCI0UMTED` writer - SCI0 Address Mismatch Snooze End Enable
pub type Sci0umtedW<'a, REG> = crate::BitWriter<'a, REG, Sci0umted>;
impl<'a, REG> Sci0umtedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze end request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sci0umted::_0)
    }
    ///Enable the snooze end request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sci0umted::_1)
    }
}
impl R {
    ///Bit 0 - AGT1 Underflow Snooze End Enable
    #[inline(always)]
    pub fn agtunfed(&self) -> AgtunfedR {
        AgtunfedR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Last DTC Transmission Completion Snooze End Enable
    #[inline(always)]
    pub fn dtczred(&self) -> DtczredR {
        DtczredR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Not Last DTC Transmission Completion Snooze End Enable
    #[inline(always)]
    pub fn dtcnzred(&self) -> DtcnzredR {
        DtcnzredR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC120 Compare Match Snooze End Enable
    #[inline(always)]
    pub fn ad0mated(&self) -> Ad0matedR {
        Ad0matedR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC120 Compare Mismatch Snooze End Enable
    #[inline(always)]
    pub fn ad0umted(&self) -> Ad0umtedR {
        Ad0umtedR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC121 Compare Match Snooze End Enable
    #[inline(always)]
    pub fn ad1mated(&self) -> Ad1matedR {
        Ad1matedR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ADC121 Compare Mismatch Snooze End Enable
    #[inline(always)]
    pub fn ad1umted(&self) -> Ad1umtedR {
        Ad1umtedR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SCI0 Address Mismatch Snooze End Enable
    #[inline(always)]
    pub fn sci0umted(&self) -> Sci0umtedR {
        Sci0umtedR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNZEDCR0")
            .field("agtunfed", &self.agtunfed())
            .field("dtczred", &self.dtczred())
            .field("dtcnzred", &self.dtcnzred())
            .field("ad0mated", &self.ad0mated())
            .field("ad0umted", &self.ad0umted())
            .field("ad1mated", &self.ad1mated())
            .field("ad1umted", &self.ad1umted())
            .field("sci0umted", &self.sci0umted())
            .finish()
    }
}
impl W {
    ///Bit 0 - AGT1 Underflow Snooze End Enable
    #[inline(always)]
    pub fn agtunfed(&mut self) -> AgtunfedW<Snzedcr0Spec> {
        AgtunfedW::new(self, 0)
    }
    ///Bit 1 - Last DTC Transmission Completion Snooze End Enable
    #[inline(always)]
    pub fn dtczred(&mut self) -> DtczredW<Snzedcr0Spec> {
        DtczredW::new(self, 1)
    }
    ///Bit 2 - Not Last DTC Transmission Completion Snooze End Enable
    #[inline(always)]
    pub fn dtcnzred(&mut self) -> DtcnzredW<Snzedcr0Spec> {
        DtcnzredW::new(self, 2)
    }
    ///Bit 3 - ADC120 Compare Match Snooze End Enable
    #[inline(always)]
    pub fn ad0mated(&mut self) -> Ad0matedW<Snzedcr0Spec> {
        Ad0matedW::new(self, 3)
    }
    ///Bit 4 - ADC120 Compare Mismatch Snooze End Enable
    #[inline(always)]
    pub fn ad0umted(&mut self) -> Ad0umtedW<Snzedcr0Spec> {
        Ad0umtedW::new(self, 4)
    }
    ///Bit 5 - ADC121 Compare Match Snooze End Enable
    #[inline(always)]
    pub fn ad1mated(&mut self) -> Ad1matedW<Snzedcr0Spec> {
        Ad1matedW::new(self, 5)
    }
    ///Bit 6 - ADC121 Compare Mismatch Snooze End Enable
    #[inline(always)]
    pub fn ad1umted(&mut self) -> Ad1umtedW<Snzedcr0Spec> {
        Ad1umtedW::new(self, 6)
    }
    ///Bit 7 - SCI0 Address Mismatch Snooze End Enable
    #[inline(always)]
    pub fn sci0umted(&mut self) -> Sci0umtedW<Snzedcr0Spec> {
        Sci0umtedW::new(self, 7)
    }
}
/**Snooze End Control Register 0

You can [`read`](crate::Reg::read) this register and get [`snzedcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Snzedcr0Spec;
impl crate::RegisterSpec for Snzedcr0Spec {
    type Ux = u8;
}
///`read()` method returns [`snzedcr0::R`](R) reader structure
impl crate::Readable for Snzedcr0Spec {}
///`write(|w| ..)` method takes [`snzedcr0::W`](W) writer structure
impl crate::Writable for Snzedcr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SNZEDCR0 to value 0
impl crate::Resettable for Snzedcr0Spec {}
