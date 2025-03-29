///Register `CFDTMSTS%s` reader
pub type R = crate::R<CfdtmstsSpec>;
///Register `CFDTMSTS%s` writer
pub type W = crate::W<CfdtmstsSpec>;
/**TX Message Buffer Transmission Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmtsts {
    ///0: No on-going transmission
    _0 = 0,
    ///1: On-going transmission
    _1 = 1,
}
impl From<Tmtsts> for bool {
    #[inline(always)]
    fn from(variant: Tmtsts) -> Self {
        variant as u8 != 0
    }
}
///Field `TMTSTS` reader - TX Message Buffer Transmission Status
pub type TmtstsR = crate::BitReader<Tmtsts>;
impl TmtstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmtsts {
        match self.bits {
            false => Tmtsts::_0,
            true => Tmtsts::_1,
        }
    }
    ///No on-going transmission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmtsts::_0
    }
    ///On-going transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmtsts::_1
    }
}
/**TX Message Buffer Transmission Result Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmtrf {
    ///0: No result
    _00 = 0,
    ///1: Transmission aborted from the TX message buffer
    _01 = 1,
    ///2: Transmission successful from the TX message buffer and transmission abort was not requested
    _10 = 2,
    ///3: Transmission successful from the TX message buffer and transmission abort was requested
    _11 = 3,
}
impl From<Tmtrf> for u8 {
    #[inline(always)]
    fn from(variant: Tmtrf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmtrf {
    type Ux = u8;
}
impl crate::IsEnum for Tmtrf {}
///Field `TMTRF` reader - TX Message Buffer Transmission Result Flag
pub type TmtrfR = crate::FieldReader<Tmtrf>;
impl TmtrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmtrf {
        match self.bits {
            0 => Tmtrf::_00,
            1 => Tmtrf::_01,
            2 => Tmtrf::_10,
            3 => Tmtrf::_11,
            _ => unreachable!(),
        }
    }
    ///No result
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tmtrf::_00
    }
    ///Transmission aborted from the TX message buffer
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tmtrf::_01
    }
    ///Transmission successful from the TX message buffer and transmission abort was not requested
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tmtrf::_10
    }
    ///Transmission successful from the TX message buffer and transmission abort was requested
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tmtrf::_11
    }
}
///Field `TMTRF` writer - TX Message Buffer Transmission Result Flag
pub type TmtrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tmtrf, crate::Safe>;
impl<'a, REG> TmtrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No result
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tmtrf::_00)
    }
    ///Transmission aborted from the TX message buffer
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tmtrf::_01)
    }
    ///Transmission successful from the TX message buffer and transmission abort was not requested
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tmtrf::_10)
    }
    ///Transmission successful from the TX message buffer and transmission abort was requested
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tmtrf::_11)
    }
}
/**TX Message Buffer Transmission Request Mirrored

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmtrm {
    ///0: TX message buffer transmission not requested
    _0 = 0,
    ///1: TX message buffer transmission requested
    _1 = 1,
}
impl From<Tmtrm> for bool {
    #[inline(always)]
    fn from(variant: Tmtrm) -> Self {
        variant as u8 != 0
    }
}
///Field `TMTRM` reader - TX Message Buffer Transmission Request Mirrored
pub type TmtrmR = crate::BitReader<Tmtrm>;
impl TmtrmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmtrm {
        match self.bits {
            false => Tmtrm::_0,
            true => Tmtrm::_1,
        }
    }
    ///TX message buffer transmission not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmtrm::_0
    }
    ///TX message buffer transmission requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmtrm::_1
    }
}
/**TX Message Buffer Transmission Abort Request Mirrored

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmtarm {
    ///0: TX message buffer transmission request abort not requested
    _0 = 0,
    ///1: TX message buffer transmission request abort requested
    _1 = 1,
}
impl From<Tmtarm> for bool {
    #[inline(always)]
    fn from(variant: Tmtarm) -> Self {
        variant as u8 != 0
    }
}
///Field `TMTARM` reader - TX Message Buffer Transmission Abort Request Mirrored
pub type TmtarmR = crate::BitReader<Tmtarm>;
impl TmtarmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmtarm {
        match self.bits {
            false => Tmtarm::_0,
            true => Tmtarm::_1,
        }
    }
    ///TX message buffer transmission request abort not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmtarm::_0
    }
    ///TX message buffer transmission request abort requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmtarm::_1
    }
}
impl R {
    ///Bit 0 - TX Message Buffer Transmission Status
    #[inline(always)]
    pub fn tmtsts(&self) -> TmtstsR {
        TmtstsR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - TX Message Buffer Transmission Result Flag
    #[inline(always)]
    pub fn tmtrf(&self) -> TmtrfR {
        TmtrfR::new((self.bits >> 1) & 3)
    }
    ///Bit 3 - TX Message Buffer Transmission Request Mirrored
    #[inline(always)]
    pub fn tmtrm(&self) -> TmtrmR {
        TmtrmR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TX Message Buffer Transmission Abort Request Mirrored
    #[inline(always)]
    pub fn tmtarm(&self) -> TmtarmR {
        TmtarmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTMSTS")
            .field("tmtsts", &self.tmtsts())
            .field("tmtrf", &self.tmtrf())
            .field("tmtrm", &self.tmtrm())
            .field("tmtarm", &self.tmtarm())
            .finish()
    }
}
impl W {
    ///Bits 1:2 - TX Message Buffer Transmission Result Flag
    #[inline(always)]
    pub fn tmtrf(&mut self) -> TmtrfW<CfdtmstsSpec> {
        TmtrfW::new(self, 1)
    }
}
/**TX Message Buffer Status Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtmstsSpec;
impl crate::RegisterSpec for CfdtmstsSpec {
    type Ux = u8;
}
///`read()` method returns [`cfdtmsts::R`](R) reader structure
impl crate::Readable for CfdtmstsSpec {}
///`write(|w| ..)` method takes [`cfdtmsts::W`](W) writer structure
impl crate::Writable for CfdtmstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTMSTS%s to value 0
impl crate::Resettable for CfdtmstsSpec {}
