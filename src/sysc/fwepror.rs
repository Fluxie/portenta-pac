///Register `FWEPROR` reader
pub type R = crate::R<FweprorSpec>;
///Register `FWEPROR` writer
pub type W = crate::W<FweprorSpec>;
/**Flash Programming and Erasure

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flwe {
    ///0: Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    _00 = 0,
    ///1: Permits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    _01 = 1,
    ///2: Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    _10 = 2,
    ///3: Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    _11 = 3,
}
impl From<Flwe> for u8 {
    #[inline(always)]
    fn from(variant: Flwe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flwe {
    type Ux = u8;
}
impl crate::IsEnum for Flwe {}
///Field `FLWE` reader - Flash Programming and Erasure
pub type FlweR = crate::FieldReader<Flwe>;
impl FlweR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Flwe {
        match self.bits {
            0 => Flwe::_00,
            1 => Flwe::_01,
            2 => Flwe::_10,
            3 => Flwe::_11,
            _ => unreachable!(),
        }
    }
    ///Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Flwe::_00
    }
    ///Permits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Flwe::_01
    }
    ///Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Flwe::_10
    }
    ///Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Flwe::_11
    }
}
///Field `FLWE` writer - Flash Programming and Erasure
pub type FlweW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flwe, crate::Safe>;
impl<'a, REG> FlweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Flwe::_00)
    }
    ///Permits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Flwe::_01)
    }
    ///Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Flwe::_10)
    }
    ///Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Flwe::_11)
    }
}
impl R {
    ///Bits 0:1 - Flash Programming and Erasure
    #[inline(always)]
    pub fn flwe(&self) -> FlweR {
        FlweR::new(self.bits & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWEPROR").field("flwe", &self.flwe()).finish()
    }
}
impl W {
    ///Bits 0:1 - Flash Programming and Erasure
    #[inline(always)]
    pub fn flwe(&mut self) -> FlweW<FweprorSpec> {
        FlweW::new(self, 0)
    }
}
/**Flash P/E Protect Register

You can [`read`](crate::Reg::read) this register and get [`fwepror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwepror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FweprorSpec;
impl crate::RegisterSpec for FweprorSpec {
    type Ux = u8;
}
///`read()` method returns [`fwepror::R`](R) reader structure
impl crate::Readable for FweprorSpec {}
///`write(|w| ..)` method takes [`fwepror::W`](W) writer structure
impl crate::Writable for FweprorSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FWEPROR to value 0x02
impl crate::Resettable for FweprorSpec {
    const RESET_VALUE: u8 = 0x02;
}
