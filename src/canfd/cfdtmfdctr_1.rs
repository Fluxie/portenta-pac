///Register `CFDTMFDCTR%s_1` reader
pub type R = crate::R<Cfdtmfdctr1Spec>;
///Register `CFDTMFDCTR%s_1` writer
pub type W = crate::W<Cfdtmfdctr1Spec>;
/**Error State Indicator bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmesi {
    ///0: CANFD frame to transmit by error active node
    _0 = 0,
    ///1: CANFD frame to transmit by error passive node
    _1 = 1,
}
impl From<Tmesi> for bool {
    #[inline(always)]
    fn from(variant: Tmesi) -> Self {
        variant as u8 != 0
    }
}
///Field `TMESI` reader - Error State Indicator bit
pub type TmesiR = crate::BitReader<Tmesi>;
impl TmesiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmesi {
        match self.bits {
            false => Tmesi::_0,
            true => Tmesi::_1,
        }
    }
    ///CANFD frame to transmit by error active node
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmesi::_0
    }
    ///CANFD frame to transmit by error passive node
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmesi::_1
    }
}
///Field `TMESI` writer - Error State Indicator bit
pub type TmesiW<'a, REG> = crate::BitWriter<'a, REG, Tmesi>;
impl<'a, REG> TmesiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CANFD frame to transmit by error active node
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmesi::_0)
    }
    ///CANFD frame to transmit by error passive node
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmesi::_1)
    }
}
/**Bit Rate Switch bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmbrs {
    ///0: CANFD frame to transmit with no bit rate switch
    _0 = 0,
    ///1: CANFD frame to transmit with bit rate switch
    _1 = 1,
}
impl From<Tmbrs> for bool {
    #[inline(always)]
    fn from(variant: Tmbrs) -> Self {
        variant as u8 != 0
    }
}
///Field `TMBRS` reader - Bit Rate Switch bit
pub type TmbrsR = crate::BitReader<Tmbrs>;
impl TmbrsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmbrs {
        match self.bits {
            false => Tmbrs::_0,
            true => Tmbrs::_1,
        }
    }
    ///CANFD frame to transmit with no bit rate switch
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmbrs::_0
    }
    ///CANFD frame to transmit with bit rate switch
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmbrs::_1
    }
}
///Field `TMBRS` writer - Bit Rate Switch bit
pub type TmbrsW<'a, REG> = crate::BitWriter<'a, REG, Tmbrs>;
impl<'a, REG> TmbrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CANFD frame to transmit with no bit rate switch
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmbrs::_0)
    }
    ///CANFD frame to transmit with bit rate switch
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmbrs::_1)
    }
}
/**CAN FD Format bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmfdf {
    ///0: Non CANFD frame to transmit
    _0 = 0,
    ///1: CANFD frame to transmit
    _1 = 1,
}
impl From<Tmfdf> for bool {
    #[inline(always)]
    fn from(variant: Tmfdf) -> Self {
        variant as u8 != 0
    }
}
///Field `TMFDF` reader - CAN FD Format bit
pub type TmfdfR = crate::BitReader<Tmfdf>;
impl TmfdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmfdf {
        match self.bits {
            false => Tmfdf::_0,
            true => Tmfdf::_1,
        }
    }
    ///Non CANFD frame to transmit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmfdf::_0
    }
    ///CANFD frame to transmit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmfdf::_1
    }
}
///Field `TMFDF` writer - CAN FD Format bit
pub type TmfdfW<'a, REG> = crate::BitWriter<'a, REG, Tmfdf>;
impl<'a, REG> TmfdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non CANFD frame to transmit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmfdf::_0)
    }
    ///CANFD frame to transmit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmfdf::_1)
    }
}
///Field `TMIFL` reader - TX Message Buffer Information Label Field
pub type TmiflR = crate::FieldReader;
///Field `TMIFL` writer - TX Message Buffer Information Label Field
pub type TmiflW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TMPTR` reader - TX Message Buffer Pointer Field
pub type TmptrR = crate::FieldReader<u16>;
///Field `TMPTR` writer - TX Message Buffer Pointer Field
pub type TmptrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Error State Indicator bit
    #[inline(always)]
    pub fn tmesi(&self) -> TmesiR {
        TmesiR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bit Rate Switch bit
    #[inline(always)]
    pub fn tmbrs(&self) -> TmbrsR {
        TmbrsR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CAN FD Format bit
    #[inline(always)]
    pub fn tmfdf(&self) -> TmfdfR {
        TmfdfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:9 - TX Message Buffer Information Label Field
    #[inline(always)]
    pub fn tmifl(&self) -> TmiflR {
        TmiflR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:31 - TX Message Buffer Pointer Field
    #[inline(always)]
    pub fn tmptr(&self) -> TmptrR {
        TmptrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTMFDCTR_1")
            .field("tmesi", &self.tmesi())
            .field("tmbrs", &self.tmbrs())
            .field("tmfdf", &self.tmfdf())
            .field("tmifl", &self.tmifl())
            .field("tmptr", &self.tmptr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error State Indicator bit
    #[inline(always)]
    pub fn tmesi(&mut self) -> TmesiW<Cfdtmfdctr1Spec> {
        TmesiW::new(self, 0)
    }
    ///Bit 1 - Bit Rate Switch bit
    #[inline(always)]
    pub fn tmbrs(&mut self) -> TmbrsW<Cfdtmfdctr1Spec> {
        TmbrsW::new(self, 1)
    }
    ///Bit 2 - CAN FD Format bit
    #[inline(always)]
    pub fn tmfdf(&mut self) -> TmfdfW<Cfdtmfdctr1Spec> {
        TmfdfW::new(self, 2)
    }
    ///Bits 8:9 - TX Message Buffer Information Label Field
    #[inline(always)]
    pub fn tmifl(&mut self) -> TmiflW<Cfdtmfdctr1Spec> {
        TmiflW::new(self, 8)
    }
    ///Bits 16:31 - TX Message Buffer Pointer Field
    #[inline(always)]
    pub fn tmptr(&mut self) -> TmptrW<Cfdtmfdctr1Spec> {
        TmptrW::new(self, 16)
    }
}
/**TX Message Buffer CANFD Control Register %s Channel i

You can [`read`](crate::Reg::read) this register and get [`cfdtmfdctr_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmfdctr_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdtmfdctr1Spec;
impl crate::RegisterSpec for Cfdtmfdctr1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdtmfdctr_1::R`](R) reader structure
impl crate::Readable for Cfdtmfdctr1Spec {}
///`write(|w| ..)` method takes [`cfdtmfdctr_1::W`](W) writer structure
impl crate::Writable for Cfdtmfdctr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTMFDCTR%s_1 to value 0
impl crate::Resettable for Cfdtmfdctr1Spec {}
