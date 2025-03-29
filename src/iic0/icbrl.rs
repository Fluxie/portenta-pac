///Register `ICBRL` reader
pub type R = crate::R<IcbrlSpec>;
///Register `ICBRL` writer
pub type W = crate::W<IcbrlSpec>;
///Field `BRL` reader - Bit Rate Low-Level Period
pub type BrlR = crate::FieldReader;
///Field `BRL` writer - Bit Rate Low-Level Period
pub type BrlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Bit Rate Low-Level Period
    #[inline(always)]
    pub fn brl(&self) -> BrlR {
        BrlR::new(self.bits & 0x1f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICBRL").field("brl", &self.brl()).finish()
    }
}
impl W {
    ///Bits 0:4 - Bit Rate Low-Level Period
    #[inline(always)]
    pub fn brl(&mut self) -> BrlW<IcbrlSpec> {
        BrlW::new(self, 0)
    }
}
/**I2C Bus Bit Rate Low-Level Register

You can [`read`](crate::Reg::read) this register and get [`icbrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icbrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcbrlSpec;
impl crate::RegisterSpec for IcbrlSpec {
    type Ux = u8;
}
///`read()` method returns [`icbrl::R`](R) reader structure
impl crate::Readable for IcbrlSpec {}
///`write(|w| ..)` method takes [`icbrl::W`](W) writer structure
impl crate::Writable for IcbrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICBRL to value 0xff
impl crate::Resettable for IcbrlSpec {
    const RESET_VALUE: u8 = 0xff;
}
