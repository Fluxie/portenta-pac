///Register `EC710TMC` reader
pub type R = crate::R<Ec710tmcSpec>;
///Register `EC710TMC` writer
pub type W = crate::W<Ec710tmcSpec>;
/**ECC Decode Input Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecdcs {
    ///0: Input lower 32 bits of RAM output data to data area of decode circuit
    _0 = 0,
    ///1: Input ECEDB31-0 in EC710TED register to data area of decode circuit
    _1 = 1,
}
impl From<Ecdcs> for bool {
    #[inline(always)]
    fn from(variant: Ecdcs) -> Self {
        variant as u8 != 0
    }
}
///Field `ECDCS` reader - ECC Decode Input Select
pub type EcdcsR = crate::BitReader<Ecdcs>;
impl EcdcsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ecdcs {
        match self.bits {
            false => Ecdcs::_0,
            true => Ecdcs::_1,
        }
    }
    ///Input lower 32 bits of RAM output data to data area of decode circuit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecdcs::_0
    }
    ///Input ECEDB31-0 in EC710TED register to data area of decode circuit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecdcs::_1
    }
}
///Field `ECDCS` writer - ECC Decode Input Select
pub type EcdcsW<'a, REG> = crate::BitWriter<'a, REG, Ecdcs>;
impl<'a, REG> EcdcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input lower 32 bits of RAM output data to data area of decode circuit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecdcs::_0)
    }
    ///Input ECEDB31-0 in EC710TED register to data area of decode circuit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecdcs::_1)
    }
}
/**ECC Test Mode Control Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ectmce {
    ///0: The access to test mode register and bit is disabled
    _0 = 0,
    ///1: The access to test mode register and bit is enabled
    _1 = 1,
}
impl From<Ectmce> for bool {
    #[inline(always)]
    fn from(variant: Ectmce) -> Self {
        variant as u8 != 0
    }
}
///Field `ECTMCE` reader - ECC Test Mode Control Enable
pub type EctmceR = crate::BitReader<Ectmce>;
impl EctmceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ectmce {
        match self.bits {
            false => Ectmce::_0,
            true => Ectmce::_1,
        }
    }
    ///The access to test mode register and bit is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ectmce::_0
    }
    ///The access to test mode register and bit is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ectmce::_1
    }
}
///Field `ECTMCE` writer - ECC Test Mode Control Enable
pub type EctmceW<'a, REG> = crate::BitWriter<'a, REG, Ectmce>;
impl<'a, REG> EctmceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The access to test mode register and bit is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ectmce::_0)
    }
    ///The access to test mode register and bit is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ectmce::_1)
    }
}
///Field `ETMA` reader - ECC Test Mode Bit Access Control
pub type EtmaR = crate::FieldReader;
///Field `ETMA` writer - ECC Test Mode Bit Access Control
pub type EtmaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 1 - ECC Decode Input Select
    #[inline(always)]
    pub fn ecdcs(&self) -> EcdcsR {
        EcdcsR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 7 - ECC Test Mode Control Enable
    #[inline(always)]
    pub fn ectmce(&self) -> EctmceR {
        EctmceR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 14:15 - ECC Test Mode Bit Access Control
    #[inline(always)]
    pub fn etma(&self) -> EtmaR {
        EtmaR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EC710TMC")
            .field("ecdcs", &self.ecdcs())
            .field("ectmce", &self.ectmce())
            .field("etma", &self.etma())
            .finish()
    }
}
impl W {
    ///Bit 1 - ECC Decode Input Select
    #[inline(always)]
    pub fn ecdcs(&mut self) -> EcdcsW<Ec710tmcSpec> {
        EcdcsW::new(self, 1)
    }
    ///Bit 7 - ECC Test Mode Control Enable
    #[inline(always)]
    pub fn ectmce(&mut self) -> EctmceW<Ec710tmcSpec> {
        EctmceW::new(self, 7)
    }
    ///Bits 14:15 - ECC Test Mode Bit Access Control
    #[inline(always)]
    pub fn etma(&mut self) -> EtmaW<Ec710tmcSpec> {
        EtmaW::new(self, 14)
    }
}
/**ECC Test Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`ec710tmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ec710tmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ec710tmcSpec;
impl crate::RegisterSpec for Ec710tmcSpec {
    type Ux = u16;
}
///`read()` method returns [`ec710tmc::R`](R) reader structure
impl crate::Readable for Ec710tmcSpec {}
///`write(|w| ..)` method takes [`ec710tmc::W`](W) writer structure
impl crate::Writable for Ec710tmcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EC710TMC to value 0
impl crate::Resettable for Ec710tmcSpec {}
