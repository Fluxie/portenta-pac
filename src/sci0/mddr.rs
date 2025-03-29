///Register `MDDR` reader
pub type R = crate::R<MddrSpec>;
///Register `MDDR` writer
pub type W = crate::W<MddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Modulation Duty Register

You can [`read`](crate::Reg::read) this register and get [`mddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MddrSpec;
impl crate::RegisterSpec for MddrSpec {
    type Ux = u8;
}
///`read()` method returns [`mddr::R`](R) reader structure
impl crate::Readable for MddrSpec {}
///`write(|w| ..)` method takes [`mddr::W`](W) writer structure
impl crate::Writable for MddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MDDR to value 0xff
impl crate::Resettable for MddrSpec {
    const RESET_VALUE: u8 = 0xff;
}
