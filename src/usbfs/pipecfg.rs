///Register `PIPECFG` reader
pub type R = crate::R<PipecfgSpec>;
///Register `PIPECFG` writer
pub type W = crate::W<PipecfgSpec>;
///Field `EPNUM` reader - Endpoint Number
pub type EpnumR = crate::FieldReader;
///Field `EPNUM` writer - Endpoint Number
pub type EpnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Transfer Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    ///0: Receiving direction
    _0 = 0,
    ///1: Transmitting direction
    _1 = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
///Field `DIR` reader - Transfer Direction
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::_0,
            true => Dir::_1,
        }
    }
    ///Receiving direction
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dir::_0
    }
    ///Transmitting direction
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dir::_1
    }
}
///Field `DIR` writer - Transfer Direction
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiving direction
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::_0)
    }
    ///Transmitting direction
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::_1)
    }
}
/**Pipe Disabled at End of Transfer

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shtnak {
    ///0: Continue pipe operation after transfer ends
    _0 = 0,
    ///1: Disable pipe after transfer ends
    _1 = 1,
}
impl From<Shtnak> for bool {
    #[inline(always)]
    fn from(variant: Shtnak) -> Self {
        variant as u8 != 0
    }
}
///Field `SHTNAK` reader - Pipe Disabled at End of Transfer
pub type ShtnakR = crate::BitReader<Shtnak>;
impl ShtnakR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Shtnak {
        match self.bits {
            false => Shtnak::_0,
            true => Shtnak::_1,
        }
    }
    ///Continue pipe operation after transfer ends
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Shtnak::_0
    }
    ///Disable pipe after transfer ends
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Shtnak::_1
    }
}
///Field `SHTNAK` writer - Pipe Disabled at End of Transfer
pub type ShtnakW<'a, REG> = crate::BitWriter<'a, REG, Shtnak>;
impl<'a, REG> ShtnakW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Continue pipe operation after transfer ends
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Shtnak::_0)
    }
    ///Disable pipe after transfer ends
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Shtnak::_1)
    }
}
/**Double Buffer Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dblb {
    ///0: Single buffer
    _0 = 0,
    ///1: Double buffer
    _1 = 1,
}
impl From<Dblb> for bool {
    #[inline(always)]
    fn from(variant: Dblb) -> Self {
        variant as u8 != 0
    }
}
///Field `DBLB` reader - Double Buffer Mode
pub type DblbR = crate::BitReader<Dblb>;
impl DblbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dblb {
        match self.bits {
            false => Dblb::_0,
            true => Dblb::_1,
        }
    }
    ///Single buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dblb::_0
    }
    ///Double buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dblb::_1
    }
}
///Field `DBLB` writer - Double Buffer Mode
pub type DblbW<'a, REG> = crate::BitWriter<'a, REG, Dblb>;
impl<'a, REG> DblbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single buffer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dblb::_0)
    }
    ///Double buffer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dblb::_1)
    }
}
/**BRDY Interrupt Operation Specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfre {
    ///0: Generate BRDY interrupt on transmitting or receiving data
    _0 = 0,
    ///1: Generate BRDY interrupt on completion of reading data
    _1 = 1,
}
impl From<Bfre> for bool {
    #[inline(always)]
    fn from(variant: Bfre) -> Self {
        variant as u8 != 0
    }
}
///Field `BFRE` reader - BRDY Interrupt Operation Specification
pub type BfreR = crate::BitReader<Bfre>;
impl BfreR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bfre {
        match self.bits {
            false => Bfre::_0,
            true => Bfre::_1,
        }
    }
    ///Generate BRDY interrupt on transmitting or receiving data
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bfre::_0
    }
    ///Generate BRDY interrupt on completion of reading data
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bfre::_1
    }
}
///Field `BFRE` writer - BRDY Interrupt Operation Specification
pub type BfreW<'a, REG> = crate::BitWriter<'a, REG, Bfre>;
impl<'a, REG> BfreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generate BRDY interrupt on transmitting or receiving data
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bfre::_0)
    }
    ///Generate BRDY interrupt on completion of reading data
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfre::_1)
    }
}
/**Transfer Type

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    ///0: Pipe not used
    _00 = 0,
    ///1: Pipes 1 and 2: Bulk transfer Pipes 3 to 5: Bulk transfer Pipes 6 to 9: Setting prohibited
    _01 = 1,
    ///2: Pipes 1 and 2: Setting prohibited Pipes 3 to 5: Setting prohibited Pipes 6 to 9: Interrupt transfer
    _10 = 2,
    ///3: Pipes 1 and 2: Isochronous transfer Pipes 3 to 5: Setting prohibited Pipes 6 to 9: Setting prohibited
    _11 = 3,
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(variant: Type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Type {
    type Ux = u8;
}
impl crate::IsEnum for Type {}
///Field `TYPE` reader - Transfer Type
pub type TypeR = crate::FieldReader<Type>;
impl TypeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Type {
        match self.bits {
            0 => Type::_00,
            1 => Type::_01,
            2 => Type::_10,
            3 => Type::_11,
            _ => unreachable!(),
        }
    }
    ///Pipe not used
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Type::_00
    }
    ///Pipes 1 and 2: Bulk transfer Pipes 3 to 5: Bulk transfer Pipes 6 to 9: Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Type::_01
    }
    ///Pipes 1 and 2: Setting prohibited Pipes 3 to 5: Setting prohibited Pipes 6 to 9: Interrupt transfer
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Type::_10
    }
    ///Pipes 1 and 2: Isochronous transfer Pipes 3 to 5: Setting prohibited Pipes 6 to 9: Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Type::_11
    }
}
///Field `TYPE` writer - Transfer Type
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Type, crate::Safe>;
impl<'a, REG> TypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Pipe not used
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Type::_00)
    }
    ///Pipes 1 and 2: Bulk transfer Pipes 3 to 5: Bulk transfer Pipes 6 to 9: Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Type::_01)
    }
    ///Pipes 1 and 2: Setting prohibited Pipes 3 to 5: Setting prohibited Pipes 6 to 9: Interrupt transfer
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Type::_10)
    }
    ///Pipes 1 and 2: Isochronous transfer Pipes 3 to 5: Setting prohibited Pipes 6 to 9: Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Type::_11)
    }
}
impl R {
    ///Bits 0:3 - Endpoint Number
    #[inline(always)]
    pub fn epnum(&self) -> EpnumR {
        EpnumR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Transfer Direction
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Pipe Disabled at End of Transfer
    #[inline(always)]
    pub fn shtnak(&self) -> ShtnakR {
        ShtnakR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Double Buffer Mode
    #[inline(always)]
    pub fn dblb(&self) -> DblbR {
        DblbR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BRDY Interrupt Operation Specification
    #[inline(always)]
    pub fn bfre(&self) -> BfreR {
        BfreR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 14:15 - Transfer Type
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPECFG")
            .field("epnum", &self.epnum())
            .field("dir", &self.dir())
            .field("shtnak", &self.shtnak())
            .field("dblb", &self.dblb())
            .field("bfre", &self.bfre())
            .field("type_", &self.type_())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Endpoint Number
    #[inline(always)]
    pub fn epnum(&mut self) -> EpnumW<PipecfgSpec> {
        EpnumW::new(self, 0)
    }
    ///Bit 4 - Transfer Direction
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<PipecfgSpec> {
        DirW::new(self, 4)
    }
    ///Bit 7 - Pipe Disabled at End of Transfer
    #[inline(always)]
    pub fn shtnak(&mut self) -> ShtnakW<PipecfgSpec> {
        ShtnakW::new(self, 7)
    }
    ///Bit 9 - Double Buffer Mode
    #[inline(always)]
    pub fn dblb(&mut self) -> DblbW<PipecfgSpec> {
        DblbW::new(self, 9)
    }
    ///Bit 10 - BRDY Interrupt Operation Specification
    #[inline(always)]
    pub fn bfre(&mut self) -> BfreW<PipecfgSpec> {
        BfreW::new(self, 10)
    }
    ///Bits 14:15 - Transfer Type
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<PipecfgSpec> {
        TypeW::new(self, 14)
    }
}
/**Pipe Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pipecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PipecfgSpec;
impl crate::RegisterSpec for PipecfgSpec {
    type Ux = u16;
}
///`read()` method returns [`pipecfg::R`](R) reader structure
impl crate::Readable for PipecfgSpec {}
///`write(|w| ..)` method takes [`pipecfg::W`](W) writer structure
impl crate::Writable for PipecfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPECFG to value 0
impl crate::Resettable for PipecfgSpec {}
