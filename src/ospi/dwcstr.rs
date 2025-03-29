///Register `DWCSTR` reader
pub type R = crate::R<DwcstrSpec>;
///Register `DWCSTR` writer
pub type W = crate::W<DwcstrSpec>;
///Field `CTWW0` reader - Device 0 single continuous write waiting cycle setting in PCLKA units
pub type Ctww0R = crate::FieldReader;
///Field `CTWW0` writer - Device 0 single continuous write waiting cycle setting in PCLKA units
pub type Ctww0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Device 0 single continuous write mode setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctw0 {
    ///0: Single continuous write mode is disabled for device 0
    _0 = 0,
    ///1: Single continuous write mode is enabled for device 0
    _1 = 1,
}
impl From<Ctw0> for bool {
    #[inline(always)]
    fn from(variant: Ctw0) -> Self {
        variant as u8 != 0
    }
}
///Field `CTW0` reader - Device 0 single continuous write mode setting
pub type Ctw0R = crate::BitReader<Ctw0>;
impl Ctw0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctw0 {
        match self.bits {
            false => Ctw0::_0,
            true => Ctw0::_1,
        }
    }
    ///Single continuous write mode is disabled for device 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctw0::_0
    }
    ///Single continuous write mode is enabled for device 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctw0::_1
    }
}
///Field `CTW0` writer - Device 0 single continuous write mode setting
pub type Ctw0W<'a, REG> = crate::BitWriter<'a, REG, Ctw0>;
impl<'a, REG> Ctw0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single continuous write mode is disabled for device 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctw0::_0)
    }
    ///Single continuous write mode is enabled for device 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctw0::_1)
    }
}
/**Device 0 Command execution interval setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvwcmd0 {
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
impl From<Dvwcmd0> for u8 {
    #[inline(always)]
    fn from(variant: Dvwcmd0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvwcmd0 {
    type Ux = u8;
}
impl crate::IsEnum for Dvwcmd0 {}
///Field `DVWCMD0` reader - Device 0 Command execution interval setting
pub type Dvwcmd0R = crate::FieldReader<Dvwcmd0>;
impl Dvwcmd0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvwcmd0 {
        match self.bits {
            0 => Dvwcmd0::_000,
            1 => Dvwcmd0::_001,
            2 => Dvwcmd0::_010,
            3 => Dvwcmd0::_011,
            4 => Dvwcmd0::_100,
            5 => Dvwcmd0::_101,
            6 => Dvwcmd0::_110,
            7 => Dvwcmd0::_111,
            _ => unreachable!(),
        }
    }
    ///2 clock cycles
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvwcmd0::_000
    }
    ///5 clock cycles
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvwcmd0::_001
    }
    ///7 clock cycles
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvwcmd0::_010
    }
    ///9 clock cycles
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvwcmd0::_011
    }
    ///11 clock cycles
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dvwcmd0::_100
    }
    ///13 clock cycles
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dvwcmd0::_101
    }
    ///15 clock cycles
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dvwcmd0::_110
    }
    ///17 clock cycles
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dvwcmd0::_111
    }
}
///Field `DVWCMD0` writer - Device 0 Command execution interval setting
pub type Dvwcmd0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Dvwcmd0, crate::Safe>;
impl<'a, REG> Dvwcmd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2 clock cycles
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd0::_000)
    }
    ///5 clock cycles
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd0::_001)
    }
    ///7 clock cycles
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd0::_010)
    }
    ///9 clock cycles
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd0::_011)
    }
    ///11 clock cycles
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd0::_100)
    }
    ///13 clock cycles
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd0::_101)
    }
    ///15 clock cycles
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd0::_110)
    }
    ///17 clock cycles
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd0::_111)
    }
}
/**Device 0 select signal pull-up timing setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvwhi0 {
    ///0: 1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)
    _000 = 0,
    ///1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    _001 = 1,
    ///2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    _010 = 2,
    ///3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    _011 = 3,
    ///4: 5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)
    _100 = 4,
    ///5: 6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    _101 = 5,
    ///6: 7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    _110 = 6,
    ///7: 8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    _111 = 7,
}
impl From<Dvwhi0> for u8 {
    #[inline(always)]
    fn from(variant: Dvwhi0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvwhi0 {
    type Ux = u8;
}
impl crate::IsEnum for Dvwhi0 {}
///Field `DVWHI0` reader - Device 0 select signal pull-up timing setting
pub type Dvwhi0R = crate::FieldReader<Dvwhi0>;
impl Dvwhi0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvwhi0 {
        match self.bits {
            0 => Dvwhi0::_000,
            1 => Dvwhi0::_001,
            2 => Dvwhi0::_010,
            3 => Dvwhi0::_011,
            4 => Dvwhi0::_100,
            5 => Dvwhi0::_101,
            6 => Dvwhi0::_110,
            7 => Dvwhi0::_111,
            _ => unreachable!(),
        }
    }
    ///1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvwhi0::_000
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvwhi0::_001
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvwhi0::_010
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvwhi0::_011
    }
    ///5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dvwhi0::_100
    }
    ///6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dvwhi0::_101
    }
    ///7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dvwhi0::_110
    }
    ///8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dvwhi0::_111
    }
}
///Field `DVWHI0` writer - Device 0 select signal pull-up timing setting
pub type Dvwhi0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Dvwhi0, crate::Safe>;
impl<'a, REG> Dvwhi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi0::_000)
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi0::_001)
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi0::_010)
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi0::_011)
    }
    ///5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi0::_100)
    }
    ///6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi0::_101)
    }
    ///7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi0::_110)
    }
    ///8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi0::_111)
    }
}
/**Device 0 select signal pull-down timing setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvwlo0 {
    ///0: Setting prohibit
    _00 = 0,
    ///1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    _01 = 1,
    ///2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    _10 = 2,
    ///3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    _11 = 3,
}
impl From<Dvwlo0> for u8 {
    #[inline(always)]
    fn from(variant: Dvwlo0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvwlo0 {
    type Ux = u8;
}
impl crate::IsEnum for Dvwlo0 {}
///Field `DVWLO0` reader - Device 0 select signal pull-down timing setting
pub type Dvwlo0R = crate::FieldReader<Dvwlo0>;
impl Dvwlo0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvwlo0 {
        match self.bits {
            0 => Dvwlo0::_00,
            1 => Dvwlo0::_01,
            2 => Dvwlo0::_10,
            3 => Dvwlo0::_11,
            _ => unreachable!(),
        }
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dvwlo0::_00
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dvwlo0::_01
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dvwlo0::_10
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dvwlo0::_11
    }
}
///Field `DVWLO0` writer - Device 0 select signal pull-down timing setting
pub type Dvwlo0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dvwlo0, crate::Safe>;
impl<'a, REG> Dvwlo0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibit
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwlo0::_00)
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwlo0::_01)
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwlo0::_10)
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwlo0::_11)
    }
}
///Field `CTWW1` reader - Device 1 single continuous write waiting cycle setting in PCLKA units
pub type Ctww1R = crate::FieldReader;
///Field `CTWW1` writer - Device 1 single continuous write waiting cycle setting in PCLKA units
pub type Ctww1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Device 1 single continuous write mode setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctw1 {
    ///0: Single continuous write mode is disabled for device 1
    _0 = 0,
    ///1: Single continuous write mode is enabled for device 1
    _1 = 1,
}
impl From<Ctw1> for bool {
    #[inline(always)]
    fn from(variant: Ctw1) -> Self {
        variant as u8 != 0
    }
}
///Field `CTW1` reader - Device 1 single continuous write mode setting
pub type Ctw1R = crate::BitReader<Ctw1>;
impl Ctw1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctw1 {
        match self.bits {
            false => Ctw1::_0,
            true => Ctw1::_1,
        }
    }
    ///Single continuous write mode is disabled for device 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctw1::_0
    }
    ///Single continuous write mode is enabled for device 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctw1::_1
    }
}
///Field `CTW1` writer - Device 1 single continuous write mode setting
pub type Ctw1W<'a, REG> = crate::BitWriter<'a, REG, Ctw1>;
impl<'a, REG> Ctw1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single continuous write mode is disabled for device 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctw1::_0)
    }
    ///Single continuous write mode is enabled for device 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctw1::_1)
    }
}
/**Device 1 Command execution interval setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvwcmd1 {
    ///0: setting prohibited
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
impl From<Dvwcmd1> for u8 {
    #[inline(always)]
    fn from(variant: Dvwcmd1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvwcmd1 {
    type Ux = u8;
}
impl crate::IsEnum for Dvwcmd1 {}
///Field `DVWCMD1` reader - Device 1 Command execution interval setting
pub type Dvwcmd1R = crate::FieldReader<Dvwcmd1>;
impl Dvwcmd1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvwcmd1 {
        match self.bits {
            0 => Dvwcmd1::_000,
            1 => Dvwcmd1::_001,
            2 => Dvwcmd1::_010,
            3 => Dvwcmd1::_011,
            4 => Dvwcmd1::_100,
            5 => Dvwcmd1::_101,
            6 => Dvwcmd1::_110,
            7 => Dvwcmd1::_111,
            _ => unreachable!(),
        }
    }
    ///setting prohibited
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvwcmd1::_000
    }
    ///5 clock cycles
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvwcmd1::_001
    }
    ///7 clock cycles
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvwcmd1::_010
    }
    ///9 clock cycles
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvwcmd1::_011
    }
    ///11 clock cycles
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dvwcmd1::_100
    }
    ///13 clock cycles
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dvwcmd1::_101
    }
    ///15 clock cycles
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dvwcmd1::_110
    }
    ///17 clock cycles
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dvwcmd1::_111
    }
}
///Field `DVWCMD1` writer - Device 1 Command execution interval setting
pub type Dvwcmd1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Dvwcmd1, crate::Safe>;
impl<'a, REG> Dvwcmd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///setting prohibited
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd1::_000)
    }
    ///5 clock cycles
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd1::_001)
    }
    ///7 clock cycles
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd1::_010)
    }
    ///9 clock cycles
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd1::_011)
    }
    ///11 clock cycles
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd1::_100)
    }
    ///13 clock cycles
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd1::_101)
    }
    ///15 clock cycles
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd1::_110)
    }
    ///17 clock cycles
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwcmd1::_111)
    }
}
/**Device 1 select signal pull-up timing setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvwhi1 {
    ///0: 1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)
    _000 = 0,
    ///1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    _001 = 1,
    ///2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    _010 = 2,
    ///3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    _011 = 3,
    ///4: 5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)
    _100 = 4,
    ///5: 6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    _101 = 5,
    ///6: 7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    _110 = 6,
    ///7: 8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    _111 = 7,
}
impl From<Dvwhi1> for u8 {
    #[inline(always)]
    fn from(variant: Dvwhi1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvwhi1 {
    type Ux = u8;
}
impl crate::IsEnum for Dvwhi1 {}
///Field `DVWHI1` reader - Device 1 select signal pull-up timing setting
pub type Dvwhi1R = crate::FieldReader<Dvwhi1>;
impl Dvwhi1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvwhi1 {
        match self.bits {
            0 => Dvwhi1::_000,
            1 => Dvwhi1::_001,
            2 => Dvwhi1::_010,
            3 => Dvwhi1::_011,
            4 => Dvwhi1::_100,
            5 => Dvwhi1::_101,
            6 => Dvwhi1::_110,
            7 => Dvwhi1::_111,
            _ => unreachable!(),
        }
    }
    ///1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvwhi1::_000
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvwhi1::_001
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvwhi1::_010
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvwhi1::_011
    }
    ///5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dvwhi1::_100
    }
    ///6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dvwhi1::_101
    }
    ///7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dvwhi1::_110
    }
    ///8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dvwhi1::_111
    }
}
///Field `DVWHI1` writer - Device 1 select signal pull-up timing setting
pub type Dvwhi1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Dvwhi1, crate::Safe>;
impl<'a, REG> Dvwhi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi1::_000)
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi1::_001)
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi1::_010)
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi1::_011)
    }
    ///5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi1::_100)
    }
    ///6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi1::_101)
    }
    ///7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi1::_110)
    }
    ///8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwhi1::_111)
    }
}
/**Device 1 select signal pull-down timing setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvwlo1 {
    ///0: Setting prohibit
    _00 = 0,
    ///1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    _01 = 1,
    ///2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    _10 = 2,
    ///3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    _11 = 3,
}
impl From<Dvwlo1> for u8 {
    #[inline(always)]
    fn from(variant: Dvwlo1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvwlo1 {
    type Ux = u8;
}
impl crate::IsEnum for Dvwlo1 {}
///Field `DVWLO1` reader - Device 1 select signal pull-down timing setting
pub type Dvwlo1R = crate::FieldReader<Dvwlo1>;
impl Dvwlo1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvwlo1 {
        match self.bits {
            0 => Dvwlo1::_00,
            1 => Dvwlo1::_01,
            2 => Dvwlo1::_10,
            3 => Dvwlo1::_11,
            _ => unreachable!(),
        }
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dvwlo1::_00
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dvwlo1::_01
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dvwlo1::_10
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dvwlo1::_11
    }
}
///Field `DVWLO1` writer - Device 1 select signal pull-down timing setting
pub type Dvwlo1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dvwlo1, crate::Safe>;
impl<'a, REG> Dvwlo1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibit
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwlo1::_00)
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwlo1::_01)
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwlo1::_10)
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dvwlo1::_11)
    }
}
impl R {
    ///Bits 0:6 - Device 0 single continuous write waiting cycle setting in PCLKA units
    #[inline(always)]
    pub fn ctww0(&self) -> Ctww0R {
        Ctww0R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Device 0 single continuous write mode setting
    #[inline(always)]
    pub fn ctw0(&self) -> Ctw0R {
        Ctw0R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Device 0 Command execution interval setting
    #[inline(always)]
    pub fn dvwcmd0(&self) -> Dvwcmd0R {
        Dvwcmd0R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - Device 0 select signal pull-up timing setting
    #[inline(always)]
    pub fn dvwhi0(&self) -> Dvwhi0R {
        Dvwhi0R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 14:15 - Device 0 select signal pull-down timing setting
    #[inline(always)]
    pub fn dvwlo0(&self) -> Dvwlo0R {
        Dvwlo0R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:22 - Device 1 single continuous write waiting cycle setting in PCLKA units
    #[inline(always)]
    pub fn ctww1(&self) -> Ctww1R {
        Ctww1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 23 - Device 1 single continuous write mode setting
    #[inline(always)]
    pub fn ctw1(&self) -> Ctw1R {
        Ctw1R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - Device 1 Command execution interval setting
    #[inline(always)]
    pub fn dvwcmd1(&self) -> Dvwcmd1R {
        Dvwcmd1R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - Device 1 select signal pull-up timing setting
    #[inline(always)]
    pub fn dvwhi1(&self) -> Dvwhi1R {
        Dvwhi1R::new(((self.bits >> 27) & 7) as u8)
    }
    ///Bits 30:31 - Device 1 select signal pull-down timing setting
    #[inline(always)]
    pub fn dvwlo1(&self) -> Dvwlo1R {
        Dvwlo1R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DWCSTR")
            .field("ctww0", &self.ctww0())
            .field("ctw0", &self.ctw0())
            .field("dvwcmd0", &self.dvwcmd0())
            .field("dvwhi0", &self.dvwhi0())
            .field("dvwlo0", &self.dvwlo0())
            .field("ctww1", &self.ctww1())
            .field("ctw1", &self.ctw1())
            .field("dvwcmd1", &self.dvwcmd1())
            .field("dvwhi1", &self.dvwhi1())
            .field("dvwlo1", &self.dvwlo1())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Device 0 single continuous write waiting cycle setting in PCLKA units
    #[inline(always)]
    pub fn ctww0(&mut self) -> Ctww0W<DwcstrSpec> {
        Ctww0W::new(self, 0)
    }
    ///Bit 7 - Device 0 single continuous write mode setting
    #[inline(always)]
    pub fn ctw0(&mut self) -> Ctw0W<DwcstrSpec> {
        Ctw0W::new(self, 7)
    }
    ///Bits 8:10 - Device 0 Command execution interval setting
    #[inline(always)]
    pub fn dvwcmd0(&mut self) -> Dvwcmd0W<DwcstrSpec> {
        Dvwcmd0W::new(self, 8)
    }
    ///Bits 11:13 - Device 0 select signal pull-up timing setting
    #[inline(always)]
    pub fn dvwhi0(&mut self) -> Dvwhi0W<DwcstrSpec> {
        Dvwhi0W::new(self, 11)
    }
    ///Bits 14:15 - Device 0 select signal pull-down timing setting
    #[inline(always)]
    pub fn dvwlo0(&mut self) -> Dvwlo0W<DwcstrSpec> {
        Dvwlo0W::new(self, 14)
    }
    ///Bits 16:22 - Device 1 single continuous write waiting cycle setting in PCLKA units
    #[inline(always)]
    pub fn ctww1(&mut self) -> Ctww1W<DwcstrSpec> {
        Ctww1W::new(self, 16)
    }
    ///Bit 23 - Device 1 single continuous write mode setting
    #[inline(always)]
    pub fn ctw1(&mut self) -> Ctw1W<DwcstrSpec> {
        Ctw1W::new(self, 23)
    }
    ///Bits 24:26 - Device 1 Command execution interval setting
    #[inline(always)]
    pub fn dvwcmd1(&mut self) -> Dvwcmd1W<DwcstrSpec> {
        Dvwcmd1W::new(self, 24)
    }
    ///Bits 27:29 - Device 1 select signal pull-up timing setting
    #[inline(always)]
    pub fn dvwhi1(&mut self) -> Dvwhi1W<DwcstrSpec> {
        Dvwhi1W::new(self, 27)
    }
    ///Bits 30:31 - Device 1 select signal pull-down timing setting
    #[inline(always)]
    pub fn dvwlo1(&mut self) -> Dvwlo1W<DwcstrSpec> {
        Dvwlo1W::new(self, 30)
    }
}
/**Device Memory Map Write Chip Select Timing Setting Register

You can [`read`](crate::Reg::read) this register and get [`dwcstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwcstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DwcstrSpec;
impl crate::RegisterSpec for DwcstrSpec {
    type Ux = u32;
}
///`read()` method returns [`dwcstr::R`](R) reader structure
impl crate::Readable for DwcstrSpec {}
///`write(|w| ..)` method takes [`dwcstr::W`](W) writer structure
impl crate::Writable for DwcstrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DWCSTR to value 0
impl crate::Resettable for DwcstrSpec {}
