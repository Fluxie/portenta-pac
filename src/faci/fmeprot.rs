///Register `FMEPROT` reader
pub type R = crate::R<FmeprotSpec>;
///Register `FMEPROT` writer
pub type W = crate::W<FmeprotSpec>;
/**Code Flash P/E Mode Entry Protection

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceprot {
    ///0: FENTRYC bit is not protected
    _0 = 0,
    ///1: FENTRYC bit is protected.
    _1 = 1,
}
impl From<Ceprot> for bool {
    #[inline(always)]
    fn from(variant: Ceprot) -> Self {
        variant as u8 != 0
    }
}
///Field `CEPROT` reader - Code Flash P/E Mode Entry Protection
pub type CeprotR = crate::BitReader<Ceprot>;
impl CeprotR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ceprot {
        match self.bits {
            false => Ceprot::_0,
            true => Ceprot::_1,
        }
    }
    ///FENTRYC bit is not protected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ceprot::_0
    }
    ///FENTRYC bit is protected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ceprot::_1
    }
}
///Field `CEPROT` writer - Code Flash P/E Mode Entry Protection
pub type CeprotW<'a, REG> = crate::BitWriter<'a, REG, Ceprot>;
impl<'a, REG> CeprotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FENTRYC bit is not protected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceprot::_0)
    }
    ///FENTRYC bit is protected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceprot::_1)
    }
}
///Field `KEY` writer - Key Code
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Code Flash P/E Mode Entry Protection
    #[inline(always)]
    pub fn ceprot(&self) -> CeprotR {
        CeprotR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMEPROT").field("ceprot", &self.ceprot()).finish()
    }
}
impl W {
    ///Bit 0 - Code Flash P/E Mode Entry Protection
    #[inline(always)]
    pub fn ceprot(&mut self) -> CeprotW<FmeprotSpec> {
        CeprotW::new(self, 0)
    }
    ///Bits 8:15 - Key Code
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<FmeprotSpec> {
        KeyW::new(self, 8)
    }
}
/**Flash P/E Mode Entry Protection Register

You can [`read`](crate::Reg::read) this register and get [`fmeprot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmeprot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FmeprotSpec;
impl crate::RegisterSpec for FmeprotSpec {
    type Ux = u16;
}
///`read()` method returns [`fmeprot::R`](R) reader structure
impl crate::Readable for FmeprotSpec {}
///`write(|w| ..)` method takes [`fmeprot::W`](W) writer structure
impl crate::Writable for FmeprotSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMEPROT to value 0x01
impl crate::Resettable for FmeprotSpec {
    const RESET_VALUE: u16 = 0x01;
}
