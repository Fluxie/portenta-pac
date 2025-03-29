///Register `RDAYAR` reader
pub type R = crate::R<RdayarSpec>;
///Register `RDAYAR` writer
pub type W = crate::W<RdayarSpec>;
///Field `DATE1` reader - 1 Day
pub type Date1R = crate::FieldReader;
///Field `DATE1` writer - 1 Day
pub type Date1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATE10` reader - 10 Days
pub type Date10R = crate::FieldReader;
///Field `DATE10` writer - 10 Days
pub type Date10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**ENB

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb {
    ///0: Do not compare register value with RDAYCNT counter value
    _0 = 0,
    ///1: Compare register value with RDAYCNT counter value
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
    ///Do not compare register value with RDAYCNT counter value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enb::_0
    }
    ///Compare register value with RDAYCNT counter value
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
    ///Do not compare register value with RDAYCNT counter value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_0)
    }
    ///Compare register value with RDAYCNT counter value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_1)
    }
}
impl R {
    ///Bits 0:3 - 1 Day
    #[inline(always)]
    pub fn date1(&self) -> Date1R {
        Date1R::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - 10 Days
    #[inline(always)]
    pub fn date10(&self) -> Date10R {
        Date10R::new((self.bits >> 4) & 3)
    }
    ///Bit 7 - ENB
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDAYAR")
            .field("date1", &self.date1())
            .field("date10", &self.date10())
            .field("enb", &self.enb())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 1 Day
    #[inline(always)]
    pub fn date1(&mut self) -> Date1W<RdayarSpec> {
        Date1W::new(self, 0)
    }
    ///Bits 4:5 - 10 Days
    #[inline(always)]
    pub fn date10(&mut self) -> Date10W<RdayarSpec> {
        Date10W::new(self, 4)
    }
    ///Bit 7 - ENB
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<RdayarSpec> {
        EnbW::new(self, 7)
    }
}
/**Date Alarm Register (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rdayar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdayar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdayarSpec;
impl crate::RegisterSpec for RdayarSpec {
    type Ux = u8;
}
///`read()` method returns [`rdayar::R`](R) reader structure
impl crate::Readable for RdayarSpec {}
///`write(|w| ..)` method takes [`rdayar::W`](W) writer structure
impl crate::Writable for RdayarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RDAYAR to value 0
impl crate::Resettable for RdayarSpec {}
