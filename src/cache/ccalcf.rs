///Register `CCALCF` reader
pub type R = crate::R<CcalcfSpec>;
///Register `CCALCF` writer
pub type W = crate::W<CcalcfSpec>;
/**C-Cache Line Size

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc {
    ///0: Prohibited
    _00 = 0,
    ///1: Cache line size 32 bytes
    _01 = 1,
    ///2: Cache line size 64 bytes
    _10 = 2,
    ///3: Prohibited
    _11 = 3,
}
impl From<Cc> for u8 {
    #[inline(always)]
    fn from(variant: Cc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc {
    type Ux = u8;
}
impl crate::IsEnum for Cc {}
///Field `CC` reader - C-Cache Line Size
pub type CcR = crate::FieldReader<Cc>;
impl CcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cc {
        match self.bits {
            0 => Cc::_00,
            1 => Cc::_01,
            2 => Cc::_10,
            3 => Cc::_11,
            _ => unreachable!(),
        }
    }
    ///Prohibited
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cc::_00
    }
    ///Cache line size 32 bytes
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cc::_01
    }
    ///Cache line size 64 bytes
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cc::_10
    }
    ///Prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cc::_11
    }
}
///Field `CC` writer - C-Cache Line Size
pub type CcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc, crate::Safe>;
impl<'a, REG> CcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Prohibited
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::_00)
    }
    ///Cache line size 32 bytes
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::_01)
    }
    ///Cache line size 64 bytes
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::_10)
    }
    ///Prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::_11)
    }
}
impl R {
    ///Bits 0:1 - C-Cache Line Size
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCALCF").field("cc", &self.cc()).finish()
    }
}
impl W {
    ///Bits 0:1 - C-Cache Line Size
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<CcalcfSpec> {
        CcW::new(self, 0)
    }
}
/**C-Cache Line Configuration Register

You can [`read`](crate::Reg::read) this register and get [`ccalcf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccalcf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CcalcfSpec;
impl crate::RegisterSpec for CcalcfSpec {
    type Ux = u32;
}
///`read()` method returns [`ccalcf::R`](R) reader structure
impl crate::Readable for CcalcfSpec {}
///`write(|w| ..)` method takes [`ccalcf::W`](W) writer structure
impl crate::Writable for CcalcfSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCALCF to value 0x01
impl crate::Resettable for CcalcfSpec {
    const RESET_VALUE: u32 = 0x01;
}
