///Register `SD_CLK_CTRL` reader
pub type R = crate::R<SdClkCtrlSpec>;
///Register `SD_CLK_CTRL` writer
pub type W = crate::W<SdClkCtrlSpec>;
/**SDHI Clock Frequency Select

Value on reset: 32*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    ///255: PCLKB
    _0xFf = 255,
    ///0: PCLKB/2
    _0x00 = 0,
    ///1: PCLKB/4
    _0x01 = 1,
    ///2: PCLKB/8
    _0x02 = 2,
    ///4: PCLKB/16
    _0x04 = 4,
    ///8: PCLKB/32
    _0x08 = 8,
    ///16: PCLKB/64
    _0x10 = 16,
    ///32: PCLKB/128
    _0x20 = 32,
    ///64: PCLKB/256
    _0x40 = 64,
    ///128: PCLKB/512
    _0x80 = 128,
    ///3: Setting prohibited
    Others = 3,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
///Field `CLKSEL` reader - SDHI Clock Frequency Select
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            255 => Clksel::_0xFf,
            0 => Clksel::_0x00,
            1 => Clksel::_0x01,
            2 => Clksel::_0x02,
            4 => Clksel::_0x04,
            8 => Clksel::_0x08,
            16 => Clksel::_0x10,
            32 => Clksel::_0x20,
            64 => Clksel::_0x40,
            128 => Clksel::_0x80,
            _ => Clksel::Others,
        }
    }
    ///PCLKB
    #[inline(always)]
    pub fn is_0x_ff(&self) -> bool {
        *self == Clksel::_0xFf
    }
    ///PCLKB/2
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == Clksel::_0x00
    }
    ///PCLKB/4
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == Clksel::_0x01
    }
    ///PCLKB/8
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == Clksel::_0x02
    }
    ///PCLKB/16
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == Clksel::_0x04
    }
    ///PCLKB/32
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == Clksel::_0x08
    }
    ///PCLKB/64
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == Clksel::_0x10
    }
    ///PCLKB/128
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == Clksel::_0x20
    }
    ///PCLKB/256
    #[inline(always)]
    pub fn is_0x40(&self) -> bool {
        *self == Clksel::_0x40
    }
    ///PCLKB/512
    #[inline(always)]
    pub fn is_0x80(&self) -> bool {
        *self == Clksel::_0x80
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Clksel::Others)
    }
}
///Field `CLKSEL` writer - SDHI Clock Frequency Select
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 8, Clksel, crate::Safe>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKB
    #[inline(always)]
    pub fn _0x_ff(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_0xFf)
    }
    ///PCLKB/2
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_0x00)
    }
    ///PCLKB/4
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_0x01)
    }
    ///PCLKB/8
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_0x02)
    }
    ///PCLKB/16
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_0x04)
    }
    ///PCLKB/32
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_0x08)
    }
    ///PCLKB/64
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_0x10)
    }
    ///PCLKB/128
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_0x20)
    }
    ///PCLKB/256
    #[inline(always)]
    pub fn _0x40(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_0x40)
    }
    ///PCLKB/512
    #[inline(always)]
    pub fn _0x80(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_0x80)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Others)
    }
}
/**SD/MMC Clock Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clken {
    ///0: Disable SD/MMC clock output (fix SDnCLK signal low)
    _0 = 0,
    ///1: Enable SD/MMC clock output
    _1 = 1,
}
impl From<Clken> for bool {
    #[inline(always)]
    fn from(variant: Clken) -> Self {
        variant as u8 != 0
    }
}
///Field `CLKEN` reader - SD/MMC Clock Output Control
pub type ClkenR = crate::BitReader<Clken>;
impl ClkenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Clken {
        match self.bits {
            false => Clken::_0,
            true => Clken::_1,
        }
    }
    ///Disable SD/MMC clock output (fix SDnCLK signal low)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Clken::_0
    }
    ///Enable SD/MMC clock output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Clken::_1
    }
}
///Field `CLKEN` writer - SD/MMC Clock Output Control
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG, Clken>;
impl<'a, REG> ClkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable SD/MMC clock output (fix SDnCLK signal low)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::_0)
    }
    ///Enable SD/MMC clock output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::_1)
    }
}
/**SD/MMC Clock Output Automatic Control Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkctrlen {
    ///0: Disable automatic control of SD/MMC clock output
    _0 = 0,
    ///1: Enable automatic control of SD/MMC clock output
    _1 = 1,
}
impl From<Clkctrlen> for bool {
    #[inline(always)]
    fn from(variant: Clkctrlen) -> Self {
        variant as u8 != 0
    }
}
///Field `CLKCTRLEN` reader - SD/MMC Clock Output Automatic Control Select
pub type ClkctrlenR = crate::BitReader<Clkctrlen>;
impl ClkctrlenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Clkctrlen {
        match self.bits {
            false => Clkctrlen::_0,
            true => Clkctrlen::_1,
        }
    }
    ///Disable automatic control of SD/MMC clock output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Clkctrlen::_0
    }
    ///Enable automatic control of SD/MMC clock output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Clkctrlen::_1
    }
}
///Field `CLKCTRLEN` writer - SD/MMC Clock Output Automatic Control Select
pub type ClkctrlenW<'a, REG> = crate::BitWriter<'a, REG, Clkctrlen>;
impl<'a, REG> ClkctrlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable automatic control of SD/MMC clock output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Clkctrlen::_0)
    }
    ///Enable automatic control of SD/MMC clock output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkctrlen::_1)
    }
}
impl R {
    ///Bits 0:7 - SDHI Clock Frequency Select
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - SD/MMC Clock Output Control
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SD/MMC Clock Output Automatic Control Select
    #[inline(always)]
    pub fn clkctrlen(&self) -> ClkctrlenR {
        ClkctrlenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_CLK_CTRL")
            .field("clksel", &self.clksel())
            .field("clken", &self.clken())
            .field("clkctrlen", &self.clkctrlen())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - SDHI Clock Frequency Select
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<SdClkCtrlSpec> {
        ClkselW::new(self, 0)
    }
    ///Bit 8 - SD/MMC Clock Output Control
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<SdClkCtrlSpec> {
        ClkenW::new(self, 8)
    }
    ///Bit 9 - SD/MMC Clock Output Automatic Control Select
    #[inline(always)]
    pub fn clkctrlen(&mut self) -> ClkctrlenW<SdClkCtrlSpec> {
        ClkctrlenW::new(self, 9)
    }
}
/**SD Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`sd_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdClkCtrlSpec;
impl crate::RegisterSpec for SdClkCtrlSpec {
    type Ux = u32;
}
///`read()` method returns [`sd_clk_ctrl::R`](R) reader structure
impl crate::Readable for SdClkCtrlSpec {}
///`write(|w| ..)` method takes [`sd_clk_ctrl::W`](W) writer structure
impl crate::Writable for SdClkCtrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_CLK_CTRL to value 0x20
impl crate::Resettable for SdClkCtrlSpec {
    const RESET_VALUE: u32 = 0x20;
}
