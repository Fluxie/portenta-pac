///Register `RHRCNT` reader
pub type R = crate::R<RhrcntSpec>;
///Register `RHRCNT` writer
pub type W = crate::W<RhrcntSpec>;
///Field `HR1` reader - 1-Hour Count
pub type Hr1R = crate::FieldReader;
///Field `HR1` writer - 1-Hour Count
pub type Hr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HR10` reader - 10-Hour Count
pub type Hr10R = crate::FieldReader;
///Field `HR10` writer - 10-Hour Count
pub type Hr10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**AM/PM select for time counter setting.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pm {
    ///0: AM
    _0 = 0,
    ///1: PM
    _1 = 1,
}
impl From<Pm> for bool {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as u8 != 0
    }
}
///Field `PM` reader - AM/PM select for time counter setting.
pub type PmR = crate::BitReader<Pm>;
impl PmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pm {
        match self.bits {
            false => Pm::_0,
            true => Pm::_1,
        }
    }
    ///AM
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pm::_0
    }
    ///PM
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pm::_1
    }
}
///Field `PM` writer - AM/PM select for time counter setting.
pub type PmW<'a, REG> = crate::BitWriter<'a, REG, Pm>;
impl<'a, REG> PmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AM
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_0)
    }
    ///PM
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_1)
    }
}
impl R {
    ///Bits 0:3 - 1-Hour Count
    #[inline(always)]
    pub fn hr1(&self) -> Hr1R {
        Hr1R::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - 10-Hour Count
    #[inline(always)]
    pub fn hr10(&self) -> Hr10R {
        Hr10R::new((self.bits >> 4) & 3)
    }
    ///Bit 6 - AM/PM select for time counter setting.
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RHRCNT")
            .field("hr1", &self.hr1())
            .field("hr10", &self.hr10())
            .field("pm", &self.pm())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 1-Hour Count
    #[inline(always)]
    pub fn hr1(&mut self) -> Hr1W<RhrcntSpec> {
        Hr1W::new(self, 0)
    }
    ///Bits 4:5 - 10-Hour Count
    #[inline(always)]
    pub fn hr10(&mut self) -> Hr10W<RhrcntSpec> {
        Hr10W::new(self, 4)
    }
    ///Bit 6 - AM/PM select for time counter setting.
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<RhrcntSpec> {
        PmW::new(self, 6)
    }
}
/**Hour Counter (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rhrcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rhrcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RhrcntSpec;
impl crate::RegisterSpec for RhrcntSpec {
    type Ux = u8;
}
///`read()` method returns [`rhrcnt::R`](R) reader structure
impl crate::Readable for RhrcntSpec {}
///`write(|w| ..)` method takes [`rhrcnt::W`](W) writer structure
impl crate::Writable for RhrcntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RHRCNT to value 0
impl crate::Resettable for RhrcntSpec {}
