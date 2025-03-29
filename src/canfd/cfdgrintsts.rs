///Register `CFDGRINTSTS%s` reader
pub type R = crate::R<CfdgrintstsSpec>;
///Register `CFDGRINTSTS%s` writer
pub type W = crate::W<CfdgrintstsSpec>;
/**TXQ Full Interrupt Flag Channel n (n = 0, 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qfif {
    ///0: Corresponding TXQ Full Interrupt flag is not set
    _0 = 0,
    ///1: Corresponding TXQ Full Interrupt flag is set
    _1 = 1,
}
impl From<Qfif> for u8 {
    #[inline(always)]
    fn from(variant: Qfif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qfif {
    type Ux = u8;
}
impl crate::IsEnum for Qfif {}
///Field `QFIF` reader - TXQ Full Interrupt Flag Channel n (n = 0, 1)
pub type QfifR = crate::FieldReader<Qfif>;
impl QfifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Qfif> {
        match self.bits {
            0 => Some(Qfif::_0),
            1 => Some(Qfif::_1),
            _ => None,
        }
    }
    ///Corresponding TXQ Full Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Qfif::_0
    }
    ///Corresponding TXQ Full Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Qfif::_1
    }
}
/**TXQ One Frame RX Interrupt Flag Channel n (n = 0, 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qofrif {
    ///0: Corresponding TXQ One Frame RX Interrupt flag is not set
    _0 = 0,
    ///1: Corresponding TXQ One Frame RX Interrupt flag is set
    _1 = 1,
}
impl From<Qofrif> for u8 {
    #[inline(always)]
    fn from(variant: Qofrif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qofrif {
    type Ux = u8;
}
impl crate::IsEnum for Qofrif {}
///Field `QOFRIF` reader - TXQ One Frame RX Interrupt Flag Channel n (n = 0, 1)
pub type QofrifR = crate::FieldReader<Qofrif>;
impl QofrifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Qofrif> {
        match self.bits {
            0 => Some(Qofrif::_0),
            1 => Some(Qofrif::_1),
            _ => None,
        }
    }
    ///Corresponding TXQ One Frame RX Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Qofrif::_0
    }
    ///Corresponding TXQ One Frame RX Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Qofrif::_1
    }
}
/**Common FIFO RX Interrupt Flag Channel n (n = 0, 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfrif {
    ///0: Corresponding Common FIFO RX Interrupt flag is not set
    _0 = 0,
    ///1: Corresponding Common FIFO RX Interrupt flag is set
    _1 = 1,
}
impl From<Cfrif> for u8 {
    #[inline(always)]
    fn from(variant: Cfrif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfrif {
    type Ux = u8;
}
impl crate::IsEnum for Cfrif {}
///Field `CFRIF` reader - Common FIFO RX Interrupt Flag Channel n (n = 0, 1)
pub type CfrifR = crate::FieldReader<Cfrif>;
impl CfrifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfrif> {
        match self.bits {
            0 => Some(Cfrif::_0),
            1 => Some(Cfrif::_1),
            _ => None,
        }
    }
    ///Corresponding Common FIFO RX Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfrif::_0
    }
    ///Corresponding Common FIFO RX Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfrif::_1
    }
}
/**Common FIFO FDC Level Full Interrupt Flag Channel n (n = 0, 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfrfif {
    ///0: Corresponding Common FIFO Full Interrupt flag is not set
    _0 = 0,
    ///1: Corresponding Common FIFO Full Interrupt flag is set
    _1 = 1,
}
impl From<Cfrfif> for u8 {
    #[inline(always)]
    fn from(variant: Cfrfif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfrfif {
    type Ux = u8;
}
impl crate::IsEnum for Cfrfif {}
///Field `CFRFIF` reader - Common FIFO FDC Level Full Interrupt Flag Channel n (n = 0, 1)
pub type CfrfifR = crate::FieldReader<Cfrfif>;
impl CfrfifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfrfif> {
        match self.bits {
            0 => Some(Cfrfif::_0),
            1 => Some(Cfrfif::_1),
            _ => None,
        }
    }
    ///Corresponding Common FIFO Full Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfrfif::_0
    }
    ///Corresponding Common FIFO Full Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfrfif::_1
    }
}
/**Common FIFO One Frame RX Interrupt Flag Channel n (n = 0, 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfofrif {
    ///0: Corresponding Common FIFO One Frame RX Interrupt flag is not set
    _0 = 0,
    ///1: Corresponding Common FIFO One Frame RX Interrupt flag is set
    _1 = 1,
}
impl From<Cfofrif> for u8 {
    #[inline(always)]
    fn from(variant: Cfofrif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfofrif {
    type Ux = u8;
}
impl crate::IsEnum for Cfofrif {}
///Field `CFOFRIF` reader - Common FIFO One Frame RX Interrupt Flag Channel n (n = 0, 1)
pub type CfofrifR = crate::FieldReader<Cfofrif>;
impl CfofrifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfofrif> {
        match self.bits {
            0 => Some(Cfofrif::_0),
            1 => Some(Cfofrif::_1),
            _ => None,
        }
    }
    ///Corresponding Common FIFO One Frame RX Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfofrif::_0
    }
    ///Corresponding Common FIFO One Frame RX Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfofrif::_1
    }
}
impl R {
    ///Bits 0:2 - TXQ Full Interrupt Flag Channel n (n = 0, 1)
    #[inline(always)]
    pub fn qfif(&self) -> QfifR {
        QfifR::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - TXQ One Frame RX Interrupt Flag Channel n (n = 0, 1)
    #[inline(always)]
    pub fn qofrif(&self) -> QofrifR {
        QofrifR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:18 - Common FIFO RX Interrupt Flag Channel n (n = 0, 1)
    #[inline(always)]
    pub fn cfrif(&self) -> CfrifR {
        CfrifR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 24:26 - Common FIFO FDC Level Full Interrupt Flag Channel n (n = 0, 1)
    #[inline(always)]
    pub fn cfrfif(&self) -> CfrfifR {
        CfrfifR::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - Common FIFO One Frame RX Interrupt Flag Channel n (n = 0, 1)
    #[inline(always)]
    pub fn cfofrif(&self) -> CfofrifR {
        CfofrifR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGRINTSTS")
            .field("qfif", &self.qfif())
            .field("qofrif", &self.qofrif())
            .field("cfrif", &self.cfrif())
            .field("cfrfif", &self.cfrfif())
            .field("cfofrif", &self.cfofrif())
            .finish()
    }
}
impl W {}
/**Global RX Interrupt Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdgrintsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgrintsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgrintstsSpec;
impl crate::RegisterSpec for CfdgrintstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgrintsts::R`](R) reader structure
impl crate::Readable for CfdgrintstsSpec {}
///`write(|w| ..)` method takes [`cfdgrintsts::W`](W) writer structure
impl crate::Writable for CfdgrintstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGRINTSTS%s to value 0
impl crate::Resettable for CfdgrintstsSpec {}
