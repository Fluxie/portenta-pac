///Register `SYOCDCR` reader
pub type R = crate::R<SyocdcrSpec>;
///Register `SYOCDCR` writer
pub type W = crate::W<SyocdcrSpec>;
/**Deep Software Standby OCD flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Docdf {
    ///0: DBIRQ is not generated
    _0 = 0,
    ///1: DBIRQ is generated
    _1 = 1,
}
impl From<Docdf> for bool {
    #[inline(always)]
    fn from(variant: Docdf) -> Self {
        variant as u8 != 0
    }
}
///Field `DOCDF` reader - Deep Software Standby OCD flag
pub type DocdfR = crate::BitReader<Docdf>;
impl DocdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Docdf {
        match self.bits {
            false => Docdf::_0,
            true => Docdf::_1,
        }
    }
    ///DBIRQ is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Docdf::_0
    }
    ///DBIRQ is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Docdf::_1
    }
}
///Field `DOCDF` writer - Deep Software Standby OCD flag
pub type DocdfW<'a, REG> = crate::BitWriter<'a, REG, Docdf>;
impl<'a, REG> DocdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DBIRQ is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Docdf::_0)
    }
    ///DBIRQ is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Docdf::_1)
    }
}
/**Debugger Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgen {
    ///0: On-chip debugger is disabled
    _0 = 0,
    ///1: On-chip debugger is enabled
    _1 = 1,
}
impl From<Dbgen> for bool {
    #[inline(always)]
    fn from(variant: Dbgen) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGEN` reader - Debugger Enable bit
pub type DbgenR = crate::BitReader<Dbgen>;
impl DbgenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dbgen {
        match self.bits {
            false => Dbgen::_0,
            true => Dbgen::_1,
        }
    }
    ///On-chip debugger is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dbgen::_0
    }
    ///On-chip debugger is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dbgen::_1
    }
}
///Field `DBGEN` writer - Debugger Enable bit
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG, Dbgen>;
impl<'a, REG> DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///On-chip debugger is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::_0)
    }
    ///On-chip debugger is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::_1)
    }
}
impl R {
    ///Bit 0 - Deep Software Standby OCD flag
    #[inline(always)]
    pub fn docdf(&self) -> DocdfR {
        DocdfR::new((self.bits & 1) != 0)
    }
    ///Bit 7 - Debugger Enable bit
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYOCDCR")
            .field("docdf", &self.docdf())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Deep Software Standby OCD flag
    #[inline(always)]
    pub fn docdf(&mut self) -> DocdfW<SyocdcrSpec> {
        DocdfW::new(self, 0)
    }
    ///Bit 7 - Debugger Enable bit
    #[inline(always)]
    pub fn dbgen(&mut self) -> DbgenW<SyocdcrSpec> {
        DbgenW::new(self, 7)
    }
}
/**System Control OCD Control Register

You can [`read`](crate::Reg::read) this register and get [`syocdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syocdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SyocdcrSpec;
impl crate::RegisterSpec for SyocdcrSpec {
    type Ux = u8;
}
///`read()` method returns [`syocdcr::R`](R) reader structure
impl crate::Readable for SyocdcrSpec {}
///`write(|w| ..)` method takes [`syocdcr::W`](W) writer structure
impl crate::Writable for SyocdcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYOCDCR to value 0
impl crate::Resettable for SyocdcrSpec {}
