///Register `FASTAT` reader
pub type R = crate::R<FastatSpec>;
///Register `FASTAT` writer
pub type W = crate::W<FastatSpec>;
/**Data Flash Memory Access Violation Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfae {
    ///0: No data flash memory access violation has occurred
    _0 = 0,
    ///1: A data flash memory access violation has occurred.
    _1 = 1,
}
impl From<Dfae> for bool {
    #[inline(always)]
    fn from(variant: Dfae) -> Self {
        variant as u8 != 0
    }
}
///Field `DFAE` reader - Data Flash Memory Access Violation Flag
pub type DfaeR = crate::BitReader<Dfae>;
impl DfaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dfae {
        match self.bits {
            false => Dfae::_0,
            true => Dfae::_1,
        }
    }
    ///No data flash memory access violation has occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dfae::_0
    }
    ///A data flash memory access violation has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dfae::_1
    }
}
///Field `DFAE` writer - Data Flash Memory Access Violation Flag
pub type DfaeW<'a, REG> = crate::BitWriter<'a, REG, Dfae>;
impl<'a, REG> DfaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No data flash memory access violation has occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfae::_0)
    }
    ///A data flash memory access violation has occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfae::_1)
    }
}
/**Command Lock Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdlk {
    ///0: The flash sequencer is not in the command-locked state
    _0 = 0,
    ///1: The flash sequencer is in the command-locked state.
    _1 = 1,
}
impl From<Cmdlk> for bool {
    #[inline(always)]
    fn from(variant: Cmdlk) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDLK` reader - Command Lock Flag
pub type CmdlkR = crate::BitReader<Cmdlk>;
impl CmdlkR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmdlk {
        match self.bits {
            false => Cmdlk::_0,
            true => Cmdlk::_1,
        }
    }
    ///The flash sequencer is not in the command-locked state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmdlk::_0
    }
    ///The flash sequencer is in the command-locked state.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmdlk::_1
    }
}
/**Code Flash Memory Access Violation Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfae {
    ///0: No code flash memory access violation has occurred
    _0 = 0,
    ///1: A code flash memory access violation has occurred.
    _1 = 1,
}
impl From<Cfae> for bool {
    #[inline(always)]
    fn from(variant: Cfae) -> Self {
        variant as u8 != 0
    }
}
///Field `CFAE` reader - Code Flash Memory Access Violation Flag
pub type CfaeR = crate::BitReader<Cfae>;
impl CfaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfae {
        match self.bits {
            false => Cfae::_0,
            true => Cfae::_1,
        }
    }
    ///No code flash memory access violation has occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfae::_0
    }
    ///A code flash memory access violation has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfae::_1
    }
}
///Field `CFAE` writer - Code Flash Memory Access Violation Flag
pub type CfaeW<'a, REG> = crate::BitWriter<'a, REG, Cfae>;
impl<'a, REG> CfaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No code flash memory access violation has occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfae::_0)
    }
    ///A code flash memory access violation has occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfae::_1)
    }
}
impl R {
    ///Bit 3 - Data Flash Memory Access Violation Flag
    #[inline(always)]
    pub fn dfae(&self) -> DfaeR {
        DfaeR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Command Lock Flag
    #[inline(always)]
    pub fn cmdlk(&self) -> CmdlkR {
        CmdlkR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Code Flash Memory Access Violation Flag
    #[inline(always)]
    pub fn cfae(&self) -> CfaeR {
        CfaeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FASTAT")
            .field("dfae", &self.dfae())
            .field("cmdlk", &self.cmdlk())
            .field("cfae", &self.cfae())
            .finish()
    }
}
impl W {
    ///Bit 3 - Data Flash Memory Access Violation Flag
    #[inline(always)]
    pub fn dfae(&mut self) -> DfaeW<FastatSpec> {
        DfaeW::new(self, 3)
    }
    ///Bit 7 - Code Flash Memory Access Violation Flag
    #[inline(always)]
    pub fn cfae(&mut self) -> CfaeW<FastatSpec> {
        CfaeW::new(self, 7)
    }
}
/**Flash Access Status Register

You can [`read`](crate::Reg::read) this register and get [`fastat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fastat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FastatSpec;
impl crate::RegisterSpec for FastatSpec {
    type Ux = u8;
}
///`read()` method returns [`fastat::R`](R) reader structure
impl crate::Readable for FastatSpec {}
///`write(|w| ..)` method takes [`fastat::W`](W) writer structure
impl crate::Writable for FastatSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FASTAT to value 0
impl crate::Resettable for FastatSpec {}
