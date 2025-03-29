///Register `PIR` reader
pub type R = crate::R<PirSpec>;
///Register `PIR` writer
pub type W = crate::W<PirSpec>;
///Field `MDC` reader - MII/RMII Management Data Clock
pub type MdcR = crate::BitReader;
///Field `MDC` writer - MII/RMII Management Data Clock
pub type MdcW<'a, REG> = crate::BitWriter<'a, REG>;
/**MII/RMII Management Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmd {
    ///0: Read
    _0 = 0,
    ///1: Write.
    _1 = 1,
}
impl From<Mmd> for bool {
    #[inline(always)]
    fn from(variant: Mmd) -> Self {
        variant as u8 != 0
    }
}
///Field `MMD` reader - MII/RMII Management Mode
pub type MmdR = crate::BitReader<Mmd>;
impl MmdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mmd {
        match self.bits {
            false => Mmd::_0,
            true => Mmd::_1,
        }
    }
    ///Read
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mmd::_0
    }
    ///Write.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mmd::_1
    }
}
///Field `MMD` writer - MII/RMII Management Mode
pub type MmdW<'a, REG> = crate::BitWriter<'a, REG, Mmd>;
impl<'a, REG> MmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mmd::_0)
    }
    ///Write.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmd::_1)
    }
}
///Field `MDO` reader - MII/RMII Management Data-Out
pub type MdoR = crate::BitReader;
///Field `MDO` writer - MII/RMII Management Data-Out
pub type MdoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDI` reader - MII/RMII Management Data-In
pub type MdiR = crate::BitReader;
impl R {
    ///Bit 0 - MII/RMII Management Data Clock
    #[inline(always)]
    pub fn mdc(&self) -> MdcR {
        MdcR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MII/RMII Management Mode
    #[inline(always)]
    pub fn mmd(&self) -> MmdR {
        MmdR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MII/RMII Management Data-Out
    #[inline(always)]
    pub fn mdo(&self) -> MdoR {
        MdoR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MII/RMII Management Data-In
    #[inline(always)]
    pub fn mdi(&self) -> MdiR {
        MdiR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIR")
            .field("mdc", &self.mdc())
            .field("mmd", &self.mmd())
            .field("mdo", &self.mdo())
            .field("mdi", &self.mdi())
            .finish()
    }
}
impl W {
    ///Bit 0 - MII/RMII Management Data Clock
    #[inline(always)]
    pub fn mdc(&mut self) -> MdcW<PirSpec> {
        MdcW::new(self, 0)
    }
    ///Bit 1 - MII/RMII Management Mode
    #[inline(always)]
    pub fn mmd(&mut self) -> MmdW<PirSpec> {
        MmdW::new(self, 1)
    }
    ///Bit 2 - MII/RMII Management Data-Out
    #[inline(always)]
    pub fn mdo(&mut self) -> MdoW<PirSpec> {
        MdoW::new(self, 2)
    }
}
/**PHY Interface Register

You can [`read`](crate::Reg::read) this register and get [`pir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PirSpec;
impl crate::RegisterSpec for PirSpec {
    type Ux = u32;
}
///`read()` method returns [`pir::R`](R) reader structure
impl crate::Readable for PirSpec {}
///`write(|w| ..)` method takes [`pir::W`](W) writer structure
impl crate::Writable for PirSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIR to value 0
impl crate::Resettable for PirSpec {}
