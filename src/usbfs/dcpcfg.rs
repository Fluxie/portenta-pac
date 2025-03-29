///Register `DCPCFG` reader
pub type R = crate::R<DcpcfgSpec>;
///Register `DCPCFG` writer
pub type W = crate::W<DcpcfgSpec>;
/**Transfer Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    ///0: Data receiving direction
    _0 = 0,
    ///1: Data transmitting direction
    _1 = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
///Field `DIR` reader - Transfer Direction
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::_0,
            true => Dir::_1,
        }
    }
    ///Data receiving direction
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dir::_0
    }
    ///Data transmitting direction
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dir::_1
    }
}
///Field `DIR` writer - Transfer Direction
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data receiving direction
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::_0)
    }
    ///Data transmitting direction
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::_1)
    }
}
/**Pipe Disabled at End of Transfer

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shtnak {
    ///0: Keep pipe open after transfer ends
    _0 = 0,
    ///1: Disable pipe after transfer ends
    _1 = 1,
}
impl From<Shtnak> for bool {
    #[inline(always)]
    fn from(variant: Shtnak) -> Self {
        variant as u8 != 0
    }
}
///Field `SHTNAK` reader - Pipe Disabled at End of Transfer
pub type ShtnakR = crate::BitReader<Shtnak>;
impl ShtnakR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Shtnak {
        match self.bits {
            false => Shtnak::_0,
            true => Shtnak::_1,
        }
    }
    ///Keep pipe open after transfer ends
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Shtnak::_0
    }
    ///Disable pipe after transfer ends
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Shtnak::_1
    }
}
///Field `SHTNAK` writer - Pipe Disabled at End of Transfer
pub type ShtnakW<'a, REG> = crate::BitWriter<'a, REG, Shtnak>;
impl<'a, REG> ShtnakW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Keep pipe open after transfer ends
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Shtnak::_0)
    }
    ///Disable pipe after transfer ends
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Shtnak::_1)
    }
}
impl R {
    ///Bit 4 - Transfer Direction
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Pipe Disabled at End of Transfer
    #[inline(always)]
    pub fn shtnak(&self) -> ShtnakR {
        ShtnakR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCPCFG")
            .field("dir", &self.dir())
            .field("shtnak", &self.shtnak())
            .finish()
    }
}
impl W {
    ///Bit 4 - Transfer Direction
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<DcpcfgSpec> {
        DirW::new(self, 4)
    }
    ///Bit 7 - Pipe Disabled at End of Transfer
    #[inline(always)]
    pub fn shtnak(&mut self) -> ShtnakW<DcpcfgSpec> {
        ShtnakW::new(self, 7)
    }
}
/**DCP Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dcpcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcpcfgSpec;
impl crate::RegisterSpec for DcpcfgSpec {
    type Ux = u16;
}
///`read()` method returns [`dcpcfg::R`](R) reader structure
impl crate::Readable for DcpcfgSpec {}
///`write(|w| ..)` method takes [`dcpcfg::W`](W) writer structure
impl crate::Writable for DcpcfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCPCFG to value 0
impl crate::Resettable for DcpcfgSpec {}
