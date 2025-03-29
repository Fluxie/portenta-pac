///Register `CFDRMNB` reader
pub type R = crate::R<CfdrmnbSpec>;
///Register `CFDRMNB` writer
pub type W = crate::W<CfdrmnbSpec>;
///Field `NRXMB` reader - Number of RX Message Buffers
pub type NrxmbR = crate::FieldReader;
///Field `NRXMB` writer - Number of RX Message Buffers
pub type NrxmbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**Reception Message Buffer Payload Data Size

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rmpls {
    ///0: 8 bytes
    _000 = 0,
    ///1: 12 bytes
    _001 = 1,
    ///2: 16 bytes
    _010 = 2,
    ///3: 20 bytes
    _011 = 3,
    ///4: 24 bytes
    _100 = 4,
    ///5: 32 bytes
    _101 = 5,
    ///6: 48 bytes
    _110 = 6,
    ///7: 64 bytes
    _111 = 7,
}
impl From<Rmpls> for u8 {
    #[inline(always)]
    fn from(variant: Rmpls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rmpls {
    type Ux = u8;
}
impl crate::IsEnum for Rmpls {}
///Field `RMPLS` reader - Reception Message Buffer Payload Data Size
pub type RmplsR = crate::FieldReader<Rmpls>;
impl RmplsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rmpls {
        match self.bits {
            0 => Rmpls::_000,
            1 => Rmpls::_001,
            2 => Rmpls::_010,
            3 => Rmpls::_011,
            4 => Rmpls::_100,
            5 => Rmpls::_101,
            6 => Rmpls::_110,
            7 => Rmpls::_111,
            _ => unreachable!(),
        }
    }
    ///8 bytes
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Rmpls::_000
    }
    ///12 bytes
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Rmpls::_001
    }
    ///16 bytes
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Rmpls::_010
    }
    ///20 bytes
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Rmpls::_011
    }
    ///24 bytes
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Rmpls::_100
    }
    ///32 bytes
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Rmpls::_101
    }
    ///48 bytes
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Rmpls::_110
    }
    ///64 bytes
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Rmpls::_111
    }
}
///Field `RMPLS` writer - Reception Message Buffer Payload Data Size
pub type RmplsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rmpls, crate::Safe>;
impl<'a, REG> RmplsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bytes
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Rmpls::_000)
    }
    ///12 bytes
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Rmpls::_001)
    }
    ///16 bytes
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Rmpls::_010)
    }
    ///20 bytes
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Rmpls::_011)
    }
    ///24 bytes
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Rmpls::_100)
    }
    ///32 bytes
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Rmpls::_101)
    }
    ///48 bytes
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Rmpls::_110)
    }
    ///64 bytes
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Rmpls::_111)
    }
}
impl R {
    ///Bits 0:7 - Number of RX Message Buffers
    #[inline(always)]
    pub fn nrxmb(&self) -> NrxmbR {
        NrxmbR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - Reception Message Buffer Payload Data Size
    #[inline(always)]
    pub fn rmpls(&self) -> RmplsR {
        RmplsR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRMNB")
            .field("nrxmb", &self.nrxmb())
            .field("rmpls", &self.rmpls())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Number of RX Message Buffers
    #[inline(always)]
    pub fn nrxmb(&mut self) -> NrxmbW<CfdrmnbSpec> {
        NrxmbW::new(self, 0)
    }
    ///Bits 8:10 - Reception Message Buffer Payload Data Size
    #[inline(always)]
    pub fn rmpls(&mut self) -> RmplsW<CfdrmnbSpec> {
        RmplsW::new(self, 8)
    }
}
/**RX Message Buffer Number Register

You can [`read`](crate::Reg::read) this register and get [`cfdrmnb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrmnb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdrmnbSpec;
impl crate::RegisterSpec for CfdrmnbSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdrmnb::R`](R) reader structure
impl crate::Readable for CfdrmnbSpec {}
///`write(|w| ..)` method takes [`cfdrmnb::W`](W) writer structure
impl crate::Writable for CfdrmnbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDRMNB to value 0
impl crate::Resettable for CfdrmnbSpec {}
