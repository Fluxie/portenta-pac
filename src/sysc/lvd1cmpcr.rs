///Register `LVD1CMPCR` reader
pub type R = crate::R<Lvd1cmpcrSpec>;
///Register `LVD1CMPCR` writer
pub type W = crate::W<Lvd1cmpcrSpec>;
/**Voltage Detection 1 Level Select (Standard voltage during drop in voltage)

Value on reset: 19*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvd1lvl {
    ///17: 2.99 V (Vdet1_1)
    _0x11 = 17,
    ///18: 2.92 V (Vdet1_2)
    _0x12 = 18,
    ///19: 2.85 V (Vdet1_3)
    _0x13 = 19,
    ///0: Setting prohibited
    Others = 0,
}
impl From<Lvd1lvl> for u8 {
    #[inline(always)]
    fn from(variant: Lvd1lvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvd1lvl {
    type Ux = u8;
}
impl crate::IsEnum for Lvd1lvl {}
///Field `LVD1LVL` reader - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)
pub type Lvd1lvlR = crate::FieldReader<Lvd1lvl>;
impl Lvd1lvlR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1lvl {
        match self.bits {
            17 => Lvd1lvl::_0x11,
            18 => Lvd1lvl::_0x12,
            19 => Lvd1lvl::_0x13,
            _ => Lvd1lvl::Others,
        }
    }
    ///2.99 V (Vdet1_1)
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == Lvd1lvl::_0x11
    }
    ///2.92 V (Vdet1_2)
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == Lvd1lvl::_0x12
    }
    ///2.85 V (Vdet1_3)
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == Lvd1lvl::_0x13
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Lvd1lvl::Others)
    }
}
///Field `LVD1LVL` writer - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)
pub type Lvd1lvlW<'a, REG> = crate::FieldWriter<'a, REG, 5, Lvd1lvl, crate::Safe>;
impl<'a, REG> Lvd1lvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2.99 V (Vdet1_1)
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x11)
    }
    ///2.92 V (Vdet1_2)
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x12)
    }
    ///2.85 V (Vdet1_3)
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x13)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::Others)
    }
}
/**Voltage Detection 1 Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1e {
    ///0: Voltage detection 1 circuit disabled
    _0 = 0,
    ///1: Voltage detection 1 circuit enabled
    _1 = 1,
}
impl From<Lvd1e> for bool {
    #[inline(always)]
    fn from(variant: Lvd1e) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD1E` reader - Voltage Detection 1 Enable
pub type Lvd1eR = crate::BitReader<Lvd1e>;
impl Lvd1eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1e {
        match self.bits {
            false => Lvd1e::_0,
            true => Lvd1e::_1,
        }
    }
    ///Voltage detection 1 circuit disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1e::_0
    }
    ///Voltage detection 1 circuit enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1e::_1
    }
}
///Field `LVD1E` writer - Voltage Detection 1 Enable
pub type Lvd1eW<'a, REG> = crate::BitWriter<'a, REG, Lvd1e>;
impl<'a, REG> Lvd1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage detection 1 circuit disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1e::_0)
    }
    ///Voltage detection 1 circuit enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1e::_1)
    }
}
impl R {
    ///Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)
    #[inline(always)]
    pub fn lvd1lvl(&self) -> Lvd1lvlR {
        Lvd1lvlR::new(self.bits & 0x1f)
    }
    ///Bit 7 - Voltage Detection 1 Enable
    #[inline(always)]
    pub fn lvd1e(&self) -> Lvd1eR {
        Lvd1eR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVD1CMPCR")
            .field("lvd1lvl", &self.lvd1lvl())
            .field("lvd1e", &self.lvd1e())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)
    #[inline(always)]
    pub fn lvd1lvl(&mut self) -> Lvd1lvlW<Lvd1cmpcrSpec> {
        Lvd1lvlW::new(self, 0)
    }
    ///Bit 7 - Voltage Detection 1 Enable
    #[inline(always)]
    pub fn lvd1e(&mut self) -> Lvd1eW<Lvd1cmpcrSpec> {
        Lvd1eW::new(self, 7)
    }
}
/**Voltage Monitoring 1 Comparator Control Register

You can [`read`](crate::Reg::read) this register and get [`lvd1cmpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1cmpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Lvd1cmpcrSpec;
impl crate::RegisterSpec for Lvd1cmpcrSpec {
    type Ux = u8;
}
///`read()` method returns [`lvd1cmpcr::R`](R) reader structure
impl crate::Readable for Lvd1cmpcrSpec {}
///`write(|w| ..)` method takes [`lvd1cmpcr::W`](W) writer structure
impl crate::Writable for Lvd1cmpcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVD1CMPCR to value 0x13
impl crate::Resettable for Lvd1cmpcrSpec {
    const RESET_VALUE: u8 = 0x13;
}
