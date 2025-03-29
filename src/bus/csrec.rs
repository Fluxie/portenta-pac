///Register `CS%sREC` reader
pub type R = crate::R<CsrecSpec>;
///Register `CS%sREC` writer
pub type W = crate::W<CsrecSpec>;
/**Read Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rrcv {
    ///0: No recovery cycle is inserted.
    _0x0 = 0,
    ///1: RRCV\[3:0\] clock cycles are inserted for read recovery.
    Others = 1,
}
impl From<Rrcv> for u8 {
    #[inline(always)]
    fn from(variant: Rrcv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rrcv {
    type Ux = u8;
}
impl crate::IsEnum for Rrcv {}
///Field `RRCV` reader - Read Recovery
pub type RrcvR = crate::FieldReader<Rrcv>;
impl RrcvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rrcv {
        match self.bits {
            0 => Rrcv::_0x0,
            _ => Rrcv::Others,
        }
    }
    ///No recovery cycle is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Rrcv::_0x0
    }
    ///RRCV\[3:0\] clock cycles are inserted for read recovery.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Rrcv::Others)
    }
}
///Field `RRCV` writer - Read Recovery
pub type RrcvW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rrcv, crate::Safe>;
impl<'a, REG> RrcvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No recovery cycle is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrcv::_0x0)
    }
    ///RRCV\[3:0\] clock cycles are inserted for read recovery.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Rrcv::Others)
    }
}
/**Write Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wrcv {
    ///0: No recovery cycle is inserted.
    _0x0 = 0,
    ///1: WRCV\[3:0\] clock cycles are inserted for write recovery.
    Others = 1,
}
impl From<Wrcv> for u8 {
    #[inline(always)]
    fn from(variant: Wrcv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wrcv {
    type Ux = u8;
}
impl crate::IsEnum for Wrcv {}
///Field `WRCV` reader - Write Recovery
pub type WrcvR = crate::FieldReader<Wrcv>;
impl WrcvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wrcv {
        match self.bits {
            0 => Wrcv::_0x0,
            _ => Wrcv::Others,
        }
    }
    ///No recovery cycle is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Wrcv::_0x0
    }
    ///WRCV\[3:0\] clock cycles are inserted for write recovery.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Wrcv::Others)
    }
}
///Field `WRCV` writer - Write Recovery
pub type WrcvW<'a, REG> = crate::FieldWriter<'a, REG, 4, Wrcv, crate::Safe>;
impl<'a, REG> WrcvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No recovery cycle is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wrcv::_0x0)
    }
    ///WRCV\[3:0\] clock cycles are inserted for write recovery.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Wrcv::Others)
    }
}
impl R {
    ///Bits 0:3 - Read Recovery
    #[inline(always)]
    pub fn rrcv(&self) -> RrcvR {
        RrcvR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - Write Recovery
    #[inline(always)]
    pub fn wrcv(&self) -> WrcvR {
        WrcvR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSREC")
            .field("rrcv", &self.rrcv())
            .field("wrcv", &self.wrcv())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Read Recovery
    #[inline(always)]
    pub fn rrcv(&mut self) -> RrcvW<CsrecSpec> {
        RrcvW::new(self, 0)
    }
    ///Bits 8:11 - Write Recovery
    #[inline(always)]
    pub fn wrcv(&mut self) -> WrcvW<CsrecSpec> {
        WrcvW::new(self, 8)
    }
}
/**CS%s Recovery Cycle Register

You can [`read`](crate::Reg::read) this register and get [`csrec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csrec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CsrecSpec;
impl crate::RegisterSpec for CsrecSpec {
    type Ux = u16;
}
///`read()` method returns [`csrec::R`](R) reader structure
impl crate::Readable for CsrecSpec {}
///`write(|w| ..)` method takes [`csrec::W`](W) writer structure
impl crate::Writable for CsrecSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CS%sREC to value 0
impl crate::Resettable for CsrecSpec {}
