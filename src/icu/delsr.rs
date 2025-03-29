///Register `DELSR%s` reader
pub type R = crate::R<DelsrSpec>;
///Register `DELSR%s` writer
pub type W = crate::W<DelsrSpec>;
/**DMAC Event Link Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Dels {
    ///0: Disable interrupts to the associated DMAC module.
    _0x00 = 0,
    ///1: Event signal number to be linked. For details, see .
    Others = 1,
}
impl From<Dels> for u16 {
    #[inline(always)]
    fn from(variant: Dels) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dels {
    type Ux = u16;
}
impl crate::IsEnum for Dels {}
///Field `DELS` reader - DMAC Event Link Select
pub type DelsR = crate::FieldReader<Dels>;
impl DelsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dels {
        match self.bits {
            0 => Dels::_0x00,
            _ => Dels::Others,
        }
    }
    ///Disable interrupts to the associated DMAC module.
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == Dels::_0x00
    }
    ///Event signal number to be linked. For details, see .
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Dels::Others)
    }
}
///Field `DELS` writer - DMAC Event Link Select
pub type DelsW<'a, REG> = crate::FieldWriter<'a, REG, 9, Dels, crate::Safe>;
impl<'a, REG> DelsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Disable interrupts to the associated DMAC module.
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Dels::_0x00)
    }
    ///Event signal number to be linked. For details, see .
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Dels::Others)
    }
}
/**DMAC Activation Request Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ir {
    ///0: No DMAC activation request occurred.
    _0 = 0,
    ///1: DMAC activation request occurred.
    _1 = 1,
}
impl From<Ir> for bool {
    #[inline(always)]
    fn from(variant: Ir) -> Self {
        variant as u8 != 0
    }
}
///Field `IR` reader - DMAC Activation Request Status Flag
pub type IrR = crate::BitReader<Ir>;
impl IrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ir {
        match self.bits {
            false => Ir::_0,
            true => Ir::_1,
        }
    }
    ///No DMAC activation request occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ir::_0
    }
    ///DMAC activation request occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ir::_1
    }
}
///Field `IR` writer - DMAC Activation Request Status Flag
pub type IrW<'a, REG> = crate::BitWriter<'a, REG, Ir>;
impl<'a, REG> IrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No DMAC activation request occurred.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ir::_0)
    }
    ///DMAC activation request occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ir::_1)
    }
}
impl R {
    ///Bits 0:8 - DMAC Event Link Select
    #[inline(always)]
    pub fn dels(&self) -> DelsR {
        DelsR::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 16 - DMAC Activation Request Status Flag
    #[inline(always)]
    pub fn ir(&self) -> IrR {
        IrR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DELSR")
            .field("dels", &self.dels())
            .field("ir", &self.ir())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - DMAC Event Link Select
    #[inline(always)]
    pub fn dels(&mut self) -> DelsW<DelsrSpec> {
        DelsW::new(self, 0)
    }
    ///Bit 16 - DMAC Activation Request Status Flag
    #[inline(always)]
    pub fn ir(&mut self) -> IrW<DelsrSpec> {
        IrW::new(self, 16)
    }
}
/**DMAC Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`delsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DelsrSpec;
impl crate::RegisterSpec for DelsrSpec {
    type Ux = u32;
}
///`read()` method returns [`delsr::R`](R) reader structure
impl crate::Readable for DelsrSpec {}
///`write(|w| ..)` method takes [`delsr::W`](W) writer structure
impl crate::Writable for DelsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DELSR%s to value 0
impl crate::Resettable for DelsrSpec {}
