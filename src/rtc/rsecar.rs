///Register `RSECAR` reader
pub type R = crate::R<RsecarSpec>;
///Register `RSECAR` writer
pub type W = crate::W<RsecarSpec>;
///Field `SEC1` reader - 1 Second
pub type Sec1R = crate::FieldReader;
///Field `SEC1` writer - 1 Second
pub type Sec1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEC10` reader - 10 Seconds
pub type Sec10R = crate::FieldReader;
///Field `SEC10` writer - 10 Seconds
pub type Sec10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**ENB

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb {
    ///0: Do not compare register value with RSECCNT counter value
    _0 = 0,
    ///1: Compare register value with RSECCNT counter value
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
    ///Do not compare register value with RSECCNT counter value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enb::_0
    }
    ///Compare register value with RSECCNT counter value
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
    ///Do not compare register value with RSECCNT counter value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_0)
    }
    ///Compare register value with RSECCNT counter value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_1)
    }
}
impl R {
    ///Bits 0:3 - 1 Second
    #[inline(always)]
    pub fn sec1(&self) -> Sec1R {
        Sec1R::new(self.bits & 0x0f)
    }
    ///Bits 4:6 - 10 Seconds
    #[inline(always)]
    pub fn sec10(&self) -> Sec10R {
        Sec10R::new((self.bits >> 4) & 7)
    }
    ///Bit 7 - ENB
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSECAR")
            .field("sec1", &self.sec1())
            .field("sec10", &self.sec10())
            .field("enb", &self.enb())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 1 Second
    #[inline(always)]
    pub fn sec1(&mut self) -> Sec1W<RsecarSpec> {
        Sec1W::new(self, 0)
    }
    ///Bits 4:6 - 10 Seconds
    #[inline(always)]
    pub fn sec10(&mut self) -> Sec10W<RsecarSpec> {
        Sec10W::new(self, 4)
    }
    ///Bit 7 - ENB
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<RsecarSpec> {
        EnbW::new(self, 7)
    }
}
/**Second Alarm Register (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rsecar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsecar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RsecarSpec;
impl crate::RegisterSpec for RsecarSpec {
    type Ux = u8;
}
///`read()` method returns [`rsecar::R`](R) reader structure
impl crate::Readable for RsecarSpec {}
///`write(|w| ..)` method takes [`rsecar::W`](W) writer structure
impl crate::Writable for RsecarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSECAR to value 0
impl crate::Resettable for RsecarSpec {}
