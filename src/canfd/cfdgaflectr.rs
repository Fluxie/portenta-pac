///Register `CFDGAFLECTR` reader
pub type R = crate::R<CfdgaflectrSpec>;
///Register `CFDGAFLECTR` writer
pub type W = crate::W<CfdgaflectrSpec>;
///Field `AFLPN` reader - Acceptance Filter List Page Number
pub type AflpnR = crate::FieldReader;
///Field `AFLPN` writer - Acceptance Filter List Page Number
pub type AflpnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Acceptance Filter List Data Access Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Afldae {
    ///0: Acceptance Filter List data access disabled
    _0 = 0,
    ///1: Acceptance Filter List data access enabled
    _1 = 1,
}
impl From<Afldae> for bool {
    #[inline(always)]
    fn from(variant: Afldae) -> Self {
        variant as u8 != 0
    }
}
///Field `AFLDAE` reader - Acceptance Filter List Data Access Enable
pub type AfldaeR = crate::BitReader<Afldae>;
impl AfldaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Afldae {
        match self.bits {
            false => Afldae::_0,
            true => Afldae::_1,
        }
    }
    ///Acceptance Filter List data access disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Afldae::_0
    }
    ///Acceptance Filter List data access enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Afldae::_1
    }
}
///Field `AFLDAE` writer - Acceptance Filter List Data Access Enable
pub type AfldaeW<'a, REG> = crate::BitWriter<'a, REG, Afldae>;
impl<'a, REG> AfldaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Acceptance Filter List data access disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Afldae::_0)
    }
    ///Acceptance Filter List data access enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Afldae::_1)
    }
}
impl R {
    ///Bits 0:3 - Acceptance Filter List Page Number
    #[inline(always)]
    pub fn aflpn(&self) -> AflpnR {
        AflpnR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Acceptance Filter List Data Access Enable
    #[inline(always)]
    pub fn afldae(&self) -> AfldaeR {
        AfldaeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGAFLECTR")
            .field("aflpn", &self.aflpn())
            .field("afldae", &self.afldae())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Acceptance Filter List Page Number
    #[inline(always)]
    pub fn aflpn(&mut self) -> AflpnW<CfdgaflectrSpec> {
        AflpnW::new(self, 0)
    }
    ///Bit 8 - Acceptance Filter List Data Access Enable
    #[inline(always)]
    pub fn afldae(&mut self) -> AfldaeW<CfdgaflectrSpec> {
        AfldaeW::new(self, 8)
    }
}
/**Global Acceptance Filter List Entry Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdgaflectr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflectr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgaflectrSpec;
impl crate::RegisterSpec for CfdgaflectrSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgaflectr::R`](R) reader structure
impl crate::Readable for CfdgaflectrSpec {}
///`write(|w| ..)` method takes [`cfdgaflectr::W`](W) writer structure
impl crate::Writable for CfdgaflectrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGAFLECTR to value 0
impl crate::Resettable for CfdgaflectrSpec {}
