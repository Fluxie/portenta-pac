///Register `SD_INFO1` reader
pub type R = crate::R<SdInfo1Spec>;
///Register `SD_INFO1` writer
pub type W = crate::W<SdInfo1Spec>;
/**Response End Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rspend {
    ///0: Response end not detected
    _0 = 0,
    ///1: Response end detected
    _1 = 1,
}
impl From<Rspend> for bool {
    #[inline(always)]
    fn from(variant: Rspend) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPEND` reader - Response End Detection Flag
pub type RspendR = crate::BitReader<Rspend>;
impl RspendR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rspend {
        match self.bits {
            false => Rspend::_0,
            true => Rspend::_1,
        }
    }
    ///Response end not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rspend::_0
    }
    ///Response end detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rspend::_1
    }
}
///Field `RSPEND` writer - Response End Detection Flag
pub type RspendW<'a, REG> = crate::BitWriter<'a, REG, Rspend>;
impl<'a, REG> RspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Response end not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rspend::_0)
    }
    ///Response end detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rspend::_1)
    }
}
/**Access End Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acend {
    ///0: Access end not detected
    _0 = 0,
    ///1: Access end detected
    _1 = 1,
}
impl From<Acend> for bool {
    #[inline(always)]
    fn from(variant: Acend) -> Self {
        variant as u8 != 0
    }
}
///Field `ACEND` reader - Access End Detection Flag
pub type AcendR = crate::BitReader<Acend>;
impl AcendR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Acend {
        match self.bits {
            false => Acend::_0,
            true => Acend::_1,
        }
    }
    ///Access end not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acend::_0
    }
    ///Access end detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acend::_1
    }
}
///Field `ACEND` writer - Access End Detection Flag
pub type AcendW<'a, REG> = crate::BitWriter<'a, REG, Acend>;
impl<'a, REG> AcendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Access end not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acend::_0)
    }
    ///Access end detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acend::_1)
    }
}
/**SDnCD Removal Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdcdrm {
    ///0: SD card/MMC removal not detected by the SDnCD pin
    _0 = 0,
    ///1: SD card/MMC removal detected by the SDnCD pin
    _1 = 1,
}
impl From<Sdcdrm> for bool {
    #[inline(always)]
    fn from(variant: Sdcdrm) -> Self {
        variant as u8 != 0
    }
}
///Field `SDCDRM` reader - SDnCD Removal Flag
pub type SdcdrmR = crate::BitReader<Sdcdrm>;
impl SdcdrmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdcdrm {
        match self.bits {
            false => Sdcdrm::_0,
            true => Sdcdrm::_1,
        }
    }
    ///SD card/MMC removal not detected by the SDnCD pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdcdrm::_0
    }
    ///SD card/MMC removal detected by the SDnCD pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdcdrm::_1
    }
}
///Field `SDCDRM` writer - SDnCD Removal Flag
pub type SdcdrmW<'a, REG> = crate::BitWriter<'a, REG, Sdcdrm>;
impl<'a, REG> SdcdrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SD card/MMC removal not detected by the SDnCD pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcdrm::_0)
    }
    ///SD card/MMC removal detected by the SDnCD pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcdrm::_1)
    }
}
/**SDnCD Insertion Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdcdin {
    ///0: SD card/MMC insertion not detected by the SDnCD pin
    _0 = 0,
    ///1: SD card/MMC insertion detected by the SDnCD pin
    _1 = 1,
}
impl From<Sdcdin> for bool {
    #[inline(always)]
    fn from(variant: Sdcdin) -> Self {
        variant as u8 != 0
    }
}
///Field `SDCDIN` reader - SDnCD Insertion Flag
pub type SdcdinR = crate::BitReader<Sdcdin>;
impl SdcdinR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdcdin {
        match self.bits {
            false => Sdcdin::_0,
            true => Sdcdin::_1,
        }
    }
    ///SD card/MMC insertion not detected by the SDnCD pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdcdin::_0
    }
    ///SD card/MMC insertion detected by the SDnCD pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdcdin::_1
    }
}
///Field `SDCDIN` writer - SDnCD Insertion Flag
pub type SdcdinW<'a, REG> = crate::BitWriter<'a, REG, Sdcdin>;
impl<'a, REG> SdcdinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SD card/MMC insertion not detected by the SDnCD pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcdin::_0)
    }
    ///SD card/MMC insertion detected by the SDnCD pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcdin::_1)
    }
}
/**SDnCD Pin Monitor Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdcdmon {
    ///0: SDnCD pin level is high
    _0 = 0,
    ///1: SDnCD pin level is low
    _1 = 1,
}
impl From<Sdcdmon> for bool {
    #[inline(always)]
    fn from(variant: Sdcdmon) -> Self {
        variant as u8 != 0
    }
}
///Field `SDCDMON` reader - SDnCD Pin Monitor Flag
pub type SdcdmonR = crate::BitReader<Sdcdmon>;
impl SdcdmonR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdcdmon {
        match self.bits {
            false => Sdcdmon::_0,
            true => Sdcdmon::_1,
        }
    }
    ///SDnCD pin level is high
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdcdmon::_0
    }
    ///SDnCD pin level is low
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdcdmon::_1
    }
}
/**SDnWP Pin Monitor Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdwpmon {
    ///0: SDnWP pin level is high
    _0 = 0,
    ///1: SDnWP pin level is low
    _1 = 1,
}
impl From<Sdwpmon> for bool {
    #[inline(always)]
    fn from(variant: Sdwpmon) -> Self {
        variant as u8 != 0
    }
}
///Field `SDWPMON` reader - SDnWP Pin Monitor Flag
pub type SdwpmonR = crate::BitReader<Sdwpmon>;
impl SdwpmonR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdwpmon {
        match self.bits {
            false => Sdwpmon::_0,
            true => Sdwpmon::_1,
        }
    }
    ///SDnWP pin level is high
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdwpmon::_0
    }
    ///SDnWP pin level is low
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdwpmon::_1
    }
}
/**SDnDAT3 Removal Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdd3rm {
    ///0: SD card/MMC removal not detected by the SDnDAT3 pin
    _0 = 0,
    ///1: SD card/MMC removal detected by the SDnDAT3 pin
    _1 = 1,
}
impl From<Sdd3rm> for bool {
    #[inline(always)]
    fn from(variant: Sdd3rm) -> Self {
        variant as u8 != 0
    }
}
///Field `SDD3RM` reader - SDnDAT3 Removal Flag
pub type Sdd3rmR = crate::BitReader<Sdd3rm>;
impl Sdd3rmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdd3rm {
        match self.bits {
            false => Sdd3rm::_0,
            true => Sdd3rm::_1,
        }
    }
    ///SD card/MMC removal not detected by the SDnDAT3 pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdd3rm::_0
    }
    ///SD card/MMC removal detected by the SDnDAT3 pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdd3rm::_1
    }
}
///Field `SDD3RM` writer - SDnDAT3 Removal Flag
pub type Sdd3rmW<'a, REG> = crate::BitWriter<'a, REG, Sdd3rm>;
impl<'a, REG> Sdd3rmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SD card/MMC removal not detected by the SDnDAT3 pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdd3rm::_0)
    }
    ///SD card/MMC removal detected by the SDnDAT3 pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdd3rm::_1)
    }
}
/**SDnDAT3 Insertion Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdd3in {
    ///0: SD card/MMC insertion not detected by the SDnDAT3 pin
    _0 = 0,
    ///1: SD card/MMC insertion detected by the SDnDAT3 pin
    _1 = 1,
}
impl From<Sdd3in> for bool {
    #[inline(always)]
    fn from(variant: Sdd3in) -> Self {
        variant as u8 != 0
    }
}
///Field `SDD3IN` reader - SDnDAT3 Insertion Flag
pub type Sdd3inR = crate::BitReader<Sdd3in>;
impl Sdd3inR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdd3in {
        match self.bits {
            false => Sdd3in::_0,
            true => Sdd3in::_1,
        }
    }
    ///SD card/MMC insertion not detected by the SDnDAT3 pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdd3in::_0
    }
    ///SD card/MMC insertion detected by the SDnDAT3 pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdd3in::_1
    }
}
///Field `SDD3IN` writer - SDnDAT3 Insertion Flag
pub type Sdd3inW<'a, REG> = crate::BitWriter<'a, REG, Sdd3in>;
impl<'a, REG> Sdd3inW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SD card/MMC insertion not detected by the SDnDAT3 pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdd3in::_0)
    }
    ///SD card/MMC insertion detected by the SDnDAT3 pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdd3in::_1)
    }
}
/**SDnDAT3 Pin Monitor Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdd3mon {
    ///0: SDnDAT3 pin level is low
    _0 = 0,
    ///1: SDnDAT3 pin level is high
    _1 = 1,
}
impl From<Sdd3mon> for bool {
    #[inline(always)]
    fn from(variant: Sdd3mon) -> Self {
        variant as u8 != 0
    }
}
///Field `SDD3MON` reader - SDnDAT3 Pin Monitor Flag
pub type Sdd3monR = crate::BitReader<Sdd3mon>;
impl Sdd3monR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdd3mon {
        match self.bits {
            false => Sdd3mon::_0,
            true => Sdd3mon::_1,
        }
    }
    ///SDnDAT3 pin level is low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdd3mon::_0
    }
    ///SDnDAT3 pin level is high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdd3mon::_1
    }
}
impl R {
    ///Bit 0 - Response End Detection Flag
    #[inline(always)]
    pub fn rspend(&self) -> RspendR {
        RspendR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Access End Detection Flag
    #[inline(always)]
    pub fn acend(&self) -> AcendR {
        AcendR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SDnCD Removal Flag
    #[inline(always)]
    pub fn sdcdrm(&self) -> SdcdrmR {
        SdcdrmR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SDnCD Insertion Flag
    #[inline(always)]
    pub fn sdcdin(&self) -> SdcdinR {
        SdcdinR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SDnCD Pin Monitor Flag
    #[inline(always)]
    pub fn sdcdmon(&self) -> SdcdmonR {
        SdcdmonR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - SDnWP Pin Monitor Flag
    #[inline(always)]
    pub fn sdwpmon(&self) -> SdwpmonR {
        SdwpmonR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SDnDAT3 Removal Flag
    #[inline(always)]
    pub fn sdd3rm(&self) -> Sdd3rmR {
        Sdd3rmR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SDnDAT3 Insertion Flag
    #[inline(always)]
    pub fn sdd3in(&self) -> Sdd3inR {
        Sdd3inR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SDnDAT3 Pin Monitor Flag
    #[inline(always)]
    pub fn sdd3mon(&self) -> Sdd3monR {
        Sdd3monR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_INFO1")
            .field("rspend", &self.rspend())
            .field("acend", &self.acend())
            .field("sdcdrm", &self.sdcdrm())
            .field("sdcdin", &self.sdcdin())
            .field("sdcdmon", &self.sdcdmon())
            .field("sdwpmon", &self.sdwpmon())
            .field("sdd3rm", &self.sdd3rm())
            .field("sdd3in", &self.sdd3in())
            .field("sdd3mon", &self.sdd3mon())
            .finish()
    }
}
impl W {
    ///Bit 0 - Response End Detection Flag
    #[inline(always)]
    pub fn rspend(&mut self) -> RspendW<SdInfo1Spec> {
        RspendW::new(self, 0)
    }
    ///Bit 2 - Access End Detection Flag
    #[inline(always)]
    pub fn acend(&mut self) -> AcendW<SdInfo1Spec> {
        AcendW::new(self, 2)
    }
    ///Bit 3 - SDnCD Removal Flag
    #[inline(always)]
    pub fn sdcdrm(&mut self) -> SdcdrmW<SdInfo1Spec> {
        SdcdrmW::new(self, 3)
    }
    ///Bit 4 - SDnCD Insertion Flag
    #[inline(always)]
    pub fn sdcdin(&mut self) -> SdcdinW<SdInfo1Spec> {
        SdcdinW::new(self, 4)
    }
    ///Bit 8 - SDnDAT3 Removal Flag
    #[inline(always)]
    pub fn sdd3rm(&mut self) -> Sdd3rmW<SdInfo1Spec> {
        Sdd3rmW::new(self, 8)
    }
    ///Bit 9 - SDnDAT3 Insertion Flag
    #[inline(always)]
    pub fn sdd3in(&mut self) -> Sdd3inW<SdInfo1Spec> {
        Sdd3inW::new(self, 9)
    }
}
/**SD Card Interrupt Flag Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_info1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdInfo1Spec;
impl crate::RegisterSpec for SdInfo1Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_info1::R`](R) reader structure
impl crate::Readable for SdInfo1Spec {}
///`write(|w| ..)` method takes [`sd_info1::W`](W) writer structure
impl crate::Writable for SdInfo1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_INFO1 to value 0
impl crate::Resettable for SdInfo1Spec {}
