///Register `SIMR3` reader
pub type R = crate::R<Simr3Spec>;
///Register `SIMR3` writer
pub type W = crate::W<Simr3Spec>;
/**Start Condition Generation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicstareq {
    ///0: Do not generate start condition
    _0 = 0,
    ///1: Generate start condition
    _1 = 1,
}
impl From<Iicstareq> for bool {
    #[inline(always)]
    fn from(variant: Iicstareq) -> Self {
        variant as u8 != 0
    }
}
///Field `IICSTAREQ` reader - Start Condition Generation
pub type IicstareqR = crate::BitReader<Iicstareq>;
impl IicstareqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicstareq {
        match self.bits {
            false => Iicstareq::_0,
            true => Iicstareq::_1,
        }
    }
    ///Do not generate start condition
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicstareq::_0
    }
    ///Generate start condition
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicstareq::_1
    }
}
///Field `IICSTAREQ` writer - Start Condition Generation
pub type IicstareqW<'a, REG> = crate::BitWriter<'a, REG, Iicstareq>;
impl<'a, REG> IicstareqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not generate start condition
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstareq::_0)
    }
    ///Generate start condition
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstareq::_1)
    }
}
/**Restart Condition Generation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicrstareq {
    ///0: Do not generate restart condition
    _0 = 0,
    ///1: Generate restart condition
    _1 = 1,
}
impl From<Iicrstareq> for bool {
    #[inline(always)]
    fn from(variant: Iicrstareq) -> Self {
        variant as u8 != 0
    }
}
///Field `IICRSTAREQ` reader - Restart Condition Generation
pub type IicrstareqR = crate::BitReader<Iicrstareq>;
impl IicrstareqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicrstareq {
        match self.bits {
            false => Iicrstareq::_0,
            true => Iicrstareq::_1,
        }
    }
    ///Do not generate restart condition
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicrstareq::_0
    }
    ///Generate restart condition
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicrstareq::_1
    }
}
///Field `IICRSTAREQ` writer - Restart Condition Generation
pub type IicrstareqW<'a, REG> = crate::BitWriter<'a, REG, Iicrstareq>;
impl<'a, REG> IicrstareqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not generate restart condition
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicrstareq::_0)
    }
    ///Generate restart condition
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicrstareq::_1)
    }
}
/**Stop Condition Generation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicstpreq {
    ///0: Do not generate stop condition
    _0 = 0,
    ///1: Generate stop condition
    _1 = 1,
}
impl From<Iicstpreq> for bool {
    #[inline(always)]
    fn from(variant: Iicstpreq) -> Self {
        variant as u8 != 0
    }
}
///Field `IICSTPREQ` reader - Stop Condition Generation
pub type IicstpreqR = crate::BitReader<Iicstpreq>;
impl IicstpreqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicstpreq {
        match self.bits {
            false => Iicstpreq::_0,
            true => Iicstpreq::_1,
        }
    }
    ///Do not generate stop condition
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicstpreq::_0
    }
    ///Generate stop condition
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicstpreq::_1
    }
}
///Field `IICSTPREQ` writer - Stop Condition Generation
pub type IicstpreqW<'a, REG> = crate::BitWriter<'a, REG, Iicstpreq>;
impl<'a, REG> IicstpreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not generate stop condition
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstpreq::_0)
    }
    ///Generate stop condition
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstpreq::_1)
    }
}
/**Issuing of Start, Restart, or Stop Condition Completed Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicstif {
    ///0: No requests are being made for generating conditions, or a condition is being generated
    _0 = 0,
    ///1: Generation of start, restart, or stop condition is complete. When 0 is written to IICSTIF, it is set to 0
    _1 = 1,
}
impl From<Iicstif> for bool {
    #[inline(always)]
    fn from(variant: Iicstif) -> Self {
        variant as u8 != 0
    }
}
///Field `IICSTIF` reader - Issuing of Start, Restart, or Stop Condition Completed Flag
pub type IicstifR = crate::BitReader<Iicstif>;
impl IicstifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicstif {
        match self.bits {
            false => Iicstif::_0,
            true => Iicstif::_1,
        }
    }
    ///No requests are being made for generating conditions, or a condition is being generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicstif::_0
    }
    ///Generation of start, restart, or stop condition is complete. When 0 is written to IICSTIF, it is set to 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicstif::_1
    }
}
///Field `IICSTIF` writer - Issuing of Start, Restart, or Stop Condition Completed Flag
pub type IicstifW<'a, REG> = crate::BitWriter<'a, REG, Iicstif>;
impl<'a, REG> IicstifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No requests are being made for generating conditions, or a condition is being generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstif::_0)
    }
    ///Generation of start, restart, or stop condition is complete. When 0 is written to IICSTIF, it is set to 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstif::_1)
    }
}
/**SDAn Output Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iicsdas {
    ///0: Output serial data
    _00 = 0,
    ///1: Generate start, restart, or stop condition
    _01 = 1,
    ///2: Output low on SDAn pin
    _10 = 2,
    ///3: Drive SDAn pin to high-impedance state
    _11 = 3,
}
impl From<Iicsdas> for u8 {
    #[inline(always)]
    fn from(variant: Iicsdas) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iicsdas {
    type Ux = u8;
}
impl crate::IsEnum for Iicsdas {}
///Field `IICSDAS` reader - SDAn Output Select
pub type IicsdasR = crate::FieldReader<Iicsdas>;
impl IicsdasR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicsdas {
        match self.bits {
            0 => Iicsdas::_00,
            1 => Iicsdas::_01,
            2 => Iicsdas::_10,
            3 => Iicsdas::_11,
            _ => unreachable!(),
        }
    }
    ///Output serial data
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Iicsdas::_00
    }
    ///Generate start, restart, or stop condition
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Iicsdas::_01
    }
    ///Output low on SDAn pin
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Iicsdas::_10
    }
    ///Drive SDAn pin to high-impedance state
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Iicsdas::_11
    }
}
///Field `IICSDAS` writer - SDAn Output Select
pub type IicsdasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iicsdas, crate::Safe>;
impl<'a, REG> IicsdasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Output serial data
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Iicsdas::_00)
    }
    ///Generate start, restart, or stop condition
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Iicsdas::_01)
    }
    ///Output low on SDAn pin
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Iicsdas::_10)
    }
    ///Drive SDAn pin to high-impedance state
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Iicsdas::_11)
    }
}
/**SCLn Output Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iicscls {
    ///0: Output serial clock
    _00 = 0,
    ///1: Generate start, restart, or stop condition
    _01 = 1,
    ///2: Output low on SCLn pin
    _10 = 2,
    ///3: Drive SCLn pin to high-impedance state
    _11 = 3,
}
impl From<Iicscls> for u8 {
    #[inline(always)]
    fn from(variant: Iicscls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iicscls {
    type Ux = u8;
}
impl crate::IsEnum for Iicscls {}
///Field `IICSCLS` reader - SCLn Output Select
pub type IicsclsR = crate::FieldReader<Iicscls>;
impl IicsclsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicscls {
        match self.bits {
            0 => Iicscls::_00,
            1 => Iicscls::_01,
            2 => Iicscls::_10,
            3 => Iicscls::_11,
            _ => unreachable!(),
        }
    }
    ///Output serial clock
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Iicscls::_00
    }
    ///Generate start, restart, or stop condition
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Iicscls::_01
    }
    ///Output low on SCLn pin
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Iicscls::_10
    }
    ///Drive SCLn pin to high-impedance state
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Iicscls::_11
    }
}
///Field `IICSCLS` writer - SCLn Output Select
pub type IicsclsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iicscls, crate::Safe>;
impl<'a, REG> IicsclsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Output serial clock
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Iicscls::_00)
    }
    ///Generate start, restart, or stop condition
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Iicscls::_01)
    }
    ///Output low on SCLn pin
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Iicscls::_10)
    }
    ///Drive SCLn pin to high-impedance state
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Iicscls::_11)
    }
}
impl R {
    ///Bit 0 - Start Condition Generation
    #[inline(always)]
    pub fn iicstareq(&self) -> IicstareqR {
        IicstareqR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Restart Condition Generation
    #[inline(always)]
    pub fn iicrstareq(&self) -> IicrstareqR {
        IicrstareqR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Stop Condition Generation
    #[inline(always)]
    pub fn iicstpreq(&self) -> IicstpreqR {
        IicstpreqR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Issuing of Start, Restart, or Stop Condition Completed Flag
    #[inline(always)]
    pub fn iicstif(&self) -> IicstifR {
        IicstifR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - SDAn Output Select
    #[inline(always)]
    pub fn iicsdas(&self) -> IicsdasR {
        IicsdasR::new((self.bits >> 4) & 3)
    }
    ///Bits 6:7 - SCLn Output Select
    #[inline(always)]
    pub fn iicscls(&self) -> IicsclsR {
        IicsclsR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIMR3")
            .field("iicstareq", &self.iicstareq())
            .field("iicrstareq", &self.iicrstareq())
            .field("iicstpreq", &self.iicstpreq())
            .field("iicstif", &self.iicstif())
            .field("iicsdas", &self.iicsdas())
            .field("iicscls", &self.iicscls())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start Condition Generation
    #[inline(always)]
    pub fn iicstareq(&mut self) -> IicstareqW<Simr3Spec> {
        IicstareqW::new(self, 0)
    }
    ///Bit 1 - Restart Condition Generation
    #[inline(always)]
    pub fn iicrstareq(&mut self) -> IicrstareqW<Simr3Spec> {
        IicrstareqW::new(self, 1)
    }
    ///Bit 2 - Stop Condition Generation
    #[inline(always)]
    pub fn iicstpreq(&mut self) -> IicstpreqW<Simr3Spec> {
        IicstpreqW::new(self, 2)
    }
    ///Bit 3 - Issuing of Start, Restart, or Stop Condition Completed Flag
    #[inline(always)]
    pub fn iicstif(&mut self) -> IicstifW<Simr3Spec> {
        IicstifW::new(self, 3)
    }
    ///Bits 4:5 - SDAn Output Select
    #[inline(always)]
    pub fn iicsdas(&mut self) -> IicsdasW<Simr3Spec> {
        IicsdasW::new(self, 4)
    }
    ///Bits 6:7 - SCLn Output Select
    #[inline(always)]
    pub fn iicscls(&mut self) -> IicsclsW<Simr3Spec> {
        IicsclsW::new(self, 6)
    }
}
/**IIC Mode Register 3

You can [`read`](crate::Reg::read) this register and get [`simr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Simr3Spec;
impl crate::RegisterSpec for Simr3Spec {
    type Ux = u8;
}
///`read()` method returns [`simr3::R`](R) reader structure
impl crate::Readable for Simr3Spec {}
///`write(|w| ..)` method takes [`simr3::W`](W) writer structure
impl crate::Writable for Simr3Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SIMR3 to value 0
impl crate::Resettable for Simr3Spec {}
