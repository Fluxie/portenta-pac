///Register `FTDRH` writer
pub type W = crate::W<FtdrhSpec>;
/**Multi-Processor Transfer Bit Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpbt {
    ///0: Data transmission cycle
    _0 = 0,
    ///1: ID transmission cycle
    _1 = 1,
}
impl From<Mpbt> for bool {
    #[inline(always)]
    fn from(variant: Mpbt) -> Self {
        variant as u8 != 0
    }
}
///Field `MPBT` writer - Multi-Processor Transfer Bit Flag
pub type MpbtW<'a, REG> = crate::BitWriter<'a, REG, Mpbt>;
impl<'a, REG> MpbtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data transmission cycle
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpbt::_0)
    }
    ///ID transmission cycle
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpbt::_1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<FtdrhSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - Multi-Processor Transfer Bit Flag
    #[inline(always)]
    pub fn mpbt(&mut self) -> MpbtW<FtdrhSpec> {
        MpbtW::new(self, 1)
    }
}
/**Transmit FIFO Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftdrh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FtdrhSpec;
impl crate::RegisterSpec for FtdrhSpec {
    type Ux = u8;
}
///`write(|w| ..)` method takes [`ftdrh::W`](W) writer structure
impl crate::Writable for FtdrhSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTDRH to value 0xff
impl crate::Resettable for FtdrhSpec {
    const RESET_VALUE: u8 = 0xff;
}
