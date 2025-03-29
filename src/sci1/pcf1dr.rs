///Register `PCF1DR` reader
pub type R = crate::R<Pcf1drSpec>;
///Register `PCF1DR` writer
pub type W = crate::W<Pcf1drSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Primary Control Field 1 Data Register

You can [`read`](crate::Reg::read) this register and get [`pcf1dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcf1dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Pcf1drSpec;
impl crate::RegisterSpec for Pcf1drSpec {
    type Ux = u8;
}
///`read()` method returns [`pcf1dr::R`](R) reader structure
impl crate::Readable for Pcf1drSpec {}
///`write(|w| ..)` method takes [`pcf1dr::W`](W) writer structure
impl crate::Writable for Pcf1drSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCF1DR to value 0
impl crate::Resettable for Pcf1drSpec {}
