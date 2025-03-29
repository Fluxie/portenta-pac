///Register `SIMR2` reader
pub type R = crate::R<Simr2Spec>;
///Register `SIMR2` writer
pub type W = crate::W<Simr2Spec>;
/**IIC Interrupt Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicintm {
    ///0: Use ACK/NACK interrupts
    _0 = 0,
    ///1: Use reception and transmission interrupts
    _1 = 1,
}
impl From<Iicintm> for bool {
    #[inline(always)]
    fn from(variant: Iicintm) -> Self {
        variant as u8 != 0
    }
}
///Field `IICINTM` reader - IIC Interrupt Mode Select
pub type IicintmR = crate::BitReader<Iicintm>;
impl IicintmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicintm {
        match self.bits {
            false => Iicintm::_0,
            true => Iicintm::_1,
        }
    }
    ///Use ACK/NACK interrupts
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicintm::_0
    }
    ///Use reception and transmission interrupts
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicintm::_1
    }
}
///Field `IICINTM` writer - IIC Interrupt Mode Select
pub type IicintmW<'a, REG> = crate::BitWriter<'a, REG, Iicintm>;
impl<'a, REG> IicintmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Use ACK/NACK interrupts
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicintm::_0)
    }
    ///Use reception and transmission interrupts
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicintm::_1)
    }
}
/**Clock Synchronization

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iiccsc {
    ///0: Do not synchronize with clock signal
    _0 = 0,
    ///1: Synchronize with clock signal
    _1 = 1,
}
impl From<Iiccsc> for bool {
    #[inline(always)]
    fn from(variant: Iiccsc) -> Self {
        variant as u8 != 0
    }
}
///Field `IICCSC` reader - Clock Synchronization
pub type IiccscR = crate::BitReader<Iiccsc>;
impl IiccscR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iiccsc {
        match self.bits {
            false => Iiccsc::_0,
            true => Iiccsc::_1,
        }
    }
    ///Do not synchronize with clock signal
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iiccsc::_0
    }
    ///Synchronize with clock signal
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iiccsc::_1
    }
}
///Field `IICCSC` writer - Clock Synchronization
pub type IiccscW<'a, REG> = crate::BitWriter<'a, REG, Iiccsc>;
impl<'a, REG> IiccscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not synchronize with clock signal
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iiccsc::_0)
    }
    ///Synchronize with clock signal
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iiccsc::_1)
    }
}
/**ACK Transmission Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicackt {
    ///0: ACK transmission
    _0 = 0,
    ///1: NACK transmission and ACK/NACK reception
    _1 = 1,
}
impl From<Iicackt> for bool {
    #[inline(always)]
    fn from(variant: Iicackt) -> Self {
        variant as u8 != 0
    }
}
///Field `IICACKT` reader - ACK Transmission Data
pub type IicacktR = crate::BitReader<Iicackt>;
impl IicacktR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iicackt {
        match self.bits {
            false => Iicackt::_0,
            true => Iicackt::_1,
        }
    }
    ///ACK transmission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicackt::_0
    }
    ///NACK transmission and ACK/NACK reception
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicackt::_1
    }
}
///Field `IICACKT` writer - ACK Transmission Data
pub type IicacktW<'a, REG> = crate::BitWriter<'a, REG, Iicackt>;
impl<'a, REG> IicacktW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ACK transmission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicackt::_0)
    }
    ///NACK transmission and ACK/NACK reception
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicackt::_1)
    }
}
impl R {
    ///Bit 0 - IIC Interrupt Mode Select
    #[inline(always)]
    pub fn iicintm(&self) -> IicintmR {
        IicintmR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock Synchronization
    #[inline(always)]
    pub fn iiccsc(&self) -> IiccscR {
        IiccscR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - ACK Transmission Data
    #[inline(always)]
    pub fn iicackt(&self) -> IicacktR {
        IicacktR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIMR2")
            .field("iicintm", &self.iicintm())
            .field("iiccsc", &self.iiccsc())
            .field("iicackt", &self.iicackt())
            .finish()
    }
}
impl W {
    ///Bit 0 - IIC Interrupt Mode Select
    #[inline(always)]
    pub fn iicintm(&mut self) -> IicintmW<Simr2Spec> {
        IicintmW::new(self, 0)
    }
    ///Bit 1 - Clock Synchronization
    #[inline(always)]
    pub fn iiccsc(&mut self) -> IiccscW<Simr2Spec> {
        IiccscW::new(self, 1)
    }
    ///Bit 5 - ACK Transmission Data
    #[inline(always)]
    pub fn iicackt(&mut self) -> IicacktW<Simr2Spec> {
        IicacktW::new(self, 5)
    }
}
/**IIC Mode Register 2

You can [`read`](crate::Reg::read) this register and get [`simr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Simr2Spec;
impl crate::RegisterSpec for Simr2Spec {
    type Ux = u8;
}
///`read()` method returns [`simr2::R`](R) reader structure
impl crate::Readable for Simr2Spec {}
///`write(|w| ..)` method takes [`simr2::W`](W) writer structure
impl crate::Writable for Simr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SIMR2 to value 0
impl crate::Resettable for Simr2Spec {}
