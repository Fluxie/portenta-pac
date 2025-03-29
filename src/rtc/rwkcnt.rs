///Register `RWKCNT` reader
pub type R = crate::R<RwkcntSpec>;
///Register `RWKCNT` writer
pub type W = crate::W<RwkcntSpec>;
/**Day-of-Week Counting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dayw {
    ///0: Sunday
    _000 = 0,
    ///1: Monday
    _001 = 1,
    ///2: Tuesday
    _010 = 2,
    ///3: Wednesday
    _011 = 3,
    ///4: Thursday
    _100 = 4,
    ///5: Friday
    _101 = 5,
    ///6: Saturday
    _110 = 6,
    ///7: Setting prohibited
    _111 = 7,
}
impl From<Dayw> for u8 {
    #[inline(always)]
    fn from(variant: Dayw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dayw {
    type Ux = u8;
}
impl crate::IsEnum for Dayw {}
///Field `DAYW` reader - Day-of-Week Counting
pub type DaywR = crate::FieldReader<Dayw>;
impl DaywR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dayw {
        match self.bits {
            0 => Dayw::_000,
            1 => Dayw::_001,
            2 => Dayw::_010,
            3 => Dayw::_011,
            4 => Dayw::_100,
            5 => Dayw::_101,
            6 => Dayw::_110,
            7 => Dayw::_111,
            _ => unreachable!(),
        }
    }
    ///Sunday
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dayw::_000
    }
    ///Monday
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dayw::_001
    }
    ///Tuesday
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dayw::_010
    }
    ///Wednesday
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dayw::_011
    }
    ///Thursday
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dayw::_100
    }
    ///Friday
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dayw::_101
    }
    ///Saturday
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dayw::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dayw::_111
    }
}
///Field `DAYW` writer - Day-of-Week Counting
pub type DaywW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dayw, crate::Safe>;
impl<'a, REG> DaywW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Sunday
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dayw::_000)
    }
    ///Monday
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dayw::_001)
    }
    ///Tuesday
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dayw::_010)
    }
    ///Wednesday
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dayw::_011)
    }
    ///Thursday
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dayw::_100)
    }
    ///Friday
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dayw::_101)
    }
    ///Saturday
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dayw::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dayw::_111)
    }
}
impl R {
    ///Bits 0:2 - Day-of-Week Counting
    #[inline(always)]
    pub fn dayw(&self) -> DaywR {
        DaywR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RWKCNT").field("dayw", &self.dayw()).finish()
    }
}
impl W {
    ///Bits 0:2 - Day-of-Week Counting
    #[inline(always)]
    pub fn dayw(&mut self) -> DaywW<RwkcntSpec> {
        DaywW::new(self, 0)
    }
}
/**Day-of-Week Counter (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rwkcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwkcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RwkcntSpec;
impl crate::RegisterSpec for RwkcntSpec {
    type Ux = u8;
}
///`read()` method returns [`rwkcnt::R`](R) reader structure
impl crate::Readable for RwkcntSpec {}
///`write(|w| ..)` method takes [`rwkcnt::W`](W) writer structure
impl crate::Writable for RwkcntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RWKCNT to value 0
impl crate::Resettable for RwkcntSpec {}
