///Register `TBRAR` reader
pub type R = crate::R<TbrarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Transmit Buffer Read Address Register

You can [`read`](crate::Reg::read) this register and get [`tbrar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TbrarSpec;
impl crate::RegisterSpec for TbrarSpec {
    type Ux = u32;
}
///`read()` method returns [`tbrar::R`](R) reader structure
impl crate::Readable for TbrarSpec {}
///`reset()` method sets TBRAR to value 0
impl crate::Resettable for TbrarSpec {}
