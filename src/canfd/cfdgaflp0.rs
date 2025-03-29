///Register `CFDGAFLP0%s` reader
pub type R = crate::R<Cfdgaflp0Spec>;
///Register `CFDGAFLP0%s` writer
pub type W = crate::W<Cfdgaflp0Spec>;
///Field `GAFLDLC` reader - Global Acceptance Filter List DLC Field
pub type GafldlcR = crate::FieldReader;
///Field `GAFLDLC` writer - Global Acceptance Filter List DLC Field
pub type GafldlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Global Acceptance Filter List Select Routing Destination 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gaflsrd0 {
    ///0: Routing target is CFIFO0
    _0 = 0,
    ///1: Routing target is TX Queue 0 instead of CFIFO0
    _1 = 1,
}
impl From<Gaflsrd0> for bool {
    #[inline(always)]
    fn from(variant: Gaflsrd0) -> Self {
        variant as u8 != 0
    }
}
///Field `GAFLSRD0` reader - Global Acceptance Filter List Select Routing Destination 0
pub type Gaflsrd0R = crate::BitReader<Gaflsrd0>;
impl Gaflsrd0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gaflsrd0 {
        match self.bits {
            false => Gaflsrd0::_0,
            true => Gaflsrd0::_1,
        }
    }
    ///Routing target is CFIFO0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gaflsrd0::_0
    }
    ///Routing target is TX Queue 0 instead of CFIFO0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gaflsrd0::_1
    }
}
///Field `GAFLSRD0` writer - Global Acceptance Filter List Select Routing Destination 0
pub type Gaflsrd0W<'a, REG> = crate::BitWriter<'a, REG, Gaflsrd0>;
impl<'a, REG> Gaflsrd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Routing target is CFIFO0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflsrd0::_0)
    }
    ///Routing target is TX Queue 0 instead of CFIFO0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflsrd0::_1)
    }
}
/**Global Acceptance Filter List Select Routing Destination 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gaflsrd1 {
    ///0: Routing target is CFIFO1
    _0 = 0,
    ///1: Routing target is TX Queue 1 instead of CFIFO1
    _1 = 1,
}
impl From<Gaflsrd1> for bool {
    #[inline(always)]
    fn from(variant: Gaflsrd1) -> Self {
        variant as u8 != 0
    }
}
///Field `GAFLSRD1` reader - Global Acceptance Filter List Select Routing Destination 1
pub type Gaflsrd1R = crate::BitReader<Gaflsrd1>;
impl Gaflsrd1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gaflsrd1 {
        match self.bits {
            false => Gaflsrd1::_0,
            true => Gaflsrd1::_1,
        }
    }
    ///Routing target is CFIFO1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gaflsrd1::_0
    }
    ///Routing target is TX Queue 1 instead of CFIFO1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gaflsrd1::_1
    }
}
///Field `GAFLSRD1` writer - Global Acceptance Filter List Select Routing Destination 1
pub type Gaflsrd1W<'a, REG> = crate::BitWriter<'a, REG, Gaflsrd1>;
impl<'a, REG> Gaflsrd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Routing target is CFIFO1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflsrd1::_0)
    }
    ///Routing target is TX Queue 1 instead of CFIFO1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflsrd1::_1)
    }
}
/**Global Acceptance Filter List Select Routing Destination 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gaflsrd2 {
    ///0: Routing target is CFIFO2
    _0 = 0,
    ///1: Routing target is TX Queue 2 instead of CFIFO2
    _1 = 1,
}
impl From<Gaflsrd2> for bool {
    #[inline(always)]
    fn from(variant: Gaflsrd2) -> Self {
        variant as u8 != 0
    }
}
///Field `GAFLSRD2` reader - Global Acceptance Filter List Select Routing Destination 2
pub type Gaflsrd2R = crate::BitReader<Gaflsrd2>;
impl Gaflsrd2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gaflsrd2 {
        match self.bits {
            false => Gaflsrd2::_0,
            true => Gaflsrd2::_1,
        }
    }
    ///Routing target is CFIFO2
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gaflsrd2::_0
    }
    ///Routing target is TX Queue 2 instead of CFIFO2
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gaflsrd2::_1
    }
}
///Field `GAFLSRD2` writer - Global Acceptance Filter List Select Routing Destination 2
pub type Gaflsrd2W<'a, REG> = crate::BitWriter<'a, REG, Gaflsrd2>;
impl<'a, REG> Gaflsrd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Routing target is CFIFO2
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflsrd2::_0)
    }
    ///Routing target is TX Queue 2 instead of CFIFO2
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflsrd2::_1)
    }
}
///Field `GAFLIFL0` reader - Global Acceptance Filter List Information Label 0
pub type Gaflifl0R = crate::BitReader;
///Field `GAFLIFL0` writer - Global Acceptance Filter List Information Label 0
pub type Gaflifl0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GAFLRMDP` reader - Global Acceptance Filter List RX Message Buffer Direction Pointer
pub type GaflrmdpR = crate::FieldReader;
///Field `GAFLRMDP` writer - Global Acceptance Filter List RX Message Buffer Direction Pointer
pub type GaflrmdpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Global Acceptance Filter List RX Message Buffer Valid

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gaflrmv {
    ///0: Single message buffer direction pointer is invalid
    _0 = 0,
    ///1: Single message buffer direction pointer is valid
    _1 = 1,
}
impl From<Gaflrmv> for bool {
    #[inline(always)]
    fn from(variant: Gaflrmv) -> Self {
        variant as u8 != 0
    }
}
///Field `GAFLRMV` reader - Global Acceptance Filter List RX Message Buffer Valid
pub type GaflrmvR = crate::BitReader<Gaflrmv>;
impl GaflrmvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gaflrmv {
        match self.bits {
            false => Gaflrmv::_0,
            true => Gaflrmv::_1,
        }
    }
    ///Single message buffer direction pointer is invalid
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gaflrmv::_0
    }
    ///Single message buffer direction pointer is valid
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gaflrmv::_1
    }
}
///Field `GAFLRMV` writer - Global Acceptance Filter List RX Message Buffer Valid
pub type GaflrmvW<'a, REG> = crate::BitWriter<'a, REG, Gaflrmv>;
impl<'a, REG> GaflrmvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single message buffer direction pointer is invalid
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflrmv::_0)
    }
    ///Single message buffer direction pointer is valid
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gaflrmv::_1)
    }
}
///Field `GAFLPTR` reader - Global Acceptance Filter List Pointer
pub type GaflptrR = crate::FieldReader<u16>;
///Field `GAFLPTR` writer - Global Acceptance Filter List Pointer
pub type GaflptrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:3 - Global Acceptance Filter List DLC Field
    #[inline(always)]
    pub fn gafldlc(&self) -> GafldlcR {
        GafldlcR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Global Acceptance Filter List Select Routing Destination 0
    #[inline(always)]
    pub fn gaflsrd0(&self) -> Gaflsrd0R {
        Gaflsrd0R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Global Acceptance Filter List Select Routing Destination 1
    #[inline(always)]
    pub fn gaflsrd1(&self) -> Gaflsrd1R {
        Gaflsrd1R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Global Acceptance Filter List Select Routing Destination 2
    #[inline(always)]
    pub fn gaflsrd2(&self) -> Gaflsrd2R {
        Gaflsrd2R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Global Acceptance Filter List Information Label 0
    #[inline(always)]
    pub fn gaflifl0(&self) -> Gaflifl0R {
        Gaflifl0R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:12 - Global Acceptance Filter List RX Message Buffer Direction Pointer
    #[inline(always)]
    pub fn gaflrmdp(&self) -> GaflrmdpR {
        GaflrmdpR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 15 - Global Acceptance Filter List RX Message Buffer Valid
    #[inline(always)]
    pub fn gaflrmv(&self) -> GaflrmvR {
        GaflrmvR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31 - Global Acceptance Filter List Pointer
    #[inline(always)]
    pub fn gaflptr(&self) -> GaflptrR {
        GaflptrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGAFLP0")
            .field("gafldlc", &self.gafldlc())
            .field("gaflsrd0", &self.gaflsrd0())
            .field("gaflsrd1", &self.gaflsrd1())
            .field("gaflsrd2", &self.gaflsrd2())
            .field("gaflifl0", &self.gaflifl0())
            .field("gaflrmdp", &self.gaflrmdp())
            .field("gaflrmv", &self.gaflrmv())
            .field("gaflptr", &self.gaflptr())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Global Acceptance Filter List DLC Field
    #[inline(always)]
    pub fn gafldlc(&mut self) -> GafldlcW<Cfdgaflp0Spec> {
        GafldlcW::new(self, 0)
    }
    ///Bit 4 - Global Acceptance Filter List Select Routing Destination 0
    #[inline(always)]
    pub fn gaflsrd0(&mut self) -> Gaflsrd0W<Cfdgaflp0Spec> {
        Gaflsrd0W::new(self, 4)
    }
    ///Bit 5 - Global Acceptance Filter List Select Routing Destination 1
    #[inline(always)]
    pub fn gaflsrd1(&mut self) -> Gaflsrd1W<Cfdgaflp0Spec> {
        Gaflsrd1W::new(self, 5)
    }
    ///Bit 6 - Global Acceptance Filter List Select Routing Destination 2
    #[inline(always)]
    pub fn gaflsrd2(&mut self) -> Gaflsrd2W<Cfdgaflp0Spec> {
        Gaflsrd2W::new(self, 6)
    }
    ///Bit 7 - Global Acceptance Filter List Information Label 0
    #[inline(always)]
    pub fn gaflifl0(&mut self) -> Gaflifl0W<Cfdgaflp0Spec> {
        Gaflifl0W::new(self, 7)
    }
    ///Bits 8:12 - Global Acceptance Filter List RX Message Buffer Direction Pointer
    #[inline(always)]
    pub fn gaflrmdp(&mut self) -> GaflrmdpW<Cfdgaflp0Spec> {
        GaflrmdpW::new(self, 8)
    }
    ///Bit 15 - Global Acceptance Filter List RX Message Buffer Valid
    #[inline(always)]
    pub fn gaflrmv(&mut self) -> GaflrmvW<Cfdgaflp0Spec> {
        GaflrmvW::new(self, 15)
    }
    ///Bits 16:31 - Global Acceptance Filter List Pointer
    #[inline(always)]
    pub fn gaflptr(&mut self) -> GaflptrW<Cfdgaflp0Spec> {
        GaflptrW::new(self, 16)
    }
}
/**Global Acceptance Filter List Pointer 0 Registers

You can [`read`](crate::Reg::read) this register and get [`cfdgaflp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdgaflp0Spec;
impl crate::RegisterSpec for Cfdgaflp0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdgaflp0::R`](R) reader structure
impl crate::Readable for Cfdgaflp0Spec {}
///`write(|w| ..)` method takes [`cfdgaflp0::W`](W) writer structure
impl crate::Writable for Cfdgaflp0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGAFLP0%s to value 0
impl crate::Resettable for Cfdgaflp0Spec {}
