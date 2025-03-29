///Register `OPCCR` reader
pub type R = crate::R<OpccrSpec>;
///Register `OPCCR` writer
pub type W = crate::W<OpccrSpec>;
/**Operating Power Control Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcm {
    ///0: High-speed mode
    _00 = 0,
    ///1: Setting prohibited
    _01 = 1,
    ///2: Setting prohibited
    _10 = 2,
    ///3: Low-speed mode
    _11 = 3,
}
impl From<Opcm> for u8 {
    #[inline(always)]
    fn from(variant: Opcm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opcm {
    type Ux = u8;
}
impl crate::IsEnum for Opcm {}
///Field `OPCM` reader - Operating Power Control Mode Select
pub type OpcmR = crate::FieldReader<Opcm>;
impl OpcmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Opcm {
        match self.bits {
            0 => Opcm::_00,
            1 => Opcm::_01,
            2 => Opcm::_10,
            3 => Opcm::_11,
            _ => unreachable!(),
        }
    }
    ///High-speed mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Opcm::_00
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Opcm::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Opcm::_10
    }
    ///Low-speed mode
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Opcm::_11
    }
}
///Field `OPCM` writer - Operating Power Control Mode Select
pub type OpcmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Opcm, crate::Safe>;
impl<'a, REG> OpcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///High-speed mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Opcm::_00)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Opcm::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Opcm::_10)
    }
    ///Low-speed mode
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Opcm::_11)
    }
}
/**Operating Power Control Mode Transition Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcmtsf {
    ///0: Transition completed
    _0 = 0,
    ///1: During transition
    _1 = 1,
}
impl From<Opcmtsf> for bool {
    #[inline(always)]
    fn from(variant: Opcmtsf) -> Self {
        variant as u8 != 0
    }
}
///Field `OPCMTSF` reader - Operating Power Control Mode Transition Status Flag
pub type OpcmtsfR = crate::BitReader<Opcmtsf>;
impl OpcmtsfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Opcmtsf {
        match self.bits {
            false => Opcmtsf::_0,
            true => Opcmtsf::_1,
        }
    }
    ///Transition completed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Opcmtsf::_0
    }
    ///During transition
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Opcmtsf::_1
    }
}
impl R {
    ///Bits 0:1 - Operating Power Control Mode Select
    #[inline(always)]
    pub fn opcm(&self) -> OpcmR {
        OpcmR::new(self.bits & 3)
    }
    ///Bit 4 - Operating Power Control Mode Transition Status Flag
    #[inline(always)]
    pub fn opcmtsf(&self) -> OpcmtsfR {
        OpcmtsfR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPCCR")
            .field("opcm", &self.opcm())
            .field("opcmtsf", &self.opcmtsf())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Operating Power Control Mode Select
    #[inline(always)]
    pub fn opcm(&mut self) -> OpcmW<OpccrSpec> {
        OpcmW::new(self, 0)
    }
}
/**Operating Power Control Register

You can [`read`](crate::Reg::read) this register and get [`opccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OpccrSpec;
impl crate::RegisterSpec for OpccrSpec {
    type Ux = u8;
}
///`read()` method returns [`opccr::R`](R) reader structure
impl crate::Readable for OpccrSpec {}
///`write(|w| ..)` method takes [`opccr::W`](W) writer structure
impl crate::Writable for OpccrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPCCR to value 0
impl crate::Resettable for OpccrSpec {}
