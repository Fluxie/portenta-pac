///Register `SD_OPTION` reader
pub type R = crate::R<SdOptionSpec>;
///Register `SD_OPTION` writer
pub type W = crate::W<SdOptionSpec>;
/**Card Detection Time Counter

Value on reset: 14*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctop {
    ///0: PCLKB × 210
    _0x0 = 0,
    ///1: PCLKB × 211
    _0x1 = 1,
    ///2: PCLKB × 212
    _0x2 = 2,
    ///3: PCLKB × 213
    _0x3 = 3,
    ///4: PCLKB × 214
    _0x4 = 4,
    ///5: PCLKB × 215
    _0x5 = 5,
    ///6: PCLKB × 216
    _0x6 = 6,
    ///7: PCLKB × 217
    _0x7 = 7,
    ///8: PCLKB × 218
    _0x8 = 8,
    ///9: PCLKB × 219
    _0x9 = 9,
    ///10: PCLKB × 220
    _0xA = 10,
    ///11: PCLKB × 221
    _0xB = 11,
    ///12: PCLKB × 222
    _0xC = 12,
    ///13: PCLKB × 223
    _0xD = 13,
    ///14: PCLKB × 224
    _0xE = 14,
    ///15: Setting prohibited
    _0xF = 15,
}
impl From<Ctop> for u8 {
    #[inline(always)]
    fn from(variant: Ctop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctop {
    type Ux = u8;
}
impl crate::IsEnum for Ctop {}
///Field `CTOP` reader - Card Detection Time Counter
pub type CtopR = crate::FieldReader<Ctop>;
impl CtopR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctop {
        match self.bits {
            0 => Ctop::_0x0,
            1 => Ctop::_0x1,
            2 => Ctop::_0x2,
            3 => Ctop::_0x3,
            4 => Ctop::_0x4,
            5 => Ctop::_0x5,
            6 => Ctop::_0x6,
            7 => Ctop::_0x7,
            8 => Ctop::_0x8,
            9 => Ctop::_0x9,
            10 => Ctop::_0xA,
            11 => Ctop::_0xB,
            12 => Ctop::_0xC,
            13 => Ctop::_0xD,
            14 => Ctop::_0xE,
            15 => Ctop::_0xF,
            _ => unreachable!(),
        }
    }
    ///PCLKB × 210
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Ctop::_0x0
    }
    ///PCLKB × 211
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Ctop::_0x1
    }
    ///PCLKB × 212
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Ctop::_0x2
    }
    ///PCLKB × 213
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Ctop::_0x3
    }
    ///PCLKB × 214
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Ctop::_0x4
    }
    ///PCLKB × 215
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Ctop::_0x5
    }
    ///PCLKB × 216
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Ctop::_0x6
    }
    ///PCLKB × 217
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Ctop::_0x7
    }
    ///PCLKB × 218
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Ctop::_0x8
    }
    ///PCLKB × 219
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Ctop::_0x9
    }
    ///PCLKB × 220
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == Ctop::_0xA
    }
    ///PCLKB × 221
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Ctop::_0xB
    }
    ///PCLKB × 222
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == Ctop::_0xC
    }
    ///PCLKB × 223
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == Ctop::_0xD
    }
    ///PCLKB × 224
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == Ctop::_0xE
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == Ctop::_0xF
    }
}
///Field `CTOP` writer - Card Detection Time Counter
pub type CtopW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ctop, crate::Safe>;
impl<'a, REG> CtopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKB × 210
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0x0)
    }
    ///PCLKB × 211
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0x1)
    }
    ///PCLKB × 212
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0x2)
    }
    ///PCLKB × 213
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0x3)
    }
    ///PCLKB × 214
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0x4)
    }
    ///PCLKB × 215
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0x5)
    }
    ///PCLKB × 216
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0x6)
    }
    ///PCLKB × 217
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0x7)
    }
    ///PCLKB × 218
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0x8)
    }
    ///PCLKB × 219
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0x9)
    }
    ///PCLKB × 220
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0xA)
    }
    ///PCLKB × 221
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0xB)
    }
    ///PCLKB × 222
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0xC)
    }
    ///PCLKB × 223
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0xD)
    }
    ///PCLKB × 224
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0xE)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Ctop::_0xF)
    }
}
/**Timeout Counter

Value on reset: 14*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Top {
    ///0: SD/MMC clock × 213
    _0x0 = 0,
    ///1: SD/MMC clock × 214
    _0x1 = 1,
    ///2: SD/MMC clock × 215
    _0x2 = 2,
    ///3: SD/MMC clock × 216
    _0x3 = 3,
    ///4: SD/MMC clock × 217
    _0x4 = 4,
    ///5: SD/MMC clock × 218
    _0x5 = 5,
    ///6: SD/MMC clock × 219
    _0x6 = 6,
    ///7: SD/MMC clock × 220
    _0x7 = 7,
    ///8: SD/MMC clock × 221
    _0x8 = 8,
    ///9: SD/MMC clock × 222
    _0x9 = 9,
    ///10: SD/MMC clock × 223
    _0xA = 10,
    ///11: SD/MMC clock × 224
    _0xB = 11,
    ///12: SD/MMC clock × 225
    _0xC = 12,
    ///13: SD/MMC clock × 226
    _0xD = 13,
    ///14: SD/MMC clock × 227
    _0xE = 14,
    ///15: Setting prohibited
    _0xF = 15,
}
impl From<Top> for u8 {
    #[inline(always)]
    fn from(variant: Top) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Top {
    type Ux = u8;
}
impl crate::IsEnum for Top {}
///Field `TOP` reader - Timeout Counter
pub type TopR = crate::FieldReader<Top>;
impl TopR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Top {
        match self.bits {
            0 => Top::_0x0,
            1 => Top::_0x1,
            2 => Top::_0x2,
            3 => Top::_0x3,
            4 => Top::_0x4,
            5 => Top::_0x5,
            6 => Top::_0x6,
            7 => Top::_0x7,
            8 => Top::_0x8,
            9 => Top::_0x9,
            10 => Top::_0xA,
            11 => Top::_0xB,
            12 => Top::_0xC,
            13 => Top::_0xD,
            14 => Top::_0xE,
            15 => Top::_0xF,
            _ => unreachable!(),
        }
    }
    ///SD/MMC clock × 213
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Top::_0x0
    }
    ///SD/MMC clock × 214
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Top::_0x1
    }
    ///SD/MMC clock × 215
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Top::_0x2
    }
    ///SD/MMC clock × 216
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Top::_0x3
    }
    ///SD/MMC clock × 217
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Top::_0x4
    }
    ///SD/MMC clock × 218
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Top::_0x5
    }
    ///SD/MMC clock × 219
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Top::_0x6
    }
    ///SD/MMC clock × 220
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Top::_0x7
    }
    ///SD/MMC clock × 221
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Top::_0x8
    }
    ///SD/MMC clock × 222
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Top::_0x9
    }
    ///SD/MMC clock × 223
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == Top::_0xA
    }
    ///SD/MMC clock × 224
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Top::_0xB
    }
    ///SD/MMC clock × 225
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == Top::_0xC
    }
    ///SD/MMC clock × 226
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == Top::_0xD
    }
    ///SD/MMC clock × 227
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == Top::_0xE
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == Top::_0xF
    }
}
///Field `TOP` writer - Timeout Counter
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 4, Top, crate::Safe>;
impl<'a, REG> TopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SD/MMC clock × 213
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0x0)
    }
    ///SD/MMC clock × 214
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0x1)
    }
    ///SD/MMC clock × 215
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0x2)
    }
    ///SD/MMC clock × 216
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0x3)
    }
    ///SD/MMC clock × 217
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0x4)
    }
    ///SD/MMC clock × 218
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0x5)
    }
    ///SD/MMC clock × 219
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0x6)
    }
    ///SD/MMC clock × 220
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0x7)
    }
    ///SD/MMC clock × 221
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0x8)
    }
    ///SD/MMC clock × 222
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0x9)
    }
    ///SD/MMC clock × 223
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0xA)
    }
    ///SD/MMC clock × 224
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0xB)
    }
    ///SD/MMC clock × 225
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0xC)
    }
    ///SD/MMC clock × 226
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0xD)
    }
    ///SD/MMC clock × 227
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0xE)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Top::_0xF)
    }
}
/**Timeout Mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toutmask {
    ///0: Activate timeout
    _0 = 0,
    ///1: Inactivate timeout (do not set RSPTO and DTO bits of SD_INFO2 or CRCBSYTO, CRCTO, RDTO, BSYTO1, BSYTO0, RSPTO1 and RSPTO0 bits of SD_ERR_STS2) When timeout occurs because of an inactivated timeout, execute a software reset to terminate the command sequence.
    _1 = 1,
}
impl From<Toutmask> for bool {
    #[inline(always)]
    fn from(variant: Toutmask) -> Self {
        variant as u8 != 0
    }
}
///Field `TOUTMASK` reader - Timeout Mask
pub type ToutmaskR = crate::BitReader<Toutmask>;
impl ToutmaskR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Toutmask {
        match self.bits {
            false => Toutmask::_0,
            true => Toutmask::_1,
        }
    }
    ///Activate timeout
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toutmask::_0
    }
    ///Inactivate timeout (do not set RSPTO and DTO bits of SD_INFO2 or CRCBSYTO, CRCTO, RDTO, BSYTO1, BSYTO0, RSPTO1 and RSPTO0 bits of SD_ERR_STS2) When timeout occurs because of an inactivated timeout, execute a software reset to terminate the command sequence.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toutmask::_1
    }
}
///Field `TOUTMASK` writer - Timeout Mask
pub type ToutmaskW<'a, REG> = crate::BitWriter<'a, REG, Toutmask>;
impl<'a, REG> ToutmaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Activate timeout
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toutmask::_0)
    }
    ///Inactivate timeout (do not set RSPTO and DTO bits of SD_INFO2 or CRCBSYTO, CRCTO, RDTO, BSYTO1, BSYTO0, RSPTO1 and RSPTO0 bits of SD_ERR_STS2) When timeout occurs because of an inactivated timeout, execute a software reset to terminate the command sequence.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toutmask::_1)
    }
}
///Field `WIDTH8` reader - Bus Width
pub type Width8R = crate::BitReader;
///Field `WIDTH8` writer - Bus Width
pub type Width8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIDTH` reader - Bus Width
pub type WidthR = crate::BitReader;
///Field `WIDTH` writer - Bus Width
pub type WidthW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Card Detection Time Counter
    #[inline(always)]
    pub fn ctop(&self) -> CtopR {
        CtopR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Timeout Counter
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Timeout Mask
    #[inline(always)]
    pub fn toutmask(&self) -> ToutmaskR {
        ToutmaskR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 13 - Bus Width
    #[inline(always)]
    pub fn width8(&self) -> Width8R {
        Width8R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Bus Width
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_OPTION")
            .field("ctop", &self.ctop())
            .field("top", &self.top())
            .field("toutmask", &self.toutmask())
            .field("width8", &self.width8())
            .field("width", &self.width())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Card Detection Time Counter
    #[inline(always)]
    pub fn ctop(&mut self) -> CtopW<SdOptionSpec> {
        CtopW::new(self, 0)
    }
    ///Bits 4:7 - Timeout Counter
    #[inline(always)]
    pub fn top(&mut self) -> TopW<SdOptionSpec> {
        TopW::new(self, 4)
    }
    ///Bit 8 - Timeout Mask
    #[inline(always)]
    pub fn toutmask(&mut self) -> ToutmaskW<SdOptionSpec> {
        ToutmaskW::new(self, 8)
    }
    ///Bit 13 - Bus Width
    #[inline(always)]
    pub fn width8(&mut self) -> Width8W<SdOptionSpec> {
        Width8W::new(self, 13)
    }
    ///Bit 15 - Bus Width
    #[inline(always)]
    pub fn width(&mut self) -> WidthW<SdOptionSpec> {
        WidthW::new(self, 15)
    }
}
/**SD Card Access Control Option Register

You can [`read`](crate::Reg::read) this register and get [`sd_option::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_option::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdOptionSpec;
impl crate::RegisterSpec for SdOptionSpec {
    type Ux = u32;
}
///`read()` method returns [`sd_option::R`](R) reader structure
impl crate::Readable for SdOptionSpec {}
///`write(|w| ..)` method takes [`sd_option::W`](W) writer structure
impl crate::Writable for SdOptionSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_OPTION to value 0x40ee
impl crate::Resettable for SdOptionSpec {
    const RESET_VALUE: u32 = 0x40ee;
}
