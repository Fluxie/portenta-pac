///Register `IPGR` reader
pub type R = crate::R<IpgrSpec>;
///Register `IPGR` writer
pub type W = crate::W<IpgrSpec>;
///Field `IPG` reader -
pub type IpgR = crate::FieldReader;
///Field `IPG` writer -
pub type IpgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4
    #[inline(always)]
    pub fn ipg(&self) -> IpgR {
        IpgR::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPGR").field("ipg", &self.ipg()).finish()
    }
}
impl W {
    ///Bits 0:4
    #[inline(always)]
    pub fn ipg(&mut self) -> IpgW<IpgrSpec> {
        IpgW::new(self, 0)
    }
}
/**Interpacket Gap Register

You can [`read`](crate::Reg::read) this register and get [`ipgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IpgrSpec;
impl crate::RegisterSpec for IpgrSpec {
    type Ux = u32;
}
///`read()` method returns [`ipgr::R`](R) reader structure
impl crate::Readable for IpgrSpec {}
///`write(|w| ..)` method takes [`ipgr::W`](W) writer structure
impl crate::Writable for IpgrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IPGR to value 0x14
impl crate::Resettable for IpgrSpec {
    const RESET_VALUE: u32 = 0x14;
}
