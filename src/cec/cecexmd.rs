///Register `CECEXMD` reader
pub type R = crate::R<CecexmdSpec>;
///Register `CECEXMD` writer
pub type W = crate::W<CecexmdSpec>;
/**Pulse Output Function Enable by Long Bit Width Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lerplen {
    ///0: Detects only a long bit width error.
    _0 = 0,
    ///1: Detects a long bit width error and outputs an error handling pulse.
    _1 = 1,
}
impl From<Lerplen> for bool {
    #[inline(always)]
    fn from(variant: Lerplen) -> Self {
        variant as u8 != 0
    }
}
///Field `LERPLEN` reader - Pulse Output Function Enable by Long Bit Width Error
pub type LerplenR = crate::BitReader<Lerplen>;
impl LerplenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lerplen {
        match self.bits {
            false => Lerplen::_0,
            true => Lerplen::_1,
        }
    }
    ///Detects only a long bit width error.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lerplen::_0
    }
    ///Detects a long bit width error and outputs an error handling pulse.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lerplen::_1
    }
}
///Field `LERPLEN` writer - Pulse Output Function Enable by Long Bit Width Error
pub type LerplenW<'a, REG> = crate::BitWriter<'a, REG, Lerplen>;
impl<'a, REG> LerplenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detects only a long bit width error.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lerplen::_0)
    }
    ///Detects a long bit width error and outputs an error handling pulse.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lerplen::_1)
    }
}
/**Start Detection Reception Restart Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rercven {
    ///0: Does not restart reception when the start bit is detected during reception.
    _0 = 0,
    ///1: Restarts reception when the start bit is detected during reception.
    _1 = 1,
}
impl From<Rercven> for bool {
    #[inline(always)]
    fn from(variant: Rercven) -> Self {
        variant as u8 != 0
    }
}
///Field `RERCVEN` reader - Start Detection Reception Restart Enable
pub type RercvenR = crate::BitReader<Rercven>;
impl RercvenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rercven {
        match self.bits {
            false => Rercven::_0,
            true => Rercven::_1,
        }
    }
    ///Does not restart reception when the start bit is detected during reception.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rercven::_0
    }
    ///Restarts reception when the start bit is detected during reception.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rercven::_1
    }
}
///Field `RERCVEN` writer - Start Detection Reception Restart Enable
pub type RercvenW<'a, REG> = crate::BitWriter<'a, REG, Rercven>;
impl<'a, REG> RercvenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not restart reception when the start bit is detected during reception.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rercven::_0)
    }
    ///Restarts reception when the start bit is detected during reception.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rercven::_1)
    }
}
/**INTDA Reception Interrupt Timing Change

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcvintdsel {
    ///0: EOM timing (9th bit of data)
    _0 = 0,
    ///1: ACK timing (10th bit of data)
    _1 = 1,
}
impl From<Rcvintdsel> for bool {
    #[inline(always)]
    fn from(variant: Rcvintdsel) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVINTDSEL` reader - INTDA Reception Interrupt Timing Change
pub type RcvintdselR = crate::BitReader<Rcvintdsel>;
impl RcvintdselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcvintdsel {
        match self.bits {
            false => Rcvintdsel::_0,
            true => Rcvintdsel::_1,
        }
    }
    ///EOM timing (9th bit of data)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcvintdsel::_0
    }
    ///ACK timing (10th bit of data)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcvintdsel::_1
    }
}
///Field `RCVINTDSEL` writer - INTDA Reception Interrupt Timing Change
pub type RcvintdselW<'a, REG> = crate::BitWriter<'a, REG, Rcvintdsel>;
impl<'a, REG> RcvintdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EOM timing (9th bit of data)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvintdsel::_0)
    }
    ///ACK timing (10th bit of data)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvintdsel::_1)
    }
}
impl R {
    ///Bit 4 - Pulse Output Function Enable by Long Bit Width Error
    #[inline(always)]
    pub fn lerplen(&self) -> LerplenR {
        LerplenR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Start Detection Reception Restart Enable
    #[inline(always)]
    pub fn rercven(&self) -> RercvenR {
        RercvenR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - INTDA Reception Interrupt Timing Change
    #[inline(always)]
    pub fn rcvintdsel(&self) -> RcvintdselR {
        RcvintdselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CECEXMD")
            .field("lerplen", &self.lerplen())
            .field("rercven", &self.rercven())
            .field("rcvintdsel", &self.rcvintdsel())
            .finish()
    }
}
impl W {
    ///Bit 4 - Pulse Output Function Enable by Long Bit Width Error
    #[inline(always)]
    pub fn lerplen(&mut self) -> LerplenW<CecexmdSpec> {
        LerplenW::new(self, 4)
    }
    ///Bit 5 - Start Detection Reception Restart Enable
    #[inline(always)]
    pub fn rercven(&mut self) -> RercvenW<CecexmdSpec> {
        RercvenW::new(self, 5)
    }
    ///Bit 7 - INTDA Reception Interrupt Timing Change
    #[inline(always)]
    pub fn rcvintdsel(&mut self) -> RcvintdselW<CecexmdSpec> {
        RcvintdselW::new(self, 7)
    }
}
/**CEC Extension Mode Register

You can [`read`](crate::Reg::read) this register and get [`cecexmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecexmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CecexmdSpec;
impl crate::RegisterSpec for CecexmdSpec {
    type Ux = u8;
}
///`read()` method returns [`cecexmd::R`](R) reader structure
impl crate::Readable for CecexmdSpec {}
///`write(|w| ..)` method takes [`cecexmd::W`](W) writer structure
impl crate::Writable for CecexmdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CECEXMD to value 0
impl crate::Resettable for CecexmdSpec {}
