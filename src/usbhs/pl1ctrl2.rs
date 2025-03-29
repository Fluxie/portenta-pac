///Register `PL1CTRL2` reader
pub type R = crate::R<Pl1ctrl2Spec>;
///Register `PL1CTRL2` writer
pub type W = crate::W<Pl1ctrl2Spec>;
///Field `HIRDMON` reader - HIRD Value Monitor
pub type HirdmonR = crate::FieldReader;
///Field `RWEMON` reader - RWE Value Monitor
pub type RwemonR = crate::BitReader;
impl R {
    ///Bits 8:11 - HIRD Value Monitor
    #[inline(always)]
    pub fn hirdmon(&self) -> HirdmonR {
        HirdmonR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - RWE Value Monitor
    #[inline(always)]
    pub fn rwemon(&self) -> RwemonR {
        RwemonR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PL1CTRL2")
            .field("hirdmon", &self.hirdmon())
            .field("rwemon", &self.rwemon())
            .finish()
    }
}
impl W {}
/**Function L1 Control Register 2

You can [`read`](crate::Reg::read) this register and get [`pl1ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl1ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Pl1ctrl2Spec;
impl crate::RegisterSpec for Pl1ctrl2Spec {
    type Ux = u16;
}
///`read()` method returns [`pl1ctrl2::R`](R) reader structure
impl crate::Readable for Pl1ctrl2Spec {}
///`write(|w| ..)` method takes [`pl1ctrl2::W`](W) writer structure
impl crate::Writable for Pl1ctrl2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PL1CTRL2 to value 0
impl crate::Resettable for Pl1ctrl2Spec {}
