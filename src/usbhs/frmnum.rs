///Register `FRMNUM` reader
pub type R = crate::R<FrmnumSpec>;
///Register `FRMNUM` writer
pub type W = crate::W<FrmnumSpec>;
///Field `FRNM` reader - Frame Number Flag
pub type FrnmR = crate::FieldReader<u16>;
/**CRC Error Detection Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crce {
    ///0: No error occurred
    _0 = 0,
    ///1: Error occurred
    _1 = 1,
}
impl From<Crce> for bool {
    #[inline(always)]
    fn from(variant: Crce) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCE` reader - CRC Error Detection Status Flag
pub type CrceR = crate::BitReader<Crce>;
impl CrceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Crce {
        match self.bits {
            false => Crce::_0,
            true => Crce::_1,
        }
    }
    ///No error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crce::_0
    }
    ///Error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crce::_1
    }
}
///Field `CRCE` writer - CRC Error Detection Status Flag
pub type CrceW<'a, REG> = crate::BitWriter<'a, REG, Crce>;
impl<'a, REG> CrceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Crce::_0)
    }
    ///Error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Crce::_1)
    }
}
/**Overrun/Underrun Detection Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrn {
    ///0: No error occurred
    _0 = 0,
    ///1: Error occurred.
    _1 = 1,
}
impl From<Ovrn> for bool {
    #[inline(always)]
    fn from(variant: Ovrn) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRN` reader - Overrun/Underrun Detection Status Flag
pub type OvrnR = crate::BitReader<Ovrn>;
impl OvrnR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ovrn {
        match self.bits {
            false => Ovrn::_0,
            true => Ovrn::_1,
        }
    }
    ///No error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovrn::_0
    }
    ///Error occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovrn::_1
    }
}
///Field `OVRN` writer - Overrun/Underrun Detection Status Flag
pub type OvrnW<'a, REG> = crate::BitWriter<'a, REG, Ovrn>;
impl<'a, REG> OvrnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrn::_0)
    }
    ///Error occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrn::_1)
    }
}
impl R {
    ///Bits 0:10 - Frame Number Flag
    #[inline(always)]
    pub fn frnm(&self) -> FrnmR {
        FrnmR::new(self.bits & 0x07ff)
    }
    ///Bit 14 - CRC Error Detection Status Flag
    #[inline(always)]
    pub fn crce(&self) -> CrceR {
        CrceR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Overrun/Underrun Detection Status Flag
    #[inline(always)]
    pub fn ovrn(&self) -> OvrnR {
        OvrnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRMNUM")
            .field("frnm", &self.frnm())
            .field("crce", &self.crce())
            .field("ovrn", &self.ovrn())
            .finish()
    }
}
impl W {
    ///Bit 14 - CRC Error Detection Status Flag
    #[inline(always)]
    pub fn crce(&mut self) -> CrceW<FrmnumSpec> {
        CrceW::new(self, 14)
    }
    ///Bit 15 - Overrun/Underrun Detection Status Flag
    #[inline(always)]
    pub fn ovrn(&mut self) -> OvrnW<FrmnumSpec> {
        OvrnW::new(self, 15)
    }
}
/**Frame Number Register

You can [`read`](crate::Reg::read) this register and get [`frmnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FrmnumSpec;
impl crate::RegisterSpec for FrmnumSpec {
    type Ux = u16;
}
///`read()` method returns [`frmnum::R`](R) reader structure
impl crate::Readable for FrmnumSpec {}
///`write(|w| ..)` method takes [`frmnum::W`](W) writer structure
impl crate::Writable for FrmnumSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FRMNUM to value 0
impl crate::Resettable for FrmnumSpec {}
