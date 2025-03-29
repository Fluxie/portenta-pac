///Register `SCALCF` reader
pub type R = crate::R<ScalcfSpec>;
///Register `SCALCF` writer
pub type W = crate::W<ScalcfSpec>;
/**S-Cache Line Size

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cs {
    ///0: Prohibited
    _00 = 0,
    ///1: Cache line size 32 bytes
    _01 = 1,
    ///2: Cache line size 64 bytes
    _10 = 2,
    ///3: Prohibited
    _11 = 3,
}
impl From<Cs> for u8 {
    #[inline(always)]
    fn from(variant: Cs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cs {
    type Ux = u8;
}
impl crate::IsEnum for Cs {}
///Field `CS` reader - S-Cache Line Size
pub type CsR = crate::FieldReader<Cs>;
impl CsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cs {
        match self.bits {
            0 => Cs::_00,
            1 => Cs::_01,
            2 => Cs::_10,
            3 => Cs::_11,
            _ => unreachable!(),
        }
    }
    ///Prohibited
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cs::_00
    }
    ///Cache line size 32 bytes
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cs::_01
    }
    ///Cache line size 64 bytes
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cs::_10
    }
    ///Prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cs::_11
    }
}
///Field `CS` writer - S-Cache Line Size
pub type CsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cs, crate::Safe>;
impl<'a, REG> CsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Prohibited
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::_00)
    }
    ///Cache line size 32 bytes
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::_01)
    }
    ///Cache line size 64 bytes
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::_10)
    }
    ///Prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::_11)
    }
}
impl R {
    ///Bits 0:1 - S-Cache Line Size
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCALCF").field("cs", &self.cs()).finish()
    }
}
impl W {
    ///Bits 0:1 - S-Cache Line Size
    #[inline(always)]
    pub fn cs(&mut self) -> CsW<ScalcfSpec> {
        CsW::new(self, 0)
    }
}
/**S-Cache Line Configuration Register

You can [`read`](crate::Reg::read) this register and get [`scalcf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scalcf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ScalcfSpec;
impl crate::RegisterSpec for ScalcfSpec {
    type Ux = u32;
}
///`read()` method returns [`scalcf::R`](R) reader structure
impl crate::Readable for ScalcfSpec {}
///`write(|w| ..)` method takes [`scalcf::W`](W) writer structure
impl crate::Writable for ScalcfSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCALCF to value 0x01
impl crate::Resettable for ScalcfSpec {
    const RESET_VALUE: u32 = 0x01;
}
