///Register `CFDTMIEC%s` reader
pub type R = crate::R<CfdtmiecSpec>;
///Register `CFDTMIEC%s` writer
pub type W = crate::W<CfdtmiecSpec>;
/**TX Message Buffer Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmie {
    ///0: TX message buffer interrupt disabled for corresponding TX message buffer
    _0 = 0,
    ///1: TX message buffer interrupt enabled for corresponding TX message buffer
    _1 = 1,
}
impl From<Tmie> for u8 {
    #[inline(always)]
    fn from(variant: Tmie) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmie {
    type Ux = u8;
}
impl crate::IsEnum for Tmie {}
///Field `TMIE` reader - TX Message Buffer Interrupt Enable
pub type TmieR = crate::FieldReader<Tmie>;
impl TmieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tmie> {
        match self.bits {
            0 => Some(Tmie::_0),
            1 => Some(Tmie::_1),
            _ => None,
        }
    }
    ///TX message buffer interrupt disabled for corresponding TX message buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmie::_0
    }
    ///TX message buffer interrupt enabled for corresponding TX message buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmie::_1
    }
}
///Field `TMIE` writer - TX Message Buffer Interrupt Enable
pub type TmieW<'a, REG> = crate::FieldWriter<'a, REG, 8, Tmie>;
impl<'a, REG> TmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TX message buffer interrupt disabled for corresponding TX message buffer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmie::_0)
    }
    ///TX message buffer interrupt enabled for corresponding TX message buffer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmie::_1)
    }
}
impl R {
    ///Bits 0:7 - TX Message Buffer Interrupt Enable
    #[inline(always)]
    pub fn tmie(&self) -> TmieR {
        TmieR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTMIEC").field("tmie", &self.tmie()).finish()
    }
}
impl W {
    ///Bits 0:7 - TX Message Buffer Interrupt Enable
    #[inline(always)]
    pub fn tmie(&mut self) -> TmieW<CfdtmiecSpec> {
        TmieW::new(self, 0)
    }
}
/**TX Message Buffer Interrupt Enable Configuration Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmiec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmiec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtmiecSpec;
impl crate::RegisterSpec for CfdtmiecSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtmiec::R`](R) reader structure
impl crate::Readable for CfdtmiecSpec {}
///`write(|w| ..)` method takes [`cfdtmiec::W`](W) writer structure
impl crate::Writable for CfdtmiecSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTMIEC%s to value 0
impl crate::Resettable for CfdtmiecSpec {}
