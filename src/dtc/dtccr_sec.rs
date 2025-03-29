///Register `DTCCR_SEC` reader
pub type R = crate::R<DtccrSecSpec>;
///Register `DTCCR_SEC` writer
pub type W = crate::W<DtccrSecSpec>;
/**DTC Transfer Information Read Skip Enable for Secure

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrs {
    ///0: Transfer information read is not skipped.
    _0 = 0,
    ///1: Transfer information read is skipped when vector numbers match.
    _1 = 1,
}
impl From<Rrs> for bool {
    #[inline(always)]
    fn from(variant: Rrs) -> Self {
        variant as u8 != 0
    }
}
///Field `RRS` reader - DTC Transfer Information Read Skip Enable for Secure
pub type RrsR = crate::BitReader<Rrs>;
impl RrsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rrs {
        match self.bits {
            false => Rrs::_0,
            true => Rrs::_1,
        }
    }
    ///Transfer information read is not skipped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rrs::_0
    }
    ///Transfer information read is skipped when vector numbers match.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rrs::_1
    }
}
///Field `RRS` writer - DTC Transfer Information Read Skip Enable for Secure
pub type RrsW<'a, REG> = crate::BitWriter<'a, REG, Rrs>;
impl<'a, REG> RrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer information read is not skipped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::_0)
    }
    ///Transfer information read is skipped when vector numbers match.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::_1)
    }
}
impl R {
    ///Bit 4 - DTC Transfer Information Read Skip Enable for Secure
    #[inline(always)]
    pub fn rrs(&self) -> RrsR {
        RrsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCCR_SEC").field("rrs", &self.rrs()).finish()
    }
}
impl W {
    ///Bit 4 - DTC Transfer Information Read Skip Enable for Secure
    #[inline(always)]
    pub fn rrs(&mut self) -> RrsW<DtccrSecSpec> {
        RrsW::new(self, 4)
    }
}
/**DTC Control Register for secure Region

You can [`read`](crate::Reg::read) this register and get [`dtccr_sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtccr_sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DtccrSecSpec;
impl crate::RegisterSpec for DtccrSecSpec {
    type Ux = u8;
}
///`read()` method returns [`dtccr_sec::R`](R) reader structure
impl crate::Readable for DtccrSecSpec {}
///`write(|w| ..)` method takes [`dtccr_sec::W`](W) writer structure
impl crate::Writable for DtccrSecSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTCCR_SEC to value 0x08
impl crate::Resettable for DtccrSecSpec {
    const RESET_VALUE: u8 = 0x08;
}
