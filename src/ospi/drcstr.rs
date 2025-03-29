///Register `DRCSTR` reader
pub type R = crate::R<DrcstrSpec>;
///Register `DRCSTR` writer
pub type W = crate::W<DrcstrSpec>;
///Field `CTRW0` reader - Device 0 single continuous read waiting cycle setting in PCLKA units
pub type Ctrw0R = crate::FieldReader;
///Field `CTRW0` writer - Device 0 single continuous read waiting cycle setting in PCLKA units
pub type Ctrw0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Device 0 single continuous read mode setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctr0 {
    ///0: Single continuous read mode is disabled for device 0.
    _0 = 0,
    ///1: Single continuous read mode is enabled for device 0.
    _1 = 1,
}
impl From<Ctr0> for bool {
    #[inline(always)]
    fn from(variant: Ctr0) -> Self {
        variant as u8 != 0
    }
}
///Field `CTR0` reader - Device 0 single continuous read mode setting
pub type Ctr0R = crate::BitReader<Ctr0>;
impl Ctr0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctr0 {
        match self.bits {
            false => Ctr0::_0,
            true => Ctr0::_1,
        }
    }
    ///Single continuous read mode is disabled for device 0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctr0::_0
    }
    ///Single continuous read mode is enabled for device 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctr0::_1
    }
}
///Field `CTR0` writer - Device 0 single continuous read mode setting
pub type Ctr0W<'a, REG> = crate::BitWriter<'a, REG, Ctr0>;
impl<'a, REG> Ctr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single continuous read mode is disabled for device 0.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctr0::_0)
    }
    ///Single continuous read mode is enabled for device 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctr0::_1)
    }
}
/**Device 0 Command execution interval setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvrdcmd0 {
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
impl From<Dvrdcmd0> for u8 {
    #[inline(always)]
    fn from(variant: Dvrdcmd0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvrdcmd0 {
    type Ux = u8;
}
impl crate::IsEnum for Dvrdcmd0 {}
///Field `DVRDCMD0` reader - Device 0 Command execution interval setting
pub type Dvrdcmd0R = crate::FieldReader<Dvrdcmd0>;
impl Dvrdcmd0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvrdcmd0 {
        match self.bits {
            0 => Dvrdcmd0::_000,
            1 => Dvrdcmd0::_001,
            2 => Dvrdcmd0::_010,
            3 => Dvrdcmd0::_011,
            4 => Dvrdcmd0::_100,
            5 => Dvrdcmd0::_101,
            6 => Dvrdcmd0::_110,
            7 => Dvrdcmd0::_111,
            _ => unreachable!(),
        }
    }
    ///2 clock cycles
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvrdcmd0::_000
    }
    ///5 clock cycles
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvrdcmd0::_001
    }
    ///7 clock cycles
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvrdcmd0::_010
    }
    ///9 clock cycles
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvrdcmd0::_011
    }
    ///11 clock cycles
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dvrdcmd0::_100
    }
    ///13 clock cycles
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dvrdcmd0::_101
    }
    ///15 clock cycles
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dvrdcmd0::_110
    }
    ///17 clock cycles
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dvrdcmd0::_111
    }
}
///Field `DVRDCMD0` writer - Device 0 Command execution interval setting
pub type Dvrdcmd0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Dvrdcmd0, crate::Safe>;
impl<'a, REG> Dvrdcmd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2 clock cycles
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd0::_000)
    }
    ///5 clock cycles
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd0::_001)
    }
    ///7 clock cycles
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd0::_010)
    }
    ///9 clock cycles
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd0::_011)
    }
    ///11 clock cycles
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd0::_100)
    }
    ///13 clock cycles
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd0::_101)
    }
    ///15 clock cycles
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd0::_110)
    }
    ///17 clock cycles
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd0::_111)
    }
}
/**Device 0 select signal pull-up timing setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvrdhi0 {
    ///0: Setting prohibit
    _000 = 0,
    ///1: Setting prohibit
    _001 = 1,
    ///2: Setting prohibit
    _010 = 2,
    ///3: Setting prohibit (DOPI mode) 5 clock cycles (Other mode)
    _011 = 3,
    ///4: Setting prohibit (DOPI mode) 6 clock cycles (Other mode)
    _100 = 4,
    ///5: 6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    _101 = 5,
    ///6: 7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    _110 = 6,
    ///7: 8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    _111 = 7,
}
impl From<Dvrdhi0> for u8 {
    #[inline(always)]
    fn from(variant: Dvrdhi0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvrdhi0 {
    type Ux = u8;
}
impl crate::IsEnum for Dvrdhi0 {}
///Field `DVRDHI0` reader - Device 0 select signal pull-up timing setting
pub type Dvrdhi0R = crate::FieldReader<Dvrdhi0>;
impl Dvrdhi0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvrdhi0 {
        match self.bits {
            0 => Dvrdhi0::_000,
            1 => Dvrdhi0::_001,
            2 => Dvrdhi0::_010,
            3 => Dvrdhi0::_011,
            4 => Dvrdhi0::_100,
            5 => Dvrdhi0::_101,
            6 => Dvrdhi0::_110,
            7 => Dvrdhi0::_111,
            _ => unreachable!(),
        }
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvrdhi0::_000
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvrdhi0::_001
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvrdhi0::_010
    }
    ///Setting prohibit (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvrdhi0::_011
    }
    ///Setting prohibit (DOPI mode) 6 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dvrdhi0::_100
    }
    ///6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dvrdhi0::_101
    }
    ///7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dvrdhi0::_110
    }
    ///8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dvrdhi0::_111
    }
}
///Field `DVRDHI0` writer - Device 0 select signal pull-up timing setting
pub type Dvrdhi0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Dvrdhi0, crate::Safe>;
impl<'a, REG> Dvrdhi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibit
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi0::_000)
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi0::_001)
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi0::_010)
    }
    ///Setting prohibit (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi0::_011)
    }
    ///Setting prohibit (DOPI mode) 6 clock cycles (Other mode)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi0::_100)
    }
    ///6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi0::_101)
    }
    ///7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi0::_110)
    }
    ///8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi0::_111)
    }
}
/**Device 0 select signal pull-down timing setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvrdlo0 {
    ///0: Setting prohibit
    _00 = 0,
    ///1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    _01 = 1,
    ///2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    _10 = 2,
    ///3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    _11 = 3,
}
impl From<Dvrdlo0> for u8 {
    #[inline(always)]
    fn from(variant: Dvrdlo0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvrdlo0 {
    type Ux = u8;
}
impl crate::IsEnum for Dvrdlo0 {}
///Field `DVRDLO0` reader - Device 0 select signal pull-down timing setting
pub type Dvrdlo0R = crate::FieldReader<Dvrdlo0>;
impl Dvrdlo0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvrdlo0 {
        match self.bits {
            0 => Dvrdlo0::_00,
            1 => Dvrdlo0::_01,
            2 => Dvrdlo0::_10,
            3 => Dvrdlo0::_11,
            _ => unreachable!(),
        }
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dvrdlo0::_00
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dvrdlo0::_01
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dvrdlo0::_10
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dvrdlo0::_11
    }
}
///Field `DVRDLO0` writer - Device 0 select signal pull-down timing setting
pub type Dvrdlo0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dvrdlo0, crate::Safe>;
impl<'a, REG> Dvrdlo0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibit
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdlo0::_00)
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdlo0::_01)
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdlo0::_10)
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdlo0::_11)
    }
}
///Field `CTRW1` reader - Device 1 single continuous read waiting cycle setting in PCLKA units
pub type Ctrw1R = crate::FieldReader;
///Field `CTRW1` writer - Device 1 single continuous read waiting cycle setting in PCLKA units
pub type Ctrw1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Device 1 single continuous read mode setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctr1 {
    ///0: Single continuous read mode is disabled for device 1.
    _0 = 0,
    ///1: Single continuous read mode is enabled for device 1.
    _1 = 1,
}
impl From<Ctr1> for bool {
    #[inline(always)]
    fn from(variant: Ctr1) -> Self {
        variant as u8 != 0
    }
}
///Field `CTR1` reader - Device 1 single continuous read mode setting
pub type Ctr1R = crate::BitReader<Ctr1>;
impl Ctr1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctr1 {
        match self.bits {
            false => Ctr1::_0,
            true => Ctr1::_1,
        }
    }
    ///Single continuous read mode is disabled for device 1.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctr1::_0
    }
    ///Single continuous read mode is enabled for device 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctr1::_1
    }
}
///Field `CTR1` writer - Device 1 single continuous read mode setting
pub type Ctr1W<'a, REG> = crate::BitWriter<'a, REG, Ctr1>;
impl<'a, REG> Ctr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single continuous read mode is disabled for device 1.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctr1::_0)
    }
    ///Single continuous read mode is enabled for device 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctr1::_1)
    }
}
/**Device 1 Command execution interval

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvrdcmd1 {
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
impl From<Dvrdcmd1> for u8 {
    #[inline(always)]
    fn from(variant: Dvrdcmd1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvrdcmd1 {
    type Ux = u8;
}
impl crate::IsEnum for Dvrdcmd1 {}
///Field `DVRDCMD1` reader - Device 1 Command execution interval
pub type Dvrdcmd1R = crate::FieldReader<Dvrdcmd1>;
impl Dvrdcmd1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvrdcmd1 {
        match self.bits {
            0 => Dvrdcmd1::_000,
            1 => Dvrdcmd1::_001,
            2 => Dvrdcmd1::_010,
            3 => Dvrdcmd1::_011,
            4 => Dvrdcmd1::_100,
            5 => Dvrdcmd1::_101,
            6 => Dvrdcmd1::_110,
            7 => Dvrdcmd1::_111,
            _ => unreachable!(),
        }
    }
    ///2 clock cycles
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvrdcmd1::_000
    }
    ///5 clock cycles
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvrdcmd1::_001
    }
    ///7 clock cycles
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvrdcmd1::_010
    }
    ///9 clock cycles
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvrdcmd1::_011
    }
    ///11 clock cycles
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dvrdcmd1::_100
    }
    ///13 clock cycles
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dvrdcmd1::_101
    }
    ///15 clock cycles
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dvrdcmd1::_110
    }
    ///17 clock cycles
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dvrdcmd1::_111
    }
}
///Field `DVRDCMD1` writer - Device 1 Command execution interval
pub type Dvrdcmd1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Dvrdcmd1, crate::Safe>;
impl<'a, REG> Dvrdcmd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2 clock cycles
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd1::_000)
    }
    ///5 clock cycles
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd1::_001)
    }
    ///7 clock cycles
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd1::_010)
    }
    ///9 clock cycles
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd1::_011)
    }
    ///11 clock cycles
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd1::_100)
    }
    ///13 clock cycles
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd1::_101)
    }
    ///15 clock cycles
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd1::_110)
    }
    ///17 clock cycles
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdcmd1::_111)
    }
}
/**Device 1 select signal High timing setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvrdhi1 {
    ///0: Setting prohibit
    _000 = 0,
    ///1: Setting prohibit
    _001 = 1,
    ///2: Setting prohibit
    _010 = 2,
    ///3: Setting prohibit (DOPI mode) 5 clock cycles (Other mode)
    _011 = 3,
    ///4: Setting prohibit (DOPI mode) 6 clock cycles (Other mode)
    _100 = 4,
    ///5: 6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    _101 = 5,
    ///6: 7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    _110 = 6,
    ///7: 8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    _111 = 7,
}
impl From<Dvrdhi1> for u8 {
    #[inline(always)]
    fn from(variant: Dvrdhi1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvrdhi1 {
    type Ux = u8;
}
impl crate::IsEnum for Dvrdhi1 {}
///Field `DVRDHI1` reader - Device 1 select signal High timing setting
pub type Dvrdhi1R = crate::FieldReader<Dvrdhi1>;
impl Dvrdhi1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvrdhi1 {
        match self.bits {
            0 => Dvrdhi1::_000,
            1 => Dvrdhi1::_001,
            2 => Dvrdhi1::_010,
            3 => Dvrdhi1::_011,
            4 => Dvrdhi1::_100,
            5 => Dvrdhi1::_101,
            6 => Dvrdhi1::_110,
            7 => Dvrdhi1::_111,
            _ => unreachable!(),
        }
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvrdhi1::_000
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvrdhi1::_001
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvrdhi1::_010
    }
    ///Setting prohibit (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvrdhi1::_011
    }
    ///Setting prohibit (DOPI mode) 6 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dvrdhi1::_100
    }
    ///6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dvrdhi1::_101
    }
    ///7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dvrdhi1::_110
    }
    ///8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dvrdhi1::_111
    }
}
///Field `DVRDHI1` writer - Device 1 select signal High timing setting
pub type Dvrdhi1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Dvrdhi1, crate::Safe>;
impl<'a, REG> Dvrdhi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibit
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi1::_000)
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi1::_001)
    }
    ///Setting prohibit
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi1::_010)
    }
    ///Setting prohibit (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi1::_011)
    }
    ///Setting prohibit (DOPI mode) 6 clock cycles (Other mode)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi1::_100)
    }
    ///6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi1::_101)
    }
    ///7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi1::_110)
    }
    ///8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdhi1::_111)
    }
}
/**Device 1 select signal pull-down timing setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvrdlo1 {
    ///0: Setting prohibited
    _00 = 0,
    ///1: 2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    _01 = 1,
    ///2: 3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    _10 = 2,
    ///3: 4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    _11 = 3,
}
impl From<Dvrdlo1> for u8 {
    #[inline(always)]
    fn from(variant: Dvrdlo1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvrdlo1 {
    type Ux = u8;
}
impl crate::IsEnum for Dvrdlo1 {}
///Field `DVRDLO1` reader - Device 1 select signal pull-down timing setting
pub type Dvrdlo1R = crate::FieldReader<Dvrdlo1>;
impl Dvrdlo1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvrdlo1 {
        match self.bits {
            0 => Dvrdlo1::_00,
            1 => Dvrdlo1::_01,
            2 => Dvrdlo1::_10,
            3 => Dvrdlo1::_11,
            _ => unreachable!(),
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dvrdlo1::_00
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dvrdlo1::_01
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dvrdlo1::_10
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dvrdlo1::_11
    }
}
///Field `DVRDLO1` writer - Device 1 select signal pull-down timing setting
pub type Dvrdlo1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dvrdlo1, crate::Safe>;
impl<'a, REG> Dvrdlo1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdlo1::_00)
    }
    ///2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdlo1::_01)
    }
    ///3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdlo1::_10)
    }
    ///4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dvrdlo1::_11)
    }
}
impl R {
    ///Bits 0:6 - Device 0 single continuous read waiting cycle setting in PCLKA units
    #[inline(always)]
    pub fn ctrw0(&self) -> Ctrw0R {
        Ctrw0R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Device 0 single continuous read mode setting
    #[inline(always)]
    pub fn ctr0(&self) -> Ctr0R {
        Ctr0R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Device 0 Command execution interval setting
    #[inline(always)]
    pub fn dvrdcmd0(&self) -> Dvrdcmd0R {
        Dvrdcmd0R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - Device 0 select signal pull-up timing setting
    #[inline(always)]
    pub fn dvrdhi0(&self) -> Dvrdhi0R {
        Dvrdhi0R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 14:15 - Device 0 select signal pull-down timing setting
    #[inline(always)]
    pub fn dvrdlo0(&self) -> Dvrdlo0R {
        Dvrdlo0R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:22 - Device 1 single continuous read waiting cycle setting in PCLKA units
    #[inline(always)]
    pub fn ctrw1(&self) -> Ctrw1R {
        Ctrw1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 23 - Device 1 single continuous read mode setting
    #[inline(always)]
    pub fn ctr1(&self) -> Ctr1R {
        Ctr1R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - Device 1 Command execution interval
    #[inline(always)]
    pub fn dvrdcmd1(&self) -> Dvrdcmd1R {
        Dvrdcmd1R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - Device 1 select signal High timing setting
    #[inline(always)]
    pub fn dvrdhi1(&self) -> Dvrdhi1R {
        Dvrdhi1R::new(((self.bits >> 27) & 7) as u8)
    }
    ///Bits 30:31 - Device 1 select signal pull-down timing setting
    #[inline(always)]
    pub fn dvrdlo1(&self) -> Dvrdlo1R {
        Dvrdlo1R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRCSTR")
            .field("ctrw0", &self.ctrw0())
            .field("ctr0", &self.ctr0())
            .field("dvrdcmd0", &self.dvrdcmd0())
            .field("dvrdhi0", &self.dvrdhi0())
            .field("dvrdlo0", &self.dvrdlo0())
            .field("ctrw1", &self.ctrw1())
            .field("ctr1", &self.ctr1())
            .field("dvrdcmd1", &self.dvrdcmd1())
            .field("dvrdhi1", &self.dvrdhi1())
            .field("dvrdlo1", &self.dvrdlo1())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Device 0 single continuous read waiting cycle setting in PCLKA units
    #[inline(always)]
    pub fn ctrw0(&mut self) -> Ctrw0W<DrcstrSpec> {
        Ctrw0W::new(self, 0)
    }
    ///Bit 7 - Device 0 single continuous read mode setting
    #[inline(always)]
    pub fn ctr0(&mut self) -> Ctr0W<DrcstrSpec> {
        Ctr0W::new(self, 7)
    }
    ///Bits 8:10 - Device 0 Command execution interval setting
    #[inline(always)]
    pub fn dvrdcmd0(&mut self) -> Dvrdcmd0W<DrcstrSpec> {
        Dvrdcmd0W::new(self, 8)
    }
    ///Bits 11:13 - Device 0 select signal pull-up timing setting
    #[inline(always)]
    pub fn dvrdhi0(&mut self) -> Dvrdhi0W<DrcstrSpec> {
        Dvrdhi0W::new(self, 11)
    }
    ///Bits 14:15 - Device 0 select signal pull-down timing setting
    #[inline(always)]
    pub fn dvrdlo0(&mut self) -> Dvrdlo0W<DrcstrSpec> {
        Dvrdlo0W::new(self, 14)
    }
    ///Bits 16:22 - Device 1 single continuous read waiting cycle setting in PCLKA units
    #[inline(always)]
    pub fn ctrw1(&mut self) -> Ctrw1W<DrcstrSpec> {
        Ctrw1W::new(self, 16)
    }
    ///Bit 23 - Device 1 single continuous read mode setting
    #[inline(always)]
    pub fn ctr1(&mut self) -> Ctr1W<DrcstrSpec> {
        Ctr1W::new(self, 23)
    }
    ///Bits 24:26 - Device 1 Command execution interval
    #[inline(always)]
    pub fn dvrdcmd1(&mut self) -> Dvrdcmd1W<DrcstrSpec> {
        Dvrdcmd1W::new(self, 24)
    }
    ///Bits 27:29 - Device 1 select signal High timing setting
    #[inline(always)]
    pub fn dvrdhi1(&mut self) -> Dvrdhi1W<DrcstrSpec> {
        Dvrdhi1W::new(self, 27)
    }
    ///Bits 30:31 - Device 1 select signal pull-down timing setting
    #[inline(always)]
    pub fn dvrdlo1(&mut self) -> Dvrdlo1W<DrcstrSpec> {
        Dvrdlo1W::new(self, 30)
    }
}
/**Device Memory Map Read Chip Select Timing Setting Register

You can [`read`](crate::Reg::read) this register and get [`drcstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drcstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DrcstrSpec;
impl crate::RegisterSpec for DrcstrSpec {
    type Ux = u32;
}
///`read()` method returns [`drcstr::R`](R) reader structure
impl crate::Readable for DrcstrSpec {}
///`write(|w| ..)` method takes [`drcstr::W`](W) writer structure
impl crate::Writable for DrcstrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DRCSTR to value 0
impl crate::Resettable for DrcstrSpec {}
