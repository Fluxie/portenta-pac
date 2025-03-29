///Register `FSADDR` reader
pub type R = crate::R<FsaddrSpec>;
///Register `FSADDR` writer
pub type W = crate::W<FsaddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**FACI Command Start Address Register

You can [`read`](crate::Reg::read) this register and get [`fsaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FsaddrSpec;
impl crate::RegisterSpec for FsaddrSpec {
    type Ux = u32;
}
///`read()` method returns [`fsaddr::R`](R) reader structure
impl crate::Readable for FsaddrSpec {}
///`write(|w| ..)` method takes [`fsaddr::W`](W) writer structure
impl crate::Writable for FsaddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FSADDR to value 0
impl crate::Resettable for FsaddrSpec {}
