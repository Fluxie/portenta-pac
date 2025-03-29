///Register `CFDRMND0` reader
pub type R = crate::R<Cfdrmnd0Spec>;
///Register `CFDRMND0` writer
pub type W = crate::W<Cfdrmnd0Spec>;
/**RX Message Buffer New Data Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Rmns {
    ///0: New data not stored in corresponding RX message buffer
    _0 = 0,
    ///1: New data stored in corresponding RX message buffer
    _1 = 1,
}
impl From<Rmns> for u32 {
    #[inline(always)]
    fn from(variant: Rmns) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rmns {
    type Ux = u32;
}
impl crate::IsEnum for Rmns {}
///Field `RMNS` reader - RX Message Buffer New Data Status
pub type RmnsR = crate::FieldReader<Rmns>;
impl RmnsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rmns> {
        match self.bits {
            0 => Some(Rmns::_0),
            1 => Some(Rmns::_1),
            _ => None,
        }
    }
    ///New data not stored in corresponding RX message buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rmns::_0
    }
    ///New data stored in corresponding RX message buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rmns::_1
    }
}
///Field `RMNS` writer - RX Message Buffer New Data Status
pub type RmnsW<'a, REG> = crate::FieldWriter<'a, REG, 32, Rmns>;
impl<'a, REG> RmnsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///New data not stored in corresponding RX message buffer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rmns::_0)
    }
    ///New data stored in corresponding RX message buffer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmns::_1)
    }
}
impl R {
    ///Bits 0:31 - RX Message Buffer New Data Status
    #[inline(always)]
    pub fn rmns(&self) -> RmnsR {
        RmnsR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRMND0").field("rmns", &self.rmns()).finish()
    }
}
impl W {
    ///Bits 0:31 - RX Message Buffer New Data Status
    #[inline(always)]
    pub fn rmns(&mut self) -> RmnsW<Cfdrmnd0Spec> {
        RmnsW::new(self, 0)
    }
}
/**RX Message Buffer New Data Register 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmnd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrmnd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdrmnd0Spec;
impl crate::RegisterSpec for Cfdrmnd0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdrmnd0::R`](R) reader structure
impl crate::Readable for Cfdrmnd0Spec {}
///`write(|w| ..)` method takes [`cfdrmnd0::W`](W) writer structure
impl crate::Writable for Cfdrmnd0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDRMND0 to value 0
impl crate::Resettable for Cfdrmnd0Spec {}
