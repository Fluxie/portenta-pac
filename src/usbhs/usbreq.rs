///Register `USBREQ` reader
pub type R = crate::R<UsbreqSpec>;
///Register `USBREQ` writer
pub type W = crate::W<UsbreqSpec>;
///Field `BMREQUESTTYPE` reader - USB request bmRequestType value
pub type BmrequesttypeR = crate::FieldReader;
///Field `BMREQUESTTYPE` writer - USB request bmRequestType value
pub type BmrequesttypeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BREQUEST` reader - USB request bRequest value
pub type BrequestR = crate::FieldReader;
///Field `BREQUEST` writer - USB request bRequest value
pub type BrequestW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - USB request bmRequestType value
    #[inline(always)]
    pub fn bmrequesttype(&self) -> BmrequesttypeR {
        BmrequesttypeR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - USB request bRequest value
    #[inline(always)]
    pub fn brequest(&self) -> BrequestR {
        BrequestR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBREQ")
            .field("bmrequesttype", &self.bmrequesttype())
            .field("brequest", &self.brequest())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - USB request bmRequestType value
    #[inline(always)]
    pub fn bmrequesttype(&mut self) -> BmrequesttypeW<UsbreqSpec> {
        BmrequesttypeW::new(self, 0)
    }
    ///Bits 8:15 - USB request bRequest value
    #[inline(always)]
    pub fn brequest(&mut self) -> BrequestW<UsbreqSpec> {
        BrequestW::new(self, 8)
    }
}
/**USB Request Type Register

You can [`read`](crate::Reg::read) this register and get [`usbreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UsbreqSpec;
impl crate::RegisterSpec for UsbreqSpec {
    type Ux = u16;
}
///`read()` method returns [`usbreq::R`](R) reader structure
impl crate::Readable for UsbreqSpec {}
///`write(|w| ..)` method takes [`usbreq::W`](W) writer structure
impl crate::Writable for UsbreqSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBREQ to value 0
impl crate::Resettable for UsbreqSpec {}
