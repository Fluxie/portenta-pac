///Register `BTZF%sERRRW` reader
pub type R = crate::R<BtzferrrwSpec>;
///Register `BTZF%sERRRW` writer
pub type W = crate::W<BtzferrrwSpec>;
/**TrustZone filter error access Read/Write Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trwstat {
    ///0: Read access
    _0 = 0,
    ///1: Write access
    _1 = 1,
}
impl From<Trwstat> for bool {
    #[inline(always)]
    fn from(variant: Trwstat) -> Self {
        variant as u8 != 0
    }
}
///Field `TRWSTAT` reader - TrustZone filter error access Read/Write Status
pub type TrwstatR = crate::BitReader<Trwstat>;
impl TrwstatR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Trwstat {
        match self.bits {
            false => Trwstat::_0,
            true => Trwstat::_1,
        }
    }
    ///Read access
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trwstat::_0
    }
    ///Write access
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trwstat::_1
    }
}
impl R {
    ///Bit 0 - TrustZone filter error access Read/Write Status
    #[inline(always)]
    pub fn trwstat(&self) -> TrwstatR {
        TrwstatR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTZFERRRW").field("trwstat", &self.trwstat()).finish()
    }
}
impl W {}
/**BUS TZF Error Read Write Register

You can [`read`](crate::Reg::read) this register and get [`btzferrrw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btzferrrw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BtzferrrwSpec;
impl crate::RegisterSpec for BtzferrrwSpec {
    type Ux = u8;
}
///`read()` method returns [`btzferrrw::R`](R) reader structure
impl crate::Readable for BtzferrrwSpec {}
///`write(|w| ..)` method takes [`btzferrrw::W`](W) writer structure
impl crate::Writable for BtzferrrwSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BTZF%sERRRW to value 0
impl crate::Resettable for BtzferrrwSpec {}
