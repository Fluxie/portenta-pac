///Register `GTINTAD` reader
pub type R = crate::R<GtintadSpec>;
///Register `GTINTAD` writer
pub type W = crate::W<GtintadSpec>;
/**Output Disable Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Grp {
    ///0: Group A output disable request is selected
    _00 = 0,
    ///1: Group B output disable request is selected
    _01 = 1,
    ///2: Group C output disable request is selected
    _10 = 2,
    ///3: Group D output disable request is selected
    _11 = 3,
}
impl From<Grp> for u8 {
    #[inline(always)]
    fn from(variant: Grp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Grp {
    type Ux = u8;
}
impl crate::IsEnum for Grp {}
///Field `GRP` reader - Output Disable Source Select
pub type GrpR = crate::FieldReader<Grp>;
impl GrpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Grp {
        match self.bits {
            0 => Grp::_00,
            1 => Grp::_01,
            2 => Grp::_10,
            3 => Grp::_11,
            _ => unreachable!(),
        }
    }
    ///Group A output disable request is selected
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Grp::_00
    }
    ///Group B output disable request is selected
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Grp::_01
    }
    ///Group C output disable request is selected
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Grp::_10
    }
    ///Group D output disable request is selected
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Grp::_11
    }
}
///Field `GRP` writer - Output Disable Source Select
pub type GrpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Grp, crate::Safe>;
impl<'a, REG> GrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Group A output disable request is selected
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Grp::_00)
    }
    ///Group B output disable request is selected
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Grp::_01)
    }
    ///Group C output disable request is selected
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Grp::_10)
    }
    ///Group D output disable request is selected
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Grp::_11)
    }
}
/**Same Time Output Level High Disable Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grpabh {
    ///0: Same time output level high disable request disabled
    _0 = 0,
    ///1: Same time output level high disable request enabled
    _1 = 1,
}
impl From<Grpabh> for bool {
    #[inline(always)]
    fn from(variant: Grpabh) -> Self {
        variant as u8 != 0
    }
}
///Field `GRPABH` reader - Same Time Output Level High Disable Request Enable
pub type GrpabhR = crate::BitReader<Grpabh>;
impl GrpabhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Grpabh {
        match self.bits {
            false => Grpabh::_0,
            true => Grpabh::_1,
        }
    }
    ///Same time output level high disable request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Grpabh::_0
    }
    ///Same time output level high disable request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Grpabh::_1
    }
}
///Field `GRPABH` writer - Same Time Output Level High Disable Request Enable
pub type GrpabhW<'a, REG> = crate::BitWriter<'a, REG, Grpabh>;
impl<'a, REG> GrpabhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Same time output level high disable request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Grpabh::_0)
    }
    ///Same time output level high disable request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Grpabh::_1)
    }
}
/**Same Time Output Level Low Disable Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grpabl {
    ///0: Same time output level low disable request disabled
    _0 = 0,
    ///1: Same time output level low disable request enabled
    _1 = 1,
}
impl From<Grpabl> for bool {
    #[inline(always)]
    fn from(variant: Grpabl) -> Self {
        variant as u8 != 0
    }
}
///Field `GRPABL` reader - Same Time Output Level Low Disable Request Enable
pub type GrpablR = crate::BitReader<Grpabl>;
impl GrpablR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Grpabl {
        match self.bits {
            false => Grpabl::_0,
            true => Grpabl::_1,
        }
    }
    ///Same time output level low disable request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Grpabl::_0
    }
    ///Same time output level low disable request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Grpabl::_1
    }
}
///Field `GRPABL` writer - Same Time Output Level Low Disable Request Enable
pub type GrpablW<'a, REG> = crate::BitWriter<'a, REG, Grpabl>;
impl<'a, REG> GrpablW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Same time output level low disable request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Grpabl::_0)
    }
    ///Same time output level low disable request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Grpabl::_1)
    }
}
impl R {
    ///Bits 24:25 - Output Disable Source Select
    #[inline(always)]
    pub fn grp(&self) -> GrpR {
        GrpR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 29 - Same Time Output Level High Disable Request Enable
    #[inline(always)]
    pub fn grpabh(&self) -> GrpabhR {
        GrpabhR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Same Time Output Level Low Disable Request Enable
    #[inline(always)]
    pub fn grpabl(&self) -> GrpablR {
        GrpablR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTINTAD")
            .field("grp", &self.grp())
            .field("grpabh", &self.grpabh())
            .field("grpabl", &self.grpabl())
            .finish()
    }
}
impl W {
    ///Bits 24:25 - Output Disable Source Select
    #[inline(always)]
    pub fn grp(&mut self) -> GrpW<GtintadSpec> {
        GrpW::new(self, 24)
    }
    ///Bit 29 - Same Time Output Level High Disable Request Enable
    #[inline(always)]
    pub fn grpabh(&mut self) -> GrpabhW<GtintadSpec> {
        GrpabhW::new(self, 29)
    }
    ///Bit 30 - Same Time Output Level Low Disable Request Enable
    #[inline(always)]
    pub fn grpabl(&mut self) -> GrpablW<GtintadSpec> {
        GrpablW::new(self, 30)
    }
}
/**General PWM Timer Interrupt Output Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtintad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtintad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtintadSpec;
impl crate::RegisterSpec for GtintadSpec {
    type Ux = u32;
}
///`read()` method returns [`gtintad::R`](R) reader structure
impl crate::Readable for GtintadSpec {}
///`write(|w| ..)` method takes [`gtintad::W`](W) writer structure
impl crate::Writable for GtintadSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTINTAD to value 0
impl crate::Resettable for GtintadSpec {}
