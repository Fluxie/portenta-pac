///Register `CFDCFTISTS` reader
pub type R = crate::R<CfdcftistsSpec>;
///Register `CFDCFTISTS` writer
pub type W = crate::W<CfdcftistsSpec>;
/**Common FIFO \[x\] TX Interrupt Flag Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfxtxif {
    ///0: Corresponding Common FIFO TX Interrupt flag is not set
    _0 = 0,
    ///1: Corresponding Common FIFO TX Interrupt flag is set
    _1 = 1,
}
impl From<Cfxtxif> for u8 {
    #[inline(always)]
    fn from(variant: Cfxtxif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfxtxif {
    type Ux = u8;
}
impl crate::IsEnum for Cfxtxif {}
///Field `CFXTXIF` reader - Common FIFO \[x\] TX Interrupt Flag Status
pub type CfxtxifR = crate::FieldReader<Cfxtxif>;
impl CfxtxifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfxtxif> {
        match self.bits {
            0 => Some(Cfxtxif::_0),
            1 => Some(Cfxtxif::_1),
            _ => None,
        }
    }
    ///Corresponding Common FIFO TX Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfxtxif::_0
    }
    ///Corresponding Common FIFO TX Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfxtxif::_1
    }
}
impl R {
    ///Bits 0:5 - Common FIFO \[x\] TX Interrupt Flag Status
    #[inline(always)]
    pub fn cfxtxif(&self) -> CfxtxifR {
        CfxtxifR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFTISTS").field("cfxtxif", &self.cfxtxif()).finish()
    }
}
impl W {}
/**Common FIFO TX Interrupt Flag Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcftists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcftists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcftistsSpec;
impl crate::RegisterSpec for CfdcftistsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcftists::R`](R) reader structure
impl crate::Readable for CfdcftistsSpec {}
///`write(|w| ..)` method takes [`cfdcftists::W`](W) writer structure
impl crate::Writable for CfdcftistsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCFTISTS to value 0
impl crate::Resettable for CfdcftistsSpec {}
