///Register `CKOCR` reader
pub type R = crate::R<CkocrSpec>;
///Register `CKOCR` writer
pub type W = crate::W<CkocrSpec>;
/**Clock Out Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckosel {
    ///0: HOCO (value after reset)
    _000 = 0,
    ///1: MOCO
    _001 = 1,
    ///2: LOCO
    _010 = 2,
    ///3: MOSC
    _011 = 3,
    ///4: SOSC
    _100 = 4,
    ///5: Setting prohibited
    _101 = 5,
    ///6: Setting prohibited
    Others = 6,
}
impl From<Ckosel> for u8 {
    #[inline(always)]
    fn from(variant: Ckosel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckosel {
    type Ux = u8;
}
impl crate::IsEnum for Ckosel {}
///Field `CKOSEL` reader - Clock Out Source Select
pub type CkoselR = crate::FieldReader<Ckosel>;
impl CkoselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ckosel {
        match self.bits {
            0 => Ckosel::_000,
            1 => Ckosel::_001,
            2 => Ckosel::_010,
            3 => Ckosel::_011,
            4 => Ckosel::_100,
            5 => Ckosel::_101,
            _ => Ckosel::Others,
        }
    }
    ///HOCO (value after reset)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ckosel::_000
    }
    ///MOCO
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ckosel::_001
    }
    ///LOCO
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ckosel::_010
    }
    ///MOSC
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ckosel::_011
    }
    ///SOSC
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ckosel::_100
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ckosel::_101
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ckosel::Others)
    }
}
///Field `CKOSEL` writer - Clock Out Source Select
pub type CkoselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ckosel, crate::Safe>;
impl<'a, REG> CkoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HOCO (value after reset)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::_000)
    }
    ///MOCO
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::_001)
    }
    ///LOCO
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::_010)
    }
    ///MOSC
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::_011)
    }
    ///SOSC
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::_100)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::_101)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::Others)
    }
}
/**Clock Output Frequency Division Ratio

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckodiv {
    ///0: x 1/1
    _000 = 0,
    ///1: x 1/2
    _001 = 1,
    ///2: x 1/4
    _010 = 2,
    ///3: x 1/8
    _011 = 3,
    ///4: x 1/16
    _100 = 4,
    ///5: x 1/32
    _101 = 5,
    ///6: x 1/64
    _110 = 6,
    ///7: x 1/128
    _111 = 7,
}
impl From<Ckodiv> for u8 {
    #[inline(always)]
    fn from(variant: Ckodiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckodiv {
    type Ux = u8;
}
impl crate::IsEnum for Ckodiv {}
///Field `CKODIV` reader - Clock Output Frequency Division Ratio
pub type CkodivR = crate::FieldReader<Ckodiv>;
impl CkodivR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ckodiv {
        match self.bits {
            0 => Ckodiv::_000,
            1 => Ckodiv::_001,
            2 => Ckodiv::_010,
            3 => Ckodiv::_011,
            4 => Ckodiv::_100,
            5 => Ckodiv::_101,
            6 => Ckodiv::_110,
            7 => Ckodiv::_111,
            _ => unreachable!(),
        }
    }
    ///x 1/1
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ckodiv::_000
    }
    ///x 1/2
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ckodiv::_001
    }
    ///x 1/4
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ckodiv::_010
    }
    ///x 1/8
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ckodiv::_011
    }
    ///x 1/16
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ckodiv::_100
    }
    ///x 1/32
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ckodiv::_101
    }
    ///x 1/64
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Ckodiv::_110
    }
    ///x 1/128
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Ckodiv::_111
    }
}
///Field `CKODIV` writer - Clock Output Frequency Division Ratio
pub type CkodivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ckodiv, crate::Safe>;
impl<'a, REG> CkodivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///x 1/1
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_000)
    }
    ///x 1/2
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_001)
    }
    ///x 1/4
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_010)
    }
    ///x 1/8
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_011)
    }
    ///x 1/16
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_100)
    }
    ///x 1/32
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_101)
    }
    ///x 1/64
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_110)
    }
    ///x 1/128
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_111)
    }
}
/**Clock Out Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckoen {
    ///0: Disable clock out
    _0 = 0,
    ///1: Enable clock out
    _1 = 1,
}
impl From<Ckoen> for bool {
    #[inline(always)]
    fn from(variant: Ckoen) -> Self {
        variant as u8 != 0
    }
}
///Field `CKOEN` reader - Clock Out Enable
pub type CkoenR = crate::BitReader<Ckoen>;
impl CkoenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ckoen {
        match self.bits {
            false => Ckoen::_0,
            true => Ckoen::_1,
        }
    }
    ///Disable clock out
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ckoen::_0
    }
    ///Enable clock out
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ckoen::_1
    }
}
///Field `CKOEN` writer - Clock Out Enable
pub type CkoenW<'a, REG> = crate::BitWriter<'a, REG, Ckoen>;
impl<'a, REG> CkoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable clock out
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoen::_0)
    }
    ///Enable clock out
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoen::_1)
    }
}
impl R {
    ///Bits 0:2 - Clock Out Source Select
    #[inline(always)]
    pub fn ckosel(&self) -> CkoselR {
        CkoselR::new(self.bits & 7)
    }
    ///Bits 4:6 - Clock Output Frequency Division Ratio
    #[inline(always)]
    pub fn ckodiv(&self) -> CkodivR {
        CkodivR::new((self.bits >> 4) & 7)
    }
    ///Bit 7 - Clock Out Enable
    #[inline(always)]
    pub fn ckoen(&self) -> CkoenR {
        CkoenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKOCR")
            .field("ckosel", &self.ckosel())
            .field("ckodiv", &self.ckodiv())
            .field("ckoen", &self.ckoen())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Clock Out Source Select
    #[inline(always)]
    pub fn ckosel(&mut self) -> CkoselW<CkocrSpec> {
        CkoselW::new(self, 0)
    }
    ///Bits 4:6 - Clock Output Frequency Division Ratio
    #[inline(always)]
    pub fn ckodiv(&mut self) -> CkodivW<CkocrSpec> {
        CkodivW::new(self, 4)
    }
    ///Bit 7 - Clock Out Enable
    #[inline(always)]
    pub fn ckoen(&mut self) -> CkoenW<CkocrSpec> {
        CkoenW::new(self, 7)
    }
}
/**Clock Out Control Register

You can [`read`](crate::Reg::read) this register and get [`ckocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkocrSpec;
impl crate::RegisterSpec for CkocrSpec {
    type Ux = u8;
}
///`read()` method returns [`ckocr::R`](R) reader structure
impl crate::Readable for CkocrSpec {}
///`write(|w| ..)` method takes [`ckocr::W`](W) writer structure
impl crate::Writable for CkocrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKOCR to value 0
impl crate::Resettable for CkocrSpec {}
