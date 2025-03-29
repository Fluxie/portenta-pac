///Register `DMTMD` reader
pub type R = crate::R<DmtmdSpec>;
///Register `DMTMD` writer
pub type W = crate::W<DmtmdSpec>;
/**Transfer Request Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dctg {
    ///0: Software request
    _00 = 0,
    ///1: Hardware request
    _01 = 1,
    ///2: Setting prohibited
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Dctg> for u8 {
    #[inline(always)]
    fn from(variant: Dctg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dctg {
    type Ux = u8;
}
impl crate::IsEnum for Dctg {}
///Field `DCTG` reader - Transfer Request Source Select
pub type DctgR = crate::FieldReader<Dctg>;
impl DctgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dctg {
        match self.bits {
            0 => Dctg::_00,
            1 => Dctg::_01,
            2 => Dctg::_10,
            3 => Dctg::_11,
            _ => unreachable!(),
        }
    }
    ///Software request
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dctg::_00
    }
    ///Hardware request
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dctg::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dctg::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dctg::_11
    }
}
///Field `DCTG` writer - Transfer Request Source Select
pub type DctgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dctg, crate::Safe>;
impl<'a, REG> DctgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Software request
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dctg::_00)
    }
    ///Hardware request
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dctg::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dctg::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dctg::_11)
    }
}
/**Transfer Data Size Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sz {
    ///0: 8 bits
    _00 = 0,
    ///1: 16 bits
    _01 = 1,
    ///2: 32 bits
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Sz> for u8 {
    #[inline(always)]
    fn from(variant: Sz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sz {
    type Ux = u8;
}
impl crate::IsEnum for Sz {}
///Field `SZ` reader - Transfer Data Size Select
pub type SzR = crate::FieldReader<Sz>;
impl SzR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sz {
        match self.bits {
            0 => Sz::_00,
            1 => Sz::_01,
            2 => Sz::_10,
            3 => Sz::_11,
            _ => unreachable!(),
        }
    }
    ///8 bits
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sz::_00
    }
    ///16 bits
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sz::_01
    }
    ///32 bits
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sz::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sz::_11
    }
}
///Field `SZ` writer - Transfer Data Size Select
pub type SzW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sz, crate::Safe>;
impl<'a, REG> SzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sz::_00)
    }
    ///16 bits
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sz::_01)
    }
    ///32 bits
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sz::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sz::_11)
    }
}
/**Transfer Keeping

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tkp {
    ///0: Transfer is stopped by completion of specified total number of transfer operations.
    _0 = 0,
    ///1: Transfer is not stopped by completion of specified total number of transfer operations (free-running).
    _1 = 1,
}
impl From<Tkp> for bool {
    #[inline(always)]
    fn from(variant: Tkp) -> Self {
        variant as u8 != 0
    }
}
///Field `TKP` reader - Transfer Keeping
pub type TkpR = crate::BitReader<Tkp>;
impl TkpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tkp {
        match self.bits {
            false => Tkp::_0,
            true => Tkp::_1,
        }
    }
    ///Transfer is stopped by completion of specified total number of transfer operations.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tkp::_0
    }
    ///Transfer is not stopped by completion of specified total number of transfer operations (free-running).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tkp::_1
    }
}
///Field `TKP` writer - Transfer Keeping
pub type TkpW<'a, REG> = crate::BitWriter<'a, REG, Tkp>;
impl<'a, REG> TkpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer is stopped by completion of specified total number of transfer operations.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tkp::_0)
    }
    ///Transfer is not stopped by completion of specified total number of transfer operations (free-running).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tkp::_1)
    }
}
/**Repeat Area Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dts {
    ///0: The destination is specified as the repeat area or block area.
    _00 = 0,
    ///1: The source is specified as the repeat area or block area.
    _01 = 1,
    ///2: The repeat area or block area is not specified.
    _10 = 2,
    ///3: Setting prohibited.
    _11 = 3,
}
impl From<Dts> for u8 {
    #[inline(always)]
    fn from(variant: Dts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dts {
    type Ux = u8;
}
impl crate::IsEnum for Dts {}
///Field `DTS` reader - Repeat Area Select
pub type DtsR = crate::FieldReader<Dts>;
impl DtsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dts {
        match self.bits {
            0 => Dts::_00,
            1 => Dts::_01,
            2 => Dts::_10,
            3 => Dts::_11,
            _ => unreachable!(),
        }
    }
    ///The destination is specified as the repeat area or block area.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dts::_00
    }
    ///The source is specified as the repeat area or block area.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dts::_01
    }
    ///The repeat area or block area is not specified.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dts::_10
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dts::_11
    }
}
///Field `DTS` writer - Repeat Area Select
pub type DtsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dts, crate::Safe>;
impl<'a, REG> DtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The destination is specified as the repeat area or block area.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dts::_00)
    }
    ///The source is specified as the repeat area or block area.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dts::_01)
    }
    ///The repeat area or block area is not specified.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dts::_10)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dts::_11)
    }
}
/**Transfer Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Md {
    ///0: Normal transfer
    _00 = 0,
    ///1: Repeat transfer
    _01 = 1,
    ///2: Block transfer
    _10 = 2,
    ///3: Repeat-block transfer
    _11 = 3,
}
impl From<Md> for u8 {
    #[inline(always)]
    fn from(variant: Md) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Md {
    type Ux = u8;
}
impl crate::IsEnum for Md {}
///Field `MD` reader - Transfer Mode Select
pub type MdR = crate::FieldReader<Md>;
impl MdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Md {
        match self.bits {
            0 => Md::_00,
            1 => Md::_01,
            2 => Md::_10,
            3 => Md::_11,
            _ => unreachable!(),
        }
    }
    ///Normal transfer
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Md::_00
    }
    ///Repeat transfer
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Md::_01
    }
    ///Block transfer
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Md::_10
    }
    ///Repeat-block transfer
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Md::_11
    }
}
///Field `MD` writer - Transfer Mode Select
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Md, crate::Safe>;
impl<'a, REG> MdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal transfer
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_00)
    }
    ///Repeat transfer
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_01)
    }
    ///Block transfer
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_10)
    }
    ///Repeat-block transfer
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_11)
    }
}
impl R {
    ///Bits 0:1 - Transfer Request Source Select
    #[inline(always)]
    pub fn dctg(&self) -> DctgR {
        DctgR::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - Transfer Data Size Select
    #[inline(always)]
    pub fn sz(&self) -> SzR {
        SzR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Transfer Keeping
    #[inline(always)]
    pub fn tkp(&self) -> TkpR {
        TkpR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 12:13 - Repeat Area Select
    #[inline(always)]
    pub fn dts(&self) -> DtsR {
        DtsR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Transfer Mode Select
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMTMD")
            .field("dctg", &self.dctg())
            .field("sz", &self.sz())
            .field("tkp", &self.tkp())
            .field("dts", &self.dts())
            .field("md", &self.md())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Transfer Request Source Select
    #[inline(always)]
    pub fn dctg(&mut self) -> DctgW<DmtmdSpec> {
        DctgW::new(self, 0)
    }
    ///Bits 8:9 - Transfer Data Size Select
    #[inline(always)]
    pub fn sz(&mut self) -> SzW<DmtmdSpec> {
        SzW::new(self, 8)
    }
    ///Bit 10 - Transfer Keeping
    #[inline(always)]
    pub fn tkp(&mut self) -> TkpW<DmtmdSpec> {
        TkpW::new(self, 10)
    }
    ///Bits 12:13 - Repeat Area Select
    #[inline(always)]
    pub fn dts(&mut self) -> DtsW<DmtmdSpec> {
        DtsW::new(self, 12)
    }
    ///Bits 14:15 - Transfer Mode Select
    #[inline(always)]
    pub fn md(&mut self) -> MdW<DmtmdSpec> {
        MdW::new(self, 14)
    }
}
/**DMA Transfer Mode Register

You can [`read`](crate::Reg::read) this register and get [`dmtmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmtmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmtmdSpec;
impl crate::RegisterSpec for DmtmdSpec {
    type Ux = u16;
}
///`read()` method returns [`dmtmd::R`](R) reader structure
impl crate::Readable for DmtmdSpec {}
///`write(|w| ..)` method takes [`dmtmd::W`](W) writer structure
impl crate::Writable for DmtmdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMTMD to value 0
impl crate::Resettable for DmtmdSpec {}
