///Register `CFDGRSTC` reader
pub type R = crate::R<CfdgrstcSpec>;
///Register `CFDGRSTC` writer
pub type W = crate::W<CfdgrstcSpec>;
/**SW Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srst {
    ///0: Normal state
    _0 = 0,
    ///1: SW reset state
    _1 = 1,
}
impl From<Srst> for bool {
    #[inline(always)]
    fn from(variant: Srst) -> Self {
        variant as u8 != 0
    }
}
///Field `SRST` reader - SW Reset
pub type SrstR = crate::BitReader<Srst>;
impl SrstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Srst {
        match self.bits {
            false => Srst::_0,
            true => Srst::_1,
        }
    }
    ///Normal state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Srst::_0
    }
    ///SW reset state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Srst::_1
    }
}
///Field `SRST` writer - SW Reset
pub type SrstW<'a, REG> = crate::BitWriter<'a, REG, Srst>;
impl<'a, REG> SrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Srst::_0)
    }
    ///SW reset state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Srst::_1)
    }
}
///Field `KEY` writer - Key Code
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - SW Reset
    #[inline(always)]
    pub fn srst(&self) -> SrstR {
        SrstR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGRSTC").field("srst", &self.srst()).finish()
    }
}
impl W {
    ///Bit 0 - SW Reset
    #[inline(always)]
    pub fn srst(&mut self) -> SrstW<CfdgrstcSpec> {
        SrstW::new(self, 0)
    }
    ///Bits 8:15 - Key Code
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<CfdgrstcSpec> {
        KeyW::new(self, 8)
    }
}
/**Global SW reset Register

You can [`read`](crate::Reg::read) this register and get [`cfdgrstc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgrstc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgrstcSpec;
impl crate::RegisterSpec for CfdgrstcSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgrstc::R`](R) reader structure
impl crate::Readable for CfdgrstcSpec {}
///`write(|w| ..)` method takes [`cfdgrstc::W`](W) writer structure
impl crate::Writable for CfdgrstcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGRSTC to value 0
impl crate::Resettable for CfdgrstcSpec {}
