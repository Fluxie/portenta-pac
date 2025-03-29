///Register `WDTSR` reader
pub type R = crate::R<WdtsrSpec>;
///Register `WDTSR` writer
pub type W = crate::W<WdtsrSpec>;
///Field `CNTVAL` reader - Down-Counter Value
pub type CntvalR = crate::FieldReader<u16>;
/**Underflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Undff {
    ///0: No underflow occurred
    _0 = 0,
    ///1: Underflow occurred
    _1 = 1,
}
impl From<Undff> for bool {
    #[inline(always)]
    fn from(variant: Undff) -> Self {
        variant as u8 != 0
    }
}
///Field `UNDFF` reader - Underflow Flag
pub type UndffR = crate::BitReader<Undff>;
impl UndffR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Undff {
        match self.bits {
            false => Undff::_0,
            true => Undff::_1,
        }
    }
    ///No underflow occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Undff::_0
    }
    ///Underflow occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Undff::_1
    }
}
///Field `UNDFF` writer - Underflow Flag
pub type UndffW<'a, REG> = crate::BitWriter<'a, REG, Undff>;
impl<'a, REG> UndffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No underflow occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Undff::_0)
    }
    ///Underflow occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Undff::_1)
    }
}
/**Refresh Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refef {
    ///0: No refresh error occurred
    _0 = 0,
    ///1: Refresh error occurred
    _1 = 1,
}
impl From<Refef> for bool {
    #[inline(always)]
    fn from(variant: Refef) -> Self {
        variant as u8 != 0
    }
}
///Field `REFEF` reader - Refresh Error Flag
pub type RefefR = crate::BitReader<Refef>;
impl RefefR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Refef {
        match self.bits {
            false => Refef::_0,
            true => Refef::_1,
        }
    }
    ///No refresh error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Refef::_0
    }
    ///Refresh error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Refef::_1
    }
}
///Field `REFEF` writer - Refresh Error Flag
pub type RefefW<'a, REG> = crate::BitWriter<'a, REG, Refef>;
impl<'a, REG> RefefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No refresh error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Refef::_0)
    }
    ///Refresh error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Refef::_1)
    }
}
impl R {
    ///Bits 0:13 - Down-Counter Value
    #[inline(always)]
    pub fn cntval(&self) -> CntvalR {
        CntvalR::new(self.bits & 0x3fff)
    }
    ///Bit 14 - Underflow Flag
    #[inline(always)]
    pub fn undff(&self) -> UndffR {
        UndffR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Refresh Error Flag
    #[inline(always)]
    pub fn refef(&self) -> RefefR {
        RefefR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTSR")
            .field("cntval", &self.cntval())
            .field("undff", &self.undff())
            .field("refef", &self.refef())
            .finish()
    }
}
impl W {
    ///Bit 14 - Underflow Flag
    #[inline(always)]
    pub fn undff(&mut self) -> UndffW<WdtsrSpec> {
        UndffW::new(self, 14)
    }
    ///Bit 15 - Refresh Error Flag
    #[inline(always)]
    pub fn refef(&mut self) -> RefefW<WdtsrSpec> {
        RefefW::new(self, 15)
    }
}
/**WDT Status Register

You can [`read`](crate::Reg::read) this register and get [`wdtsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdtsrSpec;
impl crate::RegisterSpec for WdtsrSpec {
    type Ux = u16;
}
///`read()` method returns [`wdtsr::R`](R) reader structure
impl crate::Readable for WdtsrSpec {}
///`write(|w| ..)` method takes [`wdtsr::W`](W) writer structure
impl crate::Writable for WdtsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDTSR to value 0
impl crate::Resettable for WdtsrSpec {}
