///Register `FDR` reader
pub type R = crate::R<FdrSpec>;
///Register `FDR` writer
pub type W = crate::W<FdrSpec>;
/**Receive FIFO Depth

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfd {
    ///15: 4096 bytes
    _0x0f = 15,
    ///0: settings prohibited
    Others = 0,
}
impl From<Rfd> for u8 {
    #[inline(always)]
    fn from(variant: Rfd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfd {
    type Ux = u8;
}
impl crate::IsEnum for Rfd {}
///Field `RFD` reader - Receive FIFO Depth
pub type RfdR = crate::FieldReader<Rfd>;
impl RfdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfd {
        match self.bits {
            15 => Rfd::_0x0f,
            _ => Rfd::Others,
        }
    }
    ///4096 bytes
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == Rfd::_0x0f
    }
    ///settings prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Rfd::Others)
    }
}
///Field `RFD` writer - Receive FIFO Depth
pub type RfdW<'a, REG> = crate::FieldWriter<'a, REG, 5, Rfd, crate::Safe>;
impl<'a, REG> RfdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///4096 bytes
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut crate::W<REG> {
        self.variant(Rfd::_0x0f)
    }
    ///settings prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Rfd::Others)
    }
}
/**Transmit FIFO Depth

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tfd {
    ///7: 2048 bytes
    _0x07 = 7,
    ///0: settings prohibited
    Others = 0,
}
impl From<Tfd> for u8 {
    #[inline(always)]
    fn from(variant: Tfd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tfd {
    type Ux = u8;
}
impl crate::IsEnum for Tfd {}
///Field `TFD` reader - Transmit FIFO Depth
pub type TfdR = crate::FieldReader<Tfd>;
impl TfdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tfd {
        match self.bits {
            7 => Tfd::_0x07,
            _ => Tfd::Others,
        }
    }
    ///2048 bytes
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == Tfd::_0x07
    }
    ///settings prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Tfd::Others)
    }
}
///Field `TFD` writer - Transmit FIFO Depth
pub type TfdW<'a, REG> = crate::FieldWriter<'a, REG, 5, Tfd, crate::Safe>;
impl<'a, REG> TfdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2048 bytes
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut crate::W<REG> {
        self.variant(Tfd::_0x07)
    }
    ///settings prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Tfd::Others)
    }
}
impl R {
    ///Bits 0:4 - Receive FIFO Depth
    #[inline(always)]
    pub fn rfd(&self) -> RfdR {
        RfdR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Transmit FIFO Depth
    #[inline(always)]
    pub fn tfd(&self) -> TfdR {
        TfdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDR")
            .field("rfd", &self.rfd())
            .field("tfd", &self.tfd())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Receive FIFO Depth
    #[inline(always)]
    pub fn rfd(&mut self) -> RfdW<FdrSpec> {
        RfdW::new(self, 0)
    }
    ///Bits 8:12 - Transmit FIFO Depth
    #[inline(always)]
    pub fn tfd(&mut self) -> TfdW<FdrSpec> {
        TfdW::new(self, 8)
    }
}
/**FIFO Depth Register

You can [`read`](crate::Reg::read) this register and get [`fdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FdrSpec;
impl crate::RegisterSpec for FdrSpec {
    type Ux = u32;
}
///`read()` method returns [`fdr::R`](R) reader structure
impl crate::Readable for FdrSpec {}
///`write(|w| ..)` method takes [`fdr::W`](W) writer structure
impl crate::Writable for FdrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDR to value 0
impl crate::Resettable for FdrSpec {}
