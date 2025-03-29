///Register `PIPEBUF` reader
pub type R = crate::R<PipebufSpec>;
///Register `PIPEBUF` writer
pub type W = crate::W<PipebufSpec>;
///Field `BUFNMB` reader - Buffer Number
pub type BufnmbR = crate::FieldReader;
///Field `BUFNMB` writer - Buffer Number
pub type BufnmbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BUFSIZE` reader - Buffer Size
pub type BufsizeR = crate::FieldReader;
///Field `BUFSIZE` writer - Buffer Size
pub type BufsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:7 - Buffer Number
    #[inline(always)]
    pub fn bufnmb(&self) -> BufnmbR {
        BufnmbR::new((self.bits & 0xff) as u8)
    }
    ///Bits 10:14 - Buffer Size
    #[inline(always)]
    pub fn bufsize(&self) -> BufsizeR {
        BufsizeR::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPEBUF")
            .field("bufnmb", &self.bufnmb())
            .field("bufsize", &self.bufsize())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Buffer Number
    #[inline(always)]
    pub fn bufnmb(&mut self) -> BufnmbW<PipebufSpec> {
        BufnmbW::new(self, 0)
    }
    ///Bits 10:14 - Buffer Size
    #[inline(always)]
    pub fn bufsize(&mut self) -> BufsizeW<PipebufSpec> {
        BufsizeW::new(self, 10)
    }
}
/**Pipe Buffer Register

You can [`read`](crate::Reg::read) this register and get [`pipebuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipebuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PipebufSpec;
impl crate::RegisterSpec for PipebufSpec {
    type Ux = u16;
}
///`read()` method returns [`pipebuf::R`](R) reader structure
impl crate::Readable for PipebufSpec {}
///`write(|w| ..)` method takes [`pipebuf::W`](W) writer structure
impl crate::Writable for PipebufSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPEBUF to value 0
impl crate::Resettable for PipebufSpec {}
