///Register `ADGSPCR` reader
pub type R = crate::R<AdgspcrSpec>;
///Register `ADGSPCR` writer
pub type W = crate::W<AdgspcrSpec>;
/**Group Priority Operation Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pgs {
    ///0: Operate without group priority control.
    _0 = 0,
    ///1: Operate with group priority control.
    _1 = 1,
}
impl From<Pgs> for bool {
    #[inline(always)]
    fn from(variant: Pgs) -> Self {
        variant as u8 != 0
    }
}
///Field `PGS` reader - Group Priority Operation Setting
pub type PgsR = crate::BitReader<Pgs>;
impl PgsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pgs {
        match self.bits {
            false => Pgs::_0,
            true => Pgs::_1,
        }
    }
    ///Operate without group priority control.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pgs::_0
    }
    ///Operate with group priority control.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pgs::_1
    }
}
///Field `PGS` writer - Group Priority Operation Setting
pub type PgsW<'a, REG> = crate::BitWriter<'a, REG, Pgs>;
impl<'a, REG> PgsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operate without group priority control.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pgs::_0)
    }
    ///Operate with group priority control.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pgs::_1)
    }
}
/**Lower-Priority Group Restart Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gbrscn {
    ///0: Disable rescanning of the group that was stopped in group priority operation
    _0 = 0,
    ///1: Enable rescanning of the group that was stopped in group priority operation.
    _1 = 1,
}
impl From<Gbrscn> for bool {
    #[inline(always)]
    fn from(variant: Gbrscn) -> Self {
        variant as u8 != 0
    }
}
///Field `GBRSCN` reader - Lower-Priority Group Restart Setting
pub type GbrscnR = crate::BitReader<Gbrscn>;
impl GbrscnR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gbrscn {
        match self.bits {
            false => Gbrscn::_0,
            true => Gbrscn::_1,
        }
    }
    ///Disable rescanning of the group that was stopped in group priority operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gbrscn::_0
    }
    ///Enable rescanning of the group that was stopped in group priority operation.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gbrscn::_1
    }
}
///Field `GBRSCN` writer - Lower-Priority Group Restart Setting
pub type GbrscnW<'a, REG> = crate::BitWriter<'a, REG, Gbrscn>;
impl<'a, REG> GbrscnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable rescanning of the group that was stopped in group priority operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gbrscn::_0)
    }
    ///Enable rescanning of the group that was stopped in group priority operation.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gbrscn::_1)
    }
}
/**Enabled only when PGS = 1 and GBRSCN = 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgrrs {
    ///0: Start rescanning from the first channel for scanning
    _0 = 0,
    ///1: Start rescanning from the channel for which A/D conversion is not completed.
    _1 = 1,
}
impl From<Lgrrs> for bool {
    #[inline(always)]
    fn from(variant: Lgrrs) -> Self {
        variant as u8 != 0
    }
}
///Field `LGRRS` reader - Enabled only when PGS = 1 and GBRSCN = 1.
pub type LgrrsR = crate::BitReader<Lgrrs>;
impl LgrrsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lgrrs {
        match self.bits {
            false => Lgrrs::_0,
            true => Lgrrs::_1,
        }
    }
    ///Start rescanning from the first channel for scanning
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lgrrs::_0
    }
    ///Start rescanning from the channel for which A/D conversion is not completed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lgrrs::_1
    }
}
///Field `LGRRS` writer - Enabled only when PGS = 1 and GBRSCN = 1.
pub type LgrrsW<'a, REG> = crate::BitWriter<'a, REG, Lgrrs>;
impl<'a, REG> LgrrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start rescanning from the first channel for scanning
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lgrrs::_0)
    }
    ///Start rescanning from the channel for which A/D conversion is not completed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lgrrs::_1)
    }
}
/**Single Scan Continuous Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gbrp {
    ///0: Single scan is not continuously activated.
    _0 = 0,
    ///1: Single scan for the group with the lower-priority is continuously activated.
    _1 = 1,
}
impl From<Gbrp> for bool {
    #[inline(always)]
    fn from(variant: Gbrp) -> Self {
        variant as u8 != 0
    }
}
///Field `GBRP` reader - Single Scan Continuous Start
pub type GbrpR = crate::BitReader<Gbrp>;
impl GbrpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gbrp {
        match self.bits {
            false => Gbrp::_0,
            true => Gbrp::_1,
        }
    }
    ///Single scan is not continuously activated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gbrp::_0
    }
    ///Single scan for the group with the lower-priority is continuously activated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gbrp::_1
    }
}
///Field `GBRP` writer - Single Scan Continuous Start
pub type GbrpW<'a, REG> = crate::BitWriter<'a, REG, Gbrp>;
impl<'a, REG> GbrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single scan is not continuously activated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gbrp::_0)
    }
    ///Single scan for the group with the lower-priority is continuously activated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gbrp::_1)
    }
}
impl R {
    ///Bit 0 - Group Priority Operation Setting
    #[inline(always)]
    pub fn pgs(&self) -> PgsR {
        PgsR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Lower-Priority Group Restart Setting
    #[inline(always)]
    pub fn gbrscn(&self) -> GbrscnR {
        GbrscnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 14 - Enabled only when PGS = 1 and GBRSCN = 1.
    #[inline(always)]
    pub fn lgrrs(&self) -> LgrrsR {
        LgrrsR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Single Scan Continuous Start
    #[inline(always)]
    pub fn gbrp(&self) -> GbrpR {
        GbrpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADGSPCR")
            .field("pgs", &self.pgs())
            .field("gbrscn", &self.gbrscn())
            .field("lgrrs", &self.lgrrs())
            .field("gbrp", &self.gbrp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Group Priority Operation Setting
    #[inline(always)]
    pub fn pgs(&mut self) -> PgsW<AdgspcrSpec> {
        PgsW::new(self, 0)
    }
    ///Bit 1 - Lower-Priority Group Restart Setting
    #[inline(always)]
    pub fn gbrscn(&mut self) -> GbrscnW<AdgspcrSpec> {
        GbrscnW::new(self, 1)
    }
    ///Bit 14 - Enabled only when PGS = 1 and GBRSCN = 1.
    #[inline(always)]
    pub fn lgrrs(&mut self) -> LgrrsW<AdgspcrSpec> {
        LgrrsW::new(self, 14)
    }
    ///Bit 15 - Single Scan Continuous Start
    #[inline(always)]
    pub fn gbrp(&mut self) -> GbrpW<AdgspcrSpec> {
        GbrpW::new(self, 15)
    }
}
/**A/D Group Scan Priority Control Register

You can [`read`](crate::Reg::read) this register and get [`adgspcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adgspcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdgspcrSpec;
impl crate::RegisterSpec for AdgspcrSpec {
    type Ux = u16;
}
///`read()` method returns [`adgspcr::R`](R) reader structure
impl crate::Readable for AdgspcrSpec {}
///`write(|w| ..)` method takes [`adgspcr::W`](W) writer structure
impl crate::Writable for AdgspcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADGSPCR to value 0
impl crate::Resettable for AdgspcrSpec {}
