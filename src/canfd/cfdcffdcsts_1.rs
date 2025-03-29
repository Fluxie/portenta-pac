///Register `CFDCFFDCSTS%s_1` reader
pub type R = crate::R<Cfdcffdcsts1Spec>;
///Register `CFDCFFDCSTS%s_1` writer
pub type W = crate::W<Cfdcffdcsts1Spec>;
/**Error State Indicator bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfesi {
    ///0: CAN-FD frame received or to transmit by error active node
    _0 = 0,
    ///1: CAN-FD frame received or to transmit by error passive node
    _1 = 1,
}
impl From<Cfesi> for bool {
    #[inline(always)]
    fn from(variant: Cfesi) -> Self {
        variant as u8 != 0
    }
}
///Field `CFESI` reader - Error State Indicator bit
pub type CfesiR = crate::BitReader<Cfesi>;
impl CfesiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfesi {
        match self.bits {
            false => Cfesi::_0,
            true => Cfesi::_1,
        }
    }
    ///CAN-FD frame received or to transmit by error active node
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfesi::_0
    }
    ///CAN-FD frame received or to transmit by error passive node
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfesi::_1
    }
}
///Field `CFESI` writer - Error State Indicator bit
pub type CfesiW<'a, REG> = crate::BitWriter<'a, REG, Cfesi>;
impl<'a, REG> CfesiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CAN-FD frame received or to transmit by error active node
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfesi::_0)
    }
    ///CAN-FD frame received or to transmit by error passive node
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfesi::_1)
    }
}
/**Bit Rate Switch bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfbrs {
    ///0: CAN-FD frame received or to transmit with no bit rate switch
    _0 = 0,
    ///1: CAN-FD frame received or to transmit with bit rate switch
    _1 = 1,
}
impl From<Cfbrs> for bool {
    #[inline(always)]
    fn from(variant: Cfbrs) -> Self {
        variant as u8 != 0
    }
}
///Field `CFBRS` reader - Bit Rate Switch bit
pub type CfbrsR = crate::BitReader<Cfbrs>;
impl CfbrsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfbrs {
        match self.bits {
            false => Cfbrs::_0,
            true => Cfbrs::_1,
        }
    }
    ///CAN-FD frame received or to transmit with no bit rate switch
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfbrs::_0
    }
    ///CAN-FD frame received or to transmit with bit rate switch
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfbrs::_1
    }
}
///Field `CFBRS` writer - Bit Rate Switch bit
pub type CfbrsW<'a, REG> = crate::BitWriter<'a, REG, Cfbrs>;
impl<'a, REG> CfbrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CAN-FD frame received or to transmit with no bit rate switch
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbrs::_0)
    }
    ///CAN-FD frame received or to transmit with bit rate switch
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbrs::_1)
    }
}
/**CAN FD Format bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cffdf {
    ///0: Non CAN-FD frame received or to transmit
    _0 = 0,
    ///1: CAN-FD frame received or to transmit
    _1 = 1,
}
impl From<Cffdf> for bool {
    #[inline(always)]
    fn from(variant: Cffdf) -> Self {
        variant as u8 != 0
    }
}
///Field `CFFDF` reader - CAN FD Format bit
pub type CffdfR = crate::BitReader<Cffdf>;
impl CffdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cffdf {
        match self.bits {
            false => Cffdf::_0,
            true => Cffdf::_1,
        }
    }
    ///Non CAN-FD frame received or to transmit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cffdf::_0
    }
    ///CAN-FD frame received or to transmit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cffdf::_1
    }
}
///Field `CFFDF` writer - CAN FD Format bit
pub type CffdfW<'a, REG> = crate::BitWriter<'a, REG, Cffdf>;
impl<'a, REG> CffdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non CAN-FD frame received or to transmit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cffdf::_0)
    }
    ///CAN-FD frame received or to transmit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cffdf::_1)
    }
}
///Field `CFIFL` reader - COMMON FIFO Buffer Information Label Field
pub type CfiflR = crate::FieldReader;
///Field `CFIFL` writer - COMMON FIFO Buffer Information Label Field
pub type CfiflW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CFPTR` reader - Common FIFO Buffer Pointer Field
pub type CfptrR = crate::FieldReader<u16>;
///Field `CFPTR` writer - Common FIFO Buffer Pointer Field
pub type CfptrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Error State Indicator bit
    #[inline(always)]
    pub fn cfesi(&self) -> CfesiR {
        CfesiR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bit Rate Switch bit
    #[inline(always)]
    pub fn cfbrs(&self) -> CfbrsR {
        CfbrsR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CAN FD Format bit
    #[inline(always)]
    pub fn cffdf(&self) -> CffdfR {
        CffdfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:9 - COMMON FIFO Buffer Information Label Field
    #[inline(always)]
    pub fn cfifl(&self) -> CfiflR {
        CfiflR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:31 - Common FIFO Buffer Pointer Field
    #[inline(always)]
    pub fn cfptr(&self) -> CfptrR {
        CfptrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFFDCSTS_1")
            .field("cfesi", &self.cfesi())
            .field("cfbrs", &self.cfbrs())
            .field("cffdf", &self.cffdf())
            .field("cfifl", &self.cfifl())
            .field("cfptr", &self.cfptr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error State Indicator bit
    #[inline(always)]
    pub fn cfesi(&mut self) -> CfesiW<Cfdcffdcsts1Spec> {
        CfesiW::new(self, 0)
    }
    ///Bit 1 - Bit Rate Switch bit
    #[inline(always)]
    pub fn cfbrs(&mut self) -> CfbrsW<Cfdcffdcsts1Spec> {
        CfbrsW::new(self, 1)
    }
    ///Bit 2 - CAN FD Format bit
    #[inline(always)]
    pub fn cffdf(&mut self) -> CffdfW<Cfdcffdcsts1Spec> {
        CffdfW::new(self, 2)
    }
    ///Bits 8:9 - COMMON FIFO Buffer Information Label Field
    #[inline(always)]
    pub fn cfifl(&mut self) -> CfiflW<Cfdcffdcsts1Spec> {
        CfiflW::new(self, 8)
    }
    ///Bits 16:31 - Common FIFO Buffer Pointer Field
    #[inline(always)]
    pub fn cfptr(&mut self) -> CfptrW<Cfdcffdcsts1Spec> {
        CfptrW::new(self, 16)
    }
}
/**Common FIFO Access CAN-FD Control/Status Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdcffdcsts_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcffdcsts_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdcffdcsts1Spec;
impl crate::RegisterSpec for Cfdcffdcsts1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdcffdcsts_1::R`](R) reader structure
impl crate::Readable for Cfdcffdcsts1Spec {}
///`write(|w| ..)` method takes [`cfdcffdcsts_1::W`](W) writer structure
impl crate::Writable for Cfdcffdcsts1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCFFDCSTS%s_1 to value 0
impl crate::Resettable for Cfdcffdcsts1Spec {}
