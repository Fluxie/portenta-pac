///Register `CFDTHLACC1%s` reader
pub type R = crate::R<Cfdthlacc1Spec>;
///Register `CFDTHLACC1%s` writer
pub type W = crate::W<Cfdthlacc1Spec>;
///Field `TID` reader - Transmit ID
pub type TidR = crate::FieldReader<u16>;
///Field `TIFL` reader - Transmit Information Label
pub type TiflR = crate::FieldReader;
impl R {
    ///Bits 0:15 - Transmit ID
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:17 - Transmit Information Label
    #[inline(always)]
    pub fn tifl(&self) -> TiflR {
        TiflR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTHLACC1")
            .field("tid", &self.tid())
            .field("tifl", &self.tifl())
            .finish()
    }
}
impl W {}
/**TX History List Access Registers 1

You can [`read`](crate::Reg::read) this register and get [`cfdthlacc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdthlacc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdthlacc1Spec;
impl crate::RegisterSpec for Cfdthlacc1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdthlacc1::R`](R) reader structure
impl crate::Readable for Cfdthlacc1Spec {}
///`write(|w| ..)` method takes [`cfdthlacc1::W`](W) writer structure
impl crate::Writable for Cfdthlacc1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTHLACC1%s to value 0
impl crate::Resettable for Cfdthlacc1Spec {}
