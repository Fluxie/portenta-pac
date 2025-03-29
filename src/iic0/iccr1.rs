///Register `ICCR1` reader
pub type R = crate::R<Iccr1Spec>;
///Register `ICCR1` writer
pub type W = crate::W<Iccr1Spec>;
/**SDA Line Monitor

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdai {
    ///0: SDAn line is low
    _0 = 0,
    ///1: SDAn line is high
    _1 = 1,
}
impl From<Sdai> for bool {
    #[inline(always)]
    fn from(variant: Sdai) -> Self {
        variant as u8 != 0
    }
}
///Field `SDAI` reader - SDA Line Monitor
pub type SdaiR = crate::BitReader<Sdai>;
impl SdaiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdai {
        match self.bits {
            false => Sdai::_0,
            true => Sdai::_1,
        }
    }
    ///SDAn line is low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdai::_0
    }
    ///SDAn line is high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdai::_1
    }
}
/**SCL Line Monitor

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scli {
    ///0: SCLn line is low
    _0 = 0,
    ///1: SCLn line is high
    _1 = 1,
}
impl From<Scli> for bool {
    #[inline(always)]
    fn from(variant: Scli) -> Self {
        variant as u8 != 0
    }
}
///Field `SCLI` reader - SCL Line Monitor
pub type ScliR = crate::BitReader<Scli>;
impl ScliR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Scli {
        match self.bits {
            false => Scli::_0,
            true => Scli::_1,
        }
    }
    ///SCLn line is low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Scli::_0
    }
    ///SCLn line is high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Scli::_1
    }
}
/**SDA Output Control/Monitor

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdao {
    ///0: Read: IIC drives SDAn pin low Write: IIC drives SDAn pin low
    _0 = 0,
    ///1: Read: IIC releases SDAn pin Write: IIC releases SDAn pin
    _1 = 1,
}
impl From<Sdao> for bool {
    #[inline(always)]
    fn from(variant: Sdao) -> Self {
        variant as u8 != 0
    }
}
///Field `SDAO` reader - SDA Output Control/Monitor
pub type SdaoR = crate::BitReader<Sdao>;
impl SdaoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdao {
        match self.bits {
            false => Sdao::_0,
            true => Sdao::_1,
        }
    }
    ///Read: IIC drives SDAn pin low Write: IIC drives SDAn pin low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdao::_0
    }
    ///Read: IIC releases SDAn pin Write: IIC releases SDAn pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdao::_1
    }
}
///Field `SDAO` writer - SDA Output Control/Monitor
pub type SdaoW<'a, REG> = crate::BitWriter<'a, REG, Sdao>;
impl<'a, REG> SdaoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read: IIC drives SDAn pin low Write: IIC drives SDAn pin low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdao::_0)
    }
    ///Read: IIC releases SDAn pin Write: IIC releases SDAn pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdao::_1)
    }
}
/**SCL Output Control/Monitor

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sclo {
    ///0: Read: IIC drives SCLn pin low Write: IIC drives SCLn pin low
    _0 = 0,
    ///1: Read: IIC releases SCLn pin Write: IIC releases SCLn pin
    _1 = 1,
}
impl From<Sclo> for bool {
    #[inline(always)]
    fn from(variant: Sclo) -> Self {
        variant as u8 != 0
    }
}
///Field `SCLO` reader - SCL Output Control/Monitor
pub type ScloR = crate::BitReader<Sclo>;
impl ScloR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sclo {
        match self.bits {
            false => Sclo::_0,
            true => Sclo::_1,
        }
    }
    ///Read: IIC drives SCLn pin low Write: IIC drives SCLn pin low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sclo::_0
    }
    ///Read: IIC releases SCLn pin Write: IIC releases SCLn pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sclo::_1
    }
}
///Field `SCLO` writer - SCL Output Control/Monitor
pub type ScloW<'a, REG> = crate::BitWriter<'a, REG, Sclo>;
impl<'a, REG> ScloW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read: IIC drives SCLn pin low Write: IIC drives SCLn pin low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sclo::_0)
    }
    ///Read: IIC releases SCLn pin Write: IIC releases SCLn pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sclo::_1)
    }
}
/**SCLO/SDAO Write Protect

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sowp {
    ///0: Write enable SCLO and SDAO bits
    _0 = 0,
    ///1: Write protect SCLO and SDAO bits
    _1 = 1,
}
impl From<Sowp> for bool {
    #[inline(always)]
    fn from(variant: Sowp) -> Self {
        variant as u8 != 0
    }
}
///Field `SOWP` writer - SCLO/SDAO Write Protect
pub type SowpW<'a, REG> = crate::BitWriter<'a, REG, Sowp>;
impl<'a, REG> SowpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write enable SCLO and SDAO bits
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sowp::_0)
    }
    ///Write protect SCLO and SDAO bits
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sowp::_1)
    }
}
/**Extra SCL Clock Cycle Output

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clo {
    ///0: Do not output extra SCL clock cycle (default)
    _0 = 0,
    ///1: Output extra SCL clock cycle
    _1 = 1,
}
impl From<Clo> for bool {
    #[inline(always)]
    fn from(variant: Clo) -> Self {
        variant as u8 != 0
    }
}
///Field `CLO` reader - Extra SCL Clock Cycle Output
pub type CloR = crate::BitReader<Clo>;
impl CloR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Clo {
        match self.bits {
            false => Clo::_0,
            true => Clo::_1,
        }
    }
    ///Do not output extra SCL clock cycle (default)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Clo::_0
    }
    ///Output extra SCL clock cycle
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Clo::_1
    }
}
///Field `CLO` writer - Extra SCL Clock Cycle Output
pub type CloW<'a, REG> = crate::BitWriter<'a, REG, Clo>;
impl<'a, REG> CloW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not output extra SCL clock cycle (default)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Clo::_0)
    }
    ///Output extra SCL clock cycle
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Clo::_1)
    }
}
/**I2C Bus Interface Internal Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicrst {
    ///0: Release IIC reset or internal reset
    _0 = 0,
    ///1: Initiate IIC reset or internal reset
    _1 = 1,
}
impl From<Iicrst> for bool {
    #[inline(always)]
    fn from(variant: Iicrst) -> Self {
        variant as u8 != 0
    }
}
///Field `IICRST` reader - I2C Bus Interface Internal Reset
pub type IicrstR = crate::BitReader<Iicrst>;
impl IicrstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicrst {
        match self.bits {
            false => Iicrst::_0,
            true => Iicrst::_1,
        }
    }
    ///Release IIC reset or internal reset
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicrst::_0
    }
    ///Initiate IIC reset or internal reset
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicrst::_1
    }
}
///Field `IICRST` writer - I2C Bus Interface Internal Reset
pub type IicrstW<'a, REG> = crate::BitWriter<'a, REG, Iicrst>;
impl<'a, REG> IicrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Release IIC reset or internal reset
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicrst::_0)
    }
    ///Initiate IIC reset or internal reset
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicrst::_1)
    }
}
/**I2C Bus Interface Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ice {
    ///0: Disable (SCLn and SDAn pins in inactive state)
    _0 = 0,
    ///1: Enable (SCLn and SDAn pins in active state)
    _1 = 1,
}
impl From<Ice> for bool {
    #[inline(always)]
    fn from(variant: Ice) -> Self {
        variant as u8 != 0
    }
}
///Field `ICE` reader - I2C Bus Interface Enable
pub type IceR = crate::BitReader<Ice>;
impl IceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ice {
        match self.bits {
            false => Ice::_0,
            true => Ice::_1,
        }
    }
    ///Disable (SCLn and SDAn pins in inactive state)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ice::_0
    }
    ///Enable (SCLn and SDAn pins in active state)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ice::_1
    }
}
///Field `ICE` writer - I2C Bus Interface Enable
pub type IceW<'a, REG> = crate::BitWriter<'a, REG, Ice>;
impl<'a, REG> IceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable (SCLn and SDAn pins in inactive state)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ice::_0)
    }
    ///Enable (SCLn and SDAn pins in active state)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ice::_1)
    }
}
impl R {
    ///Bit 0 - SDA Line Monitor
    #[inline(always)]
    pub fn sdai(&self) -> SdaiR {
        SdaiR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SCL Line Monitor
    #[inline(always)]
    pub fn scli(&self) -> ScliR {
        ScliR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SDA Output Control/Monitor
    #[inline(always)]
    pub fn sdao(&self) -> SdaoR {
        SdaoR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SCL Output Control/Monitor
    #[inline(always)]
    pub fn sclo(&self) -> ScloR {
        ScloR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Extra SCL Clock Cycle Output
    #[inline(always)]
    pub fn clo(&self) -> CloR {
        CloR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I2C Bus Interface Internal Reset
    #[inline(always)]
    pub fn iicrst(&self) -> IicrstR {
        IicrstR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C Bus Interface Enable
    #[inline(always)]
    pub fn ice(&self) -> IceR {
        IceR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICCR1")
            .field("sdai", &self.sdai())
            .field("scli", &self.scli())
            .field("sdao", &self.sdao())
            .field("sclo", &self.sclo())
            .field("clo", &self.clo())
            .field("iicrst", &self.iicrst())
            .field("ice", &self.ice())
            .finish()
    }
}
impl W {
    ///Bit 2 - SDA Output Control/Monitor
    #[inline(always)]
    pub fn sdao(&mut self) -> SdaoW<Iccr1Spec> {
        SdaoW::new(self, 2)
    }
    ///Bit 3 - SCL Output Control/Monitor
    #[inline(always)]
    pub fn sclo(&mut self) -> ScloW<Iccr1Spec> {
        ScloW::new(self, 3)
    }
    ///Bit 4 - SCLO/SDAO Write Protect
    #[inline(always)]
    pub fn sowp(&mut self) -> SowpW<Iccr1Spec> {
        SowpW::new(self, 4)
    }
    ///Bit 5 - Extra SCL Clock Cycle Output
    #[inline(always)]
    pub fn clo(&mut self) -> CloW<Iccr1Spec> {
        CloW::new(self, 5)
    }
    ///Bit 6 - I2C Bus Interface Internal Reset
    #[inline(always)]
    pub fn iicrst(&mut self) -> IicrstW<Iccr1Spec> {
        IicrstW::new(self, 6)
    }
    ///Bit 7 - I2C Bus Interface Enable
    #[inline(always)]
    pub fn ice(&mut self) -> IceW<Iccr1Spec> {
        IceW::new(self, 7)
    }
}
/**I2C Bus Control Register 1

You can [`read`](crate::Reg::read) this register and get [`iccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Iccr1Spec;
impl crate::RegisterSpec for Iccr1Spec {
    type Ux = u8;
}
///`read()` method returns [`iccr1::R`](R) reader structure
impl crate::Readable for Iccr1Spec {}
///`write(|w| ..)` method takes [`iccr1::W`](W) writer structure
impl crate::Writable for Iccr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICCR1 to value 0x1f
impl crate::Resettable for Iccr1Spec {
    const RESET_VALUE: u8 = 0x1f;
}
