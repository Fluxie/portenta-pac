///Register `DMBWR` reader
pub type R = crate::R<DmbwrSpec>;
///Register `DMBWR` writer
pub type W = crate::W<DmbwrSpec>;
/**Bufferable Write Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwe {
    ///0: Disables Bufferable Write
    _0 = 0,
    ///1: Enables Bufferable Write
    _1 = 1,
}
impl From<Bwe> for bool {
    #[inline(always)]
    fn from(variant: Bwe) -> Self {
        variant as u8 != 0
    }
}
///Field `BWE` reader - Bufferable Write Enable
pub type BweR = crate::BitReader<Bwe>;
impl BweR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bwe {
        match self.bits {
            false => Bwe::_0,
            true => Bwe::_1,
        }
    }
    ///Disables Bufferable Write
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bwe::_0
    }
    ///Enables Bufferable Write
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bwe::_1
    }
}
///Field `BWE` writer - Bufferable Write Enable
pub type BweW<'a, REG> = crate::BitWriter<'a, REG, Bwe>;
impl<'a, REG> BweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables Bufferable Write
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bwe::_0)
    }
    ///Enables Bufferable Write
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwe::_1)
    }
}
impl R {
    ///Bit 0 - Bufferable Write Enable
    #[inline(always)]
    pub fn bwe(&self) -> BweR {
        BweR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMBWR").field("bwe", &self.bwe()).finish()
    }
}
impl W {
    ///Bit 0 - Bufferable Write Enable
    #[inline(always)]
    pub fn bwe(&mut self) -> BweW<DmbwrSpec> {
        BweW::new(self, 0)
    }
}
/**DMA Bufferable Write Enable Register

You can [`read`](crate::Reg::read) this register and get [`dmbwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmbwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmbwrSpec;
impl crate::RegisterSpec for DmbwrSpec {
    type Ux = u8;
}
///`read()` method returns [`dmbwr::R`](R) reader structure
impl crate::Readable for DmbwrSpec {}
///`write(|w| ..)` method takes [`dmbwr::W`](W) writer structure
impl crate::Writable for DmbwrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMBWR to value 0
impl crate::Resettable for DmbwrSpec {}
