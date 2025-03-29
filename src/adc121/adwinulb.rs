///Register `ADWINULB` reader
pub type R = crate::R<AdwinulbSpec>;
///Register `ADWINULB` writer
pub type W = crate::W<AdwinulbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adwinulb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinulb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdwinulbSpec;
impl crate::RegisterSpec for AdwinulbSpec {
    type Ux = u16;
}
///`read()` method returns [`adwinulb::R`](R) reader structure
impl crate::Readable for AdwinulbSpec {}
///`write(|w| ..)` method takes [`adwinulb::W`](W) writer structure
impl crate::Writable for AdwinulbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADWINULB to value 0
impl crate::Resettable for AdwinulbSpec {}
