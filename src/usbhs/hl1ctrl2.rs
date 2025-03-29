///Register `HL1CTRL2` reader
pub type R = crate::R<Hl1ctrl2Spec>;
///Register `HL1CTRL2` writer
pub type W = crate::W<Hl1ctrl2Spec>;
///Field `L1ADDR` reader - LPM Token DeviceAddress
pub type L1addrR = crate::FieldReader;
///Field `L1ADDR` writer - LPM Token DeviceAddress
pub type L1addrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HIRD` reader - LPM Token HIRD
pub type HirdR = crate::FieldReader;
///Field `HIRD` writer - LPM Token HIRD
pub type HirdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `L1RWE` reader - LPM Token L1 RemoteWake Enable
pub type L1rweR = crate::BitReader;
///Field `L1RWE` writer - LPM Token L1 RemoteWake Enable
pub type L1rweW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BESL` reader - BESL & Alternate HIRD
pub type BeslR = crate::BitReader;
///Field `BESL` writer - BESL & Alternate HIRD
pub type BeslW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - LPM Token DeviceAddress
    #[inline(always)]
    pub fn l1addr(&self) -> L1addrR {
        L1addrR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - LPM Token HIRD
    #[inline(always)]
    pub fn hird(&self) -> HirdR {
        HirdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - LPM Token L1 RemoteWake Enable
    #[inline(always)]
    pub fn l1rwe(&self) -> L1rweR {
        L1rweR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - BESL & Alternate HIRD
    #[inline(always)]
    pub fn besl(&self) -> BeslR {
        BeslR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HL1CTRL2")
            .field("l1addr", &self.l1addr())
            .field("hird", &self.hird())
            .field("l1rwe", &self.l1rwe())
            .field("besl", &self.besl())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - LPM Token DeviceAddress
    #[inline(always)]
    pub fn l1addr(&mut self) -> L1addrW<Hl1ctrl2Spec> {
        L1addrW::new(self, 0)
    }
    ///Bits 8:11 - LPM Token HIRD
    #[inline(always)]
    pub fn hird(&mut self) -> HirdW<Hl1ctrl2Spec> {
        HirdW::new(self, 8)
    }
    ///Bit 12 - LPM Token L1 RemoteWake Enable
    #[inline(always)]
    pub fn l1rwe(&mut self) -> L1rweW<Hl1ctrl2Spec> {
        L1rweW::new(self, 12)
    }
    ///Bit 15 - BESL & Alternate HIRD
    #[inline(always)]
    pub fn besl(&mut self) -> BeslW<Hl1ctrl2Spec> {
        BeslW::new(self, 15)
    }
}
/**Host L1 Control Register 2

You can [`read`](crate::Reg::read) this register and get [`hl1ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl1ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Hl1ctrl2Spec;
impl crate::RegisterSpec for Hl1ctrl2Spec {
    type Ux = u16;
}
///`read()` method returns [`hl1ctrl2::R`](R) reader structure
impl crate::Readable for Hl1ctrl2Spec {}
///`write(|w| ..)` method takes [`hl1ctrl2::W`](W) writer structure
impl crate::Writable for Hl1ctrl2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HL1CTRL2 to value 0
impl crate::Resettable for Hl1ctrl2Spec {}
