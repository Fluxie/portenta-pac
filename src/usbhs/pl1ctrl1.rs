///Register `PL1CTRL1` reader
pub type R = crate::R<Pl1ctrl1Spec>;
///Register `PL1CTRL1` writer
pub type W = crate::W<Pl1ctrl1Spec>;
/**L1 Response Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1respen {
    ///0: Do not support LPM
    _0 = 0,
    ///1: Support LPM
    _1 = 1,
}
impl From<L1respen> for bool {
    #[inline(always)]
    fn from(variant: L1respen) -> Self {
        variant as u8 != 0
    }
}
///Field `L1RESPEN` reader - L1 Response Enable
pub type L1respenR = crate::BitReader<L1respen>;
impl L1respenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1respen {
        match self.bits {
            false => L1respen::_0,
            true => L1respen::_1,
        }
    }
    ///Do not support LPM
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1respen::_0
    }
    ///Support LPM
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1respen::_1
    }
}
///Field `L1RESPEN` writer - L1 Response Enable
pub type L1respenW<'a, REG> = crate::BitWriter<'a, REG, L1respen>;
impl<'a, REG> L1respenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not support LPM
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1respen::_0)
    }
    ///Support LPM
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1respen::_1)
    }
}
/**L1 Response Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum L1respmd {
    ///0: NYET response
    _00 = 0,
    ///1: ACK response
    _01 = 1,
    ///2: STALL response
    _10 = 2,
    ///3: Response based on L1NEGOMD setting
    _11 = 3,
}
impl From<L1respmd> for u8 {
    #[inline(always)]
    fn from(variant: L1respmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for L1respmd {
    type Ux = u8;
}
impl crate::IsEnum for L1respmd {}
///Field `L1RESPMD` reader - L1 Response Mode
pub type L1respmdR = crate::FieldReader<L1respmd>;
impl L1respmdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1respmd {
        match self.bits {
            0 => L1respmd::_00,
            1 => L1respmd::_01,
            2 => L1respmd::_10,
            3 => L1respmd::_11,
            _ => unreachable!(),
        }
    }
    ///NYET response
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == L1respmd::_00
    }
    ///ACK response
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == L1respmd::_01
    }
    ///STALL response
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == L1respmd::_10
    }
    ///Response based on L1NEGOMD setting
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == L1respmd::_11
    }
}
///Field `L1RESPMD` writer - L1 Response Mode
pub type L1respmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, L1respmd, crate::Safe>;
impl<'a, REG> L1respmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///NYET response
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(L1respmd::_00)
    }
    ///ACK response
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(L1respmd::_01)
    }
    ///STALL response
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(L1respmd::_10)
    }
    ///Response based on L1NEGOMD setting
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(L1respmd::_11)
    }
}
/**L1 Response Negotiation Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1negomd {
    ///0: Return ACK when received HIRD is larger than HIRDTHR\[3:0\]. Otherwise (including when HIRD = HIRDTHR\[3:0\]), return NYET
    _0 = 0,
    ///1: Return ACK when received HIRD is smaller than HIRDTHR\[3:0\]. Otherwise (including when HIRD = HIRDTHR\[3:0\]), return NYET
    _1 = 1,
}
impl From<L1negomd> for bool {
    #[inline(always)]
    fn from(variant: L1negomd) -> Self {
        variant as u8 != 0
    }
}
///Field `L1NEGOMD` reader - L1 Response Negotiation Control
pub type L1negomdR = crate::BitReader<L1negomd>;
impl L1negomdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1negomd {
        match self.bits {
            false => L1negomd::_0,
            true => L1negomd::_1,
        }
    }
    ///Return ACK when received HIRD is larger than HIRDTHR\[3:0\]. Otherwise (including when HIRD = HIRDTHR\[3:0\]), return NYET
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1negomd::_0
    }
    ///Return ACK when received HIRD is smaller than HIRDTHR\[3:0\]. Otherwise (including when HIRD = HIRDTHR\[3:0\]), return NYET
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1negomd::_1
    }
}
///Field `L1NEGOMD` writer - L1 Response Negotiation Control
pub type L1negomdW<'a, REG> = crate::BitWriter<'a, REG, L1negomd>;
impl<'a, REG> L1negomdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Return ACK when received HIRD is larger than HIRDTHR\[3:0\]. Otherwise (including when HIRD = HIRDTHR\[3:0\]), return NYET
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1negomd::_0)
    }
    ///Return ACK when received HIRD is smaller than HIRDTHR\[3:0\]. Otherwise (including when HIRD = HIRDTHR\[3:0\]), return NYET
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1negomd::_1)
    }
}
/**DVSQ Extension Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvsq {
    ///0: Powered state
    _0000 = 0,
    ///1: Default state
    _0001 = 1,
    ///2: Address state
    _0010 = 2,
    ///3: Configured state
    _0011 = 3,
    ///4: Suspend state
    _01xx = 4,
    ///8: L1 state
    _10xx = 8,
}
impl From<Dvsq> for u8 {
    #[inline(always)]
    fn from(variant: Dvsq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvsq {
    type Ux = u8;
}
impl crate::IsEnum for Dvsq {}
///Field `DVSQ` reader - DVSQ Extension Flag
pub type DvsqR = crate::FieldReader<Dvsq>;
impl DvsqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dvsq> {
        match self.bits {
            0 => Some(Dvsq::_0000),
            1 => Some(Dvsq::_0001),
            2 => Some(Dvsq::_0010),
            3 => Some(Dvsq::_0011),
            4 => Some(Dvsq::_01xx),
            8 => Some(Dvsq::_10xx),
            _ => None,
        }
    }
    ///Powered state
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Dvsq::_0000
    }
    ///Default state
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Dvsq::_0001
    }
    ///Address state
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Dvsq::_0010
    }
    ///Configured state
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Dvsq::_0011
    }
    ///Suspend state
    #[inline(always)]
    pub fn is_01xx(&self) -> bool {
        *self == Dvsq::_01xx
    }
    ///L1 state
    #[inline(always)]
    pub fn is_10xx(&self) -> bool {
        *self == Dvsq::_10xx
    }
}
///Field `HIRDTHR` reader - L1 Response Negotiation Threshold Value
pub type HirdthrR = crate::FieldReader;
///Field `HIRDTHR` writer - L1 Response Negotiation Threshold Value
pub type HirdthrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**PHY Control Mode at L1 Return

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1extmd {
    ///0: Do not set LPSTS.SUSPENDM bit through hardware when Host K is received
    _0 = 0,
    ///1: Set LPSTS.SUSPENDM bit through hardware when Host K is received
    _1 = 1,
}
impl From<L1extmd> for bool {
    #[inline(always)]
    fn from(variant: L1extmd) -> Self {
        variant as u8 != 0
    }
}
///Field `L1EXTMD` reader - PHY Control Mode at L1 Return
pub type L1extmdR = crate::BitReader<L1extmd>;
impl L1extmdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1extmd {
        match self.bits {
            false => L1extmd::_0,
            true => L1extmd::_1,
        }
    }
    ///Do not set LPSTS.SUSPENDM bit through hardware when Host K is received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1extmd::_0
    }
    ///Set LPSTS.SUSPENDM bit through hardware when Host K is received
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1extmd::_1
    }
}
///Field `L1EXTMD` writer - PHY Control Mode at L1 Return
pub type L1extmdW<'a, REG> = crate::BitWriter<'a, REG, L1extmd>;
impl<'a, REG> L1extmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not set LPSTS.SUSPENDM bit through hardware when Host K is received
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1extmd::_0)
    }
    ///Set LPSTS.SUSPENDM bit through hardware when Host K is received
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1extmd::_1)
    }
}
impl R {
    ///Bit 0 - L1 Response Enable
    #[inline(always)]
    pub fn l1respen(&self) -> L1respenR {
        L1respenR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - L1 Response Mode
    #[inline(always)]
    pub fn l1respmd(&self) -> L1respmdR {
        L1respmdR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - L1 Response Negotiation Control
    #[inline(always)]
    pub fn l1negomd(&self) -> L1negomdR {
        L1negomdR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - DVSQ Extension Flag
    #[inline(always)]
    pub fn dvsq(&self) -> DvsqR {
        DvsqR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - L1 Response Negotiation Threshold Value
    #[inline(always)]
    pub fn hirdthr(&self) -> HirdthrR {
        HirdthrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 14 - PHY Control Mode at L1 Return
    #[inline(always)]
    pub fn l1extmd(&self) -> L1extmdR {
        L1extmdR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PL1CTRL1")
            .field("l1respen", &self.l1respen())
            .field("l1respmd", &self.l1respmd())
            .field("l1negomd", &self.l1negomd())
            .field("dvsq", &self.dvsq())
            .field("hirdthr", &self.hirdthr())
            .field("l1extmd", &self.l1extmd())
            .finish()
    }
}
impl W {
    ///Bit 0 - L1 Response Enable
    #[inline(always)]
    pub fn l1respen(&mut self) -> L1respenW<Pl1ctrl1Spec> {
        L1respenW::new(self, 0)
    }
    ///Bits 1:2 - L1 Response Mode
    #[inline(always)]
    pub fn l1respmd(&mut self) -> L1respmdW<Pl1ctrl1Spec> {
        L1respmdW::new(self, 1)
    }
    ///Bit 3 - L1 Response Negotiation Control
    #[inline(always)]
    pub fn l1negomd(&mut self) -> L1negomdW<Pl1ctrl1Spec> {
        L1negomdW::new(self, 3)
    }
    ///Bits 8:11 - L1 Response Negotiation Threshold Value
    #[inline(always)]
    pub fn hirdthr(&mut self) -> HirdthrW<Pl1ctrl1Spec> {
        HirdthrW::new(self, 8)
    }
    ///Bit 14 - PHY Control Mode at L1 Return
    #[inline(always)]
    pub fn l1extmd(&mut self) -> L1extmdW<Pl1ctrl1Spec> {
        L1extmdW::new(self, 14)
    }
}
/**Function L1 Control Register 1

You can [`read`](crate::Reg::read) this register and get [`pl1ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl1ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Pl1ctrl1Spec;
impl crate::RegisterSpec for Pl1ctrl1Spec {
    type Ux = u16;
}
///`read()` method returns [`pl1ctrl1::R`](R) reader structure
impl crate::Readable for Pl1ctrl1Spec {}
///`write(|w| ..)` method takes [`pl1ctrl1::W`](W) writer structure
impl crate::Writable for Pl1ctrl1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PL1CTRL1 to value 0
impl crate::Resettable for Pl1ctrl1Spec {}
