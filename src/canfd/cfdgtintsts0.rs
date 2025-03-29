///Register `CFDGTINTSTS0` reader
pub type R = crate::R<Cfdgtintsts0Spec>;
///Register `CFDGTINTSTS0` writer
pub type W = crate::W<Cfdgtintsts0Spec>;
/**TX Successful Interrupt Flag Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsif0 {
    ///0: Channel n TX Successful Interrupt flag not set
    _0 = 0,
    ///1: Channel n TX Successful Interrupt flag set
    _1 = 1,
}
impl From<Tsif0> for bool {
    #[inline(always)]
    fn from(variant: Tsif0) -> Self {
        variant as u8 != 0
    }
}
///Field `TSIF0` reader - TX Successful Interrupt Flag Channel 0
pub type Tsif0R = crate::BitReader<Tsif0>;
impl Tsif0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tsif0 {
        match self.bits {
            false => Tsif0::_0,
            true => Tsif0::_1,
        }
    }
    ///Channel n TX Successful Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsif0::_0
    }
    ///Channel n TX Successful Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsif0::_1
    }
}
/**TX Abort Interrupt Flag Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tai0 {
    ///0: Channel n TX Abort Interrupt flag not set
    _0 = 0,
    ///1: Channel n TX Abort Interrupt flag set
    _1 = 1,
}
impl From<Tai0> for bool {
    #[inline(always)]
    fn from(variant: Tai0) -> Self {
        variant as u8 != 0
    }
}
///Field `TAI0` reader - TX Abort Interrupt Flag Channel 0
pub type Tai0R = crate::BitReader<Tai0>;
impl Tai0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tai0 {
        match self.bits {
            false => Tai0::_0,
            true => Tai0::_1,
        }
    }
    ///Channel n TX Abort Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tai0::_0
    }
    ///Channel n TX Abort Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tai0::_1
    }
}
/**TX Queue Interrupt Flag Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tqif0 {
    ///0: Channel n TX Queue Interrupt flag not set
    _0 = 0,
    ///1: Channel n TX Queue Interrupt flag set
    _1 = 1,
}
impl From<Tqif0> for bool {
    #[inline(always)]
    fn from(variant: Tqif0) -> Self {
        variant as u8 != 0
    }
}
///Field `TQIF0` reader - TX Queue Interrupt Flag Channel 0
pub type Tqif0R = crate::BitReader<Tqif0>;
impl Tqif0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tqif0 {
        match self.bits {
            false => Tqif0::_0,
            true => Tqif0::_1,
        }
    }
    ///Channel n TX Queue Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tqif0::_0
    }
    ///Channel n TX Queue Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tqif0::_1
    }
}
/**COM FIFO TX/GW Mode Interrupt Flag Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cftif0 {
    ///0: Channel n COM FIFO TX/GW Mode Interrupt flag not set
    _0 = 0,
    ///1: Channel n COM FIFO TX/GW Mode Interrupt flag set
    _1 = 1,
}
impl From<Cftif0> for bool {
    #[inline(always)]
    fn from(variant: Cftif0) -> Self {
        variant as u8 != 0
    }
}
///Field `CFTIF0` reader - COM FIFO TX/GW Mode Interrupt Flag Channel 0
pub type Cftif0R = crate::BitReader<Cftif0>;
impl Cftif0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cftif0 {
        match self.bits {
            false => Cftif0::_0,
            true => Cftif0::_1,
        }
    }
    ///Channel n COM FIFO TX/GW Mode Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cftif0::_0
    }
    ///Channel n COM FIFO TX/GW Mode Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cftif0::_1
    }
}
/**TX History List Interrupt Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thif0 {
    ///0: Channel n TX History List Interrupt flag not set
    _0 = 0,
    ///1: Channel n TX History List Interrupt flag set
    _1 = 1,
}
impl From<Thif0> for bool {
    #[inline(always)]
    fn from(variant: Thif0) -> Self {
        variant as u8 != 0
    }
}
///Field `THIF0` reader - TX History List Interrupt Channel 0
pub type Thif0R = crate::BitReader<Thif0>;
impl Thif0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thif0 {
        match self.bits {
            false => Thif0::_0,
            true => Thif0::_1,
        }
    }
    ///Channel n TX History List Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thif0::_0
    }
    ///Channel n TX History List Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thif0::_1
    }
}
/**TX Queue One Frame Transmission Interrupt Flag Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tqofifo {
    ///0: Channel n TX Queue One Frame Transmission Interrupt flag not set
    _0 = 0,
    ///1: Channel n TX Queue One Frame Transmission Interrupt flag set
    _1 = 1,
}
impl From<Tqofifo> for bool {
    #[inline(always)]
    fn from(variant: Tqofifo) -> Self {
        variant as u8 != 0
    }
}
///Field `TQOFIFO` reader - TX Queue One Frame Transmission Interrupt Flag Channel 0
pub type TqofifoR = crate::BitReader<Tqofifo>;
impl TqofifoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tqofifo {
        match self.bits {
            false => Tqofifo::_0,
            true => Tqofifo::_1,
        }
    }
    ///Channel n TX Queue One Frame Transmission Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tqofifo::_0
    }
    ///Channel n TX Queue One Frame Transmission Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tqofifo::_1
    }
}
/**COM FIFO One Frame Transmission Interrupt Flag Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfotifo {
    ///0: Channel n COM FIFO One Frame Transmission Interrupt flag not set
    _0 = 0,
    ///1: Channel n COM FIFO One Frame Transmission Interrupt flag set
    _1 = 1,
}
impl From<Cfotifo> for bool {
    #[inline(always)]
    fn from(variant: Cfotifo) -> Self {
        variant as u8 != 0
    }
}
///Field `CFOTIFO` reader - COM FIFO One Frame Transmission Interrupt Flag Channel 0
pub type CfotifoR = crate::BitReader<Cfotifo>;
impl CfotifoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfotifo {
        match self.bits {
            false => Cfotifo::_0,
            true => Cfotifo::_1,
        }
    }
    ///Channel n COM FIFO One Frame Transmission Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfotifo::_0
    }
    ///Channel n COM FIFO One Frame Transmission Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfotifo::_1
    }
}
/**TX Successful Interrupt Flag Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsif1 {
    ///0: Channel n TX Successful Interrupt flag not set
    _0 = 0,
    ///1: Channel n TX Successful Interrupt flag set
    _1 = 1,
}
impl From<Tsif1> for bool {
    #[inline(always)]
    fn from(variant: Tsif1) -> Self {
        variant as u8 != 0
    }
}
///Field `TSIF1` reader - TX Successful Interrupt Flag Channel 1
pub type Tsif1R = crate::BitReader<Tsif1>;
impl Tsif1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tsif1 {
        match self.bits {
            false => Tsif1::_0,
            true => Tsif1::_1,
        }
    }
    ///Channel n TX Successful Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsif1::_0
    }
    ///Channel n TX Successful Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsif1::_1
    }
}
/**TX Abort Interrupt Flag Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taif1 {
    ///0: Channel n TX Abort Interrupt flag not set
    _0 = 0,
    ///1: Channel n TX Abort Interrupt flag set
    _1 = 1,
}
impl From<Taif1> for bool {
    #[inline(always)]
    fn from(variant: Taif1) -> Self {
        variant as u8 != 0
    }
}
///Field `TAIF1` reader - TX Abort Interrupt Flag Channel 1
pub type Taif1R = crate::BitReader<Taif1>;
impl Taif1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Taif1 {
        match self.bits {
            false => Taif1::_0,
            true => Taif1::_1,
        }
    }
    ///Channel n TX Abort Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Taif1::_0
    }
    ///Channel n TX Abort Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Taif1::_1
    }
}
/**TX Queue Interrupt Flag Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tqif1 {
    ///0: Channel n TX Queue Interrupt flag not set
    _0 = 0,
    ///1: Channel n TX Queue Interrupt flag set
    _1 = 1,
}
impl From<Tqif1> for bool {
    #[inline(always)]
    fn from(variant: Tqif1) -> Self {
        variant as u8 != 0
    }
}
///Field `TQIF1` reader - TX Queue Interrupt Flag Channel 1
pub type Tqif1R = crate::BitReader<Tqif1>;
impl Tqif1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tqif1 {
        match self.bits {
            false => Tqif1::_0,
            true => Tqif1::_1,
        }
    }
    ///Channel n TX Queue Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tqif1::_0
    }
    ///Channel n TX Queue Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tqif1::_1
    }
}
/**COM FIFO TX/GW Mode Interrupt Flag Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cftif1 {
    ///0: Channel n COM FIFO TX/GW Mode Interrupt flag not set
    _0 = 0,
    ///1: Channel n COM FIFO TX/GW Mode Interrupt flag set
    _1 = 1,
}
impl From<Cftif1> for bool {
    #[inline(always)]
    fn from(variant: Cftif1) -> Self {
        variant as u8 != 0
    }
}
///Field `CFTIF1` reader - COM FIFO TX/GW Mode Interrupt Flag Channel 1
pub type Cftif1R = crate::BitReader<Cftif1>;
impl Cftif1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cftif1 {
        match self.bits {
            false => Cftif1::_0,
            true => Cftif1::_1,
        }
    }
    ///Channel n COM FIFO TX/GW Mode Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cftif1::_0
    }
    ///Channel n COM FIFO TX/GW Mode Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cftif1::_1
    }
}
/**TX History List Interrupt Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thif1 {
    ///0: Channel n TX History List Interrupt flag not set
    _0 = 0,
    ///1: Channel n TX History List Interrupt flag set
    _1 = 1,
}
impl From<Thif1> for bool {
    #[inline(always)]
    fn from(variant: Thif1) -> Self {
        variant as u8 != 0
    }
}
///Field `THIF1` reader - TX History List Interrupt Channel 1
pub type Thif1R = crate::BitReader<Thif1>;
impl Thif1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thif1 {
        match self.bits {
            false => Thif1::_0,
            true => Thif1::_1,
        }
    }
    ///Channel n TX History List Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thif1::_0
    }
    ///Channel n TX History List Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thif1::_1
    }
}
/**TX Queue One Frame Transmission Interrupt Flag Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tqofif1 {
    ///0: Channel n TX Queue One Frame Transmission Interrupt flag not set
    _0 = 0,
    ///1: Channel n TX Queue One Frame Transmission Interrupt flag set
    _1 = 1,
}
impl From<Tqofif1> for bool {
    #[inline(always)]
    fn from(variant: Tqofif1) -> Self {
        variant as u8 != 0
    }
}
///Field `TQOFIF1` reader - TX Queue One Frame Transmission Interrupt Flag Channel 1
pub type Tqofif1R = crate::BitReader<Tqofif1>;
impl Tqofif1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tqofif1 {
        match self.bits {
            false => Tqofif1::_0,
            true => Tqofif1::_1,
        }
    }
    ///Channel n TX Queue One Frame Transmission Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tqofif1::_0
    }
    ///Channel n TX Queue One Frame Transmission Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tqofif1::_1
    }
}
/**COM FIFO One Frame Transmission Interrupt Flag Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfotif1 {
    ///0: Channel n COM FIFO One Frame Transmission Interrupt flag not set
    _0 = 0,
    ///1: Channel n COM FIFO One Frame Transmission Interrupt flag set
    _1 = 1,
}
impl From<Cfotif1> for bool {
    #[inline(always)]
    fn from(variant: Cfotif1) -> Self {
        variant as u8 != 0
    }
}
///Field `CFOTIF1` reader - COM FIFO One Frame Transmission Interrupt Flag Channel 1
pub type Cfotif1R = crate::BitReader<Cfotif1>;
impl Cfotif1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfotif1 {
        match self.bits {
            false => Cfotif1::_0,
            true => Cfotif1::_1,
        }
    }
    ///Channel n COM FIFO One Frame Transmission Interrupt flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfotif1::_0
    }
    ///Channel n COM FIFO One Frame Transmission Interrupt flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfotif1::_1
    }
}
impl R {
    ///Bit 0 - TX Successful Interrupt Flag Channel 0
    #[inline(always)]
    pub fn tsif0(&self) -> Tsif0R {
        Tsif0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX Abort Interrupt Flag Channel 0
    #[inline(always)]
    pub fn tai0(&self) -> Tai0R {
        Tai0R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TX Queue Interrupt Flag Channel 0
    #[inline(always)]
    pub fn tqif0(&self) -> Tqif0R {
        Tqif0R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - COM FIFO TX/GW Mode Interrupt Flag Channel 0
    #[inline(always)]
    pub fn cftif0(&self) -> Cftif0R {
        Cftif0R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TX History List Interrupt Channel 0
    #[inline(always)]
    pub fn thif0(&self) -> Thif0R {
        Thif0R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TX Queue One Frame Transmission Interrupt Flag Channel 0
    #[inline(always)]
    pub fn tqofifo(&self) -> TqofifoR {
        TqofifoR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - COM FIFO One Frame Transmission Interrupt Flag Channel 0
    #[inline(always)]
    pub fn cfotifo(&self) -> CfotifoR {
        CfotifoR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - TX Successful Interrupt Flag Channel 1
    #[inline(always)]
    pub fn tsif1(&self) -> Tsif1R {
        Tsif1R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TX Abort Interrupt Flag Channel 1
    #[inline(always)]
    pub fn taif1(&self) -> Taif1R {
        Taif1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TX Queue Interrupt Flag Channel 1
    #[inline(always)]
    pub fn tqif1(&self) -> Tqif1R {
        Tqif1R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - COM FIFO TX/GW Mode Interrupt Flag Channel 1
    #[inline(always)]
    pub fn cftif1(&self) -> Cftif1R {
        Cftif1R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TX History List Interrupt Channel 1
    #[inline(always)]
    pub fn thif1(&self) -> Thif1R {
        Thif1R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TX Queue One Frame Transmission Interrupt Flag Channel 1
    #[inline(always)]
    pub fn tqofif1(&self) -> Tqofif1R {
        Tqofif1R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - COM FIFO One Frame Transmission Interrupt Flag Channel 1
    #[inline(always)]
    pub fn cfotif1(&self) -> Cfotif1R {
        Cfotif1R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGTINTSTS0")
            .field("tsif0", &self.tsif0())
            .field("tai0", &self.tai0())
            .field("tqif0", &self.tqif0())
            .field("cftif0", &self.cftif0())
            .field("thif0", &self.thif0())
            .field("tqofifo", &self.tqofifo())
            .field("cfotifo", &self.cfotifo())
            .field("tsif1", &self.tsif1())
            .field("taif1", &self.taif1())
            .field("tqif1", &self.tqif1())
            .field("cftif1", &self.cftif1())
            .field("thif1", &self.thif1())
            .field("tqofif1", &self.tqofif1())
            .field("cfotif1", &self.cfotif1())
            .finish()
    }
}
impl W {}
/**Global TX Interrupt Status Register 0

You can [`read`](crate::Reg::read) this register and get [`cfdgtintsts0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgtintsts0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdgtintsts0Spec;
impl crate::RegisterSpec for Cfdgtintsts0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdgtintsts0::R`](R) reader structure
impl crate::Readable for Cfdgtintsts0Spec {}
///`write(|w| ..)` method takes [`cfdgtintsts0::W`](W) writer structure
impl crate::Writable for Cfdgtintsts0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGTINTSTS0 to value 0
impl crate::Resettable for Cfdgtintsts0Spec {}
