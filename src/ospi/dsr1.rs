///Register `DSR1` reader
pub type R = crate::R<Dsr1Spec>;
///Register `DSR1` writer
pub type W = crate::W<Dsr1Spec>;
///Field `DV1SZ` reader - Device 1 size setting
pub type Dv1szR = crate::FieldReader<u32>;
///Field `DV1SZ` writer - Device 1 size setting
pub type Dv1szW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
/**Device 1 type setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dv1typ {
    ///0: Flash on device 1
    _00 = 0,
    ///1: RAM on device 1
    _01 = 1,
    ///2: No connection on device 1
    _10 = 2,
    ///3: Forbidden
    _11 = 3,
}
impl From<Dv1typ> for u8 {
    #[inline(always)]
    fn from(variant: Dv1typ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dv1typ {
    type Ux = u8;
}
impl crate::IsEnum for Dv1typ {}
///Field `DV1TYP` reader - Device 1 type setting
pub type Dv1typR = crate::FieldReader<Dv1typ>;
impl Dv1typR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dv1typ {
        match self.bits {
            0 => Dv1typ::_00,
            1 => Dv1typ::_01,
            2 => Dv1typ::_10,
            3 => Dv1typ::_11,
            _ => unreachable!(),
        }
    }
    ///Flash on device 1
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dv1typ::_00
    }
    ///RAM on device 1
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dv1typ::_01
    }
    ///No connection on device 1
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dv1typ::_10
    }
    ///Forbidden
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dv1typ::_11
    }
}
///Field `DV1TYP` writer - Device 1 type setting
pub type Dv1typW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dv1typ, crate::Safe>;
impl<'a, REG> Dv1typW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Flash on device 1
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dv1typ::_00)
    }
    ///RAM on device 1
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dv1typ::_01)
    }
    ///No connection on device 1
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dv1typ::_10)
    }
    ///Forbidden
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dv1typ::_11)
    }
}
impl R {
    ///Bits 0:29 - Device 1 size setting
    #[inline(always)]
    pub fn dv1sz(&self) -> Dv1szR {
        Dv1szR::new(self.bits & 0x3fff_ffff)
    }
    ///Bits 30:31 - Device 1 type setting
    #[inline(always)]
    pub fn dv1typ(&self) -> Dv1typR {
        Dv1typR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSR1")
            .field("dv1sz", &self.dv1sz())
            .field("dv1typ", &self.dv1typ())
            .finish()
    }
}
impl W {
    ///Bits 0:29 - Device 1 size setting
    #[inline(always)]
    pub fn dv1sz(&mut self) -> Dv1szW<Dsr1Spec> {
        Dv1szW::new(self, 0)
    }
    ///Bits 30:31 - Device 1 type setting
    #[inline(always)]
    pub fn dv1typ(&mut self) -> Dv1typW<Dsr1Spec> {
        Dv1typW::new(self, 30)
    }
}
/**Device Size Register 1

You can [`read`](crate::Reg::read) this register and get [`dsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dsr1Spec;
impl crate::RegisterSpec for Dsr1Spec {
    type Ux = u32;
}
///`read()` method returns [`dsr1::R`](R) reader structure
impl crate::Readable for Dsr1Spec {}
///`write(|w| ..)` method takes [`dsr1::W`](W) writer structure
impl crate::Writable for Dsr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DSR1 to value 0
impl crate::Resettable for Dsr1Spec {}
