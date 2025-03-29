///Register `ESR` reader
pub type R = crate::R<EsrSpec>;
///Register `ESR` writer
pub type W = crate::W<EsrSpec>;
/**Memory map read error status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mresr {
    ///1: ECC error
    _0x01 = 1,
    ///2: Preamble error
    _0x02 = 2,
    ///3: Wait OM_DQS timeout
    _0x03 = 3,
    ///128: Invalid command
    _0x80 = 128,
    ///0: Reserved
    Others = 0,
}
impl From<Mresr> for u8 {
    #[inline(always)]
    fn from(variant: Mresr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mresr {
    type Ux = u8;
}
impl crate::IsEnum for Mresr {}
///Field `MRESR` reader - Memory map read error status
pub type MresrR = crate::FieldReader<Mresr>;
impl MresrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mresr {
        match self.bits {
            1 => Mresr::_0x01,
            2 => Mresr::_0x02,
            3 => Mresr::_0x03,
            128 => Mresr::_0x80,
            _ => Mresr::Others,
        }
    }
    ///ECC error
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == Mresr::_0x01
    }
    ///Preamble error
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == Mresr::_0x02
    }
    ///Wait OM_DQS timeout
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == Mresr::_0x03
    }
    ///Invalid command
    #[inline(always)]
    pub fn is_0x80(&self) -> bool {
        *self == Mresr::_0x80
    }
    ///Reserved
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Mresr::Others)
    }
}
///Field `MRESR` writer - Memory map read error status
pub type MresrW<'a, REG> = crate::FieldWriter<'a, REG, 8, Mresr, crate::Safe>;
impl<'a, REG> MresrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ECC error
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Mresr::_0x01)
    }
    ///Preamble error
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Mresr::_0x02)
    }
    ///Wait OM_DQS timeout
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Mresr::_0x03)
    }
    ///Invalid command
    #[inline(always)]
    pub fn _0x80(self) -> &'a mut crate::W<REG> {
        self.variant(Mresr::_0x80)
    }
    ///Reserved
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Mresr::Others)
    }
}
/**Memory map write error status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mwesr {
    ///128: Invalid command
    _0x80 = 128,
    ///0: Reserved
    Others = 0,
}
impl From<Mwesr> for u8 {
    #[inline(always)]
    fn from(variant: Mwesr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mwesr {
    type Ux = u8;
}
impl crate::IsEnum for Mwesr {}
///Field `MWESR` reader - Memory map write error status
pub type MwesrR = crate::FieldReader<Mwesr>;
impl MwesrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mwesr {
        match self.bits {
            128 => Mwesr::_0x80,
            _ => Mwesr::Others,
        }
    }
    ///Invalid command
    #[inline(always)]
    pub fn is_0x80(&self) -> bool {
        *self == Mwesr::_0x80
    }
    ///Reserved
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Mwesr::Others)
    }
}
///Field `MWESR` writer - Memory map write error status
pub type MwesrW<'a, REG> = crate::FieldWriter<'a, REG, 8, Mwesr, crate::Safe>;
impl<'a, REG> MwesrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Invalid command
    #[inline(always)]
    pub fn _0x80(self) -> &'a mut crate::W<REG> {
        self.variant(Mwesr::_0x80)
    }
    ///Reserved
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Mwesr::Others)
    }
}
impl R {
    ///Bits 0:7 - Memory map read error status
    #[inline(always)]
    pub fn mresr(&self) -> MresrR {
        MresrR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Memory map write error status
    #[inline(always)]
    pub fn mwesr(&self) -> MwesrR {
        MwesrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESR")
            .field("mresr", &self.mresr())
            .field("mwesr", &self.mwesr())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Memory map read error status
    #[inline(always)]
    pub fn mresr(&mut self) -> MresrW<EsrSpec> {
        MresrW::new(self, 0)
    }
    ///Bits 8:15 - Memory map write error status
    #[inline(always)]
    pub fn mwesr(&mut self) -> MwesrW<EsrSpec> {
        MwesrW::new(self, 8)
    }
}
/**Error Status Register

You can [`read`](crate::Reg::read) this register and get [`esr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EsrSpec;
impl crate::RegisterSpec for EsrSpec {
    type Ux = u32;
}
///`read()` method returns [`esr::R`](R) reader structure
impl crate::Readable for EsrSpec {}
///`write(|w| ..)` method takes [`esr::W`](W) writer structure
impl crate::Writable for EsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ESR to value 0
impl crate::Resettable for EsrSpec {}
