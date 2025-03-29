///Register `SRAMWTSC` reader
pub type R = crate::R<SramwtscSpec>;
///Register `SRAMWTSC` writer
pub type W = crate::W<SramwtscSpec>;
/**SRAM0 wait enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram0wten {
    ///0: No wait
    _0 = 0,
    ///1: Add wait state in read access cycle to SRAM0
    _1 = 1,
}
impl From<Sram0wten> for bool {
    #[inline(always)]
    fn from(variant: Sram0wten) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM0WTEN` reader - SRAM0 wait enable
pub type Sram0wtenR = crate::BitReader<Sram0wten>;
impl Sram0wtenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sram0wten {
        match self.bits {
            false => Sram0wten::_0,
            true => Sram0wten::_1,
        }
    }
    ///No wait
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sram0wten::_0
    }
    ///Add wait state in read access cycle to SRAM0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sram0wten::_1
    }
}
///Field `SRAM0WTEN` writer - SRAM0 wait enable
pub type Sram0wtenW<'a, REG> = crate::BitWriter<'a, REG, Sram0wten>;
impl<'a, REG> Sram0wtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No wait
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sram0wten::_0)
    }
    ///Add wait state in read access cycle to SRAM0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sram0wten::_1)
    }
}
impl R {
    ///Bit 0 - SRAM0 wait enable
    #[inline(always)]
    pub fn sram0wten(&self) -> Sram0wtenR {
        Sram0wtenR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAMWTSC").field("sram0wten", &self.sram0wten()).finish()
    }
}
impl W {
    ///Bit 0 - SRAM0 wait enable
    #[inline(always)]
    pub fn sram0wten(&mut self) -> Sram0wtenW<SramwtscSpec> {
        Sram0wtenW::new(self, 0)
    }
}
/**SRAM Wait State Control Register

You can [`read`](crate::Reg::read) this register and get [`sramwtsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramwtsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SramwtscSpec;
impl crate::RegisterSpec for SramwtscSpec {
    type Ux = u8;
}
///`read()` method returns [`sramwtsc::R`](R) reader structure
impl crate::Readable for SramwtscSpec {}
///`write(|w| ..)` method takes [`sramwtsc::W`](W) writer structure
impl crate::Writable for SramwtscSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRAMWTSC to value 0x01
impl crate::Resettable for SramwtscSpec {
    const RESET_VALUE: u8 = 0x01;
}
