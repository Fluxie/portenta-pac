///Register `CFDGERFL` reader
pub type R = crate::R<CfdgerflSpec>;
///Register `CFDGERFL` writer
pub type W = crate::W<CfdgerflSpec>;
/**DLC Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Def {
    ///0: DLC error not detected
    _0 = 0,
    ///1: DLC error detected
    _1 = 1,
}
impl From<Def> for bool {
    #[inline(always)]
    fn from(variant: Def) -> Self {
        variant as u8 != 0
    }
}
///Field `DEF` reader - DLC Error Flag
pub type DefR = crate::BitReader<Def>;
impl DefR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Def {
        match self.bits {
            false => Def::_0,
            true => Def::_1,
        }
    }
    ///DLC error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Def::_0
    }
    ///DLC error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Def::_1
    }
}
///Field `DEF` writer - DLC Error Flag
pub type DefW<'a, REG> = crate::BitWriter<'a, REG, Def>;
impl<'a, REG> DefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DLC error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Def::_0)
    }
    ///DLC error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Def::_1)
    }
}
/**Message Lost Error Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mes {
    ///0: Message lost error not detected
    _0 = 0,
    ///1: Message lost error detected
    _1 = 1,
}
impl From<Mes> for bool {
    #[inline(always)]
    fn from(variant: Mes) -> Self {
        variant as u8 != 0
    }
}
///Field `MES` reader - Message Lost Error Status
pub type MesR = crate::BitReader<Mes>;
impl MesR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mes {
        match self.bits {
            false => Mes::_0,
            true => Mes::_1,
        }
    }
    ///Message lost error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mes::_0
    }
    ///Message lost error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mes::_1
    }
}
/**TX History List Entry Lost Error Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thles {
    ///0: TX history list entry lost error not detected
    _0 = 0,
    ///1: TX history list entry lost error detected
    _1 = 1,
}
impl From<Thles> for bool {
    #[inline(always)]
    fn from(variant: Thles) -> Self {
        variant as u8 != 0
    }
}
///Field `THLES` reader - TX History List Entry Lost Error Status
pub type ThlesR = crate::BitReader<Thles>;
impl ThlesR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thles {
        match self.bits {
            false => Thles::_0,
            true => Thles::_1,
        }
    }
    ///TX history list entry lost error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thles::_0
    }
    ///TX history list entry lost error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thles::_1
    }
}
/**CANFD Message Payload Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpof {
    ///0: CANFD message payload overflow not detected
    _0 = 0,
    ///1: CANFD message payload overflow detected
    _1 = 1,
}
impl From<Cmpof> for bool {
    #[inline(always)]
    fn from(variant: Cmpof) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPOF` reader - CANFD Message Payload Overflow Flag
pub type CmpofR = crate::BitReader<Cmpof>;
impl CmpofR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpof {
        match self.bits {
            false => Cmpof::_0,
            true => Cmpof::_1,
        }
    }
    ///CANFD message payload overflow not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpof::_0
    }
    ///CANFD message payload overflow detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpof::_1
    }
}
///Field `CMPOF` writer - CANFD Message Payload Overflow Flag
pub type CmpofW<'a, REG> = crate::BitWriter<'a, REG, Cmpof>;
impl<'a, REG> CmpofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CANFD message payload overflow not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpof::_0)
    }
    ///CANFD message payload overflow detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpof::_1)
    }
}
/**TXQ Message Lost Error Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qmes {
    ///0: TXQ message lost error not detected
    _0 = 0,
    ///1: TXQ message lost error detected
    _1 = 1,
}
impl From<Qmes> for bool {
    #[inline(always)]
    fn from(variant: Qmes) -> Self {
        variant as u8 != 0
    }
}
///Field `QMES` reader - TXQ Message Lost Error Status
pub type QmesR = crate::BitReader<Qmes>;
impl QmesR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Qmes {
        match self.bits {
            false => Qmes::_0,
            true => Qmes::_1,
        }
    }
    ///TXQ message lost error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Qmes::_0
    }
    ///TXQ message lost error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Qmes::_1
    }
}
/**ECC Error Flag for Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eef0 {
    ///0: ECC error not detected during TX-SCAN
    _0 = 0,
    ///1: ECC error detected during TX-SCAN
    _1 = 1,
}
impl From<Eef0> for bool {
    #[inline(always)]
    fn from(variant: Eef0) -> Self {
        variant as u8 != 0
    }
}
///Field `EEF0` reader - ECC Error Flag for Channel 0
pub type Eef0R = crate::BitReader<Eef0>;
impl Eef0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eef0 {
        match self.bits {
            false => Eef0::_0,
            true => Eef0::_1,
        }
    }
    ///ECC error not detected during TX-SCAN
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eef0::_0
    }
    ///ECC error detected during TX-SCAN
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eef0::_1
    }
}
///Field `EEF0` writer - ECC Error Flag for Channel 0
pub type Eef0W<'a, REG> = crate::BitWriter<'a, REG, Eef0>;
impl<'a, REG> Eef0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ECC error not detected during TX-SCAN
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eef0::_0)
    }
    ///ECC error detected during TX-SCAN
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eef0::_1)
    }
}
/**ECC Error Flag for Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eef1 {
    ///0: ECC error not detected during TX-SCAN
    _0 = 0,
    ///1: ECC error detected during TX-SCAN
    _1 = 1,
}
impl From<Eef1> for bool {
    #[inline(always)]
    fn from(variant: Eef1) -> Self {
        variant as u8 != 0
    }
}
///Field `EEF1` reader - ECC Error Flag for Channel 1
pub type Eef1R = crate::BitReader<Eef1>;
impl Eef1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eef1 {
        match self.bits {
            false => Eef1::_0,
            true => Eef1::_1,
        }
    }
    ///ECC error not detected during TX-SCAN
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eef1::_0
    }
    ///ECC error detected during TX-SCAN
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eef1::_1
    }
}
///Field `EEF1` writer - ECC Error Flag for Channel 1
pub type Eef1W<'a, REG> = crate::BitWriter<'a, REG, Eef1>;
impl<'a, REG> Eef1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ECC error not detected during TX-SCAN
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eef1::_0)
    }
    ///ECC error detected during TX-SCAN
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eef1::_1)
    }
}
impl R {
    ///Bit 0 - DLC Error Flag
    #[inline(always)]
    pub fn def(&self) -> DefR {
        DefR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Message Lost Error Status
    #[inline(always)]
    pub fn mes(&self) -> MesR {
        MesR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TX History List Entry Lost Error Status
    #[inline(always)]
    pub fn thles(&self) -> ThlesR {
        ThlesR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CANFD Message Payload Overflow Flag
    #[inline(always)]
    pub fn cmpof(&self) -> CmpofR {
        CmpofR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - TXQ Message Lost Error Status
    #[inline(always)]
    pub fn qmes(&self) -> QmesR {
        QmesR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - ECC Error Flag for Channel 0
    #[inline(always)]
    pub fn eef0(&self) -> Eef0R {
        Eef0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ECC Error Flag for Channel 1
    #[inline(always)]
    pub fn eef1(&self) -> Eef1R {
        Eef1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGERFL")
            .field("def", &self.def())
            .field("mes", &self.mes())
            .field("thles", &self.thles())
            .field("cmpof", &self.cmpof())
            .field("qmes", &self.qmes())
            .field("eef0", &self.eef0())
            .field("eef1", &self.eef1())
            .finish()
    }
}
impl W {
    ///Bit 0 - DLC Error Flag
    #[inline(always)]
    pub fn def(&mut self) -> DefW<CfdgerflSpec> {
        DefW::new(self, 0)
    }
    ///Bit 3 - CANFD Message Payload Overflow Flag
    #[inline(always)]
    pub fn cmpof(&mut self) -> CmpofW<CfdgerflSpec> {
        CmpofW::new(self, 3)
    }
    ///Bit 16 - ECC Error Flag for Channel 0
    #[inline(always)]
    pub fn eef0(&mut self) -> Eef0W<CfdgerflSpec> {
        Eef0W::new(self, 16)
    }
    ///Bit 17 - ECC Error Flag for Channel 1
    #[inline(always)]
    pub fn eef1(&mut self) -> Eef1W<CfdgerflSpec> {
        Eef1W::new(self, 17)
    }
}
/**Global Error Flag Register

You can [`read`](crate::Reg::read) this register and get [`cfdgerfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgerfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgerflSpec;
impl crate::RegisterSpec for CfdgerflSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgerfl::R`](R) reader structure
impl crate::Readable for CfdgerflSpec {}
///`write(|w| ..)` method takes [`cfdgerfl::W`](W) writer structure
impl crate::Writable for CfdgerflSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGERFL to value 0
impl crate::Resettable for CfdgerflSpec {}
