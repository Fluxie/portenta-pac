///Register `FCKMHZ` reader
pub type R = crate::R<FckmhzSpec>;
///Register `FCKMHZ` writer
pub type W = crate::W<FckmhzSpec>;
///Field `FCKMHZ` reader - Data Flash Access Frequency Register
pub type FckmhzR = crate::FieldReader;
///Field `FCKMHZ` writer - Data Flash Access Frequency Register
pub type FckmhzW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data Flash Access Frequency Register
    #[inline(always)]
    pub fn fckmhz(&self) -> FckmhzR {
        FckmhzR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCKMHZ").field("fckmhz", &self.fckmhz()).finish()
    }
}
impl W {
    ///Bits 0:7 - Data Flash Access Frequency Register
    #[inline(always)]
    pub fn fckmhz(&mut self) -> FckmhzW<FckmhzSpec> {
        FckmhzW::new(self, 0)
    }
}
/**Data Flash Access Frequency Register

You can [`read`](crate::Reg::read) this register and get [`fckmhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fckmhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FckmhzSpec;
impl crate::RegisterSpec for FckmhzSpec {
    type Ux = u8;
}
///`read()` method returns [`fckmhz::R`](R) reader structure
impl crate::Readable for FckmhzSpec {}
///`write(|w| ..)` method takes [`fckmhz::W`](W) writer structure
impl crate::Writable for FckmhzSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCKMHZ to value 0x3c
impl crate::Resettable for FckmhzSpec {
    const RESET_VALUE: u8 = 0x3c;
}
