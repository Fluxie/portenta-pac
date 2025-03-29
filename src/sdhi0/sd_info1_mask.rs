///Register `SD_INFO1_MASK` reader
pub type R = crate::R<SdInfo1MaskSpec>;
///Register `SD_INFO1_MASK` writer
pub type W = crate::W<SdInfo1MaskSpec>;
/**Response End Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rspendm {
    ///0: Do not mask response end interrupt request
    _0 = 0,
    ///1: Mask response end interrupt request
    _1 = 1,
}
impl From<Rspendm> for bool {
    #[inline(always)]
    fn from(variant: Rspendm) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPENDM` reader - Response End Interrupt Request Mask
pub type RspendmR = crate::BitReader<Rspendm>;
impl RspendmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rspendm {
        match self.bits {
            false => Rspendm::_0,
            true => Rspendm::_1,
        }
    }
    ///Do not mask response end interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rspendm::_0
    }
    ///Mask response end interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rspendm::_1
    }
}
///Field `RSPENDM` writer - Response End Interrupt Request Mask
pub type RspendmW<'a, REG> = crate::BitWriter<'a, REG, Rspendm>;
impl<'a, REG> RspendmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask response end interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rspendm::_0)
    }
    ///Mask response end interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rspendm::_1)
    }
}
/**Access End Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acendm {
    ///0: Do not mask access end interrupt request
    _0 = 0,
    ///1: Mask access end interrupt request
    _1 = 1,
}
impl From<Acendm> for bool {
    #[inline(always)]
    fn from(variant: Acendm) -> Self {
        variant as u8 != 0
    }
}
///Field `ACENDM` reader - Access End Interrupt Request Mask
pub type AcendmR = crate::BitReader<Acendm>;
impl AcendmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Acendm {
        match self.bits {
            false => Acendm::_0,
            true => Acendm::_1,
        }
    }
    ///Do not mask access end interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acendm::_0
    }
    ///Mask access end interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acendm::_1
    }
}
///Field `ACENDM` writer - Access End Interrupt Request Mask
pub type AcendmW<'a, REG> = crate::BitWriter<'a, REG, Acendm>;
impl<'a, REG> AcendmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask access end interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acendm::_0)
    }
    ///Mask access end interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acendm::_1)
    }
}
/**SDnCD Removal Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdcdrmm {
    ///0: Do not mask SD card/MMC removal interrupt request by the SDnCD pin
    _0 = 0,
    ///1: Mask SD card/MMC removal interrupt request by the SDnCD pin
    _1 = 1,
}
impl From<Sdcdrmm> for bool {
    #[inline(always)]
    fn from(variant: Sdcdrmm) -> Self {
        variant as u8 != 0
    }
}
///Field `SDCDRMM` reader - SDnCD Removal Interrupt Request Mask
pub type SdcdrmmR = crate::BitReader<Sdcdrmm>;
impl SdcdrmmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdcdrmm {
        match self.bits {
            false => Sdcdrmm::_0,
            true => Sdcdrmm::_1,
        }
    }
    ///Do not mask SD card/MMC removal interrupt request by the SDnCD pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdcdrmm::_0
    }
    ///Mask SD card/MMC removal interrupt request by the SDnCD pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdcdrmm::_1
    }
}
///Field `SDCDRMM` writer - SDnCD Removal Interrupt Request Mask
pub type SdcdrmmW<'a, REG> = crate::BitWriter<'a, REG, Sdcdrmm>;
impl<'a, REG> SdcdrmmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask SD card/MMC removal interrupt request by the SDnCD pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcdrmm::_0)
    }
    ///Mask SD card/MMC removal interrupt request by the SDnCD pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcdrmm::_1)
    }
}
/**SDnCD Insertion Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdcdinm {
    ///0: Do not mask SD card/MMC insertion interrupt request by the SDnCD pin
    _0 = 0,
    ///1: Mask SD card/MMC insertion interrupt request by the SDnCD pin
    _1 = 1,
}
impl From<Sdcdinm> for bool {
    #[inline(always)]
    fn from(variant: Sdcdinm) -> Self {
        variant as u8 != 0
    }
}
///Field `SDCDINM` reader - SDnCD Insertion Interrupt Request Mask
pub type SdcdinmR = crate::BitReader<Sdcdinm>;
impl SdcdinmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdcdinm {
        match self.bits {
            false => Sdcdinm::_0,
            true => Sdcdinm::_1,
        }
    }
    ///Do not mask SD card/MMC insertion interrupt request by the SDnCD pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdcdinm::_0
    }
    ///Mask SD card/MMC insertion interrupt request by the SDnCD pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdcdinm::_1
    }
}
///Field `SDCDINM` writer - SDnCD Insertion Interrupt Request Mask
pub type SdcdinmW<'a, REG> = crate::BitWriter<'a, REG, Sdcdinm>;
impl<'a, REG> SdcdinmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask SD card/MMC insertion interrupt request by the SDnCD pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcdinm::_0)
    }
    ///Mask SD card/MMC insertion interrupt request by the SDnCD pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcdinm::_1)
    }
}
/**SDnDAT3 Removal Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdd3rmm {
    ///0: Do not mask SD card/MMC removal interrupt request by the SDnDAT3 pin
    _0 = 0,
    ///1: Mask SD card/MMC removal interrupt request by the SDnDAT3 pin
    _1 = 1,
}
impl From<Sdd3rmm> for bool {
    #[inline(always)]
    fn from(variant: Sdd3rmm) -> Self {
        variant as u8 != 0
    }
}
///Field `SDD3RMM` reader - SDnDAT3 Removal Interrupt Request Mask
pub type Sdd3rmmR = crate::BitReader<Sdd3rmm>;
impl Sdd3rmmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdd3rmm {
        match self.bits {
            false => Sdd3rmm::_0,
            true => Sdd3rmm::_1,
        }
    }
    ///Do not mask SD card/MMC removal interrupt request by the SDnDAT3 pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdd3rmm::_0
    }
    ///Mask SD card/MMC removal interrupt request by the SDnDAT3 pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdd3rmm::_1
    }
}
///Field `SDD3RMM` writer - SDnDAT3 Removal Interrupt Request Mask
pub type Sdd3rmmW<'a, REG> = crate::BitWriter<'a, REG, Sdd3rmm>;
impl<'a, REG> Sdd3rmmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask SD card/MMC removal interrupt request by the SDnDAT3 pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdd3rmm::_0)
    }
    ///Mask SD card/MMC removal interrupt request by the SDnDAT3 pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdd3rmm::_1)
    }
}
/**SDnDAT3 Insertion Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdd3inm {
    ///0: Do not mask SD card/MMC insertion interrupt request by the SDnDAT3 pin
    _0 = 0,
    ///1: Mask SD card/MMC insertion interrupt request by the SDnDAT3 pin
    _1 = 1,
}
impl From<Sdd3inm> for bool {
    #[inline(always)]
    fn from(variant: Sdd3inm) -> Self {
        variant as u8 != 0
    }
}
///Field `SDD3INM` reader - SDnDAT3 Insertion Interrupt Request Mask
pub type Sdd3inmR = crate::BitReader<Sdd3inm>;
impl Sdd3inmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdd3inm {
        match self.bits {
            false => Sdd3inm::_0,
            true => Sdd3inm::_1,
        }
    }
    ///Do not mask SD card/MMC insertion interrupt request by the SDnDAT3 pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdd3inm::_0
    }
    ///Mask SD card/MMC insertion interrupt request by the SDnDAT3 pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdd3inm::_1
    }
}
///Field `SDD3INM` writer - SDnDAT3 Insertion Interrupt Request Mask
pub type Sdd3inmW<'a, REG> = crate::BitWriter<'a, REG, Sdd3inm>;
impl<'a, REG> Sdd3inmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask SD card/MMC insertion interrupt request by the SDnDAT3 pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdd3inm::_0)
    }
    ///Mask SD card/MMC insertion interrupt request by the SDnDAT3 pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdd3inm::_1)
    }
}
impl R {
    ///Bit 0 - Response End Interrupt Request Mask
    #[inline(always)]
    pub fn rspendm(&self) -> RspendmR {
        RspendmR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Access End Interrupt Request Mask
    #[inline(always)]
    pub fn acendm(&self) -> AcendmR {
        AcendmR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SDnCD Removal Interrupt Request Mask
    #[inline(always)]
    pub fn sdcdrmm(&self) -> SdcdrmmR {
        SdcdrmmR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SDnCD Insertion Interrupt Request Mask
    #[inline(always)]
    pub fn sdcdinm(&self) -> SdcdinmR {
        SdcdinmR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SDnDAT3 Removal Interrupt Request Mask
    #[inline(always)]
    pub fn sdd3rmm(&self) -> Sdd3rmmR {
        Sdd3rmmR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SDnDAT3 Insertion Interrupt Request Mask
    #[inline(always)]
    pub fn sdd3inm(&self) -> Sdd3inmR {
        Sdd3inmR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_INFO1_MASK")
            .field("rspendm", &self.rspendm())
            .field("acendm", &self.acendm())
            .field("sdcdrmm", &self.sdcdrmm())
            .field("sdcdinm", &self.sdcdinm())
            .field("sdd3rmm", &self.sdd3rmm())
            .field("sdd3inm", &self.sdd3inm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Response End Interrupt Request Mask
    #[inline(always)]
    pub fn rspendm(&mut self) -> RspendmW<SdInfo1MaskSpec> {
        RspendmW::new(self, 0)
    }
    ///Bit 2 - Access End Interrupt Request Mask
    #[inline(always)]
    pub fn acendm(&mut self) -> AcendmW<SdInfo1MaskSpec> {
        AcendmW::new(self, 2)
    }
    ///Bit 3 - SDnCD Removal Interrupt Request Mask
    #[inline(always)]
    pub fn sdcdrmm(&mut self) -> SdcdrmmW<SdInfo1MaskSpec> {
        SdcdrmmW::new(self, 3)
    }
    ///Bit 4 - SDnCD Insertion Interrupt Request Mask
    #[inline(always)]
    pub fn sdcdinm(&mut self) -> SdcdinmW<SdInfo1MaskSpec> {
        SdcdinmW::new(self, 4)
    }
    ///Bit 8 - SDnDAT3 Removal Interrupt Request Mask
    #[inline(always)]
    pub fn sdd3rmm(&mut self) -> Sdd3rmmW<SdInfo1MaskSpec> {
        Sdd3rmmW::new(self, 8)
    }
    ///Bit 9 - SDnDAT3 Insertion Interrupt Request Mask
    #[inline(always)]
    pub fn sdd3inm(&mut self) -> Sdd3inmW<SdInfo1MaskSpec> {
        Sdd3inmW::new(self, 9)
    }
}
/**SD INFO1 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sd_info1_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info1_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdInfo1MaskSpec;
impl crate::RegisterSpec for SdInfo1MaskSpec {
    type Ux = u32;
}
///`read()` method returns [`sd_info1_mask::R`](R) reader structure
impl crate::Readable for SdInfo1MaskSpec {}
///`write(|w| ..)` method takes [`sd_info1_mask::W`](W) writer structure
impl crate::Writable for SdInfo1MaskSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_INFO1_MASK to value 0x031d
impl crate::Resettable for SdInfo1MaskSpec {
    const RESET_VALUE: u32 = 0x031d;
}
