///Register `TPAUSECR` reader
pub type R = crate::R<TpausecrSpec>;
///Field `TXP` reader - PAUSE Frame Retransmit Count
pub type TxpR = crate::FieldReader;
impl R {
    ///Bits 0:7 - PAUSE Frame Retransmit Count
    #[inline(always)]
    pub fn txp(&self) -> TxpR {
        TxpR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPAUSECR").field("txp", &self.txp()).finish()
    }
}
/**PAUSE Frame Retransmit Counter

You can [`read`](crate::Reg::read) this register and get [`tpausecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TpausecrSpec;
impl crate::RegisterSpec for TpausecrSpec {
    type Ux = u32;
}
///`read()` method returns [`tpausecr::R`](R) reader structure
impl crate::Readable for TpausecrSpec {}
///`reset()` method sets TPAUSECR to value 0
impl crate::Resettable for TpausecrSpec {}
