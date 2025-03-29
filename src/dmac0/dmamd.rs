///Register `DMAMD` reader
pub type R = crate::R<DmamdSpec>;
///Register `DMAMD` writer
pub type W = crate::W<DmamdSpec>;
///Field `DARA` reader - Destination Address Extended Repeat Area
pub type DaraR = crate::FieldReader;
///Field `DARA` writer - Destination Address Extended Repeat Area
pub type DaraW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Destination Address Update Select After Reload

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dadr {
    ///0: Only reloading.
    _0 = 0,
    ///1: Add index after reloading.
    _1 = 1,
}
impl From<Dadr> for bool {
    #[inline(always)]
    fn from(variant: Dadr) -> Self {
        variant as u8 != 0
    }
}
///Field `DADR` reader - Destination Address Update Select After Reload
pub type DadrR = crate::BitReader<Dadr>;
impl DadrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dadr {
        match self.bits {
            false => Dadr::_0,
            true => Dadr::_1,
        }
    }
    ///Only reloading.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dadr::_0
    }
    ///Add index after reloading.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dadr::_1
    }
}
///Field `DADR` writer - Destination Address Update Select After Reload
pub type DadrW<'a, REG> = crate::BitWriter<'a, REG, Dadr>;
impl<'a, REG> DadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Only reloading.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dadr::_0)
    }
    ///Add index after reloading.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dadr::_1)
    }
}
/**Destination Address Update Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dm {
    ///0: Destination address is fixed.
    _00 = 0,
    ///1: Offset addition.
    _01 = 1,
    ///2: Destination address is incremented.
    _10 = 2,
    ///3: Destination address is decremented.
    _11 = 3,
}
impl From<Dm> for u8 {
    #[inline(always)]
    fn from(variant: Dm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dm {
    type Ux = u8;
}
impl crate::IsEnum for Dm {}
///Field `DM` reader - Destination Address Update Mode
pub type DmR = crate::FieldReader<Dm>;
impl DmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dm {
        match self.bits {
            0 => Dm::_00,
            1 => Dm::_01,
            2 => Dm::_10,
            3 => Dm::_11,
            _ => unreachable!(),
        }
    }
    ///Destination address is fixed.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dm::_00
    }
    ///Offset addition.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dm::_01
    }
    ///Destination address is incremented.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dm::_10
    }
    ///Destination address is decremented.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dm::_11
    }
}
///Field `DM` writer - Destination Address Update Mode
pub type DmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dm, crate::Safe>;
impl<'a, REG> DmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Destination address is fixed.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::_00)
    }
    ///Offset addition.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::_01)
    }
    ///Destination address is incremented.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::_10)
    }
    ///Destination address is decremented.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::_11)
    }
}
///Field `SARA` reader - Source Address Extended Repeat Area
pub type SaraR = crate::FieldReader;
///Field `SARA` writer - Source Address Extended Repeat Area
pub type SaraW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Source Address Update Select After Reload

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadr {
    ///0: Only reloading.
    _0 = 0,
    ///1: Add index after reloading.
    _1 = 1,
}
impl From<Sadr> for bool {
    #[inline(always)]
    fn from(variant: Sadr) -> Self {
        variant as u8 != 0
    }
}
///Field `SADR` reader - Source Address Update Select After Reload
pub type SadrR = crate::BitReader<Sadr>;
impl SadrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sadr {
        match self.bits {
            false => Sadr::_0,
            true => Sadr::_1,
        }
    }
    ///Only reloading.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sadr::_0
    }
    ///Add index after reloading.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sadr::_1
    }
}
///Field `SADR` writer - Source Address Update Select After Reload
pub type SadrW<'a, REG> = crate::BitWriter<'a, REG, Sadr>;
impl<'a, REG> SadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Only reloading.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sadr::_0)
    }
    ///Add index after reloading.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sadr::_1)
    }
}
/**Source Address Update Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sm {
    ///0: Source address is fixed.
    _00 = 0,
    ///1: Offset addition.
    _01 = 1,
    ///2: Source address is incremented.
    _10 = 2,
    ///3: Source address is decremented.
    _11 = 3,
}
impl From<Sm> for u8 {
    #[inline(always)]
    fn from(variant: Sm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sm {
    type Ux = u8;
}
impl crate::IsEnum for Sm {}
///Field `SM` reader - Source Address Update Mode
pub type SmR = crate::FieldReader<Sm>;
impl SmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sm {
        match self.bits {
            0 => Sm::_00,
            1 => Sm::_01,
            2 => Sm::_10,
            3 => Sm::_11,
            _ => unreachable!(),
        }
    }
    ///Source address is fixed.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sm::_00
    }
    ///Offset addition.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sm::_01
    }
    ///Source address is incremented.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sm::_10
    }
    ///Source address is decremented.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sm::_11
    }
}
///Field `SM` writer - Source Address Update Mode
pub type SmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sm, crate::Safe>;
impl<'a, REG> SmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Source address is fixed.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::_00)
    }
    ///Offset addition.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::_01)
    }
    ///Source address is incremented.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::_10)
    }
    ///Source address is decremented.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::_11)
    }
}
impl R {
    ///Bits 0:4 - Destination Address Extended Repeat Area
    #[inline(always)]
    pub fn dara(&self) -> DaraR {
        DaraR::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - Destination Address Update Select After Reload
    #[inline(always)]
    pub fn dadr(&self) -> DadrR {
        DadrR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Destination Address Update Mode
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:12 - Source Address Extended Repeat Area
    #[inline(always)]
    pub fn sara(&self) -> SaraR {
        SaraR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 13 - Source Address Update Select After Reload
    #[inline(always)]
    pub fn sadr(&self) -> SadrR {
        SadrR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Source Address Update Mode
    #[inline(always)]
    pub fn sm(&self) -> SmR {
        SmR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMD")
            .field("dara", &self.dara())
            .field("dadr", &self.dadr())
            .field("dm", &self.dm())
            .field("sara", &self.sara())
            .field("sadr", &self.sadr())
            .field("sm", &self.sm())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Destination Address Extended Repeat Area
    #[inline(always)]
    pub fn dara(&mut self) -> DaraW<DmamdSpec> {
        DaraW::new(self, 0)
    }
    ///Bit 5 - Destination Address Update Select After Reload
    #[inline(always)]
    pub fn dadr(&mut self) -> DadrW<DmamdSpec> {
        DadrW::new(self, 5)
    }
    ///Bits 6:7 - Destination Address Update Mode
    #[inline(always)]
    pub fn dm(&mut self) -> DmW<DmamdSpec> {
        DmW::new(self, 6)
    }
    ///Bits 8:12 - Source Address Extended Repeat Area
    #[inline(always)]
    pub fn sara(&mut self) -> SaraW<DmamdSpec> {
        SaraW::new(self, 8)
    }
    ///Bit 13 - Source Address Update Select After Reload
    #[inline(always)]
    pub fn sadr(&mut self) -> SadrW<DmamdSpec> {
        SadrW::new(self, 13)
    }
    ///Bits 14:15 - Source Address Update Mode
    #[inline(always)]
    pub fn sm(&mut self) -> SmW<DmamdSpec> {
        SmW::new(self, 14)
    }
}
/**DMA Address Mode Register

You can [`read`](crate::Reg::read) this register and get [`dmamd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmamdSpec;
impl crate::RegisterSpec for DmamdSpec {
    type Ux = u16;
}
///`read()` method returns [`dmamd::R`](R) reader structure
impl crate::Readable for DmamdSpec {}
///`write(|w| ..)` method takes [`dmamd::W`](W) writer structure
impl crate::Writable for DmamdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAMD to value 0
impl crate::Resettable for DmamdSpec {}
