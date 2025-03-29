///Register `BUSSCNTS0BIU` reader
pub type R = crate::R<Busscnts0biuSpec>;
///Register `BUSSCNTS0BIU` writer
pub type W = crate::W<Busscnts0biuSpec>;
/**Arbitration Select for three masters

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Arbs {
    ///0: EDMAC > DMAC/DTC > CPU
    _00 = 0,
    ///1: Setting prohibited
    _01 = 1,
    ///2: (EDMAC ↔ DMAC/DTC) > CPU
    _10 = 2,
    ///3: (EDMAC ↔ DMAC/DTC) ↔ CPU
    _11 = 3,
}
impl From<Arbs> for u8 {
    #[inline(always)]
    fn from(variant: Arbs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Arbs {
    type Ux = u8;
}
impl crate::IsEnum for Arbs {}
///Field `ARBS` reader - Arbitration Select for three masters
pub type ArbsR = crate::FieldReader<Arbs>;
impl ArbsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Arbs {
        match self.bits {
            0 => Arbs::_00,
            1 => Arbs::_01,
            2 => Arbs::_10,
            3 => Arbs::_11,
            _ => unreachable!(),
        }
    }
    ///EDMAC > DMAC/DTC > CPU
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Arbs::_00
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Arbs::_01
    }
    ///(EDMAC ↔ DMAC/DTC) > CPU
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Arbs::_10
    }
    ///(EDMAC ↔ DMAC/DTC) ↔ CPU
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Arbs::_11
    }
}
///Field `ARBS` writer - Arbitration Select for three masters
pub type ArbsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Arbs, crate::Safe>;
impl<'a, REG> ArbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///EDMAC > DMAC/DTC > CPU
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Arbs::_00)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Arbs::_01)
    }
    ///(EDMAC ↔ DMAC/DTC) > CPU
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Arbs::_10)
    }
    ///(EDMAC ↔ DMAC/DTC) ↔ CPU
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Arbs::_11)
    }
}
impl R {
    ///Bits 0:1 - Arbitration Select for three masters
    #[inline(always)]
    pub fn arbs(&self) -> ArbsR {
        ArbsR::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSSCNTS0BIU").field("arbs", &self.arbs()).finish()
    }
}
impl W {
    ///Bits 0:1 - Arbitration Select for three masters
    #[inline(always)]
    pub fn arbs(&mut self) -> ArbsW<Busscnts0biuSpec> {
        ArbsW::new(self, 0)
    }
}
/**Slave Bus Control Register

You can [`read`](crate::Reg::read) this register and get [`busscnts0biu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscnts0biu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Busscnts0biuSpec;
impl crate::RegisterSpec for Busscnts0biuSpec {
    type Ux = u16;
}
///`read()` method returns [`busscnts0biu::R`](R) reader structure
impl crate::Readable for Busscnts0biuSpec {}
///`write(|w| ..)` method takes [`busscnts0biu::W`](W) writer structure
impl crate::Writable for Busscnts0biuSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSSCNTS0BIU to value 0
impl crate::Resettable for Busscnts0biuSpec {}
