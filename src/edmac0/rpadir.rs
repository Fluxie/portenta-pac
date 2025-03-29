///Register `RPADIR` reader
pub type R = crate::R<RpadirSpec>;
///Register `RPADIR` writer
pub type W = crate::W<RpadirSpec>;
///Field `PADR` reader - Padding Slot
pub type PadrR = crate::FieldReader;
///Field `PADR` writer - Padding Slot
pub type PadrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
/**Padding Size

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pads {
    ///0: Do not insert padding
    _00 = 0,
    ///1: settings prohibited
    Others = 1,
}
impl From<Pads> for u8 {
    #[inline(always)]
    fn from(variant: Pads) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pads {
    type Ux = u8;
}
impl crate::IsEnum for Pads {}
///Field `PADS` reader - Padding Size
pub type PadsR = crate::FieldReader<Pads>;
impl PadsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pads {
        match self.bits {
            0 => Pads::_00,
            _ => Pads::Others,
        }
    }
    ///Do not insert padding
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Pads::_00
    }
    ///settings prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Pads::Others)
    }
}
///Field `PADS` writer - Padding Size
pub type PadsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pads, crate::Safe>;
impl<'a, REG> PadsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Do not insert padding
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Pads::_00)
    }
    ///settings prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pads::Others)
    }
}
impl R {
    ///Bits 0:5 - Padding Slot
    #[inline(always)]
    pub fn padr(&self) -> PadrR {
        PadrR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 16:17 - Padding Size
    #[inline(always)]
    pub fn pads(&self) -> PadsR {
        PadsR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPADIR")
            .field("padr", &self.padr())
            .field("pads", &self.pads())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Padding Slot
    #[inline(always)]
    pub fn padr(&mut self) -> PadrW<RpadirSpec> {
        PadrW::new(self, 0)
    }
    ///Bits 16:17 - Padding Size
    #[inline(always)]
    pub fn pads(&mut self) -> PadsW<RpadirSpec> {
        PadsW::new(self, 16)
    }
}
/**Receive Data Padding Insert Register

You can [`read`](crate::Reg::read) this register and get [`rpadir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpadir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RpadirSpec;
impl crate::RegisterSpec for RpadirSpec {
    type Ux = u32;
}
///`read()` method returns [`rpadir::R`](R) reader structure
impl crate::Readable for RpadirSpec {}
///`write(|w| ..)` method takes [`rpadir::W`](W) writer structure
impl crate::Writable for RpadirSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RPADIR to value 0
impl crate::Resettable for RpadirSpec {}
