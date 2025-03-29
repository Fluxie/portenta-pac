///Register `ICBRH` reader
pub type R = crate::R<IcbrhSpec>;
///Register `ICBRH` writer
pub type W = crate::W<IcbrhSpec>;
///Field `BRH` reader - Bit Rate High-Level Period
pub type BrhR = crate::FieldReader;
///Field `BRH` writer - Bit Rate High-Level Period
pub type BrhW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Bit Rate High-Level Period
    #[inline(always)]
    pub fn brh(&self) -> BrhR {
        BrhR::new(self.bits & 0x1f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICBRH").field("brh", &self.brh()).finish()
    }
}
impl W {
    ///Bits 0:4 - Bit Rate High-Level Period
    #[inline(always)]
    pub fn brh(&mut self) -> BrhW<IcbrhSpec> {
        BrhW::new(self, 0)
    }
}
/**I2C Bus Bit Rate High-Level Register

You can [`read`](crate::Reg::read) this register and get [`icbrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icbrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcbrhSpec;
impl crate::RegisterSpec for IcbrhSpec {
    type Ux = u8;
}
///`read()` method returns [`icbrh::R`](R) reader structure
impl crate::Readable for IcbrhSpec {}
///`write(|w| ..)` method takes [`icbrh::W`](W) writer structure
impl crate::Writable for IcbrhSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICBRH to value 0xff
impl crate::Resettable for IcbrhSpec {
    const RESET_VALUE: u8 = 0xff;
}
