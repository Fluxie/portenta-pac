///Register `FPCKAR` reader
pub type R = crate::R<FpckarSpec>;
///Register `FPCKAR` writer
pub type W = crate::W<FpckarSpec>;
///Field `PCKA` reader - Flash Sequencer Operating Clock Notification
pub type PckaR = crate::FieldReader;
///Field `PCKA` writer - Flash Sequencer Operating Clock Notification
pub type PckaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `KEY` writer - Key Code
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Flash Sequencer Operating Clock Notification
    #[inline(always)]
    pub fn pcka(&self) -> PckaR {
        PckaR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPCKAR").field("pcka", &self.pcka()).finish()
    }
}
impl W {
    ///Bits 0:7 - Flash Sequencer Operating Clock Notification
    #[inline(always)]
    pub fn pcka(&mut self) -> PckaW<FpckarSpec> {
        PckaW::new(self, 0)
    }
    ///Bits 8:15 - Key Code
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<FpckarSpec> {
        KeyW::new(self, 8)
    }
}
/**Flash Sequencer Processing Clock Notification Register

You can [`read`](crate::Reg::read) this register and get [`fpckar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpckar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FpckarSpec;
impl crate::RegisterSpec for FpckarSpec {
    type Ux = u16;
}
///`read()` method returns [`fpckar::R`](R) reader structure
impl crate::Readable for FpckarSpec {}
///`write(|w| ..)` method takes [`fpckar::W`](W) writer structure
impl crate::Writable for FpckarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FPCKAR to value 0x32
impl crate::Resettable for FpckarSpec {
    const RESET_VALUE: u16 = 0x32;
}
