///Register `PFENET` reader
pub type R = crate::R<PfenetSpec>;
///Register `PFENET` writer
pub type W = crate::W<PfenetSpec>;
/**Ethernet Mode Setting ch0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phymode0 {
    ///0: RMII mode (ETHERC channel 0)
    _0 = 0,
    ///1: MII mode (ETHERC channel 0)
    _1 = 1,
}
impl From<Phymode0> for bool {
    #[inline(always)]
    fn from(variant: Phymode0) -> Self {
        variant as u8 != 0
    }
}
///Field `PHYMODE0` reader - Ethernet Mode Setting ch0
pub type Phymode0R = crate::BitReader<Phymode0>;
impl Phymode0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Phymode0 {
        match self.bits {
            false => Phymode0::_0,
            true => Phymode0::_1,
        }
    }
    ///RMII mode (ETHERC channel 0)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Phymode0::_0
    }
    ///MII mode (ETHERC channel 0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Phymode0::_1
    }
}
///Field `PHYMODE0` writer - Ethernet Mode Setting ch0
pub type Phymode0W<'a, REG> = crate::BitWriter<'a, REG, Phymode0>;
impl<'a, REG> Phymode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RMII mode (ETHERC channel 0)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Phymode0::_0)
    }
    ///MII mode (ETHERC channel 0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Phymode0::_1)
    }
}
impl R {
    ///Bit 4 - Ethernet Mode Setting ch0
    #[inline(always)]
    pub fn phymode0(&self) -> Phymode0R {
        Phymode0R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PFENET").field("phymode0", &self.phymode0()).finish()
    }
}
impl W {
    ///Bit 4 - Ethernet Mode Setting ch0
    #[inline(always)]
    pub fn phymode0(&mut self) -> Phymode0W<PfenetSpec> {
        Phymode0W::new(self, 4)
    }
}
/**Ethernet Control Register

You can [`read`](crate::Reg::read) this register and get [`pfenet::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfenet::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PfenetSpec;
impl crate::RegisterSpec for PfenetSpec {
    type Ux = u8;
}
///`read()` method returns [`pfenet::R`](R) reader structure
impl crate::Readable for PfenetSpec {}
///`write(|w| ..)` method takes [`pfenet::W`](W) writer structure
impl crate::Writable for PfenetSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PFENET to value 0
impl crate::Resettable for PfenetSpec {}
