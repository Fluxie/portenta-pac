///Register `LPCTRL` reader
pub type R = crate::R<LpctrlSpec>;
///Register `LPCTRL` writer
pub type W = crate::W<LpctrlSpec>;
/**Resume Return Mode Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwupm {
    ///0: Hardware does not recover while CPU clock inactive
    _0 = 0,
    ///1: Hardware recovers while CPU clock inactive
    _1 = 1,
}
impl From<Hwupm> for bool {
    #[inline(always)]
    fn from(variant: Hwupm) -> Self {
        variant as u8 != 0
    }
}
///Field `HWUPM` reader - Resume Return Mode Setting
pub type HwupmR = crate::BitReader<Hwupm>;
impl HwupmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Hwupm {
        match self.bits {
            false => Hwupm::_0,
            true => Hwupm::_1,
        }
    }
    ///Hardware does not recover while CPU clock inactive
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hwupm::_0
    }
    ///Hardware recovers while CPU clock inactive
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hwupm::_1
    }
}
///Field `HWUPM` writer - Resume Return Mode Setting
pub type HwupmW<'a, REG> = crate::BitWriter<'a, REG, Hwupm>;
impl<'a, REG> HwupmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hardware does not recover while CPU clock inactive
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hwupm::_0)
    }
    ///Hardware recovers while CPU clock inactive
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hwupm::_1)
    }
}
impl R {
    ///Bit 7 - Resume Return Mode Setting
    #[inline(always)]
    pub fn hwupm(&self) -> HwupmR {
        HwupmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCTRL").field("hwupm", &self.hwupm()).finish()
    }
}
impl W {
    ///Bit 7 - Resume Return Mode Setting
    #[inline(always)]
    pub fn hwupm(&mut self) -> HwupmW<LpctrlSpec> {
        HwupmW::new(self, 7)
    }
}
/**Low Power Control Register

You can [`read`](crate::Reg::read) this register and get [`lpctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpctrlSpec;
impl crate::RegisterSpec for LpctrlSpec {
    type Ux = u16;
}
///`read()` method returns [`lpctrl::R`](R) reader structure
impl crate::Readable for LpctrlSpec {}
///`write(|w| ..)` method takes [`lpctrl::W`](W) writer structure
impl crate::Writable for LpctrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPCTRL to value 0
impl crate::Resettable for LpctrlSpec {}
