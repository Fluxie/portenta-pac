///Register `SIMR1` reader
pub type R = crate::R<Simr1Spec>;
///Register `SIMR1` writer
pub type W = crate::W<Simr1Spec>;
/**Simple IIC Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicm {
    ///0: SCMR.SMIF = 0: Asynchronous mode (including multi-processor mode), clock synchronous mode, or simple SPI mode SCMR.SMIF = 1: Smart card interface mode
    _0 = 0,
    ///1: SCMR.SMIF = 0: Simple IIC mode SCMR.SMIF = 1: Setting prohibited
    _1 = 1,
}
impl From<Iicm> for bool {
    #[inline(always)]
    fn from(variant: Iicm) -> Self {
        variant as u8 != 0
    }
}
///Field `IICM` reader - Simple IIC Mode Select
pub type IicmR = crate::BitReader<Iicm>;
impl IicmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicm {
        match self.bits {
            false => Iicm::_0,
            true => Iicm::_1,
        }
    }
    ///SCMR.SMIF = 0: Asynchronous mode (including multi-processor mode), clock synchronous mode, or simple SPI mode SCMR.SMIF = 1: Smart card interface mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicm::_0
    }
    ///SCMR.SMIF = 0: Simple IIC mode SCMR.SMIF = 1: Setting prohibited
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicm::_1
    }
}
///Field `IICM` writer - Simple IIC Mode Select
pub type IicmW<'a, REG> = crate::BitWriter<'a, REG, Iicm>;
impl<'a, REG> IicmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SCMR.SMIF = 0: Asynchronous mode (including multi-processor mode), clock synchronous mode, or simple SPI mode SCMR.SMIF = 1: Smart card interface mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicm::_0)
    }
    ///SCMR.SMIF = 0: Simple IIC mode SCMR.SMIF = 1: Setting prohibited
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicm::_1)
    }
}
/**SDAn Delay Output Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iicdl {
    ///0: No output delay
    _0x00 = 0,
    ///1: (IICDL - 1) to (IICDL) cycles
    Others = 1,
}
impl From<Iicdl> for u8 {
    #[inline(always)]
    fn from(variant: Iicdl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iicdl {
    type Ux = u8;
}
impl crate::IsEnum for Iicdl {}
///Field `IICDL` reader - SDAn Delay Output Select
pub type IicdlR = crate::FieldReader<Iicdl>;
impl IicdlR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicdl {
        match self.bits {
            0 => Iicdl::_0x00,
            _ => Iicdl::Others,
        }
    }
    ///No output delay
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == Iicdl::_0x00
    }
    ///(IICDL - 1) to (IICDL) cycles
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Iicdl::Others)
    }
}
///Field `IICDL` writer - SDAn Delay Output Select
pub type IicdlW<'a, REG> = crate::FieldWriter<'a, REG, 5, Iicdl, crate::Safe>;
impl<'a, REG> IicdlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No output delay
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Iicdl::_0x00)
    }
    ///(IICDL - 1) to (IICDL) cycles
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Iicdl::Others)
    }
}
impl R {
    ///Bit 0 - Simple IIC Mode Select
    #[inline(always)]
    pub fn iicm(&self) -> IicmR {
        IicmR::new((self.bits & 1) != 0)
    }
    ///Bits 3:7 - SDAn Delay Output Select
    #[inline(always)]
    pub fn iicdl(&self) -> IicdlR {
        IicdlR::new((self.bits >> 3) & 0x1f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIMR1")
            .field("iicm", &self.iicm())
            .field("iicdl", &self.iicdl())
            .finish()
    }
}
impl W {
    ///Bit 0 - Simple IIC Mode Select
    #[inline(always)]
    pub fn iicm(&mut self) -> IicmW<Simr1Spec> {
        IicmW::new(self, 0)
    }
    ///Bits 3:7 - SDAn Delay Output Select
    #[inline(always)]
    pub fn iicdl(&mut self) -> IicdlW<Simr1Spec> {
        IicdlW::new(self, 3)
    }
}
/**IIC Mode Register 1

You can [`read`](crate::Reg::read) this register and get [`simr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Simr1Spec;
impl crate::RegisterSpec for Simr1Spec {
    type Ux = u8;
}
///`read()` method returns [`simr1::R`](R) reader structure
impl crate::Readable for Simr1Spec {}
///`write(|w| ..)` method takes [`simr1::W`](W) writer structure
impl crate::Writable for Simr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SIMR1 to value 0
impl crate::Resettable for Simr1Spec {}
