///Register `ADBUFEN` reader
pub type R = crate::R<AdbufenSpec>;
///Register `ADBUFEN` writer
pub type W = crate::W<AdbufenSpec>;
/**Data Buffer Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufen {
    ///0: The data buffer is not used.
    _0 = 0,
    ///1: The data buffer is used.
    _1 = 1,
}
impl From<Bufen> for bool {
    #[inline(always)]
    fn from(variant: Bufen) -> Self {
        variant as u8 != 0
    }
}
///Field `BUFEN` reader - Data Buffer Enable
pub type BufenR = crate::BitReader<Bufen>;
impl BufenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bufen {
        match self.bits {
            false => Bufen::_0,
            true => Bufen::_1,
        }
    }
    ///The data buffer is not used.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bufen::_0
    }
    ///The data buffer is used.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bufen::_1
    }
}
///Field `BUFEN` writer - Data Buffer Enable
pub type BufenW<'a, REG> = crate::BitWriter<'a, REG, Bufen>;
impl<'a, REG> BufenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The data buffer is not used.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bufen::_0)
    }
    ///The data buffer is used.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bufen::_1)
    }
}
impl R {
    ///Bit 0 - Data Buffer Enable
    #[inline(always)]
    pub fn bufen(&self) -> BufenR {
        BufenR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADBUFEN").field("bufen", &self.bufen()).finish()
    }
}
impl W {
    ///Bit 0 - Data Buffer Enable
    #[inline(always)]
    pub fn bufen(&mut self) -> BufenW<AdbufenSpec> {
        BufenW::new(self, 0)
    }
}
/**A/D Data Buffer Enable Register

You can [`read`](crate::Reg::read) this register and get [`adbufen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adbufen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdbufenSpec;
impl crate::RegisterSpec for AdbufenSpec {
    type Ux = u8;
}
///`read()` method returns [`adbufen::R`](R) reader structure
impl crate::Readable for AdbufenSpec {}
///`write(|w| ..)` method takes [`adbufen::W`](W) writer structure
impl crate::Writable for AdbufenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADBUFEN to value 0
impl crate::Resettable for AdbufenSpec {}
