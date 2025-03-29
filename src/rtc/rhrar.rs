///Register `RHRAR` reader
pub type R = crate::R<RhrarSpec>;
///Register `RHRAR` writer
pub type W = crate::W<RhrarSpec>;
///Field `HR1` reader - 1 Hour
pub type Hr1R = crate::FieldReader;
///Field `HR1` writer - 1 Hour
pub type Hr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HR10` reader - 10 Hours
pub type Hr10R = crate::FieldReader;
///Field `HR10` writer - 10 Hours
pub type Hr10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**AM/PM select for alarm setting.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pm {
    ///0: AM
    _0 = 0,
    ///1: PM
    _1 = 1,
}
impl From<Pm> for bool {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as u8 != 0
    }
}
///Field `PM` reader - AM/PM select for alarm setting.
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
    ///AM
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pm::_0
    }
    ///PM
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pm::_1
    }
}
///Field `PM` writer - AM/PM select for alarm setting.
pub type PmW<'a, REG> = crate::BitWriter<'a, REG, Pm>;
impl<'a, REG> PmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AM
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_0)
    }
    ///PM
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_1)
    }
}
/**ENB

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb {
    ///0: Do not compare register value with RHRCNT counter value
    _0 = 0,
    ///1: Compare register value with RHRCNT counter value
    _1 = 1,
}
impl From<Enb> for bool {
    #[inline(always)]
    fn from(variant: Enb) -> Self {
        variant as u8 != 0
    }
}
///Field `ENB` reader - ENB
pub type EnbR = crate::BitReader<Enb>;
impl EnbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Enb {
        match self.bits {
            false => Enb::_0,
            true => Enb::_1,
        }
    }
    ///Do not compare register value with RHRCNT counter value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enb::_0
    }
    ///Compare register value with RHRCNT counter value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enb::_1
    }
}
///Field `ENB` writer - ENB
pub type EnbW<'a, REG> = crate::BitWriter<'a, REG, Enb>;
impl<'a, REG> EnbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not compare register value with RHRCNT counter value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_0)
    }
    ///Compare register value with RHRCNT counter value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_1)
    }
}
impl R {
    ///Bits 0:3 - 1 Hour
    #[inline(always)]
    pub fn hr1(&self) -> Hr1R {
        Hr1R::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - 10 Hours
    #[inline(always)]
    pub fn hr10(&self) -> Hr10R {
        Hr10R::new((self.bits >> 4) & 3)
    }
    ///Bit 6 - AM/PM select for alarm setting.
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ENB
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RHRAR")
            .field("hr1", &self.hr1())
            .field("hr10", &self.hr10())
            .field("pm", &self.pm())
            .field("enb", &self.enb())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 1 Hour
    #[inline(always)]
    pub fn hr1(&mut self) -> Hr1W<RhrarSpec> {
        Hr1W::new(self, 0)
    }
    ///Bits 4:5 - 10 Hours
    #[inline(always)]
    pub fn hr10(&mut self) -> Hr10W<RhrarSpec> {
        Hr10W::new(self, 4)
    }
    ///Bit 6 - AM/PM select for alarm setting.
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<RhrarSpec> {
        PmW::new(self, 6)
    }
    ///Bit 7 - ENB
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<RhrarSpec> {
        EnbW::new(self, 7)
    }
}
/**Hour Alarm Register (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rhrar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rhrar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RhrarSpec;
impl crate::RegisterSpec for RhrarSpec {
    type Ux = u8;
}
///`read()` method returns [`rhrar::R`](R) reader structure
impl crate::Readable for RhrarSpec {}
///`write(|w| ..)` method takes [`rhrar::W`](W) writer structure
impl crate::Writable for RhrarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RHRAR to value 0
impl crate::Resettable for RhrarSpec {}
