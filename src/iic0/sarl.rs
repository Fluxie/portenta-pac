///Register `SARL%s` reader
pub type R = crate::R<SarlSpec>;
///Register `SARL%s` writer
pub type W = crate::W<SarlSpec>;
///Field `SVA0` reader - 10-bit Address LSB
pub type Sva0R = crate::BitReader;
///Field `SVA0` writer - 10-bit Address LSB
pub type Sva0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SVA` reader - 7-bit Address/10-bit Address Lower Bits
pub type SvaR = crate::FieldReader;
///Field `SVA` writer - 7-bit Address/10-bit Address Lower Bits
pub type SvaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bit 0 - 10-bit Address LSB
    #[inline(always)]
    pub fn sva0(&self) -> Sva0R {
        Sva0R::new((self.bits & 1) != 0)
    }
    ///Bits 1:7 - 7-bit Address/10-bit Address Lower Bits
    #[inline(always)]
    pub fn sva(&self) -> SvaR {
        SvaR::new((self.bits >> 1) & 0x7f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SARL")
            .field("sva0", &self.sva0())
            .field("sva", &self.sva())
            .finish()
    }
}
impl W {
    ///Bit 0 - 10-bit Address LSB
    #[inline(always)]
    pub fn sva0(&mut self) -> Sva0W<SarlSpec> {
        Sva0W::new(self, 0)
    }
    ///Bits 1:7 - 7-bit Address/10-bit Address Lower Bits
    #[inline(always)]
    pub fn sva(&mut self) -> SvaW<SarlSpec> {
        SvaW::new(self, 1)
    }
}
/**Slave Address Register Ly

You can [`read`](crate::Reg::read) this register and get [`sarl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sarl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SarlSpec;
impl crate::RegisterSpec for SarlSpec {
    type Ux = u8;
}
///`read()` method returns [`sarl::R`](R) reader structure
impl crate::Readable for SarlSpec {}
///`write(|w| ..)` method takes [`sarl::W`](W) writer structure
impl crate::Writable for SarlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SARL%s to value 0
impl crate::Resettable for SarlSpec {}
