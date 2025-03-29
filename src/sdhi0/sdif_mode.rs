///Register `SDIF_MODE` reader
pub type R = crate::R<SdifModeSpec>;
///Register `SDIF_MODE` writer
pub type W = crate::W<SdifModeSpec>;
/**CRC Check Mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nochkcr {
    ///0: Enable CRC check
    _0 = 0,
    ///1: Disable CRC Check (ignore CRC16 valued when reading and ignore CRC status value when writing)
    _1 = 1,
}
impl From<Nochkcr> for bool {
    #[inline(always)]
    fn from(variant: Nochkcr) -> Self {
        variant as u8 != 0
    }
}
///Field `NOCHKCR` reader - CRC Check Mask
pub type NochkcrR = crate::BitReader<Nochkcr>;
impl NochkcrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nochkcr {
        match self.bits {
            false => Nochkcr::_0,
            true => Nochkcr::_1,
        }
    }
    ///Enable CRC check
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nochkcr::_0
    }
    ///Disable CRC Check (ignore CRC16 valued when reading and ignore CRC status value when writing)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nochkcr::_1
    }
}
///Field `NOCHKCR` writer - CRC Check Mask
pub type NochkcrW<'a, REG> = crate::BitWriter<'a, REG, Nochkcr>;
impl<'a, REG> NochkcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable CRC check
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nochkcr::_0)
    }
    ///Disable CRC Check (ignore CRC16 valued when reading and ignore CRC status value when writing)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nochkcr::_1)
    }
}
impl R {
    ///Bit 8 - CRC Check Mask
    #[inline(always)]
    pub fn nochkcr(&self) -> NochkcrR {
        NochkcrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIF_MODE").field("nochkcr", &self.nochkcr()).finish()
    }
}
impl W {
    ///Bit 8 - CRC Check Mask
    #[inline(always)]
    pub fn nochkcr(&mut self) -> NochkcrW<SdifModeSpec> {
        NochkcrW::new(self, 8)
    }
}
/**SD Interface Mode Setting Register

You can [`read`](crate::Reg::read) this register and get [`sdif_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdif_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdifModeSpec;
impl crate::RegisterSpec for SdifModeSpec {
    type Ux = u32;
}
///`read()` method returns [`sdif_mode::R`](R) reader structure
impl crate::Readable for SdifModeSpec {}
///`write(|w| ..)` method takes [`sdif_mode::W`](W) writer structure
impl crate::Writable for SdifModeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDIF_MODE to value 0
impl crate::Resettable for SdifModeSpec {}
