///Register `MRWCR0` reader
pub type R = crate::R<Mrwcr0Spec>;
///Register `MRWCR0` writer
pub type W = crate::W<Mrwcr0Spec>;
///Field `D0MRCMD0` reader - Memory map read command 0 setting
pub type D0mrcmd0R = crate::FieldReader;
///Field `D0MRCMD0` writer - Memory map read command 0 setting
pub type D0mrcmd0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `D0MRCMD1` reader - Memory map read command 1 setting
pub type D0mrcmd1R = crate::FieldReader;
///Field `D0MRCMD1` writer - Memory map read command 1 setting
pub type D0mrcmd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `D0MWCMD0` reader - Memory map write command 0 setting
pub type D0mwcmd0R = crate::FieldReader;
///Field `D0MWCMD0` writer - Memory map write command 0 setting
pub type D0mwcmd0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `D0MWCMD1` reader - Memory map write command 1 setting
pub type D0mwcmd1R = crate::FieldReader;
///Field `D0MWCMD1` writer - Memory map write command 1 setting
pub type D0mwcmd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Memory map read command 0 setting
    #[inline(always)]
    pub fn d0mrcmd0(&self) -> D0mrcmd0R {
        D0mrcmd0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Memory map read command 1 setting
    #[inline(always)]
    pub fn d0mrcmd1(&self) -> D0mrcmd1R {
        D0mrcmd1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Memory map write command 0 setting
    #[inline(always)]
    pub fn d0mwcmd0(&self) -> D0mwcmd0R {
        D0mwcmd0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Memory map write command 1 setting
    #[inline(always)]
    pub fn d0mwcmd1(&self) -> D0mwcmd1R {
        D0mwcmd1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRWCR0")
            .field("d0mrcmd0", &self.d0mrcmd0())
            .field("d0mrcmd1", &self.d0mrcmd1())
            .field("d0mwcmd0", &self.d0mwcmd0())
            .field("d0mwcmd1", &self.d0mwcmd1())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Memory map read command 0 setting
    #[inline(always)]
    pub fn d0mrcmd0(&mut self) -> D0mrcmd0W<Mrwcr0Spec> {
        D0mrcmd0W::new(self, 0)
    }
    ///Bits 8:15 - Memory map read command 1 setting
    #[inline(always)]
    pub fn d0mrcmd1(&mut self) -> D0mrcmd1W<Mrwcr0Spec> {
        D0mrcmd1W::new(self, 8)
    }
    ///Bits 16:23 - Memory map write command 0 setting
    #[inline(always)]
    pub fn d0mwcmd0(&mut self) -> D0mwcmd0W<Mrwcr0Spec> {
        D0mwcmd0W::new(self, 16)
    }
    ///Bits 24:31 - Memory map write command 1 setting
    #[inline(always)]
    pub fn d0mwcmd1(&mut self) -> D0mwcmd1W<Mrwcr0Spec> {
        D0mwcmd1W::new(self, 24)
    }
}
/**Memory Map Read/Write Command Register 0

You can [`read`](crate::Reg::read) this register and get [`mrwcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrwcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Mrwcr0Spec;
impl crate::RegisterSpec for Mrwcr0Spec {
    type Ux = u32;
}
///`read()` method returns [`mrwcr0::R`](R) reader structure
impl crate::Readable for Mrwcr0Spec {}
///`write(|w| ..)` method takes [`mrwcr0::W`](W) writer structure
impl crate::Writable for Mrwcr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MRWCR0 to value 0
impl crate::Resettable for Mrwcr0Spec {}
