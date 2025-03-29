///Register `SPMR` reader
pub type R = crate::R<SpmrSpec>;
///Register `SPMR` writer
pub type W = crate::W<SpmrSpec>;
/**SSn Pin Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sse {
    ///0: Disable SSn pin function
    _0 = 0,
    ///1: Enable SSn pin function
    _1 = 1,
}
impl From<Sse> for bool {
    #[inline(always)]
    fn from(variant: Sse) -> Self {
        variant as u8 != 0
    }
}
///Field `SSE` reader - SSn Pin Function Enable
pub type SseR = crate::BitReader<Sse>;
impl SseR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sse {
        match self.bits {
            false => Sse::_0,
            true => Sse::_1,
        }
    }
    ///Disable SSn pin function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sse::_0
    }
    ///Enable SSn pin function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sse::_1
    }
}
///Field `SSE` writer - SSn Pin Function Enable
pub type SseW<'a, REG> = crate::BitWriter<'a, REG, Sse>;
impl<'a, REG> SseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable SSn pin function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sse::_0)
    }
    ///Enable SSn pin function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sse::_1)
    }
}
/**CTS Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctse {
    ///0: Disable CTS function (enable RTS output function)
    _0 = 0,
    ///1: Enable CTS function
    _1 = 1,
}
impl From<Ctse> for bool {
    #[inline(always)]
    fn from(variant: Ctse) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSE` reader - CTS Enable
pub type CtseR = crate::BitReader<Ctse>;
impl CtseR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctse {
        match self.bits {
            false => Ctse::_0,
            true => Ctse::_1,
        }
    }
    ///Disable CTS function (enable RTS output function)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctse::_0
    }
    ///Enable CTS function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctse::_1
    }
}
///Field `CTSE` writer - CTS Enable
pub type CtseW<'a, REG> = crate::BitWriter<'a, REG, Ctse>;
impl<'a, REG> CtseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable CTS function (enable RTS output function)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctse::_0)
    }
    ///Enable CTS function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctse::_1)
    }
}
/**Master Slave Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mss {
    ///0: Transmit through TXDn pin and receive through RXDn pin (master mode)
    _0 = 0,
    ///1: Receive through TXDn pin and transmit through RXDn pin (slave mode)
    _1 = 1,
}
impl From<Mss> for bool {
    #[inline(always)]
    fn from(variant: Mss) -> Self {
        variant as u8 != 0
    }
}
///Field `MSS` reader - Master Slave Select
pub type MssR = crate::BitReader<Mss>;
impl MssR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mss {
        match self.bits {
            false => Mss::_0,
            true => Mss::_1,
        }
    }
    ///Transmit through TXDn pin and receive through RXDn pin (master mode)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mss::_0
    }
    ///Receive through TXDn pin and transmit through RXDn pin (slave mode)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mss::_1
    }
}
///Field `MSS` writer - Master Slave Select
pub type MssW<'a, REG> = crate::BitWriter<'a, REG, Mss>;
impl<'a, REG> MssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit through TXDn pin and receive through RXDn pin (master mode)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mss::_0)
    }
    ///Receive through TXDn pin and transmit through RXDn pin (slave mode)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mss::_1)
    }
}
/**CTS external pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctspen {
    ///0: Alternate setting to use CTS and RTS functions as either one terminal
    _0 = 0,
    ///1: Dedicated setting for separately using CTS and RTS functions with 2 terminals These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved.
    _1 = 1,
}
impl From<Ctspen> for bool {
    #[inline(always)]
    fn from(variant: Ctspen) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSPEN` reader - CTS external pin Enable
pub type CtspenR = crate::BitReader<Ctspen>;
impl CtspenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctspen {
        match self.bits {
            false => Ctspen::_0,
            true => Ctspen::_1,
        }
    }
    ///Alternate setting to use CTS and RTS functions as either one terminal
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctspen::_0
    }
    ///Dedicated setting for separately using CTS and RTS functions with 2 terminals These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctspen::_1
    }
}
///Field `CTSPEN` writer - CTS external pin Enable
pub type CtspenW<'a, REG> = crate::BitWriter<'a, REG, Ctspen>;
impl<'a, REG> CtspenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alternate setting to use CTS and RTS functions as either one terminal
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctspen::_0)
    }
    ///Dedicated setting for separately using CTS and RTS functions with 2 terminals These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctspen::_1)
    }
}
/**Mode Fault Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mff {
    ///0: No mode fault error
    _0 = 0,
    ///1: Mode fault error
    _1 = 1,
}
impl From<Mff> for bool {
    #[inline(always)]
    fn from(variant: Mff) -> Self {
        variant as u8 != 0
    }
}
///Field `MFF` reader - Mode Fault Flag
pub type MffR = crate::BitReader<Mff>;
impl MffR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mff {
        match self.bits {
            false => Mff::_0,
            true => Mff::_1,
        }
    }
    ///No mode fault error
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mff::_0
    }
    ///Mode fault error
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mff::_1
    }
}
///Field `MFF` writer - Mode Fault Flag
pub type MffW<'a, REG> = crate::BitWriter<'a, REG, Mff>;
impl<'a, REG> MffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No mode fault error
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mff::_0)
    }
    ///Mode fault error
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mff::_1)
    }
}
/**Clock Polarity Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckpol {
    ///0: Do not invert clock polarity
    _0 = 0,
    ///1: Invert clock polarity
    _1 = 1,
}
impl From<Ckpol> for bool {
    #[inline(always)]
    fn from(variant: Ckpol) -> Self {
        variant as u8 != 0
    }
}
///Field `CKPOL` reader - Clock Polarity Select
pub type CkpolR = crate::BitReader<Ckpol>;
impl CkpolR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ckpol {
        match self.bits {
            false => Ckpol::_0,
            true => Ckpol::_1,
        }
    }
    ///Do not invert clock polarity
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ckpol::_0
    }
    ///Invert clock polarity
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ckpol::_1
    }
}
///Field `CKPOL` writer - Clock Polarity Select
pub type CkpolW<'a, REG> = crate::BitWriter<'a, REG, Ckpol>;
impl<'a, REG> CkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not invert clock polarity
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::_0)
    }
    ///Invert clock polarity
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::_1)
    }
}
/**Clock Phase Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckph {
    ///0: Do not delay clock
    _0 = 0,
    ///1: Delay clock
    _1 = 1,
}
impl From<Ckph> for bool {
    #[inline(always)]
    fn from(variant: Ckph) -> Self {
        variant as u8 != 0
    }
}
///Field `CKPH` reader - Clock Phase Select
pub type CkphR = crate::BitReader<Ckph>;
impl CkphR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ckph {
        match self.bits {
            false => Ckph::_0,
            true => Ckph::_1,
        }
    }
    ///Do not delay clock
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ckph::_0
    }
    ///Delay clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ckph::_1
    }
}
///Field `CKPH` writer - Clock Phase Select
pub type CkphW<'a, REG> = crate::BitWriter<'a, REG, Ckph>;
impl<'a, REG> CkphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not delay clock
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckph::_0)
    }
    ///Delay clock
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckph::_1)
    }
}
impl R {
    ///Bit 0 - SSn Pin Function Enable
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CTS Enable
    #[inline(always)]
    pub fn ctse(&self) -> CtseR {
        CtseR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master Slave Select
    #[inline(always)]
    pub fn mss(&self) -> MssR {
        MssR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CTS external pin Enable
    #[inline(always)]
    pub fn ctspen(&self) -> CtspenR {
        CtspenR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Mode Fault Flag
    #[inline(always)]
    pub fn mff(&self) -> MffR {
        MffR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Clock Polarity Select
    #[inline(always)]
    pub fn ckpol(&self) -> CkpolR {
        CkpolR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Clock Phase Select
    #[inline(always)]
    pub fn ckph(&self) -> CkphR {
        CkphR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPMR")
            .field("sse", &self.sse())
            .field("ctse", &self.ctse())
            .field("mss", &self.mss())
            .field("ctspen", &self.ctspen())
            .field("mff", &self.mff())
            .field("ckpol", &self.ckpol())
            .field("ckph", &self.ckph())
            .finish()
    }
}
impl W {
    ///Bit 0 - SSn Pin Function Enable
    #[inline(always)]
    pub fn sse(&mut self) -> SseW<SpmrSpec> {
        SseW::new(self, 0)
    }
    ///Bit 1 - CTS Enable
    #[inline(always)]
    pub fn ctse(&mut self) -> CtseW<SpmrSpec> {
        CtseW::new(self, 1)
    }
    ///Bit 2 - Master Slave Select
    #[inline(always)]
    pub fn mss(&mut self) -> MssW<SpmrSpec> {
        MssW::new(self, 2)
    }
    ///Bit 3 - CTS external pin Enable
    #[inline(always)]
    pub fn ctspen(&mut self) -> CtspenW<SpmrSpec> {
        CtspenW::new(self, 3)
    }
    ///Bit 4 - Mode Fault Flag
    #[inline(always)]
    pub fn mff(&mut self) -> MffW<SpmrSpec> {
        MffW::new(self, 4)
    }
    ///Bit 6 - Clock Polarity Select
    #[inline(always)]
    pub fn ckpol(&mut self) -> CkpolW<SpmrSpec> {
        CkpolW::new(self, 6)
    }
    ///Bit 7 - Clock Phase Select
    #[inline(always)]
    pub fn ckph(&mut self) -> CkphW<SpmrSpec> {
        CkphW::new(self, 7)
    }
}
/**SPI Mode Register

You can [`read`](crate::Reg::read) this register and get [`spmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpmrSpec;
impl crate::RegisterSpec for SpmrSpec {
    type Ux = u8;
}
///`read()` method returns [`spmr::R`](R) reader structure
impl crate::Readable for SpmrSpec {}
///`write(|w| ..)` method takes [`spmr::W`](W) writer structure
impl crate::Writable for SpmrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPMR to value 0
impl crate::Resettable for SpmrSpec {}
