///Register `ICWUR` reader
pub type R = crate::R<IcwurSpec>;
///Register `ICWUR` writer
pub type W = crate::W<IcwurSpec>;
/**Wakeup Analog Filter Additional Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuafa {
    ///0: Do not add the wakeup analog filter
    _0 = 0,
    ///1: Add the wakeup analog filter
    _1 = 1,
}
impl From<Wuafa> for bool {
    #[inline(always)]
    fn from(variant: Wuafa) -> Self {
        variant as u8 != 0
    }
}
///Field `WUAFA` reader - Wakeup Analog Filter Additional Selection
pub type WuafaR = crate::BitReader<Wuafa>;
impl WuafaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wuafa {
        match self.bits {
            false => Wuafa::_0,
            true => Wuafa::_1,
        }
    }
    ///Do not add the wakeup analog filter
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuafa::_0
    }
    ///Add the wakeup analog filter
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuafa::_1
    }
}
///Field `WUAFA` writer - Wakeup Analog Filter Additional Selection
pub type WuafaW<'a, REG> = crate::BitWriter<'a, REG, Wuafa>;
impl<'a, REG> WuafaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not add the wakeup analog filter
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuafa::_0)
    }
    ///Add the wakeup analog filter
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuafa::_1)
    }
}
///Field `WUACK` reader - ACK Bit for Wakeup Mode
pub type WuackR = crate::BitReader;
///Field `WUACK` writer - ACK Bit for Wakeup Mode
pub type WuackW<'a, REG> = crate::BitWriter<'a, REG>;
/**Wakeup Event Occurrence Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf {
    ///0: Slave address not matching during wakeup
    _0 = 0,
    ///1: Slave address matching during wakeup
    _1 = 1,
}
impl From<Wuf> for bool {
    #[inline(always)]
    fn from(variant: Wuf) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF` reader - Wakeup Event Occurrence Flag
pub type WufR = crate::BitReader<Wuf>;
impl WufR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wuf {
        match self.bits {
            false => Wuf::_0,
            true => Wuf::_1,
        }
    }
    ///Slave address not matching during wakeup
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf::_0
    }
    ///Slave address matching during wakeup
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf::_1
    }
}
///Field `WUF` writer - Wakeup Event Occurrence Flag
pub type WufW<'a, REG> = crate::BitWriter<'a, REG, Wuf>;
impl<'a, REG> WufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave address not matching during wakeup
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf::_0)
    }
    ///Slave address matching during wakeup
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf::_1)
    }
}
/**Wakeup Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuie {
    ///0: Disable wakeup interrupt request (IIC0_WUI)
    _0 = 0,
    ///1: Enable wakeup interrupt request (IIC0_WUI)
    _1 = 1,
}
impl From<Wuie> for bool {
    #[inline(always)]
    fn from(variant: Wuie) -> Self {
        variant as u8 != 0
    }
}
///Field `WUIE` reader - Wakeup Interrupt Request Enable
pub type WuieR = crate::BitReader<Wuie>;
impl WuieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wuie {
        match self.bits {
            false => Wuie::_0,
            true => Wuie::_1,
        }
    }
    ///Disable wakeup interrupt request (IIC0_WUI)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuie::_0
    }
    ///Enable wakeup interrupt request (IIC0_WUI)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuie::_1
    }
}
///Field `WUIE` writer - Wakeup Interrupt Request Enable
pub type WuieW<'a, REG> = crate::BitWriter<'a, REG, Wuie>;
impl<'a, REG> WuieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable wakeup interrupt request (IIC0_WUI)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuie::_0)
    }
    ///Enable wakeup interrupt request (IIC0_WUI)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuie::_1)
    }
}
/**Wakeup Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wue {
    ///0: Disable wakeup function
    _0 = 0,
    ///1: Enable wakeup function
    _1 = 1,
}
impl From<Wue> for bool {
    #[inline(always)]
    fn from(variant: Wue) -> Self {
        variant as u8 != 0
    }
}
///Field `WUE` reader - Wakeup Function Enable
pub type WueR = crate::BitReader<Wue>;
impl WueR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wue {
        match self.bits {
            false => Wue::_0,
            true => Wue::_1,
        }
    }
    ///Disable wakeup function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wue::_0
    }
    ///Enable wakeup function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wue::_1
    }
}
///Field `WUE` writer - Wakeup Function Enable
pub type WueW<'a, REG> = crate::BitWriter<'a, REG, Wue>;
impl<'a, REG> WueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable wakeup function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wue::_0)
    }
    ///Enable wakeup function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wue::_1)
    }
}
impl R {
    ///Bit 0 - Wakeup Analog Filter Additional Selection
    #[inline(always)]
    pub fn wuafa(&self) -> WuafaR {
        WuafaR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - ACK Bit for Wakeup Mode
    #[inline(always)]
    pub fn wuack(&self) -> WuackR {
        WuackR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wakeup Event Occurrence Flag
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup Interrupt Request Enable
    #[inline(always)]
    pub fn wuie(&self) -> WuieR {
        WuieR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Wakeup Function Enable
    #[inline(always)]
    pub fn wue(&self) -> WueR {
        WueR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICWUR")
            .field("wuafa", &self.wuafa())
            .field("wuack", &self.wuack())
            .field("wuf", &self.wuf())
            .field("wuie", &self.wuie())
            .field("wue", &self.wue())
            .finish()
    }
}
impl W {
    ///Bit 0 - Wakeup Analog Filter Additional Selection
    #[inline(always)]
    pub fn wuafa(&mut self) -> WuafaW<IcwurSpec> {
        WuafaW::new(self, 0)
    }
    ///Bit 4 - ACK Bit for Wakeup Mode
    #[inline(always)]
    pub fn wuack(&mut self) -> WuackW<IcwurSpec> {
        WuackW::new(self, 4)
    }
    ///Bit 5 - Wakeup Event Occurrence Flag
    #[inline(always)]
    pub fn wuf(&mut self) -> WufW<IcwurSpec> {
        WufW::new(self, 5)
    }
    ///Bit 6 - Wakeup Interrupt Request Enable
    #[inline(always)]
    pub fn wuie(&mut self) -> WuieW<IcwurSpec> {
        WuieW::new(self, 6)
    }
    ///Bit 7 - Wakeup Function Enable
    #[inline(always)]
    pub fn wue(&mut self) -> WueW<IcwurSpec> {
        WueW::new(self, 7)
    }
}
/**I2C Bus Wakeup Unit Register

You can [`read`](crate::Reg::read) this register and get [`icwur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icwur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcwurSpec;
impl crate::RegisterSpec for IcwurSpec {
    type Ux = u8;
}
///`read()` method returns [`icwur::R`](R) reader structure
impl crate::Readable for IcwurSpec {}
///`write(|w| ..)` method takes [`icwur::W`](W) writer structure
impl crate::Writable for IcwurSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICWUR to value 0x10
impl crate::Resettable for IcwurSpec {
    const RESET_VALUE: u8 = 0x10;
}
