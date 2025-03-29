///Register `SFMCMD` reader
pub type R = crate::R<SfmcmdSpec>;
///Register `SFMCMD` writer
pub type W = crate::W<SfmcmdSpec>;
/**Mode select for communication with the SPI bus

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcom {
    ///0: ROM access mode
    _0 = 0,
    ///1: Direct communication mode
    _1 = 1,
}
impl From<Dcom> for bool {
    #[inline(always)]
    fn from(variant: Dcom) -> Self {
        variant as u8 != 0
    }
}
///Field `DCOM` reader - Mode select for communication with the SPI bus
pub type DcomR = crate::BitReader<Dcom>;
impl DcomR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dcom {
        match self.bits {
            false => Dcom::_0,
            true => Dcom::_1,
        }
    }
    ///ROM access mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcom::_0
    }
    ///Direct communication mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcom::_1
    }
}
///Field `DCOM` writer - Mode select for communication with the SPI bus
pub type DcomW<'a, REG> = crate::BitWriter<'a, REG, Dcom>;
impl<'a, REG> DcomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ROM access mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcom::_0)
    }
    ///Direct communication mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcom::_1)
    }
}
impl R {
    ///Bit 0 - Mode select for communication with the SPI bus
    #[inline(always)]
    pub fn dcom(&self) -> DcomR {
        DcomR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMCMD").field("dcom", &self.dcom()).finish()
    }
}
impl W {
    ///Bit 0 - Mode select for communication with the SPI bus
    #[inline(always)]
    pub fn dcom(&mut self) -> DcomW<SfmcmdSpec> {
        DcomW::new(self, 0)
    }
}
/**Communication Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmcmdSpec;
impl crate::RegisterSpec for SfmcmdSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmcmd::R`](R) reader structure
impl crate::Readable for SfmcmdSpec {}
///`write(|w| ..)` method takes [`sfmcmd::W`](W) writer structure
impl crate::Writable for SfmcmdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMCMD to value 0
impl crate::Resettable for SfmcmdSpec {}
