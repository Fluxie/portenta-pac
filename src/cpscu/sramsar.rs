///Register `SRAMSAR` reader
pub type R = crate::R<SramsarSpec>;
///Register `SRAMSAR` writer
pub type W = crate::W<SramsarSpec>;
/**Security attributes of registers for SRAM Protection

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramsa0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-Secure
    _1 = 1,
}
impl From<Sramsa0> for bool {
    #[inline(always)]
    fn from(variant: Sramsa0) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAMSA0` reader - Security attributes of registers for SRAM Protection
pub type Sramsa0R = crate::BitReader<Sramsa0>;
impl Sramsa0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sramsa0 {
        match self.bits {
            false => Sramsa0::_0,
            true => Sramsa0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sramsa0::_0
    }
    ///Non-Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sramsa0::_1
    }
}
///Field `SRAMSA0` writer - Security attributes of registers for SRAM Protection
pub type Sramsa0W<'a, REG> = crate::BitWriter<'a, REG, Sramsa0>;
impl<'a, REG> Sramsa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsa0::_0)
    }
    ///Non-Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsa0::_1)
    }
}
/**Security attributes of registers for SRAM Protection 2

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramsa1 {
    ///0: Secure
    _0 = 0,
    ///1: Non-Secure
    _1 = 1,
}
impl From<Sramsa1> for bool {
    #[inline(always)]
    fn from(variant: Sramsa1) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAMSA1` reader - Security attributes of registers for SRAM Protection 2
pub type Sramsa1R = crate::BitReader<Sramsa1>;
impl Sramsa1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sramsa1 {
        match self.bits {
            false => Sramsa1::_0,
            true => Sramsa1::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sramsa1::_0
    }
    ///Non-Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sramsa1::_1
    }
}
///Field `SRAMSA1` writer - Security attributes of registers for SRAM Protection 2
pub type Sramsa1W<'a, REG> = crate::BitWriter<'a, REG, Sramsa1>;
impl<'a, REG> Sramsa1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsa1::_0)
    }
    ///Non-Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsa1::_1)
    }
}
/**Security attributes of registers for ECC Relation

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramsa2 {
    ///0: Secure
    _0 = 0,
    ///1: Non-Secure
    _1 = 1,
}
impl From<Sramsa2> for bool {
    #[inline(always)]
    fn from(variant: Sramsa2) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAMSA2` reader - Security attributes of registers for ECC Relation
pub type Sramsa2R = crate::BitReader<Sramsa2>;
impl Sramsa2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sramsa2 {
        match self.bits {
            false => Sramsa2::_0,
            true => Sramsa2::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sramsa2::_0
    }
    ///Non-Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sramsa2::_1
    }
}
///Field `SRAMSA2` writer - Security attributes of registers for ECC Relation
pub type Sramsa2W<'a, REG> = crate::BitWriter<'a, REG, Sramsa2>;
impl<'a, REG> Sramsa2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsa2::_0)
    }
    ///Non-Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsa2::_1)
    }
}
impl R {
    ///Bit 0 - Security attributes of registers for SRAM Protection
    #[inline(always)]
    pub fn sramsa0(&self) -> Sramsa0R {
        Sramsa0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security attributes of registers for SRAM Protection 2
    #[inline(always)]
    pub fn sramsa1(&self) -> Sramsa1R {
        Sramsa1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security attributes of registers for ECC Relation
    #[inline(always)]
    pub fn sramsa2(&self) -> Sramsa2R {
        Sramsa2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAMSAR")
            .field("sramsa0", &self.sramsa0())
            .field("sramsa1", &self.sramsa1())
            .field("sramsa2", &self.sramsa2())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security attributes of registers for SRAM Protection
    #[inline(always)]
    pub fn sramsa0(&mut self) -> Sramsa0W<SramsarSpec> {
        Sramsa0W::new(self, 0)
    }
    ///Bit 1 - Security attributes of registers for SRAM Protection 2
    #[inline(always)]
    pub fn sramsa1(&mut self) -> Sramsa1W<SramsarSpec> {
        Sramsa1W::new(self, 1)
    }
    ///Bit 2 - Security attributes of registers for ECC Relation
    #[inline(always)]
    pub fn sramsa2(&mut self) -> Sramsa2W<SramsarSpec> {
        Sramsa2W::new(self, 2)
    }
}
/**SRAM Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`sramsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SramsarSpec;
impl crate::RegisterSpec for SramsarSpec {
    type Ux = u32;
}
///`read()` method returns [`sramsar::R`](R) reader structure
impl crate::Readable for SramsarSpec {}
///`write(|w| ..)` method takes [`sramsar::W`](W) writer structure
impl crate::Writable for SramsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRAMSAR to value 0xffff_ffff
impl crate::Resettable for SramsarSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
