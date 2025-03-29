///Register `CFDCFRISTS` reader
pub type R = crate::R<CfdcfristsSpec>;
///Register `CFDCFRISTS` writer
pub type W = crate::W<CfdcfristsSpec>;
/**Common FIFO RX Interrupt Flag Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfxrxif {
    ///0: Corresponding Common FIFO RX Interrupt flag is not set
    _0 = 0,
    ///1: Corresponding Common FIFO RX Interrupt flag is set
    _1 = 1,
}
impl From<Cfxrxif> for u8 {
    #[inline(always)]
    fn from(variant: Cfxrxif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfxrxif {
    type Ux = u8;
}
impl crate::IsEnum for Cfxrxif {}
///Field `CFXRXIF` reader - Common FIFO RX Interrupt Flag Status
pub type CfxrxifR = crate::FieldReader<Cfxrxif>;
impl CfxrxifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfxrxif> {
        match self.bits {
            0 => Some(Cfxrxif::_0),
            1 => Some(Cfxrxif::_1),
            _ => None,
        }
    }
    ///Corresponding Common FIFO RX Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfxrxif::_0
    }
    ///Corresponding Common FIFO RX Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfxrxif::_1
    }
}
impl R {
    ///Bits 0:5 - Common FIFO RX Interrupt Flag Status
    #[inline(always)]
    pub fn cfxrxif(&self) -> CfxrxifR {
        CfxrxifR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFRISTS").field("cfxrxif", &self.cfxrxif()).finish()
    }
}
impl W {}
/**Common FIFO RX Interrupt Flag Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfrists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfrists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfristsSpec;
impl crate::RegisterSpec for CfdcfristsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfrists::R`](R) reader structure
impl crate::Readable for CfdcfristsSpec {}
///`write(|w| ..)` method takes [`cfdcfrists::W`](W) writer structure
impl crate::Writable for CfdcfristsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCFRISTS to value 0
impl crate::Resettable for CfdcfristsSpec {}
