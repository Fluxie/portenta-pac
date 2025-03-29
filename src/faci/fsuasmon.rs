///Register `FSUASMON` reader
pub type R = crate::R<FsuasmonSpec>;
/**Protection Programming Flag to set Boot Flag and Startup Area Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fspr {
    ///0: Protected state
    _0 = 0,
    ///1: Non-protected state.
    _1 = 1,
}
impl From<Fspr> for bool {
    #[inline(always)]
    fn from(variant: Fspr) -> Self {
        variant as u8 != 0
    }
}
///Field `FSPR` reader - Protection Programming Flag to set Boot Flag and Startup Area Control
pub type FsprR = crate::BitReader<Fspr>;
impl FsprR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fspr {
        match self.bits {
            false => Fspr::_0,
            true => Fspr::_1,
        }
    }
    ///Protected state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fspr::_0
    }
    ///Non-protected state.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fspr::_1
    }
}
/**Flag of Startup Area Select for Boot Swap

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Btflg {
    ///0: The startup area is the alternate block (block 1)
    _0 = 0,
    ///1: The startup area is the default block (block 0).
    _1 = 1,
}
impl From<Btflg> for bool {
    #[inline(always)]
    fn from(variant: Btflg) -> Self {
        variant as u8 != 0
    }
}
///Field `BTFLG` reader - Flag of Startup Area Select for Boot Swap
pub type BtflgR = crate::BitReader<Btflg>;
impl BtflgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Btflg {
        match self.bits {
            false => Btflg::_0,
            true => Btflg::_1,
        }
    }
    ///The startup area is the alternate block (block 1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Btflg::_0
    }
    ///The startup area is the default block (block 0).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Btflg::_1
    }
}
impl R {
    ///Bit 15 - Protection Programming Flag to set Boot Flag and Startup Area Control
    #[inline(always)]
    pub fn fspr(&self) -> FsprR {
        FsprR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 31 - Flag of Startup Area Select for Boot Swap
    #[inline(always)]
    pub fn btflg(&self) -> BtflgR {
        BtflgR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSUASMON")
            .field("fspr", &self.fspr())
            .field("btflg", &self.btflg())
            .finish()
    }
}
/**Flash Startup Area Select Monitor Register

You can [`read`](crate::Reg::read) this register and get [`fsuasmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FsuasmonSpec;
impl crate::RegisterSpec for FsuasmonSpec {
    type Ux = u32;
}
///`read()` method returns [`fsuasmon::R`](R) reader structure
impl crate::Readable for FsuasmonSpec {}
///`reset()` method sets FSUASMON to value 0
impl crate::Resettable for FsuasmonSpec {}
