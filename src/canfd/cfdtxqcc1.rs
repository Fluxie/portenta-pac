///Register `CFDTXQCC1%s` reader
pub type R = crate::R<Cfdtxqcc1Spec>;
///Register `CFDTXQCC1%s` writer
pub type W = crate::W<Cfdtxqcc1Spec>;
/**TX Queue Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txqe {
    ///0: TX Queue disabled
    _0 = 0,
    ///1: TX Queue enabled
    _1 = 1,
}
impl From<Txqe> for bool {
    #[inline(always)]
    fn from(variant: Txqe) -> Self {
        variant as u8 != 0
    }
}
///Field `TXQE` reader - TX Queue Enable
pub type TxqeR = crate::BitReader<Txqe>;
impl TxqeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txqe {
        match self.bits {
            false => Txqe::_0,
            true => Txqe::_1,
        }
    }
    ///TX Queue disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txqe::_0
    }
    ///TX Queue enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txqe::_1
    }
}
///Field `TXQE` writer - TX Queue Enable
pub type TxqeW<'a, REG> = crate::BitWriter<'a, REG, Txqe>;
impl<'a, REG> TxqeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX Queue disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txqe::_0)
    }
    ///TX Queue enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txqe::_1)
    }
}
/**TX Queue Gateway Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txqgwe {
    ///0: TX Queue GW mode disabled
    _0 = 0,
    ///1: TX Queue GW mode enabled
    _1 = 1,
}
impl From<Txqgwe> for bool {
    #[inline(always)]
    fn from(variant: Txqgwe) -> Self {
        variant as u8 != 0
    }
}
///Field `TXQGWE` reader - TX Queue Gateway Mode Enable
pub type TxqgweR = crate::BitReader<Txqgwe>;
impl TxqgweR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txqgwe {
        match self.bits {
            false => Txqgwe::_0,
            true => Txqgwe::_1,
        }
    }
    ///TX Queue GW mode disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txqgwe::_0
    }
    ///TX Queue GW mode enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txqgwe::_1
    }
}
///Field `TXQGWE` writer - TX Queue Gateway Mode Enable
pub type TxqgweW<'a, REG> = crate::BitWriter<'a, REG, Txqgwe>;
impl<'a, REG> TxqgweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX Queue GW mode disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txqgwe::_0)
    }
    ///TX Queue GW mode enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txqgwe::_1)
    }
}
/**TX Queue TX Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txqtxie {
    ///0: TX Queue TX interrupt disabled
    _0 = 0,
    ///1: TX Queue TX interrupt enabled
    _1 = 1,
}
impl From<Txqtxie> for bool {
    #[inline(always)]
    fn from(variant: Txqtxie) -> Self {
        variant as u8 != 0
    }
}
///Field `TXQTXIE` reader - TX Queue TX Interrupt Enable
pub type TxqtxieR = crate::BitReader<Txqtxie>;
impl TxqtxieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txqtxie {
        match self.bits {
            false => Txqtxie::_0,
            true => Txqtxie::_1,
        }
    }
    ///TX Queue TX interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txqtxie::_0
    }
    ///TX Queue TX interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txqtxie::_1
    }
}
///Field `TXQTXIE` writer - TX Queue TX Interrupt Enable
pub type TxqtxieW<'a, REG> = crate::BitWriter<'a, REG, Txqtxie>;
impl<'a, REG> TxqtxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX Queue TX interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txqtxie::_0)
    }
    ///TX Queue TX interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txqtxie::_1)
    }
}
/**TX Queue Interrupt Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txqim {
    ///0: When the last message is successfully transmitted
    _0 = 0,
    ///1: At every successful transmission
    _1 = 1,
}
impl From<Txqim> for bool {
    #[inline(always)]
    fn from(variant: Txqim) -> Self {
        variant as u8 != 0
    }
}
///Field `TXQIM` reader - TX Queue Interrupt Mode
pub type TxqimR = crate::BitReader<Txqim>;
impl TxqimR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txqim {
        match self.bits {
            false => Txqim::_0,
            true => Txqim::_1,
        }
    }
    ///When the last message is successfully transmitted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txqim::_0
    }
    ///At every successful transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txqim::_1
    }
}
///Field `TXQIM` writer - TX Queue Interrupt Mode
pub type TxqimW<'a, REG> = crate::BitWriter<'a, REG, Txqim>;
impl<'a, REG> TxqimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When the last message is successfully transmitted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txqim::_0)
    }
    ///At every successful transmission
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txqim::_1)
    }
}
///Field `TXQDC` reader - TX Queue Depth Configuration
pub type TxqdcR = crate::FieldReader;
///Field `TXQDC` writer - TX Queue Depth Configuration
pub type TxqdcW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**TXQ Full Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txqfie {
    ///0: TX Queue full interrupt generation disabled
    _0 = 0,
    ///1: TX Queue full interrupt generation enabled
    _1 = 1,
}
impl From<Txqfie> for bool {
    #[inline(always)]
    fn from(variant: Txqfie) -> Self {
        variant as u8 != 0
    }
}
///Field `TXQFIE` reader - TXQ Full Interrupt Enable
pub type TxqfieR = crate::BitReader<Txqfie>;
impl TxqfieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txqfie {
        match self.bits {
            false => Txqfie::_0,
            true => Txqfie::_1,
        }
    }
    ///TX Queue full interrupt generation disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txqfie::_0
    }
    ///TX Queue full interrupt generation enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txqfie::_1
    }
}
///Field `TXQFIE` writer - TXQ Full Interrupt Enable
pub type TxqfieW<'a, REG> = crate::BitWriter<'a, REG, Txqfie>;
impl<'a, REG> TxqfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX Queue full interrupt generation disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txqfie::_0)
    }
    ///TX Queue full interrupt generation enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txqfie::_1)
    }
}
/**TXQ One Frame Reception Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txqofrxie {
    ///0: One Frame RX interrupt generation disabled
    _0 = 0,
    ///1: One Frame RX interrupt generation enabled
    _1 = 1,
}
impl From<Txqofrxie> for bool {
    #[inline(always)]
    fn from(variant: Txqofrxie) -> Self {
        variant as u8 != 0
    }
}
///Field `TXQOFRXIE` reader - TXQ One Frame Reception Interrupt Enable
pub type TxqofrxieR = crate::BitReader<Txqofrxie>;
impl TxqofrxieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txqofrxie {
        match self.bits {
            false => Txqofrxie::_0,
            true => Txqofrxie::_1,
        }
    }
    ///One Frame RX interrupt generation disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txqofrxie::_0
    }
    ///One Frame RX interrupt generation enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txqofrxie::_1
    }
}
///Field `TXQOFRXIE` writer - TXQ One Frame Reception Interrupt Enable
pub type TxqofrxieW<'a, REG> = crate::BitWriter<'a, REG, Txqofrxie>;
impl<'a, REG> TxqofrxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///One Frame RX interrupt generation disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txqofrxie::_0)
    }
    ///One Frame RX interrupt generation enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txqofrxie::_1)
    }
}
/**TXQ One Frame Transmission Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txqoftxie {
    ///0: One Frame TX interrupt generation disabled
    _0 = 0,
    ///1: One Frame TX interrupt generation enabled
    _1 = 1,
}
impl From<Txqoftxie> for bool {
    #[inline(always)]
    fn from(variant: Txqoftxie) -> Self {
        variant as u8 != 0
    }
}
///Field `TXQOFTXIE` reader - TXQ One Frame Transmission Interrupt Enable
pub type TxqoftxieR = crate::BitReader<Txqoftxie>;
impl TxqoftxieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txqoftxie {
        match self.bits {
            false => Txqoftxie::_0,
            true => Txqoftxie::_1,
        }
    }
    ///One Frame TX interrupt generation disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txqoftxie::_0
    }
    ///One Frame TX interrupt generation enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txqoftxie::_1
    }
}
///Field `TXQOFTXIE` writer - TXQ One Frame Transmission Interrupt Enable
pub type TxqoftxieW<'a, REG> = crate::BitWriter<'a, REG, Txqoftxie>;
impl<'a, REG> TxqoftxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///One Frame TX interrupt generation disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txqoftxie::_0)
    }
    ///One Frame TX interrupt generation enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txqoftxie::_1)
    }
}
impl R {
    ///Bit 0 - TX Queue Enable
    #[inline(always)]
    pub fn txqe(&self) -> TxqeR {
        TxqeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX Queue Gateway Mode Enable
    #[inline(always)]
    pub fn txqgwe(&self) -> TxqgweR {
        TxqgweR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - TX Queue TX Interrupt Enable
    #[inline(always)]
    pub fn txqtxie(&self) -> TxqtxieR {
        TxqtxieR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - TX Queue Interrupt Mode
    #[inline(always)]
    pub fn txqim(&self) -> TxqimR {
        TxqimR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:12 - TX Queue Depth Configuration
    #[inline(always)]
    pub fn txqdc(&self) -> TxqdcR {
        TxqdcR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 16 - TXQ Full Interrupt Enable
    #[inline(always)]
    pub fn txqfie(&self) -> TxqfieR {
        TxqfieR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TXQ One Frame Reception Interrupt Enable
    #[inline(always)]
    pub fn txqofrxie(&self) -> TxqofrxieR {
        TxqofrxieR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TXQ One Frame Transmission Interrupt Enable
    #[inline(always)]
    pub fn txqoftxie(&self) -> TxqoftxieR {
        TxqoftxieR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQCC1")
            .field("txqe", &self.txqe())
            .field("txqgwe", &self.txqgwe())
            .field("txqtxie", &self.txqtxie())
            .field("txqim", &self.txqim())
            .field("txqdc", &self.txqdc())
            .field("txqfie", &self.txqfie())
            .field("txqofrxie", &self.txqofrxie())
            .field("txqoftxie", &self.txqoftxie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX Queue Enable
    #[inline(always)]
    pub fn txqe(&mut self) -> TxqeW<Cfdtxqcc1Spec> {
        TxqeW::new(self, 0)
    }
    ///Bit 1 - TX Queue Gateway Mode Enable
    #[inline(always)]
    pub fn txqgwe(&mut self) -> TxqgweW<Cfdtxqcc1Spec> {
        TxqgweW::new(self, 1)
    }
    ///Bit 5 - TX Queue TX Interrupt Enable
    #[inline(always)]
    pub fn txqtxie(&mut self) -> TxqtxieW<Cfdtxqcc1Spec> {
        TxqtxieW::new(self, 5)
    }
    ///Bit 7 - TX Queue Interrupt Mode
    #[inline(always)]
    pub fn txqim(&mut self) -> TxqimW<Cfdtxqcc1Spec> {
        TxqimW::new(self, 7)
    }
    ///Bits 8:12 - TX Queue Depth Configuration
    #[inline(always)]
    pub fn txqdc(&mut self) -> TxqdcW<Cfdtxqcc1Spec> {
        TxqdcW::new(self, 8)
    }
    ///Bit 16 - TXQ Full Interrupt Enable
    #[inline(always)]
    pub fn txqfie(&mut self) -> TxqfieW<Cfdtxqcc1Spec> {
        TxqfieW::new(self, 16)
    }
    ///Bit 17 - TXQ One Frame Reception Interrupt Enable
    #[inline(always)]
    pub fn txqofrxie(&mut self) -> TxqofrxieW<Cfdtxqcc1Spec> {
        TxqofrxieW::new(self, 17)
    }
    ///Bit 18 - TXQ One Frame Transmission Interrupt Enable
    #[inline(always)]
    pub fn txqoftxie(&mut self) -> TxqoftxieW<Cfdtxqcc1Spec> {
        TxqoftxieW::new(self, 18)
    }
}
/**TX Queue Configuration/Control Registers 1%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqcc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqcc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdtxqcc1Spec;
impl crate::RegisterSpec for Cfdtxqcc1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqcc1::R`](R) reader structure
impl crate::Readable for Cfdtxqcc1Spec {}
///`write(|w| ..)` method takes [`cfdtxqcc1::W`](W) writer structure
impl crate::Writable for Cfdtxqcc1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTXQCC1%s to value 0
impl crate::Resettable for Cfdtxqcc1Spec {}
