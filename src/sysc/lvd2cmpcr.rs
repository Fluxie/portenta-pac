///Register `LVD2CMPCR` reader
pub type R = crate::R<Lvd2cmpcrSpec>;
///Register `LVD2CMPCR` writer
pub type W = crate::W<Lvd2cmpcrSpec>;
/**Voltage Detection 2 Level Select (Standard voltage during drop in voltage)

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvd2lvl {
    ///5: 2.99 V (Vdet2_1)
    _101 = 5,
    ///6: 2.92 V (Vdet2_2)
    _110 = 6,
    ///7: 2.85 V (Vdet2_3)
    _111 = 7,
    ///0: Setting prohibited
    Others = 0,
}
impl From<Lvd2lvl> for u8 {
    #[inline(always)]
    fn from(variant: Lvd2lvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvd2lvl {
    type Ux = u8;
}
impl crate::IsEnum for Lvd2lvl {}
///Field `LVD2LVL` reader - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)
pub type Lvd2lvlR = crate::FieldReader<Lvd2lvl>;
impl Lvd2lvlR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2lvl {
        match self.bits {
            5 => Lvd2lvl::_101,
            6 => Lvd2lvl::_110,
            7 => Lvd2lvl::_111,
            _ => Lvd2lvl::Others,
        }
    }
    ///2.99 V (Vdet2_1)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Lvd2lvl::_101
    }
    ///2.92 V (Vdet2_2)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Lvd2lvl::_110
    }
    ///2.85 V (Vdet2_3)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Lvd2lvl::_111
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Lvd2lvl::Others)
    }
}
///Field `LVD2LVL` writer - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)
pub type Lvd2lvlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lvd2lvl, crate::Safe>;
impl<'a, REG> Lvd2lvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2.99 V (Vdet2_1)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::_101)
    }
    ///2.92 V (Vdet2_2)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::_110)
    }
    ///2.85 V (Vdet2_3)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::_111)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::Others)
    }
}
/**Voltage Detection 2 Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2e {
    ///0: Voltage detection 2 circuit disabled
    _0 = 0,
    ///1: Voltage detection 2 circuit enabled
    _1 = 1,
}
impl From<Lvd2e> for bool {
    #[inline(always)]
    fn from(variant: Lvd2e) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD2E` reader - Voltage Detection 2 Enable
pub type Lvd2eR = crate::BitReader<Lvd2e>;
impl Lvd2eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2e {
        match self.bits {
            false => Lvd2e::_0,
            true => Lvd2e::_1,
        }
    }
    ///Voltage detection 2 circuit disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2e::_0
    }
    ///Voltage detection 2 circuit enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2e::_1
    }
}
///Field `LVD2E` writer - Voltage Detection 2 Enable
pub type Lvd2eW<'a, REG> = crate::BitWriter<'a, REG, Lvd2e>;
impl<'a, REG> Lvd2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage detection 2 circuit disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2e::_0)
    }
    ///Voltage detection 2 circuit enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2e::_1)
    }
}
impl R {
    ///Bits 0:2 - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)
    #[inline(always)]
    pub fn lvd2lvl(&self) -> Lvd2lvlR {
        Lvd2lvlR::new(self.bits & 7)
    }
    ///Bit 7 - Voltage Detection 2 Enable
    #[inline(always)]
    pub fn lvd2e(&self) -> Lvd2eR {
        Lvd2eR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVD2CMPCR")
            .field("lvd2lvl", &self.lvd2lvl())
            .field("lvd2e", &self.lvd2e())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)
    #[inline(always)]
    pub fn lvd2lvl(&mut self) -> Lvd2lvlW<Lvd2cmpcrSpec> {
        Lvd2lvlW::new(self, 0)
    }
    ///Bit 7 - Voltage Detection 2 Enable
    #[inline(always)]
    pub fn lvd2e(&mut self) -> Lvd2eW<Lvd2cmpcrSpec> {
        Lvd2eW::new(self, 7)
    }
}
/**Voltage Monitoring 2 Comparator Control Register

You can [`read`](crate::Reg::read) this register and get [`lvd2cmpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd2cmpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Lvd2cmpcrSpec;
impl crate::RegisterSpec for Lvd2cmpcrSpec {
    type Ux = u8;
}
///`read()` method returns [`lvd2cmpcr::R`](R) reader structure
impl crate::Readable for Lvd2cmpcrSpec {}
///`write(|w| ..)` method takes [`lvd2cmpcr::W`](W) writer structure
impl crate::Writable for Lvd2cmpcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVD2CMPCR to value 0x07
impl crate::Resettable for Lvd2cmpcrSpec {
    const RESET_VALUE: u8 = 0x07;
}
