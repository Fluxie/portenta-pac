///Register `CR1` reader
pub type R = crate::R<Cr1Spec>;
///Register `CR1` writer
pub type W = crate::W<Cr1Spec>;
/**Break Field Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfe {
    ///0: Break Field detection is disabled.
    _0 = 0,
    ///1: Break Field detection is enabled.
    _1 = 1,
}
impl From<Bfe> for bool {
    #[inline(always)]
    fn from(variant: Bfe) -> Self {
        variant as u8 != 0
    }
}
///Field `BFE` reader - Break Field Enable
pub type BfeR = crate::BitReader<Bfe>;
impl BfeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bfe {
        match self.bits {
            false => Bfe::_0,
            true => Bfe::_1,
        }
    }
    ///Break Field detection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bfe::_0
    }
    ///Break Field detection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bfe::_1
    }
}
///Field `BFE` writer - Break Field Enable
pub type BfeW<'a, REG> = crate::BitWriter<'a, REG, Bfe>;
impl<'a, REG> BfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break Field detection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bfe::_0)
    }
    ///Break Field detection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfe::_1)
    }
}
/**Control Field 0 Reception Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf0re {
    ///0: Reception of Control Field 0 is disabled.
    _0 = 0,
    ///1: Reception of Control Field 0 is enabled.
    _1 = 1,
}
impl From<Cf0re> for bool {
    #[inline(always)]
    fn from(variant: Cf0re) -> Self {
        variant as u8 != 0
    }
}
///Field `CF0RE` reader - Control Field 0 Reception Enable
pub type Cf0reR = crate::BitReader<Cf0re>;
impl Cf0reR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf0re {
        match self.bits {
            false => Cf0re::_0,
            true => Cf0re::_1,
        }
    }
    ///Reception of Control Field 0 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf0re::_0
    }
    ///Reception of Control Field 0 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf0re::_1
    }
}
///Field `CF0RE` writer - Control Field 0 Reception Enable
pub type Cf0reW<'a, REG> = crate::BitWriter<'a, REG, Cf0re>;
impl<'a, REG> Cf0reW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reception of Control Field 0 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0re::_0)
    }
    ///Reception of Control Field 0 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0re::_1)
    }
}
/**Control Field 1 Data Register Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cf1ds {
    ///0: Selects comparison with the value in PCF1DR.
    _00 = 0,
    ///1: Selects comparison with the value in SCF1DR.
    _01 = 1,
    ///2: Selects comparison with the values in PCF1DR and SCF1DR.
    _10 = 2,
    ///3: Setting prohibited.
    _11 = 3,
}
impl From<Cf1ds> for u8 {
    #[inline(always)]
    fn from(variant: Cf1ds) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cf1ds {
    type Ux = u8;
}
impl crate::IsEnum for Cf1ds {}
///Field `CF1DS` reader - Control Field 1 Data Register Select
pub type Cf1dsR = crate::FieldReader<Cf1ds>;
impl Cf1dsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf1ds {
        match self.bits {
            0 => Cf1ds::_00,
            1 => Cf1ds::_01,
            2 => Cf1ds::_10,
            3 => Cf1ds::_11,
            _ => unreachable!(),
        }
    }
    ///Selects comparison with the value in PCF1DR.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cf1ds::_00
    }
    ///Selects comparison with the value in SCF1DR.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cf1ds::_01
    }
    ///Selects comparison with the values in PCF1DR and SCF1DR.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cf1ds::_10
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cf1ds::_11
    }
}
///Field `CF1DS` writer - Control Field 1 Data Register Select
pub type Cf1dsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cf1ds, crate::Safe>;
impl<'a, REG> Cf1dsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Selects comparison with the value in PCF1DR.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ds::_00)
    }
    ///Selects comparison with the value in SCF1DR.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ds::_01)
    }
    ///Selects comparison with the values in PCF1DR and SCF1DR.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ds::_10)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ds::_11)
    }
}
/**Priority Interrupt Bit Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pibe {
    ///0: The priority interrupt bit is disabled.
    _0 = 0,
    ///1: The priority interrupt bit is enabled.
    _1 = 1,
}
impl From<Pibe> for bool {
    #[inline(always)]
    fn from(variant: Pibe) -> Self {
        variant as u8 != 0
    }
}
///Field `PIBE` reader - Priority Interrupt Bit Enable
pub type PibeR = crate::BitReader<Pibe>;
impl PibeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pibe {
        match self.bits {
            false => Pibe::_0,
            true => Pibe::_1,
        }
    }
    ///The priority interrupt bit is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pibe::_0
    }
    ///The priority interrupt bit is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pibe::_1
    }
}
///Field `PIBE` writer - Priority Interrupt Bit Enable
pub type PibeW<'a, REG> = crate::BitWriter<'a, REG, Pibe>;
impl<'a, REG> PibeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The priority interrupt bit is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pibe::_0)
    }
    ///The priority interrupt bit is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pibe::_1)
    }
}
/**Priority Interrupt Bit Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pibs {
    ///0: 0th bit of Control Field 1
    _000 = 0,
    ///1: 1st bit of Control Field 1
    _001 = 1,
    ///2: 2nd bit of Control Field 1
    _010 = 2,
    ///3: 3rd bit of Control Field 1
    _011 = 3,
    ///4: 4th bit of Control Field 1
    _100 = 4,
    ///5: 5th bit of Control Field 1
    _101 = 5,
    ///6: 6th bit of Control Field 1
    _110 = 6,
    ///7: 7th bit of Control Field 1
    _111 = 7,
}
impl From<Pibs> for u8 {
    #[inline(always)]
    fn from(variant: Pibs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pibs {
    type Ux = u8;
}
impl crate::IsEnum for Pibs {}
///Field `PIBS` reader - Priority Interrupt Bit Select
pub type PibsR = crate::FieldReader<Pibs>;
impl PibsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pibs {
        match self.bits {
            0 => Pibs::_000,
            1 => Pibs::_001,
            2 => Pibs::_010,
            3 => Pibs::_011,
            4 => Pibs::_100,
            5 => Pibs::_101,
            6 => Pibs::_110,
            7 => Pibs::_111,
            _ => unreachable!(),
        }
    }
    ///0th bit of Control Field 1
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Pibs::_000
    }
    ///1st bit of Control Field 1
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Pibs::_001
    }
    ///2nd bit of Control Field 1
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Pibs::_010
    }
    ///3rd bit of Control Field 1
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Pibs::_011
    }
    ///4th bit of Control Field 1
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Pibs::_100
    }
    ///5th bit of Control Field 1
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Pibs::_101
    }
    ///6th bit of Control Field 1
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Pibs::_110
    }
    ///7th bit of Control Field 1
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Pibs::_111
    }
}
///Field `PIBS` writer - Priority Interrupt Bit Select
pub type PibsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pibs, crate::Safe>;
impl<'a, REG> PibsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///0th bit of Control Field 1
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Pibs::_000)
    }
    ///1st bit of Control Field 1
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Pibs::_001)
    }
    ///2nd bit of Control Field 1
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Pibs::_010)
    }
    ///3rd bit of Control Field 1
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Pibs::_011)
    }
    ///4th bit of Control Field 1
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Pibs::_100)
    }
    ///5th bit of Control Field 1
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Pibs::_101)
    }
    ///6th bit of Control Field 1
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Pibs::_110)
    }
    ///7th bit of Control Field 1
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Pibs::_111)
    }
}
impl R {
    ///Bit 0 - Break Field Enable
    #[inline(always)]
    pub fn bfe(&self) -> BfeR {
        BfeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Control Field 0 Reception Enable
    #[inline(always)]
    pub fn cf0re(&self) -> Cf0reR {
        Cf0reR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Control Field 1 Data Register Select
    #[inline(always)]
    pub fn cf1ds(&self) -> Cf1dsR {
        Cf1dsR::new((self.bits >> 2) & 3)
    }
    ///Bit 4 - Priority Interrupt Bit Enable
    #[inline(always)]
    pub fn pibe(&self) -> PibeR {
        PibeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - Priority Interrupt Bit Select
    #[inline(always)]
    pub fn pibs(&self) -> PibsR {
        PibsR::new((self.bits >> 5) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("bfe", &self.bfe())
            .field("cf0re", &self.cf0re())
            .field("cf1ds", &self.cf1ds())
            .field("pibe", &self.pibe())
            .field("pibs", &self.pibs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Break Field Enable
    #[inline(always)]
    pub fn bfe(&mut self) -> BfeW<Cr1Spec> {
        BfeW::new(self, 0)
    }
    ///Bit 1 - Control Field 0 Reception Enable
    #[inline(always)]
    pub fn cf0re(&mut self) -> Cf0reW<Cr1Spec> {
        Cf0reW::new(self, 1)
    }
    ///Bits 2:3 - Control Field 1 Data Register Select
    #[inline(always)]
    pub fn cf1ds(&mut self) -> Cf1dsW<Cr1Spec> {
        Cf1dsW::new(self, 2)
    }
    ///Bit 4 - Priority Interrupt Bit Enable
    #[inline(always)]
    pub fn pibe(&mut self) -> PibeW<Cr1Spec> {
        PibeW::new(self, 4)
    }
    ///Bits 5:7 - Priority Interrupt Bit Select
    #[inline(always)]
    pub fn pibs(&mut self) -> PibsW<Cr1Spec> {
        PibsW::new(self, 5)
    }
}
/**Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u8;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for Cr1Spec {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for Cr1Spec {}
