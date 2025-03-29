///Register `AGTIOSEL` reader
pub type R = crate::R<AgtioselSpec>;
///Register `AGTIOSEL` writer
pub type W = crate::W<AgtioselSpec>;
/**AGTIOn Pin Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    ///0: Select Pm/AGTIO as AGTIO. Pm/AGTIO can not be used as AGTIO input pin in Deep Software Standby mode. (m = 100, 301, 407, and 705 (AGT0), m = 204 and 400 (AGT1), m = 103 (AGT2), m = 600(AGT3).)
    _00 = 0,
    ///1: Select P404/AGTIO as AGTIO P404/AGTIO can be used as AGTIO input pin in Deep Software Standby mode. P404/AGTIOn is input only. It cannot be used for output.
    _01 = 1,
    ///2: Select P402/AGTIO as AGTIO P402/AGTIO can be used as AGTIO input pin in Deep Software Standby mode. P402/AGTIOn is input only. It cannot be used for output.
    _10 = 2,
    ///3: Select P403/AGTIO as AGTIO. P403/AGTIO can be used as AGTIO input pin in Deep Software Standby mode. P403/AGTIOn is input only. It cannot be used for output.
    _11 = 3,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
///Field `SEL` reader - AGTIOn Pin Select
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            0 => Sel::_00,
            1 => Sel::_01,
            2 => Sel::_10,
            3 => Sel::_11,
            _ => unreachable!(),
        }
    }
    ///Select Pm/AGTIO as AGTIO. Pm/AGTIO can not be used as AGTIO input pin in Deep Software Standby mode. (m = 100, 301, 407, and 705 (AGT0), m = 204 and 400 (AGT1), m = 103 (AGT2), m = 600(AGT3).)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sel::_00
    }
    ///Select P404/AGTIO as AGTIO P404/AGTIO can be used as AGTIO input pin in Deep Software Standby mode. P404/AGTIOn is input only. It cannot be used for output.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sel::_01
    }
    ///Select P402/AGTIO as AGTIO P402/AGTIO can be used as AGTIO input pin in Deep Software Standby mode. P402/AGTIOn is input only. It cannot be used for output.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sel::_10
    }
    ///Select P403/AGTIO as AGTIO. P403/AGTIO can be used as AGTIO input pin in Deep Software Standby mode. P403/AGTIOn is input only. It cannot be used for output.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sel::_11
    }
}
///Field `SEL` writer - AGTIOn Pin Select
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sel, crate::Safe>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select Pm/AGTIO as AGTIO. Pm/AGTIO can not be used as AGTIO input pin in Deep Software Standby mode. (m = 100, 301, 407, and 705 (AGT0), m = 204 and 400 (AGT1), m = 103 (AGT2), m = 600(AGT3).)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_00)
    }
    ///Select P404/AGTIO as AGTIO P404/AGTIO can be used as AGTIO input pin in Deep Software Standby mode. P404/AGTIOn is input only. It cannot be used for output.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_01)
    }
    ///Select P402/AGTIO as AGTIO P402/AGTIO can be used as AGTIO input pin in Deep Software Standby mode. P402/AGTIOn is input only. It cannot be used for output.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_10)
    }
    ///Select P403/AGTIO as AGTIO. P403/AGTIO can be used as AGTIO input pin in Deep Software Standby mode. P403/AGTIOn is input only. It cannot be used for output.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_11)
    }
}
/**AGTIOn Pin Input Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ties {
    ///0: External event input is disabled during Software Standby mode
    _0 = 0,
    ///1: External event input is enabled during Software Standby mode
    _1 = 1,
}
impl From<Ties> for bool {
    #[inline(always)]
    fn from(variant: Ties) -> Self {
        variant as u8 != 0
    }
}
///Field `TIES` reader - AGTIOn Pin Input Enable
pub type TiesR = crate::BitReader<Ties>;
impl TiesR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ties {
        match self.bits {
            false => Ties::_0,
            true => Ties::_1,
        }
    }
    ///External event input is disabled during Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ties::_0
    }
    ///External event input is enabled during Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ties::_1
    }
}
///Field `TIES` writer - AGTIOn Pin Input Enable
pub type TiesW<'a, REG> = crate::BitWriter<'a, REG, Ties>;
impl<'a, REG> TiesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External event input is disabled during Software Standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ties::_0)
    }
    ///External event input is enabled during Software Standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ties::_1)
    }
}
impl R {
    ///Bits 0:1 - AGTIOn Pin Select
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(self.bits & 3)
    }
    ///Bit 4 - AGTIOn Pin Input Enable
    #[inline(always)]
    pub fn ties(&self) -> TiesR {
        TiesR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTIOSEL")
            .field("sel", &self.sel())
            .field("ties", &self.ties())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - AGTIOn Pin Select
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<AgtioselSpec> {
        SelW::new(self, 0)
    }
    ///Bit 4 - AGTIOn Pin Input Enable
    #[inline(always)]
    pub fn ties(&mut self) -> TiesW<AgtioselSpec> {
        TiesW::new(self, 4)
    }
}
/**AGT Pin Select Register

You can [`read`](crate::Reg::read) this register and get [`agtiosel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtiosel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AgtioselSpec;
impl crate::RegisterSpec for AgtioselSpec {
    type Ux = u8;
}
///`read()` method returns [`agtiosel::R`](R) reader structure
impl crate::Readable for AgtioselSpec {}
///`write(|w| ..)` method takes [`agtiosel::W`](W) writer structure
impl crate::Writable for AgtioselSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTIOSEL to value 0
impl crate::Resettable for AgtioselSpec {}
