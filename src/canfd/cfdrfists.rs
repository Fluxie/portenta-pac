///Register `CFDRFISTS` reader
pub type R = crate::R<CfdrfistsSpec>;
///Register `CFDRFISTS` writer
pub type W = crate::W<CfdrfistsSpec>;
/**RX FIFO\[x\] Interrupt Flag Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfxif {
    ///0: Corresponding RX FIFO Interrupt flag not set
    _0 = 0,
    ///1: Corresponding RX FIFO Interrupt flag set
    _1 = 1,
}
impl From<Rfxif> for u8 {
    #[inline(always)]
    fn from(variant: Rfxif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfxif {
    type Ux = u8;
}
impl crate::IsEnum for Rfxif {}
///Field `RFXIF` reader - RX FIFO\[x\] Interrupt Flag Status
pub type RfxifR = crate::FieldReader<Rfxif>;
impl RfxifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rfxif> {
        match self.bits {
            0 => Some(Rfxif::_0),
            1 => Some(Rfxif::_1),
            _ => None,
        }
    }
    ///Corresponding RX FIFO Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfxif::_0
    }
    ///Corresponding RX FIFO Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfxif::_1
    }
}
/**RX FIFO\[x\] Interrupt Full Flag Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfxffll {
    ///0: Corresponding RX FIFO Interrupt Full flag not set
    _0 = 0,
    ///1: Corresponding RX FIFO Interrupt Full flag set
    _1 = 1,
}
impl From<Rfxffll> for u8 {
    #[inline(always)]
    fn from(variant: Rfxffll) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfxffll {
    type Ux = u8;
}
impl crate::IsEnum for Rfxffll {}
///Field `RFXFFLL` reader - RX FIFO\[x\] Interrupt Full Flag Status
pub type RfxffllR = crate::FieldReader<Rfxffll>;
impl RfxffllR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rfxffll> {
        match self.bits {
            0 => Some(Rfxffll::_0),
            1 => Some(Rfxffll::_1),
            _ => None,
        }
    }
    ///Corresponding RX FIFO Interrupt Full flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfxffll::_0
    }
    ///Corresponding RX FIFO Interrupt Full flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfxffll::_1
    }
}
impl R {
    ///Bits 0:7 - RX FIFO\[x\] Interrupt Flag Status
    #[inline(always)]
    pub fn rfxif(&self) -> RfxifR {
        RfxifR::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - RX FIFO\[x\] Interrupt Full Flag Status
    #[inline(always)]
    pub fn rfxffll(&self) -> RfxffllR {
        RfxffllR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRFISTS")
            .field("rfxif", &self.rfxif())
            .field("rfxffll", &self.rfxffll())
            .finish()
    }
}
impl W {}
/**RX FIFO Interrupt Flag Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdrfists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrfists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdrfistsSpec;
impl crate::RegisterSpec for CfdrfistsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdrfists::R`](R) reader structure
impl crate::Readable for CfdrfistsSpec {}
///`write(|w| ..)` method takes [`cfdrfists::W`](W) writer structure
impl crate::Writable for CfdrfistsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDRFISTS to value 0
impl crate::Resettable for CfdrfistsSpec {}
