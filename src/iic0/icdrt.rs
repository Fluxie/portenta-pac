///Register `ICDRT` reader
pub type R = crate::R<IcdrtSpec>;
///Register `ICDRT` writer
pub type W = crate::W<IcdrtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**I2C Bus Transmit Data Register

You can [`read`](crate::Reg::read) this register and get [`icdrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icdrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcdrtSpec;
impl crate::RegisterSpec for IcdrtSpec {
    type Ux = u8;
}
///`read()` method returns [`icdrt::R`](R) reader structure
impl crate::Readable for IcdrtSpec {}
///`write(|w| ..)` method takes [`icdrt::W`](W) writer structure
impl crate::Writable for IcdrtSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICDRT to value 0xff
impl crate::Resettable for IcdrtSpec {
    const RESET_VALUE: u8 = 0xff;
}
