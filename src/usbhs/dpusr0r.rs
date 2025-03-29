///Register `DPUSR0R` reader
pub type R = crate::R<Dpusr0rSpec>;
///Field `DOVCAHM` reader - OVRCURA Input Flag
pub type DovcahmR = crate::BitReader;
///Field `DOVCBHM` reader - OVRCURB Input Flag
pub type DovcbhmR = crate::BitReader;
///Field `DVBSTSHM` reader - VBUS Input Flag
pub type DvbstshmR = crate::BitReader;
impl R {
    ///Bit 20 - OVRCURA Input Flag
    #[inline(always)]
    pub fn dovcahm(&self) -> DovcahmR {
        DovcahmR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OVRCURB Input Flag
    #[inline(always)]
    pub fn dovcbhm(&self) -> DovcbhmR {
        DovcbhmR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - VBUS Input Flag
    #[inline(always)]
    pub fn dvbstshm(&self) -> DvbstshmR {
        DvbstshmR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPUSR0R")
            .field("dovcahm", &self.dovcahm())
            .field("dovcbhm", &self.dovcbhm())
            .field("dvbstshm", &self.dvbstshm())
            .finish()
    }
}
/**Deep Software Standby USB Transceiver Control/Pin Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dpusr0r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpusr0rSpec;
impl crate::RegisterSpec for Dpusr0rSpec {
    type Ux = u32;
}
///`read()` method returns [`dpusr0r::R`](R) reader structure
impl crate::Readable for Dpusr0rSpec {}
///`reset()` method sets DPUSR0R to value 0
impl crate::Resettable for Dpusr0rSpec {}
