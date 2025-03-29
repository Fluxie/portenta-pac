///Register `CFIFOSEL` reader
pub type R = crate::R<CfifoselSpec>;
///Register `CFIFOSEL` writer
pub type W = crate::W<CfifoselSpec>;
/**CFIFO Port Access Pipe Specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Curpipe {
    ///0: Default Control Pipe
    _0x0 = 0,
    ///1: Pipe 1
    _0x1 = 1,
    ///2: Pipe 2
    _0x2 = 2,
    ///3: Pipe 3
    _0x3 = 3,
    ///4: Pipe 4
    _0x4 = 4,
    ///5: Pipe 5
    _0x5 = 5,
    ///6: Pipe 6
    _0x6 = 6,
    ///7: Pipe 7
    _0x7 = 7,
    ///8: Pipe 8
    _0x8 = 8,
    ///9: Pipe 9
    _0x9 = 9,
    ///10: Setting prohibited
    Others = 10,
}
impl From<Curpipe> for u8 {
    #[inline(always)]
    fn from(variant: Curpipe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Curpipe {
    type Ux = u8;
}
impl crate::IsEnum for Curpipe {}
///Field `CURPIPE` reader - CFIFO Port Access Pipe Specification
pub type CurpipeR = crate::FieldReader<Curpipe>;
impl CurpipeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Curpipe {
        match self.bits {
            0 => Curpipe::_0x0,
            1 => Curpipe::_0x1,
            2 => Curpipe::_0x2,
            3 => Curpipe::_0x3,
            4 => Curpipe::_0x4,
            5 => Curpipe::_0x5,
            6 => Curpipe::_0x6,
            7 => Curpipe::_0x7,
            8 => Curpipe::_0x8,
            9 => Curpipe::_0x9,
            _ => Curpipe::Others,
        }
    }
    ///Default Control Pipe
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Curpipe::_0x0
    }
    ///Pipe 1
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Curpipe::_0x1
    }
    ///Pipe 2
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Curpipe::_0x2
    }
    ///Pipe 3
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Curpipe::_0x3
    }
    ///Pipe 4
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Curpipe::_0x4
    }
    ///Pipe 5
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Curpipe::_0x5
    }
    ///Pipe 6
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Curpipe::_0x6
    }
    ///Pipe 7
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Curpipe::_0x7
    }
    ///Pipe 8
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Curpipe::_0x8
    }
    ///Pipe 9
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Curpipe::_0x9
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Curpipe::Others)
    }
}
///Field `CURPIPE` writer - CFIFO Port Access Pipe Specification
pub type CurpipeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Curpipe, crate::Safe>;
impl<'a, REG> CurpipeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Default Control Pipe
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0x0)
    }
    ///Pipe 1
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0x1)
    }
    ///Pipe 2
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0x2)
    }
    ///Pipe 3
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0x3)
    }
    ///Pipe 4
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0x4)
    }
    ///Pipe 5
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0x5)
    }
    ///Pipe 6
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0x6)
    }
    ///Pipe 7
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0x7)
    }
    ///Pipe 8
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0x8)
    }
    ///Pipe 9
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0x9)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::Others)
    }
}
/**CFIFO Port Access Direction When DCP Is Selected

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isel {
    ///0: Select reading from the FIFO buffer
    _0 = 0,
    ///1: Select writing to the FIFO buffer
    _1 = 1,
}
impl From<Isel> for bool {
    #[inline(always)]
    fn from(variant: Isel) -> Self {
        variant as u8 != 0
    }
}
///Field `ISEL` reader - CFIFO Port Access Direction When DCP Is Selected
pub type IselR = crate::BitReader<Isel>;
impl IselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Isel {
        match self.bits {
            false => Isel::_0,
            true => Isel::_1,
        }
    }
    ///Select reading from the FIFO buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Isel::_0
    }
    ///Select writing to the FIFO buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Isel::_1
    }
}
///Field `ISEL` writer - CFIFO Port Access Direction When DCP Is Selected
pub type IselW<'a, REG> = crate::BitWriter<'a, REG, Isel>;
impl<'a, REG> IselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select reading from the FIFO buffer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_0)
    }
    ///Select writing to the FIFO buffer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_1)
    }
}
/**CFIFO Port Endian Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bigend {
    ///0: Little endian
    _0 = 0,
    ///1: Big endian
    _1 = 1,
}
impl From<Bigend> for bool {
    #[inline(always)]
    fn from(variant: Bigend) -> Self {
        variant as u8 != 0
    }
}
///Field `BIGEND` reader - CFIFO Port Endian Control
pub type BigendR = crate::BitReader<Bigend>;
impl BigendR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bigend {
        match self.bits {
            false => Bigend::_0,
            true => Bigend::_1,
        }
    }
    ///Little endian
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bigend::_0
    }
    ///Big endian
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bigend::_1
    }
}
///Field `BIGEND` writer - CFIFO Port Endian Control
pub type BigendW<'a, REG> = crate::BitWriter<'a, REG, Bigend>;
impl<'a, REG> BigendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Little endian
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bigend::_0)
    }
    ///Big endian
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bigend::_1)
    }
}
/**CFIFO Port Access Bit Width

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbw {
    ///0: 8-bit width
    _0 = 0,
    ///1: 16-bit width
    _1 = 1,
}
impl From<Mbw> for bool {
    #[inline(always)]
    fn from(variant: Mbw) -> Self {
        variant as u8 != 0
    }
}
///Field `MBW` reader - CFIFO Port Access Bit Width
pub type MbwR = crate::BitReader<Mbw>;
impl MbwR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mbw {
        match self.bits {
            false => Mbw::_0,
            true => Mbw::_1,
        }
    }
    ///8-bit width
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mbw::_0
    }
    ///16-bit width
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mbw::_1
    }
}
///Field `MBW` writer - CFIFO Port Access Bit Width
pub type MbwW<'a, REG> = crate::BitWriter<'a, REG, Mbw>;
impl<'a, REG> MbwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///8-bit width
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mbw::_0)
    }
    ///16-bit width
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mbw::_1)
    }
}
/**Buffer Pointer Rewind

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rew {
    ///0: Do not rewind buffer pointer
    _0 = 0,
    ///1: Rewind buffer pointer
    _1 = 1,
}
impl From<Rew> for bool {
    #[inline(always)]
    fn from(variant: Rew) -> Self {
        variant as u8 != 0
    }
}
///Field `REW` writer - Buffer Pointer Rewind
pub type RewW<'a, REG> = crate::BitWriter<'a, REG, Rew>;
impl<'a, REG> RewW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not rewind buffer pointer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rew::_0)
    }
    ///Rewind buffer pointer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rew::_1)
    }
}
/**Read Count Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcnt {
    ///0: The DTLN\[8:0\] bits (CFIFOCTR.DTLN\[8:0\], D0FIFOCTR.DTLN\[8:0\], D1FIFOCTR.DTLN\[8:0\]) are cleared when all receive data is read from the CFIFO. In double buffer mode, the DTLN\[8:0\] value is cleared when all data is read from only a single plane.
    _0 = 0,
    ///1: The DTLN\[8:0\] bits are decremented each time the receive data is read from the CFIFO.
    _1 = 1,
}
impl From<Rcnt> for bool {
    #[inline(always)]
    fn from(variant: Rcnt) -> Self {
        variant as u8 != 0
    }
}
///Field `RCNT` reader - Read Count Mode
pub type RcntR = crate::BitReader<Rcnt>;
impl RcntR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcnt {
        match self.bits {
            false => Rcnt::_0,
            true => Rcnt::_1,
        }
    }
    ///The DTLN\[8:0\] bits (CFIFOCTR.DTLN\[8:0\], D0FIFOCTR.DTLN\[8:0\], D1FIFOCTR.DTLN\[8:0\]) are cleared when all receive data is read from the CFIFO. In double buffer mode, the DTLN\[8:0\] value is cleared when all data is read from only a single plane.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcnt::_0
    }
    ///The DTLN\[8:0\] bits are decremented each time the receive data is read from the CFIFO.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcnt::_1
    }
}
///Field `RCNT` writer - Read Count Mode
pub type RcntW<'a, REG> = crate::BitWriter<'a, REG, Rcnt>;
impl<'a, REG> RcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The DTLN\[8:0\] bits (CFIFOCTR.DTLN\[8:0\], D0FIFOCTR.DTLN\[8:0\], D1FIFOCTR.DTLN\[8:0\]) are cleared when all receive data is read from the CFIFO. In double buffer mode, the DTLN\[8:0\] value is cleared when all data is read from only a single plane.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcnt::_0)
    }
    ///The DTLN\[8:0\] bits are decremented each time the receive data is read from the CFIFO.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcnt::_1)
    }
}
impl R {
    ///Bits 0:3 - CFIFO Port Access Pipe Specification
    #[inline(always)]
    pub fn curpipe(&self) -> CurpipeR {
        CurpipeR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 5 - CFIFO Port Access Direction When DCP Is Selected
    #[inline(always)]
    pub fn isel(&self) -> IselR {
        IselR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - CFIFO Port Endian Control
    #[inline(always)]
    pub fn bigend(&self) -> BigendR {
        BigendR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - CFIFO Port Access Bit Width
    #[inline(always)]
    pub fn mbw(&self) -> MbwR {
        MbwR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Read Count Mode
    #[inline(always)]
    pub fn rcnt(&self) -> RcntR {
        RcntR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFIFOSEL")
            .field("curpipe", &self.curpipe())
            .field("isel", &self.isel())
            .field("bigend", &self.bigend())
            .field("mbw", &self.mbw())
            .field("rcnt", &self.rcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - CFIFO Port Access Pipe Specification
    #[inline(always)]
    pub fn curpipe(&mut self) -> CurpipeW<CfifoselSpec> {
        CurpipeW::new(self, 0)
    }
    ///Bit 5 - CFIFO Port Access Direction When DCP Is Selected
    #[inline(always)]
    pub fn isel(&mut self) -> IselW<CfifoselSpec> {
        IselW::new(self, 5)
    }
    ///Bit 8 - CFIFO Port Endian Control
    #[inline(always)]
    pub fn bigend(&mut self) -> BigendW<CfifoselSpec> {
        BigendW::new(self, 8)
    }
    ///Bit 10 - CFIFO Port Access Bit Width
    #[inline(always)]
    pub fn mbw(&mut self) -> MbwW<CfifoselSpec> {
        MbwW::new(self, 10)
    }
    ///Bit 14 - Buffer Pointer Rewind
    #[inline(always)]
    pub fn rew(&mut self) -> RewW<CfifoselSpec> {
        RewW::new(self, 14)
    }
    ///Bit 15 - Read Count Mode
    #[inline(always)]
    pub fn rcnt(&mut self) -> RcntW<CfifoselSpec> {
        RcntW::new(self, 15)
    }
}
/**CFIFO Port Select Register

You can [`read`](crate::Reg::read) this register and get [`cfifosel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifosel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfifoselSpec;
impl crate::RegisterSpec for CfifoselSpec {
    type Ux = u16;
}
///`read()` method returns [`cfifosel::R`](R) reader structure
impl crate::Readable for CfifoselSpec {}
///`write(|w| ..)` method takes [`cfifosel::W`](W) writer structure
impl crate::Writable for CfifoselSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFOSEL to value 0
impl crate::Resettable for CfifoselSpec {}
