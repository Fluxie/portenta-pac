///Register `ELSEGR%s` reader
pub type R = crate::R<ElsegrSpec>;
///Register `ELSEGR%s` writer
pub type W = crate::W<ElsegrSpec>;
/**Software Event Generation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seg {
    ///0: Normal operation
    _0 = 0,
    ///1: Software event is generated.
    _1 = 1,
}
impl From<Seg> for bool {
    #[inline(always)]
    fn from(variant: Seg) -> Self {
        variant as u8 != 0
    }
}
///Field `SEG` writer - Software Event Generation
pub type SegW<'a, REG> = crate::BitWriter<'a, REG, Seg>;
impl<'a, REG> SegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Seg::_0)
    }
    ///Software event is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Seg::_1)
    }
}
/**SEG Bit Write Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum We {
    ///0: Write to SEG bit disabled.
    _0 = 0,
    ///1: Write to SEG bit enabled.
    _1 = 1,
}
impl From<We> for bool {
    #[inline(always)]
    fn from(variant: We) -> Self {
        variant as u8 != 0
    }
}
///Field `WE` reader - SEG Bit Write Enable
pub type WeR = crate::BitReader<We>;
impl WeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> We {
        match self.bits {
            false => We::_0,
            true => We::_1,
        }
    }
    ///Write to SEG bit disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == We::_0
    }
    ///Write to SEG bit enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == We::_1
    }
}
///Field `WE` writer - SEG Bit Write Enable
pub type WeW<'a, REG> = crate::BitWriter<'a, REG, We>;
impl<'a, REG> WeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write to SEG bit disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(We::_0)
    }
    ///Write to SEG bit enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(We::_1)
    }
}
/**ELSEGR Register Write Disable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wi {
    ///0: Write to ELSEGR register enabled.
    _0 = 0,
    ///1: Write to ELSEGR register disabled.
    _1 = 1,
}
impl From<Wi> for bool {
    #[inline(always)]
    fn from(variant: Wi) -> Self {
        variant as u8 != 0
    }
}
///Field `WI` writer - ELSEGR Register Write Disable
pub type WiW<'a, REG> = crate::BitWriter<'a, REG, Wi>;
impl<'a, REG> WiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write to ELSEGR register enabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wi::_0)
    }
    ///Write to ELSEGR register disabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wi::_1)
    }
}
impl R {
    ///Bit 6 - SEG Bit Write Enable
    #[inline(always)]
    pub fn we(&self) -> WeR {
        WeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ELSEGR").field("we", &self.we()).finish()
    }
}
impl W {
    ///Bit 0 - Software Event Generation
    #[inline(always)]
    pub fn seg(&mut self) -> SegW<ElsegrSpec> {
        SegW::new(self, 0)
    }
    ///Bit 6 - SEG Bit Write Enable
    #[inline(always)]
    pub fn we(&mut self) -> WeW<ElsegrSpec> {
        WeW::new(self, 6)
    }
    ///Bit 7 - ELSEGR Register Write Disable
    #[inline(always)]
    pub fn wi(&mut self) -> WiW<ElsegrSpec> {
        WiW::new(self, 7)
    }
}
/**Event Link Software Event Generation Register %s

You can [`read`](crate::Reg::read) this register and get [`elsegr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsegr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ElsegrSpec;
impl crate::RegisterSpec for ElsegrSpec {
    type Ux = u8;
}
///`read()` method returns [`elsegr::R`](R) reader structure
impl crate::Readable for ElsegrSpec {}
///`write(|w| ..)` method takes [`elsegr::W`](W) writer structure
impl crate::Writable for ElsegrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ELSEGR%s to value 0x80
impl crate::Resettable for ElsegrSpec {
    const RESET_VALUE: u8 = 0x80;
}
