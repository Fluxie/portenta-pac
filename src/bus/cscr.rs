///Register `CS%sCR` reader
pub type R = crate::R<CscrSpec>;
///Register `CS%sCR` writer
pub type W = crate::W<CscrSpec>;
/**Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exenb {
    ///0: Operation is disabled.
    _0 = 0,
    ///1: Operation is enabled.
    _1 = 1,
}
impl From<Exenb> for bool {
    #[inline(always)]
    fn from(variant: Exenb) -> Self {
        variant as u8 != 0
    }
}
///Field `EXENB` reader - Operation Enable
pub type ExenbR = crate::BitReader<Exenb>;
impl ExenbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Exenb {
        match self.bits {
            false => Exenb::_0,
            true => Exenb::_1,
        }
    }
    ///Operation is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Exenb::_0
    }
    ///Operation is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Exenb::_1
    }
}
///Field `EXENB` writer - Operation Enable
pub type ExenbW<'a, REG> = crate::BitWriter<'a, REG, Exenb>;
impl<'a, REG> ExenbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operation is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Exenb::_0)
    }
    ///Operation is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Exenb::_1)
    }
}
/**External Bus Width Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bsize {
    ///0: A 16-bit bus space is selected.
    _00 = 0,
    ///2: An 8-bit bus space is selected.
    _10 = 2,
    ///1: Setting prohibited
    Others = 1,
}
impl From<Bsize> for u8 {
    #[inline(always)]
    fn from(variant: Bsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bsize {
    type Ux = u8;
}
impl crate::IsEnum for Bsize {}
///Field `BSIZE` reader - External Bus Width Select
pub type BsizeR = crate::FieldReader<Bsize>;
impl BsizeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bsize {
        match self.bits {
            0 => Bsize::_00,
            2 => Bsize::_10,
            _ => Bsize::Others,
        }
    }
    ///A 16-bit bus space is selected.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Bsize::_00
    }
    ///An 8-bit bus space is selected.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Bsize::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Bsize::Others)
    }
}
///Field `BSIZE` writer - External Bus Width Select
pub type BsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bsize, crate::Safe>;
impl<'a, REG> BsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///A 16-bit bus space is selected.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Bsize::_00)
    }
    ///An 8-bit bus space is selected.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Bsize::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Bsize::Others)
    }
}
/**Endian Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emode {
    ///0: Little-endian
    _0 = 0,
    ///1: Big-endian
    _1 = 1,
}
impl From<Emode> for bool {
    #[inline(always)]
    fn from(variant: Emode) -> Self {
        variant as u8 != 0
    }
}
///Field `EMODE` reader - Endian Mode
pub type EmodeR = crate::BitReader<Emode>;
impl EmodeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Emode {
        match self.bits {
            false => Emode::_0,
            true => Emode::_1,
        }
    }
    ///Little-endian
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Emode::_0
    }
    ///Big-endian
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Emode::_1
    }
}
///Field `EMODE` writer - Endian Mode
pub type EmodeW<'a, REG> = crate::BitWriter<'a, REG, Emode>;
impl<'a, REG> EmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Little-endian
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Emode::_0)
    }
    ///Big-endian
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Emode::_1)
    }
}
/**Address/Data Multiplexed I/O Interface Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpxen {
    ///0: Separate bus interface is selected for area n.
    _0 = 0,
    ///1: Address/data multiplexed I/O interface is selected for area n.
    _1 = 1,
}
impl From<Mpxen> for bool {
    #[inline(always)]
    fn from(variant: Mpxen) -> Self {
        variant as u8 != 0
    }
}
///Field `MPXEN` reader - Address/Data Multiplexed I/O Interface Select
pub type MpxenR = crate::BitReader<Mpxen>;
impl MpxenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mpxen {
        match self.bits {
            false => Mpxen::_0,
            true => Mpxen::_1,
        }
    }
    ///Separate bus interface is selected for area n.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpxen::_0
    }
    ///Address/data multiplexed I/O interface is selected for area n.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mpxen::_1
    }
}
///Field `MPXEN` writer - Address/Data Multiplexed I/O Interface Select
pub type MpxenW<'a, REG> = crate::BitWriter<'a, REG, Mpxen>;
impl<'a, REG> MpxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Separate bus interface is selected for area n.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpxen::_0)
    }
    ///Address/data multiplexed I/O interface is selected for area n.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpxen::_1)
    }
}
impl R {
    ///Bit 0 - Operation Enable
    #[inline(always)]
    pub fn exenb(&self) -> ExenbR {
        ExenbR::new((self.bits & 1) != 0)
    }
    ///Bits 4:5 - External Bus Width Select
    #[inline(always)]
    pub fn bsize(&self) -> BsizeR {
        BsizeR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - Endian Mode
    #[inline(always)]
    pub fn emode(&self) -> EmodeR {
        EmodeR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Address/Data Multiplexed I/O Interface Select
    #[inline(always)]
    pub fn mpxen(&self) -> MpxenR {
        MpxenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSCR")
            .field("exenb", &self.exenb())
            .field("bsize", &self.bsize())
            .field("emode", &self.emode())
            .field("mpxen", &self.mpxen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Operation Enable
    #[inline(always)]
    pub fn exenb(&mut self) -> ExenbW<CscrSpec> {
        ExenbW::new(self, 0)
    }
    ///Bits 4:5 - External Bus Width Select
    #[inline(always)]
    pub fn bsize(&mut self) -> BsizeW<CscrSpec> {
        BsizeW::new(self, 4)
    }
    ///Bit 8 - Endian Mode
    #[inline(always)]
    pub fn emode(&mut self) -> EmodeW<CscrSpec> {
        EmodeW::new(self, 8)
    }
    ///Bit 12 - Address/Data Multiplexed I/O Interface Select
    #[inline(always)]
    pub fn mpxen(&mut self) -> MpxenW<CscrSpec> {
        MpxenW::new(self, 12)
    }
}
/**CS%s Control Register

You can [`read`](crate::Reg::read) this register and get [`cscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CscrSpec;
impl crate::RegisterSpec for CscrSpec {
    type Ux = u16;
}
///`read()` method returns [`cscr::R`](R) reader structure
impl crate::Readable for CscrSpec {}
///`write(|w| ..)` method takes [`cscr::W`](W) writer structure
impl crate::Writable for CscrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CS%sCR to value 0
impl crate::Resettable for CscrSpec {}
