///Register `DCSTR` reader
pub type R = crate::R<DcstrSpec>;
///Register `DCSTR` writer
pub type W = crate::W<DcstrSpec>;
/**Device Command execution interval setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvselcmd {
    ///0: 2 clock cycles
    _000 = 0,
    ///1: 5 clock cycles
    _001 = 1,
    ///2: 7 clock cycles
    _010 = 2,
    ///3: 9 clock cycles
    _011 = 3,
    ///4: 11 clock cycles
    _100 = 4,
    ///5: 13 clock cycles
    _101 = 5,
    ///6: 15 clock cycles
    _110 = 6,
    ///7: 17 clock cycles
    _111 = 7,
}
impl From<Dvselcmd> for u8 {
    #[inline(always)]
    fn from(variant: Dvselcmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvselcmd {
    type Ux = u8;
}
impl crate::IsEnum for Dvselcmd {}
///Field `DVSELCMD` reader - Device Command execution interval setting
pub type DvselcmdR = crate::FieldReader<Dvselcmd>;
impl DvselcmdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvselcmd {
        match self.bits {
            0 => Dvselcmd::_000,
            1 => Dvselcmd::_001,
            2 => Dvselcmd::_010,
            3 => Dvselcmd::_011,
            4 => Dvselcmd::_100,
            5 => Dvselcmd::_101,
            6 => Dvselcmd::_110,
            7 => Dvselcmd::_111,
            _ => unreachable!(),
        }
    }
    ///2 clock cycles
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvselcmd::_000
    }
    ///5 clock cycles
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvselcmd::_001
    }
    ///7 clock cycles
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvselcmd::_010
    }
    ///9 clock cycles
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvselcmd::_011
    }
    ///11 clock cycles
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dvselcmd::_100
    }
    ///13 clock cycles
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dvselcmd::_101
    }
    ///15 clock cycles
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dvselcmd::_110
    }
    ///17 clock cycles
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dvselcmd::_111
    }
}
///Field `DVSELCMD` writer - Device Command execution interval setting
pub type DvselcmdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dvselcmd, crate::Safe>;
impl<'a, REG> DvselcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2 clock cycles
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselcmd::_000)
    }
    ///5 clock cycles
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselcmd::_001)
    }
    ///7 clock cycles
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselcmd::_010)
    }
    ///9 clock cycles
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselcmd::_011)
    }
    ///11 clock cycles
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselcmd::_100)
    }
    ///13 clock cycles
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselcmd::_101)
    }
    ///15 clock cycles
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselcmd::_110)
    }
    ///17 clock cycles
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselcmd::_111)
    }
}
/**Device select signal pull-up timing setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvselhi {
    ///0: Setting prohibited
    _000 = 0,
    ///1: Setting prohibited
    _001 = 1,
    ///2: Setting prohibited
    _010 = 2,
    ///3: Setting prohibited (DOPI mode) 5 clock cycles (Other mode)
    _011 = 3,
    ///4: Setting prohibited (DOPI mode) 6 clock cycles (Other mode)
    _100 = 4,
    ///5: 6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    _101 = 5,
    ///6: 7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    _110 = 6,
    ///7: 8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    _111 = 7,
}
impl From<Dvselhi> for u8 {
    #[inline(always)]
    fn from(variant: Dvselhi) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvselhi {
    type Ux = u8;
}
impl crate::IsEnum for Dvselhi {}
///Field `DVSELHI` reader - Device select signal pull-up timing setting
pub type DvselhiR = crate::FieldReader<Dvselhi>;
impl DvselhiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvselhi {
        match self.bits {
            0 => Dvselhi::_000,
            1 => Dvselhi::_001,
            2 => Dvselhi::_010,
            3 => Dvselhi::_011,
            4 => Dvselhi::_100,
            5 => Dvselhi::_101,
            6 => Dvselhi::_110,
            7 => Dvselhi::_111,
            _ => unreachable!(),
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvselhi::_000
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvselhi::_001
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvselhi::_010
    }
    ///Setting prohibited (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvselhi::_011
    }
    ///Setting prohibited (DOPI mode) 6 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dvselhi::_100
    }
    ///6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dvselhi::_101
    }
    ///7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dvselhi::_110
    }
    ///8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dvselhi::_111
    }
}
///Field `DVSELHI` writer - Device select signal pull-up timing setting
pub type DvselhiW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dvselhi, crate::Safe>;
impl<'a, REG> DvselhiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselhi::_000)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselhi::_001)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselhi::_010)
    }
    ///Setting prohibited (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselhi::_011)
    }
    ///Setting prohibited (DOPI mode) 6 clock cycles (Other mode)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselhi::_100)
    }
    ///6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselhi::_101)
    }
    ///7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselhi::_110)
    }
    ///8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dvselhi::_111)
    }
}
/**Device select signal pull-down timing setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvsello {
    ///0: Setting prohibit
    _00 = 0,
    ///1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    _01 = 1,
    ///2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    _10 = 2,
    ///3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    _11 = 3,
}
impl From<Dvsello> for u8 {
    #[inline(always)]
    fn from(variant: Dvsello) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvsello {
    type Ux = u8;
}
impl crate::IsEnum for Dvsello {}
///Field `DVSELLO` reader - Device select signal pull-down timing setting
pub type DvselloR = crate::FieldReader<Dvsello>;
impl DvselloR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvsello {
        match self.bits {
            0 => Dvsello::_00,
            1 => Dvsello::_01,
            2 => Dvsello::_10,
            3 => Dvsello::_11,
            _ => unreachable!(),
        }
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dvsello::_00
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dvsello::_01
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dvsello::_10
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dvsello::_11
    }
}
///Field `DVSELLO` writer - Device select signal pull-down timing setting
pub type DvselloW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dvsello, crate::Safe>;
impl<'a, REG> DvselloW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibit
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dvsello::_00)
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dvsello::_01)
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dvsello::_10)
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dvsello::_11)
    }
}
impl R {
    ///Bits 8:10 - Device Command execution interval setting
    #[inline(always)]
    pub fn dvselcmd(&self) -> DvselcmdR {
        DvselcmdR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - Device select signal pull-up timing setting
    #[inline(always)]
    pub fn dvselhi(&self) -> DvselhiR {
        DvselhiR::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 14:15 - Device select signal pull-down timing setting
    #[inline(always)]
    pub fn dvsello(&self) -> DvselloR {
        DvselloR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCSTR")
            .field("dvselcmd", &self.dvselcmd())
            .field("dvselhi", &self.dvselhi())
            .field("dvsello", &self.dvsello())
            .finish()
    }
}
impl W {
    ///Bits 8:10 - Device Command execution interval setting
    #[inline(always)]
    pub fn dvselcmd(&mut self) -> DvselcmdW<DcstrSpec> {
        DvselcmdW::new(self, 8)
    }
    ///Bits 11:13 - Device select signal pull-up timing setting
    #[inline(always)]
    pub fn dvselhi(&mut self) -> DvselhiW<DcstrSpec> {
        DvselhiW::new(self, 11)
    }
    ///Bits 14:15 - Device select signal pull-down timing setting
    #[inline(always)]
    pub fn dvsello(&mut self) -> DvselloW<DcstrSpec> {
        DvselloW::new(self, 14)
    }
}
/**Device Chip Select Timing Setting Register

You can [`read`](crate::Reg::read) this register and get [`dcstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcstrSpec;
impl crate::RegisterSpec for DcstrSpec {
    type Ux = u32;
}
///`read()` method returns [`dcstr::R`](R) reader structure
impl crate::Readable for DcstrSpec {}
///`write(|w| ..)` method takes [`dcstr::W`](W) writer structure
impl crate::Writable for DcstrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCSTR to value 0
impl crate::Resettable for DcstrSpec {}
