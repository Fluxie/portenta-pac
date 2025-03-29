///Register `CS%sWCR2` reader
pub type R = crate::R<Cswcr2Spec>;
///Register `CS%sWCR2` writer
pub type W = crate::W<Cswcr2Spec>;
///Field `CSROFF` reader - Read-Access CS Extension Cycle Select
pub type CsroffR = crate::FieldReader;
///Field `CSROFF` writer - Read-Access CS Extension Cycle Select
pub type CsroffW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CSWOFF` reader - Write-Access CS Extension Cycle Select
pub type CswoffR = crate::FieldReader;
///Field `CSWOFF` writer - Write-Access CS Extension Cycle Select
pub type CswoffW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `WDOFF` reader - Write Data Output Extension Cycle Select
pub type WdoffR = crate::FieldReader;
///Field `WDOFF` writer - Write Data Output Extension Cycle Select
pub type WdoffW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AWAIT` reader - Address Cycle Wait Select
pub type AwaitR = crate::FieldReader;
///Field `AWAIT` writer - Address Cycle Wait Select
pub type AwaitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RDON` reader - RD Assert Wait Select
pub type RdonR = crate::FieldReader;
///Field `RDON` writer - RD Assert Wait Select
pub type RdonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `WRON` reader - WR Assert Wait Select
pub type WronR = crate::FieldReader;
///Field `WRON` writer - WR Assert Wait Select
pub type WronW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `WDON` reader - Write Data Output Wait Select
pub type WdonR = crate::FieldReader;
///Field `WDON` writer - Write Data Output Wait Select
pub type WdonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CSON` reader - CS Assert Wait Select
pub type CsonR = crate::FieldReader;
///Field `CSON` writer - CS Assert Wait Select
pub type CsonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Read-Access CS Extension Cycle Select
    #[inline(always)]
    pub fn csroff(&self) -> CsroffR {
        CsroffR::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Write-Access CS Extension Cycle Select
    #[inline(always)]
    pub fn cswoff(&self) -> CswoffR {
        CswoffR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Write Data Output Extension Cycle Select
    #[inline(always)]
    pub fn wdoff(&self) -> WdoffR {
        WdoffR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:13 - Address Cycle Wait Select
    #[inline(always)]
    pub fn await_(&self) -> AwaitR {
        AwaitR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - RD Assert Wait Select
    #[inline(always)]
    pub fn rdon(&self) -> RdonR {
        RdonR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - WR Assert Wait Select
    #[inline(always)]
    pub fn wron(&self) -> WronR {
        WronR::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - Write Data Output Wait Select
    #[inline(always)]
    pub fn wdon(&self) -> WdonR {
        WdonR::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - CS Assert Wait Select
    #[inline(always)]
    pub fn cson(&self) -> CsonR {
        CsonR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSWCR2")
            .field("csroff", &self.csroff())
            .field("cswoff", &self.cswoff())
            .field("wdoff", &self.wdoff())
            .field("await_", &self.await_())
            .field("rdon", &self.rdon())
            .field("wron", &self.wron())
            .field("wdon", &self.wdon())
            .field("cson", &self.cson())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Read-Access CS Extension Cycle Select
    #[inline(always)]
    pub fn csroff(&mut self) -> CsroffW<Cswcr2Spec> {
        CsroffW::new(self, 0)
    }
    ///Bits 4:6 - Write-Access CS Extension Cycle Select
    #[inline(always)]
    pub fn cswoff(&mut self) -> CswoffW<Cswcr2Spec> {
        CswoffW::new(self, 4)
    }
    ///Bits 8:10 - Write Data Output Extension Cycle Select
    #[inline(always)]
    pub fn wdoff(&mut self) -> WdoffW<Cswcr2Spec> {
        WdoffW::new(self, 8)
    }
    ///Bits 12:13 - Address Cycle Wait Select
    #[inline(always)]
    pub fn await_(&mut self) -> AwaitW<Cswcr2Spec> {
        AwaitW::new(self, 12)
    }
    ///Bits 16:18 - RD Assert Wait Select
    #[inline(always)]
    pub fn rdon(&mut self) -> RdonW<Cswcr2Spec> {
        RdonW::new(self, 16)
    }
    ///Bits 20:22 - WR Assert Wait Select
    #[inline(always)]
    pub fn wron(&mut self) -> WronW<Cswcr2Spec> {
        WronW::new(self, 20)
    }
    ///Bits 24:26 - Write Data Output Wait Select
    #[inline(always)]
    pub fn wdon(&mut self) -> WdonW<Cswcr2Spec> {
        WdonW::new(self, 24)
    }
    ///Bits 28:30 - CS Assert Wait Select
    #[inline(always)]
    pub fn cson(&mut self) -> CsonW<Cswcr2Spec> {
        CsonW::new(self, 28)
    }
}
/**CS%s Wait Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cswcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cswcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cswcr2Spec;
impl crate::RegisterSpec for Cswcr2Spec {
    type Ux = u32;
}
///`read()` method returns [`cswcr2::R`](R) reader structure
impl crate::Readable for Cswcr2Spec {}
///`write(|w| ..)` method takes [`cswcr2::W`](W) writer structure
impl crate::Writable for Cswcr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CS%sWCR2 to value 0x07
impl crate::Resettable for Cswcr2Spec {
    const RESET_VALUE: u32 = 0x07;
}
