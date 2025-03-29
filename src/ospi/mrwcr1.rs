///Register `MRWCR1` reader
pub type R = crate::R<Mrwcr1Spec>;
///Register `MRWCR1` writer
pub type W = crate::W<Mrwcr1Spec>;
///Field `D1MRCMD0` reader - Memory map read command 0 setting
pub type D1mrcmd0R = crate::FieldReader;
///Field `D1MRCMD0` writer - Memory map read command 0 setting
pub type D1mrcmd0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `D1MRCMD1` reader - Memory map read command 1 setting
pub type D1mrcmd1R = crate::FieldReader;
///Field `D1MRCMD1` writer - Memory map read command 1 setting
pub type D1mrcmd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `D1MWCMD0` reader - Memory map write command 0 setting
pub type D1mwcmd0R = crate::FieldReader;
///Field `D1MWCMD0` writer - Memory map write command 0 setting
pub type D1mwcmd0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `D1MWCMD1` reader - Memory map write command 1 setting
pub type D1mwcmd1R = crate::FieldReader;
///Field `D1MWCMD1` writer - Memory map write command 1 setting
pub type D1mwcmd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Memory map read command 0 setting
    #[inline(always)]
    pub fn d1mrcmd0(&self) -> D1mrcmd0R {
        D1mrcmd0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Memory map read command 1 setting
    #[inline(always)]
    pub fn d1mrcmd1(&self) -> D1mrcmd1R {
        D1mrcmd1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Memory map write command 0 setting
    #[inline(always)]
    pub fn d1mwcmd0(&self) -> D1mwcmd0R {
        D1mwcmd0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Memory map write command 1 setting
    #[inline(always)]
    pub fn d1mwcmd1(&self) -> D1mwcmd1R {
        D1mwcmd1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRWCR1")
            .field("d1mrcmd0", &self.d1mrcmd0())
            .field("d1mrcmd1", &self.d1mrcmd1())
            .field("d1mwcmd0", &self.d1mwcmd0())
            .field("d1mwcmd1", &self.d1mwcmd1())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Memory map read command 0 setting
    #[inline(always)]
    pub fn d1mrcmd0(&mut self) -> D1mrcmd0W<Mrwcr1Spec> {
        D1mrcmd0W::new(self, 0)
    }
    ///Bits 8:15 - Memory map read command 1 setting
    #[inline(always)]
    pub fn d1mrcmd1(&mut self) -> D1mrcmd1W<Mrwcr1Spec> {
        D1mrcmd1W::new(self, 8)
    }
    ///Bits 16:23 - Memory map write command 0 setting
    #[inline(always)]
    pub fn d1mwcmd0(&mut self) -> D1mwcmd0W<Mrwcr1Spec> {
        D1mwcmd0W::new(self, 16)
    }
    ///Bits 24:31 - Memory map write command 1 setting
    #[inline(always)]
    pub fn d1mwcmd1(&mut self) -> D1mwcmd1W<Mrwcr1Spec> {
        D1mwcmd1W::new(self, 24)
    }
}
/**Memory Map Read/Write Command Register 1

You can [`read`](crate::Reg::read) this register and get [`mrwcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrwcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Mrwcr1Spec;
impl crate::RegisterSpec for Mrwcr1Spec {
    type Ux = u32;
}
///`read()` method returns [`mrwcr1::R`](R) reader structure
impl crate::Readable for Mrwcr1Spec {}
///`write(|w| ..)` method takes [`mrwcr1::W`](W) writer structure
impl crate::Writable for Mrwcr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MRWCR1 to value 0
impl crate::Resettable for Mrwcr1Spec {}
