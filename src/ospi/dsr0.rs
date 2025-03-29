///Register `DSR0` reader
pub type R = crate::R<Dsr0Spec>;
///Register `DSR0` writer
pub type W = crate::W<Dsr0Spec>;
///Field `DV0SZ` reader - Device 0 size setting
pub type Dv0szR = crate::FieldReader<u32>;
///Field `DV0SZ` writer - Device 0 size setting
pub type Dv0szW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
/**Device 0 type setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dv0typ {
    ///0: Flash on device 0
    _00 = 0,
    ///1: RAM on device 0
    _01 = 1,
    ///2: No connection on device 0
    _10 = 2,
    ///3: Forbidden
    _11 = 3,
}
impl From<Dv0typ> for u8 {
    #[inline(always)]
    fn from(variant: Dv0typ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dv0typ {
    type Ux = u8;
}
impl crate::IsEnum for Dv0typ {}
///Field `DV0TYP` reader - Device 0 type setting
pub type Dv0typR = crate::FieldReader<Dv0typ>;
impl Dv0typR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dv0typ {
        match self.bits {
            0 => Dv0typ::_00,
            1 => Dv0typ::_01,
            2 => Dv0typ::_10,
            3 => Dv0typ::_11,
            _ => unreachable!(),
        }
    }
    ///Flash on device 0
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dv0typ::_00
    }
    ///RAM on device 0
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dv0typ::_01
    }
    ///No connection on device 0
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dv0typ::_10
    }
    ///Forbidden
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dv0typ::_11
    }
}
///Field `DV0TYP` writer - Device 0 type setting
pub type Dv0typW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dv0typ, crate::Safe>;
impl<'a, REG> Dv0typW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Flash on device 0
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dv0typ::_00)
    }
    ///RAM on device 0
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dv0typ::_01)
    }
    ///No connection on device 0
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dv0typ::_10)
    }
    ///Forbidden
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dv0typ::_11)
    }
}
impl R {
    ///Bits 0:29 - Device 0 size setting
    #[inline(always)]
    pub fn dv0sz(&self) -> Dv0szR {
        Dv0szR::new(self.bits & 0x3fff_ffff)
    }
    ///Bits 30:31 - Device 0 type setting
    #[inline(always)]
    pub fn dv0typ(&self) -> Dv0typR {
        Dv0typR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSR0")
            .field("dv0sz", &self.dv0sz())
            .field("dv0typ", &self.dv0typ())
            .finish()
    }
}
impl W {
    ///Bits 0:29 - Device 0 size setting
    #[inline(always)]
    pub fn dv0sz(&mut self) -> Dv0szW<Dsr0Spec> {
        Dv0szW::new(self, 0)
    }
    ///Bits 30:31 - Device 0 type setting
    #[inline(always)]
    pub fn dv0typ(&mut self) -> Dv0typW<Dsr0Spec> {
        Dv0typW::new(self, 30)
    }
}
/**Device Size Register 0

You can [`read`](crate::Reg::read) this register and get [`dsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dsr0Spec;
impl crate::RegisterSpec for Dsr0Spec {
    type Ux = u32;
}
///`read()` method returns [`dsr0::R`](R) reader structure
impl crate::Readable for Dsr0Spec {}
///`write(|w| ..)` method takes [`dsr0::W`](W) writer structure
impl crate::Writable for Dsr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DSR0 to value 0
impl crate::Resettable for Dsr0Spec {}
