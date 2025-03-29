///Register `GTICLF` reader
pub type R = crate::R<GticlfSpec>;
///Register `GTICLF` writer
pub type W = crate::W<GticlfSpec>;
/**GTIOCnA Output Logical Operation Function Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iclfa {
    ///0: A (no delay)
    _000 = 0,
    ///1: NOT A (no delay)
    _001 = 1,
    ///2: C (1PCLKD delay)
    _010 = 2,
    ///3: NOT C (1PCLKD delay)
    _011 = 3,
    ///4: A AND C (1PCLKD delay)
    _100 = 4,
    ///5: A OR C (1PCLKD delay)
    _101 = 5,
    ///6: A EXOR C (1PCLKD delay)
    _110 = 6,
    ///7: A NOR C (1PCLKD delay)
    _111 = 7,
}
impl From<Iclfa> for u8 {
    #[inline(always)]
    fn from(variant: Iclfa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iclfa {
    type Ux = u8;
}
impl crate::IsEnum for Iclfa {}
///Field `ICLFA` reader - GTIOCnA Output Logical Operation Function Select
pub type IclfaR = crate::FieldReader<Iclfa>;
impl IclfaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iclfa {
        match self.bits {
            0 => Iclfa::_000,
            1 => Iclfa::_001,
            2 => Iclfa::_010,
            3 => Iclfa::_011,
            4 => Iclfa::_100,
            5 => Iclfa::_101,
            6 => Iclfa::_110,
            7 => Iclfa::_111,
            _ => unreachable!(),
        }
    }
    ///A (no delay)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Iclfa::_000
    }
    ///NOT A (no delay)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Iclfa::_001
    }
    ///C (1PCLKD delay)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Iclfa::_010
    }
    ///NOT C (1PCLKD delay)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Iclfa::_011
    }
    ///A AND C (1PCLKD delay)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Iclfa::_100
    }
    ///A OR C (1PCLKD delay)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Iclfa::_101
    }
    ///A EXOR C (1PCLKD delay)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Iclfa::_110
    }
    ///A NOR C (1PCLKD delay)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Iclfa::_111
    }
}
///Field `ICLFA` writer - GTIOCnA Output Logical Operation Function Select
pub type IclfaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Iclfa, crate::Safe>;
impl<'a, REG> IclfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///A (no delay)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfa::_000)
    }
    ///NOT A (no delay)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfa::_001)
    }
    ///C (1PCLKD delay)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfa::_010)
    }
    ///NOT C (1PCLKD delay)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfa::_011)
    }
    ///A AND C (1PCLKD delay)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfa::_100)
    }
    ///A OR C (1PCLKD delay)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfa::_101)
    }
    ///A EXOR C (1PCLKD delay)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfa::_110)
    }
    ///A NOR C (1PCLKD delay)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfa::_111)
    }
}
/**Inter Channel Signal C Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iclfselc {
    ///0: GTIOC0A
    _0x00 = 0,
    ///1: GTIOC0B
    _0x01 = 1,
    ///2: GTIOC1A
    _0x02 = 2,
    ///3: GTIOC1B
    _0x03 = 3,
    ///4: GTIOC2A
    _0x04 = 4,
    ///5: GTIOC2B
    _0x05 = 5,
    ///6: GTIOC3A
    _0x06 = 6,
    ///7: GTIOC3B
    _0x07 = 7,
    ///8: GTIOC4A
    _0x08 = 8,
    ///9: GTIOC4B
    _0x09 = 9,
    ///10: GTIOC5A
    _0x0a = 10,
    ///11: GTIOC5B
    _0x0b = 11,
    ///12: GTIOC6A
    _0x0c = 12,
    ///13: GTIOC6B
    _0x0d = 13,
    ///14: GTIOC7A
    _0x0e = 14,
    ///15: GTIOC7B
    _0x0f = 15,
    ///16: GTIOC8A
    _0x10 = 16,
    ///17: GTIOC8B
    _0x11 = 17,
    ///18: GTIOC9A
    _0x12 = 18,
    ///19: GTIOC9B
    _0x13 = 19,
    ///20: Setting prohibited
    Others = 20,
}
impl From<Iclfselc> for u8 {
    #[inline(always)]
    fn from(variant: Iclfselc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iclfselc {
    type Ux = u8;
}
impl crate::IsEnum for Iclfselc {}
///Field `ICLFSELC` reader - Inter Channel Signal C Select
pub type IclfselcR = crate::FieldReader<Iclfselc>;
impl IclfselcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iclfselc {
        match self.bits {
            0 => Iclfselc::_0x00,
            1 => Iclfselc::_0x01,
            2 => Iclfselc::_0x02,
            3 => Iclfselc::_0x03,
            4 => Iclfselc::_0x04,
            5 => Iclfselc::_0x05,
            6 => Iclfselc::_0x06,
            7 => Iclfselc::_0x07,
            8 => Iclfselc::_0x08,
            9 => Iclfselc::_0x09,
            10 => Iclfselc::_0x0a,
            11 => Iclfselc::_0x0b,
            12 => Iclfselc::_0x0c,
            13 => Iclfselc::_0x0d,
            14 => Iclfselc::_0x0e,
            15 => Iclfselc::_0x0f,
            16 => Iclfselc::_0x10,
            17 => Iclfselc::_0x11,
            18 => Iclfselc::_0x12,
            19 => Iclfselc::_0x13,
            _ => Iclfselc::Others,
        }
    }
    ///GTIOC0A
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == Iclfselc::_0x00
    }
    ///GTIOC0B
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == Iclfselc::_0x01
    }
    ///GTIOC1A
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == Iclfselc::_0x02
    }
    ///GTIOC1B
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == Iclfselc::_0x03
    }
    ///GTIOC2A
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == Iclfselc::_0x04
    }
    ///GTIOC2B
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == Iclfselc::_0x05
    }
    ///GTIOC3A
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == Iclfselc::_0x06
    }
    ///GTIOC3B
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == Iclfselc::_0x07
    }
    ///GTIOC4A
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == Iclfselc::_0x08
    }
    ///GTIOC4B
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == Iclfselc::_0x09
    }
    ///GTIOC5A
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == Iclfselc::_0x0a
    }
    ///GTIOC5B
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == Iclfselc::_0x0b
    }
    ///GTIOC6A
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == Iclfselc::_0x0c
    }
    ///GTIOC6B
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == Iclfselc::_0x0d
    }
    ///GTIOC7A
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == Iclfselc::_0x0e
    }
    ///GTIOC7B
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == Iclfselc::_0x0f
    }
    ///GTIOC8A
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == Iclfselc::_0x10
    }
    ///GTIOC8B
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == Iclfselc::_0x11
    }
    ///GTIOC9A
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == Iclfselc::_0x12
    }
    ///GTIOC9B
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == Iclfselc::_0x13
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Iclfselc::Others)
    }
}
///Field `ICLFSELC` writer - Inter Channel Signal C Select
pub type IclfselcW<'a, REG> = crate::FieldWriter<'a, REG, 6, Iclfselc, crate::Safe>;
impl<'a, REG> IclfselcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///GTIOC0A
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x00)
    }
    ///GTIOC0B
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x01)
    }
    ///GTIOC1A
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x02)
    }
    ///GTIOC1B
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x03)
    }
    ///GTIOC2A
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x04)
    }
    ///GTIOC2B
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x05)
    }
    ///GTIOC3A
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x06)
    }
    ///GTIOC3B
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x07)
    }
    ///GTIOC4A
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x08)
    }
    ///GTIOC4B
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x09)
    }
    ///GTIOC5A
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x0a)
    }
    ///GTIOC5B
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x0b)
    }
    ///GTIOC6A
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x0c)
    }
    ///GTIOC6B
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x0d)
    }
    ///GTIOC7A
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x0e)
    }
    ///GTIOC7B
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x0f)
    }
    ///GTIOC8A
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x10)
    }
    ///GTIOC8B
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x11)
    }
    ///GTIOC9A
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x12)
    }
    ///GTIOC9B
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::_0x13)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfselc::Others)
    }
}
/**GTIOCnB Output Logical Operation Function Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iclfb {
    ///0: B (no delay)
    _000 = 0,
    ///1: NOT B (no delay)
    _001 = 1,
    ///2: D (1PCLKD delay)
    _010 = 2,
    ///3: NOT D (1PCLKD delay)
    _011 = 3,
    ///4: B AND D (1PCLKD delay)
    _100 = 4,
    ///5: B OR D (1PCLKDn delay)
    _101 = 5,
    ///6: B EXOR D (1PCLKD delay)
    _110 = 6,
    ///7: B NOR D (1PCLKD delay)
    _111 = 7,
}
impl From<Iclfb> for u8 {
    #[inline(always)]
    fn from(variant: Iclfb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iclfb {
    type Ux = u8;
}
impl crate::IsEnum for Iclfb {}
///Field `ICLFB` reader - GTIOCnB Output Logical Operation Function Select
pub type IclfbR = crate::FieldReader<Iclfb>;
impl IclfbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iclfb {
        match self.bits {
            0 => Iclfb::_000,
            1 => Iclfb::_001,
            2 => Iclfb::_010,
            3 => Iclfb::_011,
            4 => Iclfb::_100,
            5 => Iclfb::_101,
            6 => Iclfb::_110,
            7 => Iclfb::_111,
            _ => unreachable!(),
        }
    }
    ///B (no delay)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Iclfb::_000
    }
    ///NOT B (no delay)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Iclfb::_001
    }
    ///D (1PCLKD delay)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Iclfb::_010
    }
    ///NOT D (1PCLKD delay)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Iclfb::_011
    }
    ///B AND D (1PCLKD delay)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Iclfb::_100
    }
    ///B OR D (1PCLKDn delay)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Iclfb::_101
    }
    ///B EXOR D (1PCLKD delay)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Iclfb::_110
    }
    ///B NOR D (1PCLKD delay)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Iclfb::_111
    }
}
///Field `ICLFB` writer - GTIOCnB Output Logical Operation Function Select
pub type IclfbW<'a, REG> = crate::FieldWriter<'a, REG, 3, Iclfb, crate::Safe>;
impl<'a, REG> IclfbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///B (no delay)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfb::_000)
    }
    ///NOT B (no delay)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfb::_001)
    }
    ///D (1PCLKD delay)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfb::_010)
    }
    ///NOT D (1PCLKD delay)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfb::_011)
    }
    ///B AND D (1PCLKD delay)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfb::_100)
    }
    ///B OR D (1PCLKDn delay)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfb::_101)
    }
    ///B EXOR D (1PCLKD delay)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfb::_110)
    }
    ///B NOR D (1PCLKD delay)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfb::_111)
    }
}
/**Inter Channel Signal D Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iclfseld {
    ///0: GTIOC0A
    _0x00 = 0,
    ///1: GTIOC0B
    _0x01 = 1,
    ///2: GTIOC1A
    _0x02 = 2,
    ///3: GTIOC1B
    _0x03 = 3,
    ///4: GTIOC2A
    _0x04 = 4,
    ///5: GTIOC2B
    _0x05 = 5,
    ///6: GTIOC3A
    _0x06 = 6,
    ///7: GTIOC3B
    _0x07 = 7,
    ///8: GTIOC4A
    _0x08 = 8,
    ///9: GTIOC4B
    _0x09 = 9,
    ///10: GTIOC5A
    _0x0a = 10,
    ///11: GTIOC5B
    _0x0b = 11,
    ///12: GTIOC6A
    _0x0c = 12,
    ///13: GTIOC6B
    _0x0d = 13,
    ///14: GTIOC7A
    _0x0e = 14,
    ///15: GTIOC7B
    _0x0f = 15,
    ///16: GTIOC8A
    _0x10 = 16,
    ///17: GTIOC8B
    _0x11 = 17,
    ///18: GTIOC9A
    _0x12 = 18,
    ///19: GTIOC9B
    _0x13 = 19,
    ///20: Setting prohibited
    Others = 20,
}
impl From<Iclfseld> for u8 {
    #[inline(always)]
    fn from(variant: Iclfseld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iclfseld {
    type Ux = u8;
}
impl crate::IsEnum for Iclfseld {}
///Field `ICLFSELD` reader - Inter Channel Signal D Select
pub type IclfseldR = crate::FieldReader<Iclfseld>;
impl IclfseldR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iclfseld {
        match self.bits {
            0 => Iclfseld::_0x00,
            1 => Iclfseld::_0x01,
            2 => Iclfseld::_0x02,
            3 => Iclfseld::_0x03,
            4 => Iclfseld::_0x04,
            5 => Iclfseld::_0x05,
            6 => Iclfseld::_0x06,
            7 => Iclfseld::_0x07,
            8 => Iclfseld::_0x08,
            9 => Iclfseld::_0x09,
            10 => Iclfseld::_0x0a,
            11 => Iclfseld::_0x0b,
            12 => Iclfseld::_0x0c,
            13 => Iclfseld::_0x0d,
            14 => Iclfseld::_0x0e,
            15 => Iclfseld::_0x0f,
            16 => Iclfseld::_0x10,
            17 => Iclfseld::_0x11,
            18 => Iclfseld::_0x12,
            19 => Iclfseld::_0x13,
            _ => Iclfseld::Others,
        }
    }
    ///GTIOC0A
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == Iclfseld::_0x00
    }
    ///GTIOC0B
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == Iclfseld::_0x01
    }
    ///GTIOC1A
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == Iclfseld::_0x02
    }
    ///GTIOC1B
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == Iclfseld::_0x03
    }
    ///GTIOC2A
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == Iclfseld::_0x04
    }
    ///GTIOC2B
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == Iclfseld::_0x05
    }
    ///GTIOC3A
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == Iclfseld::_0x06
    }
    ///GTIOC3B
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == Iclfseld::_0x07
    }
    ///GTIOC4A
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == Iclfseld::_0x08
    }
    ///GTIOC4B
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == Iclfseld::_0x09
    }
    ///GTIOC5A
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == Iclfseld::_0x0a
    }
    ///GTIOC5B
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == Iclfseld::_0x0b
    }
    ///GTIOC6A
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == Iclfseld::_0x0c
    }
    ///GTIOC6B
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == Iclfseld::_0x0d
    }
    ///GTIOC7A
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == Iclfseld::_0x0e
    }
    ///GTIOC7B
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == Iclfseld::_0x0f
    }
    ///GTIOC8A
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == Iclfseld::_0x10
    }
    ///GTIOC8B
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == Iclfseld::_0x11
    }
    ///GTIOC9A
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == Iclfseld::_0x12
    }
    ///GTIOC9B
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == Iclfseld::_0x13
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Iclfseld::Others)
    }
}
///Field `ICLFSELD` writer - Inter Channel Signal D Select
pub type IclfseldW<'a, REG> = crate::FieldWriter<'a, REG, 6, Iclfseld, crate::Safe>;
impl<'a, REG> IclfseldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///GTIOC0A
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x00)
    }
    ///GTIOC0B
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x01)
    }
    ///GTIOC1A
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x02)
    }
    ///GTIOC1B
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x03)
    }
    ///GTIOC2A
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x04)
    }
    ///GTIOC2B
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x05)
    }
    ///GTIOC3A
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x06)
    }
    ///GTIOC3B
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x07)
    }
    ///GTIOC4A
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x08)
    }
    ///GTIOC4B
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x09)
    }
    ///GTIOC5A
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x0a)
    }
    ///GTIOC5B
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x0b)
    }
    ///GTIOC6A
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x0c)
    }
    ///GTIOC6B
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x0d)
    }
    ///GTIOC7A
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x0e)
    }
    ///GTIOC7B
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x0f)
    }
    ///GTIOC8A
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x10)
    }
    ///GTIOC8B
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x11)
    }
    ///GTIOC9A
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x12)
    }
    ///GTIOC9B
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::_0x13)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Iclfseld::Others)
    }
}
impl R {
    ///Bits 0:2 - GTIOCnA Output Logical Operation Function Select
    #[inline(always)]
    pub fn iclfa(&self) -> IclfaR {
        IclfaR::new((self.bits & 7) as u8)
    }
    ///Bits 4:9 - Inter Channel Signal C Select
    #[inline(always)]
    pub fn iclfselc(&self) -> IclfselcR {
        IclfselcR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    ///Bits 16:18 - GTIOCnB Output Logical Operation Function Select
    #[inline(always)]
    pub fn iclfb(&self) -> IclfbR {
        IclfbR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:25 - Inter Channel Signal D Select
    #[inline(always)]
    pub fn iclfseld(&self) -> IclfseldR {
        IclfseldR::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTICLF")
            .field("iclfa", &self.iclfa())
            .field("iclfselc", &self.iclfselc())
            .field("iclfb", &self.iclfb())
            .field("iclfseld", &self.iclfseld())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - GTIOCnA Output Logical Operation Function Select
    #[inline(always)]
    pub fn iclfa(&mut self) -> IclfaW<GticlfSpec> {
        IclfaW::new(self, 0)
    }
    ///Bits 4:9 - Inter Channel Signal C Select
    #[inline(always)]
    pub fn iclfselc(&mut self) -> IclfselcW<GticlfSpec> {
        IclfselcW::new(self, 4)
    }
    ///Bits 16:18 - GTIOCnB Output Logical Operation Function Select
    #[inline(always)]
    pub fn iclfb(&mut self) -> IclfbW<GticlfSpec> {
        IclfbW::new(self, 16)
    }
    ///Bits 20:25 - Inter Channel Signal D Select
    #[inline(always)]
    pub fn iclfseld(&mut self) -> IclfseldW<GticlfSpec> {
        IclfseldW::new(self, 20)
    }
}
/**General PWM Timer Inter Channel Logical Operation Function Setting Register

You can [`read`](crate::Reg::read) this register and get [`gticlf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticlf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GticlfSpec;
impl crate::RegisterSpec for GticlfSpec {
    type Ux = u32;
}
///`read()` method returns [`gticlf::R`](R) reader structure
impl crate::Readable for GticlfSpec {}
///`write(|w| ..)` method takes [`gticlf::W`](W) writer structure
impl crate::Writable for GticlfSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTICLF to value 0
impl crate::Resettable for GticlfSpec {}
