///Register `ICMR1` reader
pub type R = crate::R<Icmr1Spec>;
///Register `ICMR1` writer
pub type W = crate::W<Icmr1Spec>;
/**Bit Counter

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bc {
    ///0: 9 bits
    _000 = 0,
    ///1: 2 bits
    _001 = 1,
    ///2: 3 bits
    _010 = 2,
    ///3: 4 bits
    _011 = 3,
    ///4: 5 bits
    _100 = 4,
    ///5: 6 bits
    _101 = 5,
    ///6: 7 bits
    _110 = 6,
    ///7: 8 bits
    _111 = 7,
}
impl From<Bc> for u8 {
    #[inline(always)]
    fn from(variant: Bc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bc {
    type Ux = u8;
}
impl crate::IsEnum for Bc {}
///Field `BC` reader - Bit Counter
pub type BcR = crate::FieldReader<Bc>;
impl BcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bc {
        match self.bits {
            0 => Bc::_000,
            1 => Bc::_001,
            2 => Bc::_010,
            3 => Bc::_011,
            4 => Bc::_100,
            5 => Bc::_101,
            6 => Bc::_110,
            7 => Bc::_111,
            _ => unreachable!(),
        }
    }
    ///9 bits
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Bc::_000
    }
    ///2 bits
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Bc::_001
    }
    ///3 bits
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Bc::_010
    }
    ///4 bits
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Bc::_011
    }
    ///5 bits
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Bc::_100
    }
    ///6 bits
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Bc::_101
    }
    ///7 bits
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Bc::_110
    }
    ///8 bits
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Bc::_111
    }
}
///Field `BC` writer - Bit Counter
pub type BcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bc, crate::Safe>;
impl<'a, REG> BcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///9 bits
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_000)
    }
    ///2 bits
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_001)
    }
    ///3 bits
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_010)
    }
    ///4 bits
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_011)
    }
    ///5 bits
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_100)
    }
    ///6 bits
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_101)
    }
    ///7 bits
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_110)
    }
    ///8 bits
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_111)
    }
}
/**BC Write Protect

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcwp {
    ///0: Write enable BC\[2:0\] bits
    _0 = 0,
    ///1: Write protect BC\[2:0\] bits
    _1 = 1,
}
impl From<Bcwp> for bool {
    #[inline(always)]
    fn from(variant: Bcwp) -> Self {
        variant as u8 != 0
    }
}
///Field `BCWP` writer - BC Write Protect
pub type BcwpW<'a, REG> = crate::BitWriter<'a, REG, Bcwp>;
impl<'a, REG> BcwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write enable BC\[2:0\] bits
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bcwp::_0)
    }
    ///Write protect BC\[2:0\] bits
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bcwp::_1)
    }
}
///Field `CKS` reader - Internal Reference Clock Select
pub type CksR = crate::FieldReader;
///Field `CKS` writer - Internal Reference Clock Select
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**MST/TRS Write Protect

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtwp {
    ///0: Write protect MST and TRS bits in ICCR2
    _0 = 0,
    ///1: Write enable MST and TRS bits in ICCR2
    _1 = 1,
}
impl From<Mtwp> for bool {
    #[inline(always)]
    fn from(variant: Mtwp) -> Self {
        variant as u8 != 0
    }
}
///Field `MTWP` reader - MST/TRS Write Protect
pub type MtwpR = crate::BitReader<Mtwp>;
impl MtwpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mtwp {
        match self.bits {
            false => Mtwp::_0,
            true => Mtwp::_1,
        }
    }
    ///Write protect MST and TRS bits in ICCR2
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mtwp::_0
    }
    ///Write enable MST and TRS bits in ICCR2
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mtwp::_1
    }
}
///Field `MTWP` writer - MST/TRS Write Protect
pub type MtwpW<'a, REG> = crate::BitWriter<'a, REG, Mtwp>;
impl<'a, REG> MtwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write protect MST and TRS bits in ICCR2
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mtwp::_0)
    }
    ///Write enable MST and TRS bits in ICCR2
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtwp::_1)
    }
}
impl R {
    ///Bits 0:2 - Bit Counter
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(self.bits & 7)
    }
    ///Bits 4:6 - Internal Reference Clock Select
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new((self.bits >> 4) & 7)
    }
    ///Bit 7 - MST/TRS Write Protect
    #[inline(always)]
    pub fn mtwp(&self) -> MtwpR {
        MtwpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICMR1")
            .field("bc", &self.bc())
            .field("cks", &self.cks())
            .field("mtwp", &self.mtwp())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Bit Counter
    #[inline(always)]
    pub fn bc(&mut self) -> BcW<Icmr1Spec> {
        BcW::new(self, 0)
    }
    ///Bit 3 - BC Write Protect
    #[inline(always)]
    pub fn bcwp(&mut self) -> BcwpW<Icmr1Spec> {
        BcwpW::new(self, 3)
    }
    ///Bits 4:6 - Internal Reference Clock Select
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<Icmr1Spec> {
        CksW::new(self, 4)
    }
    ///Bit 7 - MST/TRS Write Protect
    #[inline(always)]
    pub fn mtwp(&mut self) -> MtwpW<Icmr1Spec> {
        MtwpW::new(self, 7)
    }
}
/**I2C Bus Mode Register 1

You can [`read`](crate::Reg::read) this register and get [`icmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Icmr1Spec;
impl crate::RegisterSpec for Icmr1Spec {
    type Ux = u8;
}
///`read()` method returns [`icmr1::R`](R) reader structure
impl crate::Readable for Icmr1Spec {}
///`write(|w| ..)` method takes [`icmr1::W`](W) writer structure
impl crate::Writable for Icmr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICMR1 to value 0x08
impl crate::Resettable for Icmr1Spec {
    const RESET_VALUE: u8 = 0x08;
}
