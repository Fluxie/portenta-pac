///Register `DVCHGR` reader
pub type R = crate::R<DvchgrSpec>;
///Register `DVCHGR` writer
pub type W = crate::W<DvchgrSpec>;
/**Device State Change

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dvchg {
    ///0: Disable writes to the USBADDR.STSRECOV\[3:0\] and USBADDR.USBADDR\[6:0\] bits
    _0 = 0,
    ///1: Enable writes to the USBADDR.STSRECOV\[3:0\] and USBADDR.USBADDR\[6:0\] bits
    _1 = 1,
}
impl From<Dvchg> for bool {
    #[inline(always)]
    fn from(variant: Dvchg) -> Self {
        variant as u8 != 0
    }
}
///Field `DVCHG` reader - Device State Change
pub type DvchgR = crate::BitReader<Dvchg>;
impl DvchgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvchg {
        match self.bits {
            false => Dvchg::_0,
            true => Dvchg::_1,
        }
    }
    ///Disable writes to the USBADDR.STSRECOV\[3:0\] and USBADDR.USBADDR\[6:0\] bits
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dvchg::_0
    }
    ///Enable writes to the USBADDR.STSRECOV\[3:0\] and USBADDR.USBADDR\[6:0\] bits
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dvchg::_1
    }
}
///Field `DVCHG` writer - Device State Change
pub type DvchgW<'a, REG> = crate::BitWriter<'a, REG, Dvchg>;
impl<'a, REG> DvchgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable writes to the USBADDR.STSRECOV\[3:0\] and USBADDR.USBADDR\[6:0\] bits
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dvchg::_0)
    }
    ///Enable writes to the USBADDR.STSRECOV\[3:0\] and USBADDR.USBADDR\[6:0\] bits
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dvchg::_1)
    }
}
impl R {
    ///Bit 15 - Device State Change
    #[inline(always)]
    pub fn dvchg(&self) -> DvchgR {
        DvchgR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVCHGR").field("dvchg", &self.dvchg()).finish()
    }
}
impl W {
    ///Bit 15 - Device State Change
    #[inline(always)]
    pub fn dvchg(&mut self) -> DvchgW<DvchgrSpec> {
        DvchgW::new(self, 15)
    }
}
/**Device State Change Register

You can [`read`](crate::Reg::read) this register and get [`dvchgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvchgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DvchgrSpec;
impl crate::RegisterSpec for DvchgrSpec {
    type Ux = u16;
}
///`read()` method returns [`dvchgr::R`](R) reader structure
impl crate::Readable for DvchgrSpec {}
///`write(|w| ..)` method takes [`dvchgr::W`](W) writer structure
impl crate::Writable for DvchgrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DVCHGR to value 0
impl crate::Resettable for DvchgrSpec {}
