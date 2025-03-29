///Register `CECFC` reader
pub type R = crate::R<CecfcSpec>;
///Register `CECFC` writer
pub type W = crate::W<CecfcSpec>;
/**Overrun Error Detection Flag Clear Trigger

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Octrg {
    ///0: Does not clear overrun error detection flag.
    _0 = 0,
    ///1: Clears overrun error detection flag.
    _1 = 1,
}
impl From<Octrg> for bool {
    #[inline(always)]
    fn from(variant: Octrg) -> Self {
        variant as u8 != 0
    }
}
///Field `OCTRG` writer - Overrun Error Detection Flag Clear Trigger
pub type OctrgW<'a, REG> = crate::BitWriter<'a, REG, Octrg>;
impl<'a, REG> OctrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not clear overrun error detection flag.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Octrg::_0)
    }
    ///Clears overrun error detection flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Octrg::_1)
    }
}
/**Underrun Error Detection Flag Clear Trigger

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctrg {
    ///0: Does not clear underrun error detection flag.
    _0 = 0,
    ///1: Clears underrun error detection flag.
    _1 = 1,
}
impl From<Uctrg> for bool {
    #[inline(always)]
    fn from(variant: Uctrg) -> Self {
        variant as u8 != 0
    }
}
///Field `UCTRG` writer - Underrun Error Detection Flag Clear Trigger
pub type UctrgW<'a, REG> = crate::BitWriter<'a, REG, Uctrg>;
impl<'a, REG> UctrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not clear underrun error detection flag.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctrg::_0)
    }
    ///Clears underrun error detection flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctrg::_1)
    }
}
/**ACK Error Detection Flag Clear Trigger

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackctrg {
    ///0: Does not clear ACK error detection flag.
    _0 = 0,
    ///1: Clears ACK error detection flag.
    _1 = 1,
}
impl From<Ackctrg> for bool {
    #[inline(always)]
    fn from(variant: Ackctrg) -> Self {
        variant as u8 != 0
    }
}
///Field `ACKCTRG` writer - ACK Error Detection Flag Clear Trigger
pub type AckctrgW<'a, REG> = crate::BitWriter<'a, REG, Ackctrg>;
impl<'a, REG> AckctrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not clear ACK error detection flag.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ackctrg::_0)
    }
    ///Clears ACK error detection flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ackctrg::_1)
    }
}
/**Timing Error Detection Flag Clear Trigger

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tctrg {
    ///0: Does not clear timing error detection flag.
    _0 = 0,
    ///1: Clears timing error detection flag.
    _1 = 1,
}
impl From<Tctrg> for bool {
    #[inline(always)]
    fn from(variant: Tctrg) -> Self {
        variant as u8 != 0
    }
}
///Field `TCTRG` writer - Timing Error Detection Flag Clear Trigger
pub type TctrgW<'a, REG> = crate::BitWriter<'a, REG, Tctrg>;
impl<'a, REG> TctrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not clear timing error detection flag.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tctrg::_0)
    }
    ///Clears timing error detection flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tctrg::_1)
    }
}
/**Transmission Error Detection Flag Clear Trigger

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txctrg {
    ///0: Does not clear transmission error detection flag.
    _0 = 0,
    ///1: Clears transmission error detection flag.
    _1 = 1,
}
impl From<Txctrg> for bool {
    #[inline(always)]
    fn from(variant: Txctrg) -> Self {
        variant as u8 != 0
    }
}
///Field `TXCTRG` writer - Transmission Error Detection Flag Clear Trigger
pub type TxctrgW<'a, REG> = crate::BitWriter<'a, REG, Txctrg>;
impl<'a, REG> TxctrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not clear transmission error detection flag.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txctrg::_0)
    }
    ///Clears transmission error detection flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txctrg::_1)
    }
}
/**Arbitration Loss Detection Flag Clear Trigger

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Actrg {
    ///0: Does not clear arbitration loss detection flag.
    _0 = 0,
    ///1: Clears arbitration loss detection flag.
    _1 = 1,
}
impl From<Actrg> for bool {
    #[inline(always)]
    fn from(variant: Actrg) -> Self {
        variant as u8 != 0
    }
}
///Field `ACTRG` writer - Arbitration Loss Detection Flag Clear Trigger
pub type ActrgW<'a, REG> = crate::BitWriter<'a, REG, Actrg>;
impl<'a, REG> ActrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not clear arbitration loss detection flag.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Actrg::_0)
    }
    ///Clears arbitration loss detection flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Actrg::_1)
    }
}
/**Bus Lock Error Detection Flag Clear Trigger

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blctrg {
    ///0: Does not clear bus lock error detection flag.
    _0 = 0,
    ///1: Clears bus lock error detection flag.
    _1 = 1,
}
impl From<Blctrg> for bool {
    #[inline(always)]
    fn from(variant: Blctrg) -> Self {
        variant as u8 != 0
    }
}
///Field `BLCTRG` writer - Bus Lock Error Detection Flag Clear Trigger
pub type BlctrgW<'a, REG> = crate::BitWriter<'a, REG, Blctrg>;
impl<'a, REG> BlctrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not clear bus lock error detection flag.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Blctrg::_0)
    }
    ///Clears bus lock error detection flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Blctrg::_1)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CECFC").finish()
    }
}
impl W {
    ///Bit 0 - Overrun Error Detection Flag Clear Trigger
    #[inline(always)]
    pub fn octrg(&mut self) -> OctrgW<CecfcSpec> {
        OctrgW::new(self, 0)
    }
    ///Bit 1 - Underrun Error Detection Flag Clear Trigger
    #[inline(always)]
    pub fn uctrg(&mut self) -> UctrgW<CecfcSpec> {
        UctrgW::new(self, 1)
    }
    ///Bit 2 - ACK Error Detection Flag Clear Trigger
    #[inline(always)]
    pub fn ackctrg(&mut self) -> AckctrgW<CecfcSpec> {
        AckctrgW::new(self, 2)
    }
    ///Bit 3 - Timing Error Detection Flag Clear Trigger
    #[inline(always)]
    pub fn tctrg(&mut self) -> TctrgW<CecfcSpec> {
        TctrgW::new(self, 3)
    }
    ///Bit 4 - Transmission Error Detection Flag Clear Trigger
    #[inline(always)]
    pub fn txctrg(&mut self) -> TxctrgW<CecfcSpec> {
        TxctrgW::new(self, 4)
    }
    ///Bit 5 - Arbitration Loss Detection Flag Clear Trigger
    #[inline(always)]
    pub fn actrg(&mut self) -> ActrgW<CecfcSpec> {
        ActrgW::new(self, 5)
    }
    ///Bit 6 - Bus Lock Error Detection Flag Clear Trigger
    #[inline(always)]
    pub fn blctrg(&mut self) -> BlctrgW<CecfcSpec> {
        BlctrgW::new(self, 6)
    }
}
/**CEC Communication Error Flag Clear Trigger Register

You can [`read`](crate::Reg::read) this register and get [`cecfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CecfcSpec;
impl crate::RegisterSpec for CecfcSpec {
    type Ux = u8;
}
///`read()` method returns [`cecfc::R`](R) reader structure
impl crate::Readable for CecfcSpec {}
///`write(|w| ..)` method takes [`cecfc::W`](W) writer structure
impl crate::Writable for CecfcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CECFC to value 0
impl crate::Resettable for CecfcSpec {}
