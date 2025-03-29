///Register `SD_INFO2_MASK` reader
pub type R = crate::R<SdInfo2MaskSpec>;
///Register `SD_INFO2_MASK` writer
pub type W = crate::W<SdInfo2MaskSpec>;
/**Command Error Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdem {
    ///0: Do not mask command error interrupt request
    _0 = 0,
    ///1: Mask command error interrupt request
    _1 = 1,
}
impl From<Cmdem> for bool {
    #[inline(always)]
    fn from(variant: Cmdem) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDEM` reader - Command Error Interrupt Request Mask
pub type CmdemR = crate::BitReader<Cmdem>;
impl CmdemR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmdem {
        match self.bits {
            false => Cmdem::_0,
            true => Cmdem::_1,
        }
    }
    ///Do not mask command error interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmdem::_0
    }
    ///Mask command error interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmdem::_1
    }
}
///Field `CMDEM` writer - Command Error Interrupt Request Mask
pub type CmdemW<'a, REG> = crate::BitWriter<'a, REG, Cmdem>;
impl<'a, REG> CmdemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask command error interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdem::_0)
    }
    ///Mask command error interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdem::_1)
    }
}
/**CRC Error Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcem {
    ///0: Do not mask CRC error interrupt request
    _0 = 0,
    ///1: Mask CRC error interrupt request
    _1 = 1,
}
impl From<Crcem> for bool {
    #[inline(always)]
    fn from(variant: Crcem) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCEM` reader - CRC Error Interrupt Request Mask
pub type CrcemR = crate::BitReader<Crcem>;
impl CrcemR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Crcem {
        match self.bits {
            false => Crcem::_0,
            true => Crcem::_1,
        }
    }
    ///Do not mask CRC error interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crcem::_0
    }
    ///Mask CRC error interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crcem::_1
    }
}
///Field `CRCEM` writer - CRC Error Interrupt Request Mask
pub type CrcemW<'a, REG> = crate::BitWriter<'a, REG, Crcem>;
impl<'a, REG> CrcemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask CRC error interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcem::_0)
    }
    ///Mask CRC error interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcem::_1)
    }
}
/**End Bit Error Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endem {
    ///0: Do not mask end bit detection error interrupt request
    _0 = 0,
    ///1: Mask end bit detection error interrupt request
    _1 = 1,
}
impl From<Endem> for bool {
    #[inline(always)]
    fn from(variant: Endem) -> Self {
        variant as u8 != 0
    }
}
///Field `ENDEM` reader - End Bit Error Interrupt Request Mask
pub type EndemR = crate::BitReader<Endem>;
impl EndemR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Endem {
        match self.bits {
            false => Endem::_0,
            true => Endem::_1,
        }
    }
    ///Do not mask end bit detection error interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Endem::_0
    }
    ///Mask end bit detection error interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Endem::_1
    }
}
///Field `ENDEM` writer - End Bit Error Interrupt Request Mask
pub type EndemW<'a, REG> = crate::BitWriter<'a, REG, Endem>;
impl<'a, REG> EndemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask end bit detection error interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Endem::_0)
    }
    ///Mask end bit detection error interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Endem::_1)
    }
}
/**Data Timeout Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtom {
    ///0: Do not mask data timeout interrupt request
    _0 = 0,
    ///1: Mask data timeout interrupt request
    _1 = 1,
}
impl From<Dtom> for bool {
    #[inline(always)]
    fn from(variant: Dtom) -> Self {
        variant as u8 != 0
    }
}
///Field `DTOM` reader - Data Timeout Interrupt Request Mask
pub type DtomR = crate::BitReader<Dtom>;
impl DtomR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dtom {
        match self.bits {
            false => Dtom::_0,
            true => Dtom::_1,
        }
    }
    ///Do not mask data timeout interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtom::_0
    }
    ///Mask data timeout interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtom::_1
    }
}
///Field `DTOM` writer - Data Timeout Interrupt Request Mask
pub type DtomW<'a, REG> = crate::BitWriter<'a, REG, Dtom>;
impl<'a, REG> DtomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask data timeout interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtom::_0)
    }
    ///Mask data timeout interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtom::_1)
    }
}
/**SD_BUF0 Register Illegal Write Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilwm {
    ///0: Do not mask illegal write detection interrupt request for the SD_BUF0 register
    _0 = 0,
    ///1: Mask illegal write detection interrupt request for the SD_BUF0 register
    _1 = 1,
}
impl From<Ilwm> for bool {
    #[inline(always)]
    fn from(variant: Ilwm) -> Self {
        variant as u8 != 0
    }
}
///Field `ILWM` reader - SD_BUF0 Register Illegal Write Interrupt Request Mask
pub type IlwmR = crate::BitReader<Ilwm>;
impl IlwmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ilwm {
        match self.bits {
            false => Ilwm::_0,
            true => Ilwm::_1,
        }
    }
    ///Do not mask illegal write detection interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilwm::_0
    }
    ///Mask illegal write detection interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilwm::_1
    }
}
///Field `ILWM` writer - SD_BUF0 Register Illegal Write Interrupt Request Mask
pub type IlwmW<'a, REG> = crate::BitWriter<'a, REG, Ilwm>;
impl<'a, REG> IlwmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask illegal write detection interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ilwm::_0)
    }
    ///Mask illegal write detection interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ilwm::_1)
    }
}
/**SD_BUF0 Register Illegal Read Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilrm {
    ///0: Do not mask illegal read detection interrupt request for the SD_BUF0 register
    _0 = 0,
    ///1: Mask illegal read detection interrupt request for the SD_BUF0 register
    _1 = 1,
}
impl From<Ilrm> for bool {
    #[inline(always)]
    fn from(variant: Ilrm) -> Self {
        variant as u8 != 0
    }
}
///Field `ILRM` reader - SD_BUF0 Register Illegal Read Interrupt Request Mask
pub type IlrmR = crate::BitReader<Ilrm>;
impl IlrmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ilrm {
        match self.bits {
            false => Ilrm::_0,
            true => Ilrm::_1,
        }
    }
    ///Do not mask illegal read detection interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilrm::_0
    }
    ///Mask illegal read detection interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilrm::_1
    }
}
///Field `ILRM` writer - SD_BUF0 Register Illegal Read Interrupt Request Mask
pub type IlrmW<'a, REG> = crate::BitWriter<'a, REG, Ilrm>;
impl<'a, REG> IlrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask illegal read detection interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ilrm::_0)
    }
    ///Mask illegal read detection interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ilrm::_1)
    }
}
/**Response Timeout Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsptom {
    ///0: Do not mask response timeout interrupt request
    _0 = 0,
    ///1: Mask response timeout interrupt request
    _1 = 1,
}
impl From<Rsptom> for bool {
    #[inline(always)]
    fn from(variant: Rsptom) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPTOM` reader - Response Timeout Interrupt Request Mask
pub type RsptomR = crate::BitReader<Rsptom>;
impl RsptomR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rsptom {
        match self.bits {
            false => Rsptom::_0,
            true => Rsptom::_1,
        }
    }
    ///Do not mask response timeout interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rsptom::_0
    }
    ///Mask response timeout interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rsptom::_1
    }
}
///Field `RSPTOM` writer - Response Timeout Interrupt Request Mask
pub type RsptomW<'a, REG> = crate::BitWriter<'a, REG, Rsptom>;
impl<'a, REG> RsptomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask response timeout interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptom::_0)
    }
    ///Mask response timeout interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptom::_1)
    }
}
/**BRE Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brem {
    ///0: Do not mask read enable interrupt request for the SD buffer
    _0 = 0,
    ///1: Mask read enable interrupt request for the SD buffer
    _1 = 1,
}
impl From<Brem> for bool {
    #[inline(always)]
    fn from(variant: Brem) -> Self {
        variant as u8 != 0
    }
}
///Field `BREM` reader - BRE Interrupt Request Mask
pub type BremR = crate::BitReader<Brem>;
impl BremR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Brem {
        match self.bits {
            false => Brem::_0,
            true => Brem::_1,
        }
    }
    ///Do not mask read enable interrupt request for the SD buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Brem::_0
    }
    ///Mask read enable interrupt request for the SD buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Brem::_1
    }
}
///Field `BREM` writer - BRE Interrupt Request Mask
pub type BremW<'a, REG> = crate::BitWriter<'a, REG, Brem>;
impl<'a, REG> BremW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask read enable interrupt request for the SD buffer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Brem::_0)
    }
    ///Mask read enable interrupt request for the SD buffer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Brem::_1)
    }
}
/**BWE Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwem {
    ///0: Do not mask write enable interrupt request for the SD_BUF0 register
    _0 = 0,
    ///1: Mask write enable interrupt request for the SD_BUF0 register
    _1 = 1,
}
impl From<Bwem> for bool {
    #[inline(always)]
    fn from(variant: Bwem) -> Self {
        variant as u8 != 0
    }
}
///Field `BWEM` reader - BWE Interrupt Request Mask
pub type BwemR = crate::BitReader<Bwem>;
impl BwemR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bwem {
        match self.bits {
            false => Bwem::_0,
            true => Bwem::_1,
        }
    }
    ///Do not mask write enable interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bwem::_0
    }
    ///Mask write enable interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bwem::_1
    }
}
///Field `BWEM` writer - BWE Interrupt Request Mask
pub type BwemW<'a, REG> = crate::BitWriter<'a, REG, Bwem>;
impl<'a, REG> BwemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask write enable interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bwem::_0)
    }
    ///Mask write enable interrupt request for the SD_BUF0 register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwem::_1)
    }
}
/**Illegal Access Error Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilam {
    ///0: Do not mask illegal access error interrupt request
    _0 = 0,
    ///1: Mask illegal access error interrupt request
    _1 = 1,
}
impl From<Ilam> for bool {
    #[inline(always)]
    fn from(variant: Ilam) -> Self {
        variant as u8 != 0
    }
}
///Field `ILAM` reader - Illegal Access Error Interrupt Request Mask
pub type IlamR = crate::BitReader<Ilam>;
impl IlamR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ilam {
        match self.bits {
            false => Ilam::_0,
            true => Ilam::_1,
        }
    }
    ///Do not mask illegal access error interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilam::_0
    }
    ///Mask illegal access error interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilam::_1
    }
}
///Field `ILAM` writer - Illegal Access Error Interrupt Request Mask
pub type IlamW<'a, REG> = crate::BitWriter<'a, REG, Ilam>;
impl<'a, REG> IlamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not mask illegal access error interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ilam::_0)
    }
    ///Mask illegal access error interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ilam::_1)
    }
}
impl R {
    ///Bit 0 - Command Error Interrupt Request Mask
    #[inline(always)]
    pub fn cmdem(&self) -> CmdemR {
        CmdemR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CRC Error Interrupt Request Mask
    #[inline(always)]
    pub fn crcem(&self) -> CrcemR {
        CrcemR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End Bit Error Interrupt Request Mask
    #[inline(always)]
    pub fn endem(&self) -> EndemR {
        EndemR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Data Timeout Interrupt Request Mask
    #[inline(always)]
    pub fn dtom(&self) -> DtomR {
        DtomR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SD_BUF0 Register Illegal Write Interrupt Request Mask
    #[inline(always)]
    pub fn ilwm(&self) -> IlwmR {
        IlwmR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SD_BUF0 Register Illegal Read Interrupt Request Mask
    #[inline(always)]
    pub fn ilrm(&self) -> IlrmR {
        IlrmR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Response Timeout Interrupt Request Mask
    #[inline(always)]
    pub fn rsptom(&self) -> RsptomR {
        RsptomR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - BRE Interrupt Request Mask
    #[inline(always)]
    pub fn brem(&self) -> BremR {
        BremR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BWE Interrupt Request Mask
    #[inline(always)]
    pub fn bwem(&self) -> BwemR {
        BwemR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - Illegal Access Error Interrupt Request Mask
    #[inline(always)]
    pub fn ilam(&self) -> IlamR {
        IlamR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_INFO2_MASK")
            .field("cmdem", &self.cmdem())
            .field("crcem", &self.crcem())
            .field("endem", &self.endem())
            .field("dtom", &self.dtom())
            .field("ilwm", &self.ilwm())
            .field("ilrm", &self.ilrm())
            .field("rsptom", &self.rsptom())
            .field("brem", &self.brem())
            .field("bwem", &self.bwem())
            .field("ilam", &self.ilam())
            .finish()
    }
}
impl W {
    ///Bit 0 - Command Error Interrupt Request Mask
    #[inline(always)]
    pub fn cmdem(&mut self) -> CmdemW<SdInfo2MaskSpec> {
        CmdemW::new(self, 0)
    }
    ///Bit 1 - CRC Error Interrupt Request Mask
    #[inline(always)]
    pub fn crcem(&mut self) -> CrcemW<SdInfo2MaskSpec> {
        CrcemW::new(self, 1)
    }
    ///Bit 2 - End Bit Error Interrupt Request Mask
    #[inline(always)]
    pub fn endem(&mut self) -> EndemW<SdInfo2MaskSpec> {
        EndemW::new(self, 2)
    }
    ///Bit 3 - Data Timeout Interrupt Request Mask
    #[inline(always)]
    pub fn dtom(&mut self) -> DtomW<SdInfo2MaskSpec> {
        DtomW::new(self, 3)
    }
    ///Bit 4 - SD_BUF0 Register Illegal Write Interrupt Request Mask
    #[inline(always)]
    pub fn ilwm(&mut self) -> IlwmW<SdInfo2MaskSpec> {
        IlwmW::new(self, 4)
    }
    ///Bit 5 - SD_BUF0 Register Illegal Read Interrupt Request Mask
    #[inline(always)]
    pub fn ilrm(&mut self) -> IlrmW<SdInfo2MaskSpec> {
        IlrmW::new(self, 5)
    }
    ///Bit 6 - Response Timeout Interrupt Request Mask
    #[inline(always)]
    pub fn rsptom(&mut self) -> RsptomW<SdInfo2MaskSpec> {
        RsptomW::new(self, 6)
    }
    ///Bit 8 - BRE Interrupt Request Mask
    #[inline(always)]
    pub fn brem(&mut self) -> BremW<SdInfo2MaskSpec> {
        BremW::new(self, 8)
    }
    ///Bit 9 - BWE Interrupt Request Mask
    #[inline(always)]
    pub fn bwem(&mut self) -> BwemW<SdInfo2MaskSpec> {
        BwemW::new(self, 9)
    }
    ///Bit 15 - Illegal Access Error Interrupt Request Mask
    #[inline(always)]
    pub fn ilam(&mut self) -> IlamW<SdInfo2MaskSpec> {
        IlamW::new(self, 15)
    }
}
/**SD INFO2 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sd_info2_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info2_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdInfo2MaskSpec;
impl crate::RegisterSpec for SdInfo2MaskSpec {
    type Ux = u32;
}
///`read()` method returns [`sd_info2_mask::R`](R) reader structure
impl crate::Readable for SdInfo2MaskSpec {}
///`write(|w| ..)` method takes [`sd_info2_mask::W`](W) writer structure
impl crate::Writable for SdInfo2MaskSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_INFO2_MASK to value 0x8b7f
impl crate::Resettable for SdInfo2MaskSpec {
    const RESET_VALUE: u32 = 0x8b7f;
}
