///Register `STBRAMSAR` reader
pub type R = crate::R<StbramsarSpec>;
///Register `STBRAMSAR` writer
pub type W = crate::W<StbramsarSpec>;
/**Security attributes of each region for Standby RAM

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nsbstbr {
    ///0: Region7-0 are all Secure.
    _0x0 = 0,
    ///1: Region7 is Non-secure. Region6-0 are Secure
    _0x1 = 1,
    ///2: Region7-6 are Non-secure. Region5-0 are Secure.
    _0x2 = 2,
    ///3: Region7-5 are Non-secure. Region4-0 are Secure.
    _0x3 = 3,
    ///4: Region7-4 are Non-secure. Region 3-0 are Secure.
    _0x4 = 4,
    ///5: Region7-3 are Non-secure. Region 2-0 are Secure.
    _0x5 = 5,
    ///6: Region7-2 are Non-secure. Region 1-0 are Secure.
    _0x6 = 6,
    ///7: Region7-1 are Non-Secure. Region0 is Secure.
    _0x7 = 7,
    ///8: Region7-0 are all Non-Secure.
    Others = 8,
}
impl From<Nsbstbr> for u8 {
    #[inline(always)]
    fn from(variant: Nsbstbr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nsbstbr {
    type Ux = u8;
}
impl crate::IsEnum for Nsbstbr {}
///Field `NSBSTBR` reader - Security attributes of each region for Standby RAM
pub type NsbstbrR = crate::FieldReader<Nsbstbr>;
impl NsbstbrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nsbstbr {
        match self.bits {
            0 => Nsbstbr::_0x0,
            1 => Nsbstbr::_0x1,
            2 => Nsbstbr::_0x2,
            3 => Nsbstbr::_0x3,
            4 => Nsbstbr::_0x4,
            5 => Nsbstbr::_0x5,
            6 => Nsbstbr::_0x6,
            7 => Nsbstbr::_0x7,
            _ => Nsbstbr::Others,
        }
    }
    ///Region7-0 are all Secure.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Nsbstbr::_0x0
    }
    ///Region7 is Non-secure. Region6-0 are Secure
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Nsbstbr::_0x1
    }
    ///Region7-6 are Non-secure. Region5-0 are Secure.
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Nsbstbr::_0x2
    }
    ///Region7-5 are Non-secure. Region4-0 are Secure.
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Nsbstbr::_0x3
    }
    ///Region7-4 are Non-secure. Region 3-0 are Secure.
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Nsbstbr::_0x4
    }
    ///Region7-3 are Non-secure. Region 2-0 are Secure.
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Nsbstbr::_0x5
    }
    ///Region7-2 are Non-secure. Region 1-0 are Secure.
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Nsbstbr::_0x6
    }
    ///Region7-1 are Non-Secure. Region0 is Secure.
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Nsbstbr::_0x7
    }
    ///Region7-0 are all Non-Secure.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Nsbstbr::Others)
    }
}
///Field `NSBSTBR` writer - Security attributes of each region for Standby RAM
pub type NsbstbrW<'a, REG> = crate::FieldWriter<'a, REG, 4, Nsbstbr, crate::Safe>;
impl<'a, REG> NsbstbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Region7-0 are all Secure.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Nsbstbr::_0x0)
    }
    ///Region7 is Non-secure. Region6-0 are Secure
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Nsbstbr::_0x1)
    }
    ///Region7-6 are Non-secure. Region5-0 are Secure.
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Nsbstbr::_0x2)
    }
    ///Region7-5 are Non-secure. Region4-0 are Secure.
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Nsbstbr::_0x3)
    }
    ///Region7-4 are Non-secure. Region 3-0 are Secure.
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Nsbstbr::_0x4)
    }
    ///Region7-3 are Non-secure. Region 2-0 are Secure.
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Nsbstbr::_0x5)
    }
    ///Region7-2 are Non-secure. Region 1-0 are Secure.
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Nsbstbr::_0x6)
    }
    ///Region7-1 are Non-Secure. Region0 is Secure.
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Nsbstbr::_0x7)
    }
    ///Region7-0 are all Non-Secure.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Nsbstbr::Others)
    }
}
impl R {
    ///Bits 0:3 - Security attributes of each region for Standby RAM
    #[inline(always)]
    pub fn nsbstbr(&self) -> NsbstbrR {
        NsbstbrR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STBRAMSAR").field("nsbstbr", &self.nsbstbr()).finish()
    }
}
impl W {
    ///Bits 0:3 - Security attributes of each region for Standby RAM
    #[inline(always)]
    pub fn nsbstbr(&mut self) -> NsbstbrW<StbramsarSpec> {
        NsbstbrW::new(self, 0)
    }
}
/**Standby RAM memory Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`stbramsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stbramsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StbramsarSpec;
impl crate::RegisterSpec for StbramsarSpec {
    type Ux = u32;
}
///`read()` method returns [`stbramsar::R`](R) reader structure
impl crate::Readable for StbramsarSpec {}
///`write(|w| ..)` method takes [`stbramsar::W`](W) writer structure
impl crate::Writable for StbramsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STBRAMSAR to value 0xffff_fff0
impl crate::Resettable for StbramsarSpec {
    const RESET_VALUE: u32 = 0xffff_fff0;
}
