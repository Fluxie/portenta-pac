///Register `HL1CTRL1` reader
pub type R = crate::R<Hl1ctrl1Spec>;
///Register `HL1CTRL1` writer
pub type W = crate::W<Hl1ctrl1Spec>;
///Field `L1REQ` reader - L1 Transition Request
pub type L1reqR = crate::BitReader;
///Field `L1REQ` writer - L1 Transition Request
pub type L1reqW<'a, REG> = crate::BitWriter<'a, REG>;
/**L1 Request Completion Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum L1status {
    ///0: ACK received
    _00 = 0,
    ///1: NYET received
    _01 = 1,
    ///2: STALL received
    _10 = 2,
    ///3: Transaction error
    _11 = 3,
}
impl From<L1status> for u8 {
    #[inline(always)]
    fn from(variant: L1status) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for L1status {
    type Ux = u8;
}
impl crate::IsEnum for L1status {}
///Field `L1STATUS` reader - L1 Request Completion Status
pub type L1statusR = crate::FieldReader<L1status>;
impl L1statusR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1status {
        match self.bits {
            0 => L1status::_00,
            1 => L1status::_01,
            2 => L1status::_10,
            3 => L1status::_11,
            _ => unreachable!(),
        }
    }
    ///ACK received
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == L1status::_00
    }
    ///NYET received
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == L1status::_01
    }
    ///STALL received
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == L1status::_10
    }
    ///Transaction error
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == L1status::_11
    }
}
impl R {
    ///Bit 0 - L1 Transition Request
    #[inline(always)]
    pub fn l1req(&self) -> L1reqR {
        L1reqR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - L1 Request Completion Status
    #[inline(always)]
    pub fn l1status(&self) -> L1statusR {
        L1statusR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HL1CTRL1")
            .field("l1req", &self.l1req())
            .field("l1status", &self.l1status())
            .finish()
    }
}
impl W {
    ///Bit 0 - L1 Transition Request
    #[inline(always)]
    pub fn l1req(&mut self) -> L1reqW<Hl1ctrl1Spec> {
        L1reqW::new(self, 0)
    }
}
/**Host L1 Control Register 1

You can [`read`](crate::Reg::read) this register and get [`hl1ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl1ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Hl1ctrl1Spec;
impl crate::RegisterSpec for Hl1ctrl1Spec {
    type Ux = u16;
}
///`read()` method returns [`hl1ctrl1::R`](R) reader structure
impl crate::Readable for Hl1ctrl1Spec {}
///`write(|w| ..)` method takes [`hl1ctrl1::W`](W) writer structure
impl crate::Writable for Hl1ctrl1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HL1CTRL1 to value 0
impl crate::Resettable for Hl1ctrl1Spec {}
