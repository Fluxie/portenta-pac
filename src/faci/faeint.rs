///Register `FAEINT` reader
pub type R = crate::R<FaeintSpec>;
///Register `FAEINT` writer
pub type W = crate::W<FaeintSpec>;
/**Data Flash Memory Access Violation Interrupt Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfaeie {
    ///0: Generation of an FIFERR interrupt request is disabled when FASTAT.DFAE is set to 1
    _0 = 0,
    ///1: Generation of an FIFERR interrupt request is enabled when FASTAT.DFAE is set to 1.
    _1 = 1,
}
impl From<Dfaeie> for bool {
    #[inline(always)]
    fn from(variant: Dfaeie) -> Self {
        variant as u8 != 0
    }
}
///Field `DFAEIE` reader - Data Flash Memory Access Violation Interrupt Enable
pub type DfaeieR = crate::BitReader<Dfaeie>;
impl DfaeieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dfaeie {
        match self.bits {
            false => Dfaeie::_0,
            true => Dfaeie::_1,
        }
    }
    ///Generation of an FIFERR interrupt request is disabled when FASTAT.DFAE is set to 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dfaeie::_0
    }
    ///Generation of an FIFERR interrupt request is enabled when FASTAT.DFAE is set to 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dfaeie::_1
    }
}
///Field `DFAEIE` writer - Data Flash Memory Access Violation Interrupt Enable
pub type DfaeieW<'a, REG> = crate::BitWriter<'a, REG, Dfaeie>;
impl<'a, REG> DfaeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generation of an FIFERR interrupt request is disabled when FASTAT.DFAE is set to 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfaeie::_0)
    }
    ///Generation of an FIFERR interrupt request is enabled when FASTAT.DFAE is set to 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfaeie::_1)
    }
}
/**Command Lock Interrupt Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdlkie {
    ///0: Generation of an FIFERR interrupt request is disabled when FASTAT.CMDLK is set to 1
    _0 = 0,
    ///1: Generation of an FIFERR interrupt request is enabled when FASTAT.CMDLK is set to 1.
    _1 = 1,
}
impl From<Cmdlkie> for bool {
    #[inline(always)]
    fn from(variant: Cmdlkie) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDLKIE` reader - Command Lock Interrupt Enable
pub type CmdlkieR = crate::BitReader<Cmdlkie>;
impl CmdlkieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmdlkie {
        match self.bits {
            false => Cmdlkie::_0,
            true => Cmdlkie::_1,
        }
    }
    ///Generation of an FIFERR interrupt request is disabled when FASTAT.CMDLK is set to 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmdlkie::_0
    }
    ///Generation of an FIFERR interrupt request is enabled when FASTAT.CMDLK is set to 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmdlkie::_1
    }
}
///Field `CMDLKIE` writer - Command Lock Interrupt Enable
pub type CmdlkieW<'a, REG> = crate::BitWriter<'a, REG, Cmdlkie>;
impl<'a, REG> CmdlkieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generation of an FIFERR interrupt request is disabled when FASTAT.CMDLK is set to 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdlkie::_0)
    }
    ///Generation of an FIFERR interrupt request is enabled when FASTAT.CMDLK is set to 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdlkie::_1)
    }
}
/**Code Flash Memory Access Violation Interrupt Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfaeie {
    ///0: Generation of an FIFERR interrupt request is disabled when FASTAT.CFAE is set to 1
    _0 = 0,
    ///1: Generation of an FIFERR interrupt request is enabled when FASTAT.CFAE is set to 1.
    _1 = 1,
}
impl From<Cfaeie> for bool {
    #[inline(always)]
    fn from(variant: Cfaeie) -> Self {
        variant as u8 != 0
    }
}
///Field `CFAEIE` reader - Code Flash Memory Access Violation Interrupt Enable
pub type CfaeieR = crate::BitReader<Cfaeie>;
impl CfaeieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfaeie {
        match self.bits {
            false => Cfaeie::_0,
            true => Cfaeie::_1,
        }
    }
    ///Generation of an FIFERR interrupt request is disabled when FASTAT.CFAE is set to 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfaeie::_0
    }
    ///Generation of an FIFERR interrupt request is enabled when FASTAT.CFAE is set to 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfaeie::_1
    }
}
///Field `CFAEIE` writer - Code Flash Memory Access Violation Interrupt Enable
pub type CfaeieW<'a, REG> = crate::BitWriter<'a, REG, Cfaeie>;
impl<'a, REG> CfaeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generation of an FIFERR interrupt request is disabled when FASTAT.CFAE is set to 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfaeie::_0)
    }
    ///Generation of an FIFERR interrupt request is enabled when FASTAT.CFAE is set to 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfaeie::_1)
    }
}
impl R {
    ///Bit 3 - Data Flash Memory Access Violation Interrupt Enable
    #[inline(always)]
    pub fn dfaeie(&self) -> DfaeieR {
        DfaeieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Command Lock Interrupt Enable
    #[inline(always)]
    pub fn cmdlkie(&self) -> CmdlkieR {
        CmdlkieR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Code Flash Memory Access Violation Interrupt Enable
    #[inline(always)]
    pub fn cfaeie(&self) -> CfaeieR {
        CfaeieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FAEINT")
            .field("dfaeie", &self.dfaeie())
            .field("cmdlkie", &self.cmdlkie())
            .field("cfaeie", &self.cfaeie())
            .finish()
    }
}
impl W {
    ///Bit 3 - Data Flash Memory Access Violation Interrupt Enable
    #[inline(always)]
    pub fn dfaeie(&mut self) -> DfaeieW<FaeintSpec> {
        DfaeieW::new(self, 3)
    }
    ///Bit 4 - Command Lock Interrupt Enable
    #[inline(always)]
    pub fn cmdlkie(&mut self) -> CmdlkieW<FaeintSpec> {
        CmdlkieW::new(self, 4)
    }
    ///Bit 7 - Code Flash Memory Access Violation Interrupt Enable
    #[inline(always)]
    pub fn cfaeie(&mut self) -> CfaeieW<FaeintSpec> {
        CfaeieW::new(self, 7)
    }
}
/**Flash Access Error Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`faeint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faeint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FaeintSpec;
impl crate::RegisterSpec for FaeintSpec {
    type Ux = u8;
}
///`read()` method returns [`faeint::R`](R) reader structure
impl crate::Readable for FaeintSpec {}
///`write(|w| ..)` method takes [`faeint::W`](W) writer structure
impl crate::Writable for FaeintSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FAEINT to value 0x98
impl crate::Resettable for FaeintSpec {
    const RESET_VALUE: u8 = 0x98;
}
