///Register `TRSCER` reader
pub type R = crate::R<TrscerSpec>;
///Register `TRSCER` writer
pub type W = crate::W<TrscerSpec>;
/**RRF Flag Copy Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrfce {
    ///0: Reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor
    _0 = 0,
    ///1: Do not reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor.
    _1 = 1,
}
impl From<Rrfce> for bool {
    #[inline(always)]
    fn from(variant: Rrfce) -> Self {
        variant as u8 != 0
    }
}
///Field `RRFCE` reader - RRF Flag Copy Enable
pub type RrfceR = crate::BitReader<Rrfce>;
impl RrfceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rrfce {
        match self.bits {
            false => Rrfce::_0,
            true => Rrfce::_1,
        }
    }
    ///Reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rrfce::_0
    }
    ///Do not reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rrfce::_1
    }
}
///Field `RRFCE` writer - RRF Flag Copy Enable
pub type RrfceW<'a, REG> = crate::BitWriter<'a, REG, Rrfce>;
impl<'a, REG> RrfceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfce::_0)
    }
    ///Do not reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfce::_1)
    }
}
/**RMAF Flag Copy Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmafce {
    ///0: Reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor
    _0 = 0,
    ///1: Do not reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor.
    _1 = 1,
}
impl From<Rmafce> for bool {
    #[inline(always)]
    fn from(variant: Rmafce) -> Self {
        variant as u8 != 0
    }
}
///Field `RMAFCE` reader - RMAF Flag Copy Enable
pub type RmafceR = crate::BitReader<Rmafce>;
impl RmafceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rmafce {
        match self.bits {
            false => Rmafce::_0,
            true => Rmafce::_1,
        }
    }
    ///Reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rmafce::_0
    }
    ///Do not reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rmafce::_1
    }
}
///Field `RMAFCE` writer - RMAF Flag Copy Enable
pub type RmafceW<'a, REG> = crate::BitWriter<'a, REG, Rmafce>;
impl<'a, REG> RmafceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rmafce::_0)
    }
    ///Do not reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmafce::_1)
    }
}
impl R {
    ///Bit 4 - RRF Flag Copy Enable
    #[inline(always)]
    pub fn rrfce(&self) -> RrfceR {
        RrfceR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - RMAF Flag Copy Enable
    #[inline(always)]
    pub fn rmafce(&self) -> RmafceR {
        RmafceR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRSCER")
            .field("rrfce", &self.rrfce())
            .field("rmafce", &self.rmafce())
            .finish()
    }
}
impl W {
    ///Bit 4 - RRF Flag Copy Enable
    #[inline(always)]
    pub fn rrfce(&mut self) -> RrfceW<TrscerSpec> {
        RrfceW::new(self, 4)
    }
    ///Bit 7 - RMAF Flag Copy Enable
    #[inline(always)]
    pub fn rmafce(&mut self) -> RmafceW<TrscerSpec> {
        RmafceW::new(self, 7)
    }
}
/**ETHERC/EDMAC Transmit/Receive Status Copy Enable Register

You can [`read`](crate::Reg::read) this register and get [`trscer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trscer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TrscerSpec;
impl crate::RegisterSpec for TrscerSpec {
    type Ux = u32;
}
///`read()` method returns [`trscer::R`](R) reader structure
impl crate::Readable for TrscerSpec {}
///`write(|w| ..)` method takes [`trscer::W`](W) writer structure
impl crate::Writable for TrscerSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRSCER to value 0
impl crate::Resettable for TrscerSpec {}
