///Register `CFDCFCCE%s` reader
pub type R = crate::R<CfdcfcceSpec>;
///Register `CFDCFCCE%s` writer
pub type W = crate::W<CfdcfcceSpec>;
/**Common FIFO Full Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cffie {
    ///0: FIFO Interrupt generation disabled
    _0 = 0,
    ///1: FIFO Interrupt generation enabled
    _1 = 1,
}
impl From<Cffie> for bool {
    #[inline(always)]
    fn from(variant: Cffie) -> Self {
        variant as u8 != 0
    }
}
///Field `CFFIE` reader - Common FIFO Full Interrupt Enable
pub type CffieR = crate::BitReader<Cffie>;
impl CffieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cffie {
        match self.bits {
            false => Cffie::_0,
            true => Cffie::_1,
        }
    }
    ///FIFO Interrupt generation disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cffie::_0
    }
    ///FIFO Interrupt generation enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cffie::_1
    }
}
///Field `CFFIE` writer - Common FIFO Full Interrupt Enable
pub type CffieW<'a, REG> = crate::BitWriter<'a, REG, Cffie>;
impl<'a, REG> CffieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO Interrupt generation disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cffie::_0)
    }
    ///FIFO Interrupt generation enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cffie::_1)
    }
}
/**Common FIFO One Frame Reception Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfofrxie {
    ///0: One Frame RX interrupt generation disabled
    _0 = 0,
    ///1: One Frame RX interrupt generation enabled
    _1 = 1,
}
impl From<Cfofrxie> for bool {
    #[inline(always)]
    fn from(variant: Cfofrxie) -> Self {
        variant as u8 != 0
    }
}
///Field `CFOFRXIE` reader - Common FIFO One Frame Reception Interrupt Enable
pub type CfofrxieR = crate::BitReader<Cfofrxie>;
impl CfofrxieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfofrxie {
        match self.bits {
            false => Cfofrxie::_0,
            true => Cfofrxie::_1,
        }
    }
    ///One Frame RX interrupt generation disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfofrxie::_0
    }
    ///One Frame RX interrupt generation enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfofrxie::_1
    }
}
///Field `CFOFRXIE` writer - Common FIFO One Frame Reception Interrupt Enable
pub type CfofrxieW<'a, REG> = crate::BitWriter<'a, REG, Cfofrxie>;
impl<'a, REG> CfofrxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///One Frame RX interrupt generation disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfofrxie::_0)
    }
    ///One Frame RX interrupt generation enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfofrxie::_1)
    }
}
/**Common FIFO One Frame Transmission Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfoftxie {
    ///0: One Frame TX interrupt generation disabled
    _0 = 0,
    ///1: One Frame TX interrupt generation enabled
    _1 = 1,
}
impl From<Cfoftxie> for bool {
    #[inline(always)]
    fn from(variant: Cfoftxie) -> Self {
        variant as u8 != 0
    }
}
///Field `CFOFTXIE` reader - Common FIFO One Frame Transmission Interrupt Enable
pub type CfoftxieR = crate::BitReader<Cfoftxie>;
impl CfoftxieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfoftxie {
        match self.bits {
            false => Cfoftxie::_0,
            true => Cfoftxie::_1,
        }
    }
    ///One Frame TX interrupt generation disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfoftxie::_0
    }
    ///One Frame TX interrupt generation enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfoftxie::_1
    }
}
///Field `CFOFTXIE` writer - Common FIFO One Frame Transmission Interrupt Enable
pub type CfoftxieW<'a, REG> = crate::BitWriter<'a, REG, Cfoftxie>;
impl<'a, REG> CfoftxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///One Frame TX interrupt generation disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfoftxie::_0)
    }
    ///One Frame TX interrupt generation enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfoftxie::_1)
    }
}
/**Common FIFO Message Overwrite Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfmowm {
    ///0: Message discarded mode
    _0 = 0,
    ///1: Message overwrite mode
    _1 = 1,
}
impl From<Cfmowm> for bool {
    #[inline(always)]
    fn from(variant: Cfmowm) -> Self {
        variant as u8 != 0
    }
}
///Field `CFMOWM` reader - Common FIFO Message Overwrite Mode
pub type CfmowmR = crate::BitReader<Cfmowm>;
impl CfmowmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfmowm {
        match self.bits {
            false => Cfmowm::_0,
            true => Cfmowm::_1,
        }
    }
    ///Message discarded mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfmowm::_0
    }
    ///Message overwrite mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfmowm::_1
    }
}
///Field `CFMOWM` writer - Common FIFO Message Overwrite Mode
pub type CfmowmW<'a, REG> = crate::BitWriter<'a, REG, Cfmowm>;
impl<'a, REG> CfmowmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Message discarded mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmowm::_0)
    }
    ///Message overwrite mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmowm::_1)
    }
}
/**Common FIFO Buffering Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfbme {
    ///0: Transmission from Common FIFO
    _0 = 0,
    ///1: Transmission halt from Common FIFO
    _1 = 1,
}
impl From<Cfbme> for bool {
    #[inline(always)]
    fn from(variant: Cfbme) -> Self {
        variant as u8 != 0
    }
}
///Field `CFBME` reader - Common FIFO Buffering Mode Enable
pub type CfbmeR = crate::BitReader<Cfbme>;
impl CfbmeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfbme {
        match self.bits {
            false => Cfbme::_0,
            true => Cfbme::_1,
        }
    }
    ///Transmission from Common FIFO
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfbme::_0
    }
    ///Transmission halt from Common FIFO
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfbme::_1
    }
}
///Field `CFBME` writer - Common FIFO Buffering Mode Enable
pub type CfbmeW<'a, REG> = crate::BitWriter<'a, REG, Cfbme>;
impl<'a, REG> CfbmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmission from Common FIFO
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbme::_0)
    }
    ///Transmission halt from Common FIFO
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbme::_1)
    }
}
impl R {
    ///Bit 0 - Common FIFO Full Interrupt Enable
    #[inline(always)]
    pub fn cffie(&self) -> CffieR {
        CffieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Common FIFO One Frame Reception Interrupt Enable
    #[inline(always)]
    pub fn cfofrxie(&self) -> CfofrxieR {
        CfofrxieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Common FIFO One Frame Transmission Interrupt Enable
    #[inline(always)]
    pub fn cfoftxie(&self) -> CfoftxieR {
        CfoftxieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Common FIFO Message Overwrite Mode
    #[inline(always)]
    pub fn cfmowm(&self) -> CfmowmR {
        CfmowmR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Common FIFO Buffering Mode Enable
    #[inline(always)]
    pub fn cfbme(&self) -> CfbmeR {
        CfbmeR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFCCE")
            .field("cffie", &self.cffie())
            .field("cfofrxie", &self.cfofrxie())
            .field("cfoftxie", &self.cfoftxie())
            .field("cfmowm", &self.cfmowm())
            .field("cfbme", &self.cfbme())
            .finish()
    }
}
impl W {
    ///Bit 0 - Common FIFO Full Interrupt Enable
    #[inline(always)]
    pub fn cffie(&mut self) -> CffieW<CfdcfcceSpec> {
        CffieW::new(self, 0)
    }
    ///Bit 1 - Common FIFO One Frame Reception Interrupt Enable
    #[inline(always)]
    pub fn cfofrxie(&mut self) -> CfofrxieW<CfdcfcceSpec> {
        CfofrxieW::new(self, 1)
    }
    ///Bit 2 - Common FIFO One Frame Transmission Interrupt Enable
    #[inline(always)]
    pub fn cfoftxie(&mut self) -> CfoftxieW<CfdcfcceSpec> {
        CfoftxieW::new(self, 2)
    }
    ///Bit 8 - Common FIFO Message Overwrite Mode
    #[inline(always)]
    pub fn cfmowm(&mut self) -> CfmowmW<CfdcfcceSpec> {
        CfmowmW::new(self, 8)
    }
    ///Bit 16 - Common FIFO Buffering Mode Enable
    #[inline(always)]
    pub fn cfbme(&mut self) -> CfbmeW<CfdcfcceSpec> {
        CfbmeW::new(self, 16)
    }
}
/**Common FIFO Configuration/Control Enhancement Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdcfcce::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfcce::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfcceSpec;
impl crate::RegisterSpec for CfdcfcceSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfcce::R`](R) reader structure
impl crate::Readable for CfdcfcceSpec {}
///`write(|w| ..)` method takes [`cfdcfcce::W`](W) writer structure
impl crate::Writable for CfdcfcceSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCFCCE%s to value 0
impl crate::Resettable for CfdcfcceSpec {}
