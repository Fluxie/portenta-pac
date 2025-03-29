///Register `TRIMD` reader
pub type R = crate::R<TrimdSpec>;
///Register `TRIMD` writer
pub type W = crate::W<TrimdSpec>;
/**Transmit Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tis {
    ///0: Disable transmit interrupts
    _0 = 0,
    ///1: Enable transmit Interrupts.
    _1 = 1,
}
impl From<Tis> for bool {
    #[inline(always)]
    fn from(variant: Tis) -> Self {
        variant as u8 != 0
    }
}
///Field `TIS` reader - Transmit Interrupt Enable
pub type TisR = crate::BitReader<Tis>;
impl TisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tis {
        match self.bits {
            false => Tis::_0,
            true => Tis::_1,
        }
    }
    ///Disable transmit interrupts
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tis::_0
    }
    ///Enable transmit Interrupts.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tis::_1
    }
}
///Field `TIS` writer - Transmit Interrupt Enable
pub type TisW<'a, REG> = crate::BitWriter<'a, REG, Tis>;
impl<'a, REG> TisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable transmit interrupts
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tis::_0)
    }
    ///Enable transmit Interrupts.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tis::_1)
    }
}
/**Transmit Interrupt Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim {
    ///0: Select transmission complete interrupt mode, where an interrupt occurs when a frame is transmitted
    _0 = 0,
    ///1: Select write-back complete interrupt mode, where an interrupt occurs when write-back to the transmit descriptor is complete while the TWBI bit is 1.
    _1 = 1,
}
impl From<Tim> for bool {
    #[inline(always)]
    fn from(variant: Tim) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM` reader - Transmit Interrupt Mode
pub type TimR = crate::BitReader<Tim>;
impl TimR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tim {
        match self.bits {
            false => Tim::_0,
            true => Tim::_1,
        }
    }
    ///Select transmission complete interrupt mode, where an interrupt occurs when a frame is transmitted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tim::_0
    }
    ///Select write-back complete interrupt mode, where an interrupt occurs when write-back to the transmit descriptor is complete while the TWBI bit is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tim::_1
    }
}
///Field `TIM` writer - Transmit Interrupt Mode
pub type TimW<'a, REG> = crate::BitWriter<'a, REG, Tim>;
impl<'a, REG> TimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select transmission complete interrupt mode, where an interrupt occurs when a frame is transmitted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim::_0)
    }
    ///Select write-back complete interrupt mode, where an interrupt occurs when write-back to the transmit descriptor is complete while the TWBI bit is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim::_1)
    }
}
impl R {
    ///Bit 0 - Transmit Interrupt Enable
    #[inline(always)]
    pub fn tis(&self) -> TisR {
        TisR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Transmit Interrupt Mode
    #[inline(always)]
    pub fn tim(&self) -> TimR {
        TimR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIMD")
            .field("tis", &self.tis())
            .field("tim", &self.tim())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transmit Interrupt Enable
    #[inline(always)]
    pub fn tis(&mut self) -> TisW<TrimdSpec> {
        TisW::new(self, 0)
    }
    ///Bit 4 - Transmit Interrupt Mode
    #[inline(always)]
    pub fn tim(&mut self) -> TimW<TrimdSpec> {
        TimW::new(self, 4)
    }
}
/**Transmit Interrupt Setting Register

You can [`read`](crate::Reg::read) this register and get [`trimd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trimd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TrimdSpec;
impl crate::RegisterSpec for TrimdSpec {
    type Ux = u32;
}
///`read()` method returns [`trimd::R`](R) reader structure
impl crate::Readable for TrimdSpec {}
///`write(|w| ..)` method takes [`trimd::W`](W) writer structure
impl crate::Writable for TrimdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRIMD to value 0
impl crate::Resettable for TrimdSpec {}
