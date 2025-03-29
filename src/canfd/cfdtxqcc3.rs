///Register `CFDTXQCC3%s` reader
pub type R = crate::R<Cfdtxqcc3Spec>;
///Register `CFDTXQCC3%s` writer
pub type W = crate::W<Cfdtxqcc3Spec>;
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
    ///Bit 18 - TXQ One Frame Transmission Interrupt Enable
    #[inline(always)]
    pub fn txqoftxie(&self) -> TxqoftxieR {
        TxqoftxieR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQCC3")
            .field("txqe", &self.txqe())
            .field("txqtxie", &self.txqtxie())
            .field("txqim", &self.txqim())
            .field("txqdc", &self.txqdc())
            .field("txqoftxie", &self.txqoftxie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX Queue Enable
    #[inline(always)]
    pub fn txqe(&mut self) -> TxqeW<Cfdtxqcc3Spec> {
        TxqeW::new(self, 0)
    }
    ///Bit 5 - TX Queue TX Interrupt Enable
    #[inline(always)]
    pub fn txqtxie(&mut self) -> TxqtxieW<Cfdtxqcc3Spec> {
        TxqtxieW::new(self, 5)
    }
    ///Bit 7 - TX Queue Interrupt Mode
    #[inline(always)]
    pub fn txqim(&mut self) -> TxqimW<Cfdtxqcc3Spec> {
        TxqimW::new(self, 7)
    }
    ///Bits 8:12 - TX Queue Depth Configuration
    #[inline(always)]
    pub fn txqdc(&mut self) -> TxqdcW<Cfdtxqcc3Spec> {
        TxqdcW::new(self, 8)
    }
    ///Bit 18 - TXQ One Frame Transmission Interrupt Enable
    #[inline(always)]
    pub fn txqoftxie(&mut self) -> TxqoftxieW<Cfdtxqcc3Spec> {
        TxqoftxieW::new(self, 18)
    }
}
/**TX Queue Configuration/Control Registers 3%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqcc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqcc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdtxqcc3Spec;
impl crate::RegisterSpec for Cfdtxqcc3Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqcc3::R`](R) reader structure
impl crate::Readable for Cfdtxqcc3Spec {}
///`write(|w| ..)` method takes [`cfdtxqcc3::W`](W) writer structure
impl crate::Writable for Cfdtxqcc3Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTXQCC3%s to value 0
impl crate::Resettable for Cfdtxqcc3Spec {}
