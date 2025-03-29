///Register `CFDC%sFDCRC` reader
pub type R = crate::R<CfdcfdcrcSpec>;
///Register `CFDC%sFDCRC` writer
pub type W = crate::W<CfdcfdcrcSpec>;
///Field `CRCREG` reader - CRC Register value
pub type CrcregR = crate::FieldReader<u32>;
///Field `SCNT` reader - Stuff bit count
pub type ScntR = crate::FieldReader;
impl R {
    ///Bits 0:20 - CRC Register value
    #[inline(always)]
    pub fn crcreg(&self) -> CrcregR {
        CrcregR::new(self.bits & 0x001f_ffff)
    }
    ///Bits 24:27 - Stuff bit count
    #[inline(always)]
    pub fn scnt(&self) -> ScntR {
        ScntR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFDCRC")
            .field("crcreg", &self.crcreg())
            .field("scnt", &self.scnt())
            .finish()
    }
}
impl W {}
/**Channel %s CANFD CRC Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfdcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfdcrcSpec;
impl crate::RegisterSpec for CfdcfdcrcSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfdcrc::R`](R) reader structure
impl crate::Readable for CfdcfdcrcSpec {}
///`write(|w| ..)` method takes [`cfdcfdcrc::W`](W) writer structure
impl crate::Writable for CfdcfdcrcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDC%sFDCRC to value 0
impl crate::Resettable for CfdcfdcrcSpec {}
