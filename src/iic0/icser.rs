///Register `ICSER` reader
pub type R = crate::R<IcserSpec>;
///Register `ICSER` writer
pub type W = crate::W<IcserSpec>;
/**Slave Address Register 0 Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sar0e {
    ///0: Disable slave address in SARL0 and SARU0
    _0 = 0,
    ///1: Enable slave address in SARL0 and SARU0
    _1 = 1,
}
impl From<Sar0e> for bool {
    #[inline(always)]
    fn from(variant: Sar0e) -> Self {
        variant as u8 != 0
    }
}
///Field `SAR0E` reader - Slave Address Register 0 Enable
pub type Sar0eR = crate::BitReader<Sar0e>;
impl Sar0eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sar0e {
        match self.bits {
            false => Sar0e::_0,
            true => Sar0e::_1,
        }
    }
    ///Disable slave address in SARL0 and SARU0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sar0e::_0
    }
    ///Enable slave address in SARL0 and SARU0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sar0e::_1
    }
}
///Field `SAR0E` writer - Slave Address Register 0 Enable
pub type Sar0eW<'a, REG> = crate::BitWriter<'a, REG, Sar0e>;
impl<'a, REG> Sar0eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable slave address in SARL0 and SARU0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sar0e::_0)
    }
    ///Enable slave address in SARL0 and SARU0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sar0e::_1)
    }
}
/**Slave Address Register 1 Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sar1e {
    ///0: Disable slave address in SARL1 and SARU1
    _0 = 0,
    ///1: Enable slave address in SARL1 and SARU1
    _1 = 1,
}
impl From<Sar1e> for bool {
    #[inline(always)]
    fn from(variant: Sar1e) -> Self {
        variant as u8 != 0
    }
}
///Field `SAR1E` reader - Slave Address Register 1 Enable
pub type Sar1eR = crate::BitReader<Sar1e>;
impl Sar1eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sar1e {
        match self.bits {
            false => Sar1e::_0,
            true => Sar1e::_1,
        }
    }
    ///Disable slave address in SARL1 and SARU1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sar1e::_0
    }
    ///Enable slave address in SARL1 and SARU1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sar1e::_1
    }
}
///Field `SAR1E` writer - Slave Address Register 1 Enable
pub type Sar1eW<'a, REG> = crate::BitWriter<'a, REG, Sar1e>;
impl<'a, REG> Sar1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable slave address in SARL1 and SARU1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sar1e::_0)
    }
    ///Enable slave address in SARL1 and SARU1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sar1e::_1)
    }
}
/**Slave Address Register 2 Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sar2e {
    ///0: Disable slave address in SARL2 and SARU2
    _0 = 0,
    ///1: Enable slave address in SARL2 and SARU2
    _1 = 1,
}
impl From<Sar2e> for bool {
    #[inline(always)]
    fn from(variant: Sar2e) -> Self {
        variant as u8 != 0
    }
}
///Field `SAR2E` reader - Slave Address Register 2 Enable
pub type Sar2eR = crate::BitReader<Sar2e>;
impl Sar2eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sar2e {
        match self.bits {
            false => Sar2e::_0,
            true => Sar2e::_1,
        }
    }
    ///Disable slave address in SARL2 and SARU2
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sar2e::_0
    }
    ///Enable slave address in SARL2 and SARU2
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sar2e::_1
    }
}
///Field `SAR2E` writer - Slave Address Register 2 Enable
pub type Sar2eW<'a, REG> = crate::BitWriter<'a, REG, Sar2e>;
impl<'a, REG> Sar2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable slave address in SARL2 and SARU2
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sar2e::_0)
    }
    ///Enable slave address in SARL2 and SARU2
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sar2e::_1)
    }
}
/**General Call Address Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gcae {
    ///0: Disable general call address detection
    _0 = 0,
    ///1: Enable general call address detection
    _1 = 1,
}
impl From<Gcae> for bool {
    #[inline(always)]
    fn from(variant: Gcae) -> Self {
        variant as u8 != 0
    }
}
///Field `GCAE` reader - General Call Address Enable
pub type GcaeR = crate::BitReader<Gcae>;
impl GcaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gcae {
        match self.bits {
            false => Gcae::_0,
            true => Gcae::_1,
        }
    }
    ///Disable general call address detection
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gcae::_0
    }
    ///Enable general call address detection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gcae::_1
    }
}
///Field `GCAE` writer - General Call Address Enable
pub type GcaeW<'a, REG> = crate::BitWriter<'a, REG, Gcae>;
impl<'a, REG> GcaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable general call address detection
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gcae::_0)
    }
    ///Enable general call address detection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gcae::_1)
    }
}
/**Device-ID Address Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dide {
    ///0: Disable device-ID address detection
    _0 = 0,
    ///1: Enable device-ID address detection
    _1 = 1,
}
impl From<Dide> for bool {
    #[inline(always)]
    fn from(variant: Dide) -> Self {
        variant as u8 != 0
    }
}
///Field `DIDE` reader - Device-ID Address Detection Enable
pub type DideR = crate::BitReader<Dide>;
impl DideR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dide {
        match self.bits {
            false => Dide::_0,
            true => Dide::_1,
        }
    }
    ///Disable device-ID address detection
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dide::_0
    }
    ///Enable device-ID address detection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dide::_1
    }
}
///Field `DIDE` writer - Device-ID Address Detection Enable
pub type DideW<'a, REG> = crate::BitWriter<'a, REG, Dide>;
impl<'a, REG> DideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable device-ID address detection
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dide::_0)
    }
    ///Enable device-ID address detection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dide::_1)
    }
}
/**Host Address Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hoae {
    ///0: Disable host address detection
    _0 = 0,
    ///1: Enable host address detection
    _1 = 1,
}
impl From<Hoae> for bool {
    #[inline(always)]
    fn from(variant: Hoae) -> Self {
        variant as u8 != 0
    }
}
///Field `HOAE` reader - Host Address Enable
pub type HoaeR = crate::BitReader<Hoae>;
impl HoaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Hoae {
        match self.bits {
            false => Hoae::_0,
            true => Hoae::_1,
        }
    }
    ///Disable host address detection
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hoae::_0
    }
    ///Enable host address detection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hoae::_1
    }
}
///Field `HOAE` writer - Host Address Enable
pub type HoaeW<'a, REG> = crate::BitWriter<'a, REG, Hoae>;
impl<'a, REG> HoaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable host address detection
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hoae::_0)
    }
    ///Enable host address detection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hoae::_1)
    }
}
impl R {
    ///Bit 0 - Slave Address Register 0 Enable
    #[inline(always)]
    pub fn sar0e(&self) -> Sar0eR {
        Sar0eR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Slave Address Register 1 Enable
    #[inline(always)]
    pub fn sar1e(&self) -> Sar1eR {
        Sar1eR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Slave Address Register 2 Enable
    #[inline(always)]
    pub fn sar2e(&self) -> Sar2eR {
        Sar2eR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - General Call Address Enable
    #[inline(always)]
    pub fn gcae(&self) -> GcaeR {
        GcaeR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Device-ID Address Detection Enable
    #[inline(always)]
    pub fn dide(&self) -> DideR {
        DideR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Host Address Enable
    #[inline(always)]
    pub fn hoae(&self) -> HoaeR {
        HoaeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSER")
            .field("sar0e", &self.sar0e())
            .field("sar1e", &self.sar1e())
            .field("sar2e", &self.sar2e())
            .field("gcae", &self.gcae())
            .field("dide", &self.dide())
            .field("hoae", &self.hoae())
            .finish()
    }
}
impl W {
    ///Bit 0 - Slave Address Register 0 Enable
    #[inline(always)]
    pub fn sar0e(&mut self) -> Sar0eW<IcserSpec> {
        Sar0eW::new(self, 0)
    }
    ///Bit 1 - Slave Address Register 1 Enable
    #[inline(always)]
    pub fn sar1e(&mut self) -> Sar1eW<IcserSpec> {
        Sar1eW::new(self, 1)
    }
    ///Bit 2 - Slave Address Register 2 Enable
    #[inline(always)]
    pub fn sar2e(&mut self) -> Sar2eW<IcserSpec> {
        Sar2eW::new(self, 2)
    }
    ///Bit 3 - General Call Address Enable
    #[inline(always)]
    pub fn gcae(&mut self) -> GcaeW<IcserSpec> {
        GcaeW::new(self, 3)
    }
    ///Bit 5 - Device-ID Address Detection Enable
    #[inline(always)]
    pub fn dide(&mut self) -> DideW<IcserSpec> {
        DideW::new(self, 5)
    }
    ///Bit 7 - Host Address Enable
    #[inline(always)]
    pub fn hoae(&mut self) -> HoaeW<IcserSpec> {
        HoaeW::new(self, 7)
    }
}
/**I2C Bus Status Enable Register

You can [`read`](crate::Reg::read) this register and get [`icser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcserSpec;
impl crate::RegisterSpec for IcserSpec {
    type Ux = u8;
}
///`read()` method returns [`icser::R`](R) reader structure
impl crate::Readable for IcserSpec {}
///`write(|w| ..)` method takes [`icser::W`](W) writer structure
impl crate::Writable for IcserSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICSER to value 0x09
impl crate::Resettable for IcserSpec {
    const RESET_VALUE: u8 = 0x09;
}
