///Register `SCF1DR` reader
pub type R = crate::R<Scf1drSpec>;
///Register `SCF1DR` writer
pub type W = crate::W<Scf1drSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Secondary Control Field 1 Data Register

You can [`read`](crate::Reg::read) this register and get [`scf1dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scf1dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Scf1drSpec;
impl crate::RegisterSpec for Scf1drSpec {
    type Ux = u8;
}
///`read()` method returns [`scf1dr::R`](R) reader structure
impl crate::Readable for Scf1drSpec {}
///`write(|w| ..)` method takes [`scf1dr::W`](W) writer structure
impl crate::Writable for Scf1drSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCF1DR to value 0
impl crate::Resettable for Scf1drSpec {}
