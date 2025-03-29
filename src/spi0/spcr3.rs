///Register `SPCR3` reader
pub type R = crate::R<Spcr3Spec>;
///Register `SPCR3` writer
pub type W = crate::W<Spcr3Spec>;
/**Extended Communication Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etxmd {
    ///0: Full-duplex synchronous or transmit-only serial communications. \[the SPCR.TXMD bit is enabled\]
    _0 = 0,
    ///1: Receive-only serial communications in slave mode (SPCR.MSTR bit = 0). \[the SPCR.TXMD bit is disabled\] Setting is prohibited in master mode (SPCR.MSTR bit = 1).
    _1 = 1,
}
impl From<Etxmd> for bool {
    #[inline(always)]
    fn from(variant: Etxmd) -> Self {
        variant as u8 != 0
    }
}
///Field `ETXMD` reader - Extended Communication Mode Select
pub type EtxmdR = crate::BitReader<Etxmd>;
impl EtxmdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Etxmd {
        match self.bits {
            false => Etxmd::_0,
            true => Etxmd::_1,
        }
    }
    ///Full-duplex synchronous or transmit-only serial communications. \[the SPCR.TXMD bit is enabled\]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Etxmd::_0
    }
    ///Receive-only serial communications in slave mode (SPCR.MSTR bit = 0). \[the SPCR.TXMD bit is disabled\] Setting is prohibited in master mode (SPCR.MSTR bit = 1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Etxmd::_1
    }
}
///Field `ETXMD` writer - Extended Communication Mode Select
pub type EtxmdW<'a, REG> = crate::BitWriter<'a, REG, Etxmd>;
impl<'a, REG> EtxmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Full-duplex synchronous or transmit-only serial communications. \[the SPCR.TXMD bit is enabled\]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Etxmd::_0)
    }
    ///Receive-only serial communications in slave mode (SPCR.MSTR bit = 0). \[the SPCR.TXMD bit is disabled\] Setting is prohibited in master mode (SPCR.MSTR bit = 1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Etxmd::_1)
    }
}
/**Between Burst Transfer Frames Delay Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfds {
    ///0: Delay (RSPCK delay, SSL negation delay and next-access delay) between frames is inserted in burst transfer.
    _0 = 0,
    ///1: Delay between frames is not inserted in burst transfer.
    _1 = 1,
}
impl From<Bfds> for bool {
    #[inline(always)]
    fn from(variant: Bfds) -> Self {
        variant as u8 != 0
    }
}
///Field `BFDS` reader - Between Burst Transfer Frames Delay Select
pub type BfdsR = crate::BitReader<Bfds>;
impl BfdsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bfds {
        match self.bits {
            false => Bfds::_0,
            true => Bfds::_1,
        }
    }
    ///Delay (RSPCK delay, SSL negation delay and next-access delay) between frames is inserted in burst transfer.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bfds::_0
    }
    ///Delay between frames is not inserted in burst transfer.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bfds::_1
    }
}
///Field `BFDS` writer - Between Burst Transfer Frames Delay Select
pub type BfdsW<'a, REG> = crate::BitWriter<'a, REG, Bfds>;
impl<'a, REG> BfdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Delay (RSPCK delay, SSL negation delay and next-access delay) between frames is inserted in burst transfer.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bfds::_0)
    }
    ///Delay between frames is not inserted in burst transfer.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfds::_1)
    }
}
/**SPI Communication End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cendie {
    ///0: Communication end interrupt request is disabled.
    _0 = 0,
    ///1: Communication end interrupt request is enabled.
    _1 = 1,
}
impl From<Cendie> for bool {
    #[inline(always)]
    fn from(variant: Cendie) -> Self {
        variant as u8 != 0
    }
}
///Field `CENDIE` reader - SPI Communication End Interrupt Enable
pub type CendieR = crate::BitReader<Cendie>;
impl CendieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cendie {
        match self.bits {
            false => Cendie::_0,
            true => Cendie::_1,
        }
    }
    ///Communication end interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cendie::_0
    }
    ///Communication end interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cendie::_1
    }
}
///Field `CENDIE` writer - SPI Communication End Interrupt Enable
pub type CendieW<'a, REG> = crate::BitWriter<'a, REG, Cendie>;
impl<'a, REG> CendieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Communication end interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cendie::_0)
    }
    ///Communication end interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cendie::_1)
    }
}
impl R {
    ///Bit 0 - Extended Communication Mode Select
    #[inline(always)]
    pub fn etxmd(&self) -> EtxmdR {
        EtxmdR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Between Burst Transfer Frames Delay Select
    #[inline(always)]
    pub fn bfds(&self) -> BfdsR {
        BfdsR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - SPI Communication End Interrupt Enable
    #[inline(always)]
    pub fn cendie(&self) -> CendieR {
        CendieR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPCR3")
            .field("etxmd", &self.etxmd())
            .field("bfds", &self.bfds())
            .field("cendie", &self.cendie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Extended Communication Mode Select
    #[inline(always)]
    pub fn etxmd(&mut self) -> EtxmdW<Spcr3Spec> {
        EtxmdW::new(self, 0)
    }
    ///Bit 1 - Between Burst Transfer Frames Delay Select
    #[inline(always)]
    pub fn bfds(&mut self) -> BfdsW<Spcr3Spec> {
        BfdsW::new(self, 1)
    }
    ///Bit 4 - SPI Communication End Interrupt Enable
    #[inline(always)]
    pub fn cendie(&mut self) -> CendieW<Spcr3Spec> {
        CendieW::new(self, 4)
    }
}
/**SPI Control Register 3

You can [`read`](crate::Reg::read) this register and get [`spcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Spcr3Spec;
impl crate::RegisterSpec for Spcr3Spec {
    type Ux = u8;
}
///`read()` method returns [`spcr3::R`](R) reader structure
impl crate::Readable for Spcr3Spec {}
///`write(|w| ..)` method takes [`spcr3::W`](W) writer structure
impl crate::Writable for Spcr3Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPCR3 to value 0
impl crate::Resettable for Spcr3Spec {}
