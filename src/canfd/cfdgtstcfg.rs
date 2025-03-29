///Register `CFDGTSTCFG` reader
pub type R = crate::R<CfdgtstcfgSpec>;
///Register `CFDGTSTCFG` writer
pub type W = crate::W<CfdgtstcfgSpec>;
/**Channel n Internal CAN Bus Communication Test Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Icbce {
    ///0: Channel n internal CAN bus communication disabled
    _0 = 0,
    ///1: Channel n internal CAN bus communication enabled
    _1 = 1,
}
impl From<Icbce> for u8 {
    #[inline(always)]
    fn from(variant: Icbce) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Icbce {
    type Ux = u8;
}
impl crate::IsEnum for Icbce {}
///Field `ICBCE` reader - Channel n Internal CAN Bus Communication Test Mode Enable
pub type IcbceR = crate::FieldReader<Icbce>;
impl IcbceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Icbce> {
        match self.bits {
            0 => Some(Icbce::_0),
            1 => Some(Icbce::_1),
            _ => None,
        }
    }
    ///Channel n internal CAN bus communication disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Icbce::_0
    }
    ///Channel n internal CAN bus communication enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Icbce::_1
    }
}
///Field `ICBCE` writer - Channel n Internal CAN Bus Communication Test Mode Enable
pub type IcbceW<'a, REG> = crate::FieldWriter<'a, REG, 2, Icbce>;
impl<'a, REG> IcbceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Channel n internal CAN bus communication disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Icbce::_0)
    }
    ///Channel n internal CAN bus communication enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Icbce::_1)
    }
}
///Field `RTMPS` reader - RAM Test Mode Page Select
pub type RtmpsR = crate::FieldReader<u16>;
///Field `RTMPS` writer - RAM Test Mode Page Select
pub type RtmpsW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:1 - Channel n Internal CAN Bus Communication Test Mode Enable
    #[inline(always)]
    pub fn icbce(&self) -> IcbceR {
        IcbceR::new((self.bits & 3) as u8)
    }
    ///Bits 16:25 - RAM Test Mode Page Select
    #[inline(always)]
    pub fn rtmps(&self) -> RtmpsR {
        RtmpsR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGTSTCFG")
            .field("icbce", &self.icbce())
            .field("rtmps", &self.rtmps())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Channel n Internal CAN Bus Communication Test Mode Enable
    #[inline(always)]
    pub fn icbce(&mut self) -> IcbceW<CfdgtstcfgSpec> {
        IcbceW::new(self, 0)
    }
    ///Bits 16:25 - RAM Test Mode Page Select
    #[inline(always)]
    pub fn rtmps(&mut self) -> RtmpsW<CfdgtstcfgSpec> {
        RtmpsW::new(self, 16)
    }
}
/**Global Test Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdgtstcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgtstcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgtstcfgSpec;
impl crate::RegisterSpec for CfdgtstcfgSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgtstcfg::R`](R) reader structure
impl crate::Readable for CfdgtstcfgSpec {}
///`write(|w| ..)` method takes [`cfdgtstcfg::W`](W) writer structure
impl crate::Writable for CfdgtstcfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGTSTCFG to value 0
impl crate::Resettable for CfdgtstcfgSpec {}
