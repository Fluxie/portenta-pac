///Register `LVD1CR1` reader
pub type R = crate::R<Lvd1cr1Spec>;
///Register `LVD1CR1` writer
pub type W = crate::W<Lvd1cr1Spec>;
/**Voltage Monitor 1 Interrupt Generation Condition Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idtsel {
    ///0: When VCC >= Vdet1 (rise) is detected
    _00 = 0,
    ///1: When VCC < Vdet1 (fall) is detected
    _01 = 1,
    ///2: When fall and rise are detected
    _10 = 2,
    ///3: Settings prohibited
    _11 = 3,
}
impl From<Idtsel> for u8 {
    #[inline(always)]
    fn from(variant: Idtsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idtsel {
    type Ux = u8;
}
impl crate::IsEnum for Idtsel {}
///Field `IDTSEL` reader - Voltage Monitor 1 Interrupt Generation Condition Select
pub type IdtselR = crate::FieldReader<Idtsel>;
impl IdtselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Idtsel {
        match self.bits {
            0 => Idtsel::_00,
            1 => Idtsel::_01,
            2 => Idtsel::_10,
            3 => Idtsel::_11,
            _ => unreachable!(),
        }
    }
    ///When VCC >= Vdet1 (rise) is detected
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Idtsel::_00
    }
    ///When VCC < Vdet1 (fall) is detected
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Idtsel::_01
    }
    ///When fall and rise are detected
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Idtsel::_10
    }
    ///Settings prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Idtsel::_11
    }
}
///Field `IDTSEL` writer - Voltage Monitor 1 Interrupt Generation Condition Select
pub type IdtselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Idtsel, crate::Safe>;
impl<'a, REG> IdtselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///When VCC >= Vdet1 (rise) is detected
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Idtsel::_00)
    }
    ///When VCC < Vdet1 (fall) is detected
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Idtsel::_01)
    }
    ///When fall and rise are detected
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Idtsel::_10)
    }
    ///Settings prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Idtsel::_11)
    }
}
/**Voltage Monitor 1 Interrupt Type Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqsel {
    ///0: Non-maskable interrupt
    _0 = 0,
    ///1: Maskable interrupt
    _1 = 1,
}
impl From<Irqsel> for bool {
    #[inline(always)]
    fn from(variant: Irqsel) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQSEL` reader - Voltage Monitor 1 Interrupt Type Select
pub type IrqselR = crate::BitReader<Irqsel>;
impl IrqselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Irqsel {
        match self.bits {
            false => Irqsel::_0,
            true => Irqsel::_1,
        }
    }
    ///Non-maskable interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqsel::_0
    }
    ///Maskable interrupt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqsel::_1
    }
}
///Field `IRQSEL` writer - Voltage Monitor 1 Interrupt Type Select
pub type IrqselW<'a, REG> = crate::BitWriter<'a, REG, Irqsel>;
impl<'a, REG> IrqselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non-maskable interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqsel::_0)
    }
    ///Maskable interrupt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqsel::_1)
    }
}
impl R {
    ///Bits 0:1 - Voltage Monitor 1 Interrupt Generation Condition Select
    #[inline(always)]
    pub fn idtsel(&self) -> IdtselR {
        IdtselR::new(self.bits & 3)
    }
    ///Bit 2 - Voltage Monitor 1 Interrupt Type Select
    #[inline(always)]
    pub fn irqsel(&self) -> IrqselR {
        IrqselR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVD1CR1")
            .field("idtsel", &self.idtsel())
            .field("irqsel", &self.irqsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Voltage Monitor 1 Interrupt Generation Condition Select
    #[inline(always)]
    pub fn idtsel(&mut self) -> IdtselW<Lvd1cr1Spec> {
        IdtselW::new(self, 0)
    }
    ///Bit 2 - Voltage Monitor 1 Interrupt Type Select
    #[inline(always)]
    pub fn irqsel(&mut self) -> IrqselW<Lvd1cr1Spec> {
        IrqselW::new(self, 2)
    }
}
/**Voltage Monitor 1 Circuit Control Register

You can [`read`](crate::Reg::read) this register and get [`lvd1cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Lvd1cr1Spec;
impl crate::RegisterSpec for Lvd1cr1Spec {
    type Ux = u8;
}
///`read()` method returns [`lvd1cr1::R`](R) reader structure
impl crate::Readable for Lvd1cr1Spec {}
///`write(|w| ..)` method takes [`lvd1cr1::W`](W) writer structure
impl crate::Writable for Lvd1cr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVD1CR1 to value 0x01
impl crate::Resettable for Lvd1cr1Spec {
    const RESET_VALUE: u8 = 0x01;
}
