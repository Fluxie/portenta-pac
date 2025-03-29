///Register `SFMSAC` reader
pub type R = crate::R<SfmsacSpec>;
///Register `SFMSAC` writer
pub type W = crate::W<SfmsacSpec>;
/**Number of address bytes select for the serial interface

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sfmas {
    ///0: 1 byte
    _00 = 0,
    ///1: 2 bytes
    _01 = 1,
    ///2: 3 bytes
    _10 = 2,
    ///3: 4 bytes
    _11 = 3,
}
impl From<Sfmas> for u8 {
    #[inline(always)]
    fn from(variant: Sfmas) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sfmas {
    type Ux = u8;
}
impl crate::IsEnum for Sfmas {}
///Field `SFMAS` reader - Number of address bytes select for the serial interface
pub type SfmasR = crate::FieldReader<Sfmas>;
impl SfmasR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmas {
        match self.bits {
            0 => Sfmas::_00,
            1 => Sfmas::_01,
            2 => Sfmas::_10,
            3 => Sfmas::_11,
            _ => unreachable!(),
        }
    }
    ///1 byte
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sfmas::_00
    }
    ///2 bytes
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sfmas::_01
    }
    ///3 bytes
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sfmas::_10
    }
    ///4 bytes
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sfmas::_11
    }
}
///Field `SFMAS` writer - Number of address bytes select for the serial interface
pub type SfmasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sfmas, crate::Safe>;
impl<'a, REG> SfmasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 byte
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmas::_00)
    }
    ///2 bytes
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmas::_01)
    }
    ///3 bytes
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmas::_10)
    }
    ///4 bytes
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmas::_11)
    }
}
/**Selection of instruction code automatically generated when the serial interface address width is 4 bytes

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfm4bc {
    ///0: Do not use 4-byte address read instruction code
    _0 = 0,
    ///1: Use 4-byte address read instruction code
    _1 = 1,
}
impl From<Sfm4bc> for bool {
    #[inline(always)]
    fn from(variant: Sfm4bc) -> Self {
        variant as u8 != 0
    }
}
///Field `SFM4BC` reader - Selection of instruction code automatically generated when the serial interface address width is 4 bytes
pub type Sfm4bcR = crate::BitReader<Sfm4bc>;
impl Sfm4bcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfm4bc {
        match self.bits {
            false => Sfm4bc::_0,
            true => Sfm4bc::_1,
        }
    }
    ///Do not use 4-byte address read instruction code
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfm4bc::_0
    }
    ///Use 4-byte address read instruction code
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfm4bc::_1
    }
}
///Field `SFM4BC` writer - Selection of instruction code automatically generated when the serial interface address width is 4 bytes
pub type Sfm4bcW<'a, REG> = crate::BitWriter<'a, REG, Sfm4bc>;
impl<'a, REG> Sfm4bcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use 4-byte address read instruction code
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfm4bc::_0)
    }
    ///Use 4-byte address read instruction code
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfm4bc::_1)
    }
}
impl R {
    ///Bits 0:1 - Number of address bytes select for the serial interface
    #[inline(always)]
    pub fn sfmas(&self) -> SfmasR {
        SfmasR::new((self.bits & 3) as u8)
    }
    ///Bit 4 - Selection of instruction code automatically generated when the serial interface address width is 4 bytes
    #[inline(always)]
    pub fn sfm4bc(&self) -> Sfm4bcR {
        Sfm4bcR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMSAC")
            .field("sfmas", &self.sfmas())
            .field("sfm4bc", &self.sfm4bc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Number of address bytes select for the serial interface
    #[inline(always)]
    pub fn sfmas(&mut self) -> SfmasW<SfmsacSpec> {
        SfmasW::new(self, 0)
    }
    ///Bit 4 - Selection of instruction code automatically generated when the serial interface address width is 4 bytes
    #[inline(always)]
    pub fn sfm4bc(&mut self) -> Sfm4bcW<SfmsacSpec> {
        Sfm4bcW::new(self, 4)
    }
}
/**Address Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmsacSpec;
impl crate::RegisterSpec for SfmsacSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmsac::R`](R) reader structure
impl crate::Readable for SfmsacSpec {}
///`write(|w| ..)` method takes [`sfmsac::W`](W) writer structure
impl crate::Writable for SfmsacSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSAC to value 0x02
impl crate::Resettable for SfmsacSpec {
    const RESET_VALUE: u32 = 0x02;
}
