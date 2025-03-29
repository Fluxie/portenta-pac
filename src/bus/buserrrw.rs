///Register `BUS%sERRRW` reader
pub type R = crate::R<BuserrrwSpec>;
///Register `BUS%sERRRW` writer
pub type W = crate::W<BuserrrwSpec>;
/**Error Access Read/Write Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwstat {
    ///0: Read access
    _0 = 0,
    ///1: Write access
    _1 = 1,
}
impl From<Rwstat> for bool {
    #[inline(always)]
    fn from(variant: Rwstat) -> Self {
        variant as u8 != 0
    }
}
///Field `RWSTAT` reader - Error Access Read/Write Status
pub type RwstatR = crate::BitReader<Rwstat>;
impl RwstatR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rwstat {
        match self.bits {
            false => Rwstat::_0,
            true => Rwstat::_1,
        }
    }
    ///Read access
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rwstat::_0
    }
    ///Write access
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rwstat::_1
    }
}
impl R {
    ///Bit 0 - Error Access Read/Write Status
    #[inline(always)]
    pub fn rwstat(&self) -> RwstatR {
        RwstatR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSERRRW").field("rwstat", &self.rwstat()).finish()
    }
}
impl W {}
/**BUS Error Read Write Register

You can [`read`](crate::Reg::read) this register and get [`buserrrw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserrrw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BuserrrwSpec;
impl crate::RegisterSpec for BuserrrwSpec {
    type Ux = u8;
}
///`read()` method returns [`buserrrw::R`](R) reader structure
impl crate::Readable for BuserrrwSpec {}
///`write(|w| ..)` method takes [`buserrrw::W`](W) writer structure
impl crate::Writable for BuserrrwSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUS%sERRRW to value 0
impl crate::Resettable for BuserrrwSpec {}
