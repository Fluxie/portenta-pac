///Register `MSTPCRA` reader
pub type R = crate::R<MstpcraSpec>;
///Register `MSTPCRA` writer
pub type W = crate::W<MstpcraSpec>;
/**SRAM0 Module Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpa0 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpa0> for bool {
    #[inline(always)]
    fn from(variant: Mstpa0) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPA0` reader - SRAM0 Module Stop
pub type Mstpa0R = crate::BitReader<Mstpa0>;
impl Mstpa0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpa0 {
        match self.bits {
            false => Mstpa0::_0,
            true => Mstpa0::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpa0::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpa0::_1
    }
}
///Field `MSTPA0` writer - SRAM0 Module Stop
pub type Mstpa0W<'a, REG> = crate::BitWriter<'a, REG, Mstpa0>;
impl<'a, REG> Mstpa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa0::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa0::_1)
    }
}
/**Standby SRAM Module Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpa7 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpa7> for bool {
    #[inline(always)]
    fn from(variant: Mstpa7) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPA7` reader - Standby SRAM Module Stop
pub type Mstpa7R = crate::BitReader<Mstpa7>;
impl Mstpa7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpa7 {
        match self.bits {
            false => Mstpa7::_0,
            true => Mstpa7::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpa7::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpa7::_1
    }
}
///Field `MSTPA7` writer - Standby SRAM Module Stop
pub type Mstpa7W<'a, REG> = crate::BitWriter<'a, REG, Mstpa7>;
impl<'a, REG> Mstpa7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa7::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa7::_1)
    }
}
/**DMA Controller/Data Transfer Controller Module Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpa22 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpa22> for bool {
    #[inline(always)]
    fn from(variant: Mstpa22) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPA22` reader - DMA Controller/Data Transfer Controller Module Stop
pub type Mstpa22R = crate::BitReader<Mstpa22>;
impl Mstpa22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpa22 {
        match self.bits {
            false => Mstpa22::_0,
            true => Mstpa22::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpa22::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpa22::_1
    }
}
///Field `MSTPA22` writer - DMA Controller/Data Transfer Controller Module Stop
pub type Mstpa22W<'a, REG> = crate::BitWriter<'a, REG, Mstpa22>;
impl<'a, REG> Mstpa22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa22::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa22::_1)
    }
}
impl R {
    ///Bit 0 - SRAM0 Module Stop
    #[inline(always)]
    pub fn mstpa0(&self) -> Mstpa0R {
        Mstpa0R::new((self.bits & 1) != 0)
    }
    ///Bit 7 - Standby SRAM Module Stop
    #[inline(always)]
    pub fn mstpa7(&self) -> Mstpa7R {
        Mstpa7R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 22 - DMA Controller/Data Transfer Controller Module Stop
    #[inline(always)]
    pub fn mstpa22(&self) -> Mstpa22R {
        Mstpa22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTPCRA")
            .field("mstpa0", &self.mstpa0())
            .field("mstpa7", &self.mstpa7())
            .field("mstpa22", &self.mstpa22())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM0 Module Stop
    #[inline(always)]
    pub fn mstpa0(&mut self) -> Mstpa0W<MstpcraSpec> {
        Mstpa0W::new(self, 0)
    }
    ///Bit 7 - Standby SRAM Module Stop
    #[inline(always)]
    pub fn mstpa7(&mut self) -> Mstpa7W<MstpcraSpec> {
        Mstpa7W::new(self, 7)
    }
    ///Bit 22 - DMA Controller/Data Transfer Controller Module Stop
    #[inline(always)]
    pub fn mstpa22(&mut self) -> Mstpa22W<MstpcraSpec> {
        Mstpa22W::new(self, 22)
    }
}
/**Module Stop Control Register A

You can [`read`](crate::Reg::read) this register and get [`mstpcra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MstpcraSpec;
impl crate::RegisterSpec for MstpcraSpec {
    type Ux = u32;
}
///`read()` method returns [`mstpcra::R`](R) reader structure
impl crate::Readable for MstpcraSpec {}
///`write(|w| ..)` method takes [`mstpcra::W`](W) writer structure
impl crate::Writable for MstpcraSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTPCRA to value 0xffbf_ff7e
impl crate::Resettable for MstpcraSpec {
    const RESET_VALUE: u32 = 0xffbf_ff7e;
}
