///Register `SISR` reader
pub type R = crate::R<SisrSpec>;
/**ACK Reception Data Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicackr {
    ///0: ACK received
    _0 = 0,
    ///1: NACK received
    _1 = 1,
}
impl From<Iicackr> for bool {
    #[inline(always)]
    fn from(variant: Iicackr) -> Self {
        variant as u8 != 0
    }
}
///Field `IICACKR` reader - ACK Reception Data Flag
pub type IicackrR = crate::BitReader<Iicackr>;
impl IicackrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicackr {
        match self.bits {
            false => Iicackr::_0,
            true => Iicackr::_1,
        }
    }
    ///ACK received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicackr::_0
    }
    ///NACK received
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicackr::_1
    }
}
impl R {
    ///Bit 0 - ACK Reception Data Flag
    #[inline(always)]
    pub fn iicackr(&self) -> IicackrR {
        IicackrR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SISR").field("iicackr", &self.iicackr()).finish()
    }
}
/**IIC Status Register

You can [`read`](crate::Reg::read) this register and get [`sisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SisrSpec;
impl crate::RegisterSpec for SisrSpec {
    type Ux = u8;
}
///`read()` method returns [`sisr::R`](R) reader structure
impl crate::Readable for SisrSpec {}
///`reset()` method sets SISR to value 0
impl crate::Resettable for SisrSpec {}
