///Register `MRWCSR` reader
pub type R = crate::R<MrwcsrSpec>;
///Register `MRWCSR` writer
pub type W = crate::W<MrwcsrSpec>;
///Field `MRAL0` reader - Device 0 read address length setting
pub type Mral0R = crate::FieldReader;
///Field `MRAL0` writer - Device 0 read address length setting
pub type Mral0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MRCL0` reader - Device 0 read command length setting
pub type Mrcl0R = crate::FieldReader;
///Field `MRCL0` writer - Device 0 read command length setting
pub type Mrcl0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Device 0 read order setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mro0 {
    ///0: Read order is byte0, byte1, byte2, byte3.
    _0 = 0,
    ///1: Read order is byte1, byte0, byte3, byte2.
    _1 = 1,
}
impl From<Mro0> for bool {
    #[inline(always)]
    fn from(variant: Mro0) -> Self {
        variant as u8 != 0
    }
}
///Field `MRO0` reader - Device 0 read order setting
pub type Mro0R = crate::BitReader<Mro0>;
impl Mro0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mro0 {
        match self.bits {
            false => Mro0::_0,
            true => Mro0::_1,
        }
    }
    ///Read order is byte0, byte1, byte2, byte3.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mro0::_0
    }
    ///Read order is byte1, byte0, byte3, byte2.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mro0::_1
    }
}
///Field `MRO0` writer - Device 0 read order setting
pub type Mro0W<'a, REG> = crate::BitWriter<'a, REG, Mro0>;
impl<'a, REG> Mro0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read order is byte0, byte1, byte2, byte3.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mro0::_0)
    }
    ///Read order is byte1, byte0, byte3, byte2.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mro0::_1)
    }
}
/**Preamble bit enable for mem0 memory-map read

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pren0 {
    ///0: No check preamble bit
    _0 = 0,
    ///1: Check preamble bit from OctaFlash (if OctaFlash is connected to device 0)
    _1 = 1,
}
impl From<Pren0> for bool {
    #[inline(always)]
    fn from(variant: Pren0) -> Self {
        variant as u8 != 0
    }
}
///Field `PREN0` reader - Preamble bit enable for mem0 memory-map read
pub type Pren0R = crate::BitReader<Pren0>;
impl Pren0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pren0 {
        match self.bits {
            false => Pren0::_0,
            true => Pren0::_1,
        }
    }
    ///No check preamble bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pren0::_0
    }
    ///Check preamble bit from OctaFlash (if OctaFlash is connected to device 0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pren0::_1
    }
}
///Field `PREN0` writer - Preamble bit enable for mem0 memory-map read
pub type Pren0W<'a, REG> = crate::BitWriter<'a, REG, Pren0>;
impl<'a, REG> Pren0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No check preamble bit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pren0::_0)
    }
    ///Check preamble bit from OctaFlash (if OctaFlash is connected to device 0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pren0::_1)
    }
}
///Field `MWAL0` reader - Device 0 write address length setting
pub type Mwal0R = crate::FieldReader;
///Field `MWAL0` writer - Device 0 write address length setting
pub type Mwal0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MWCL0` reader - Device 0 write command length setting
pub type Mwcl0R = crate::FieldReader;
///Field `MWCL0` writer - Device 0 write command length setting
pub type Mwcl0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Device 0 write order setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mwo0 {
    ///0: Write order is byte0, byte1, byte2, byte3.
    _0 = 0,
    ///1: Write order is byte1, byte0, byte3, byte2.
    _1 = 1,
}
impl From<Mwo0> for bool {
    #[inline(always)]
    fn from(variant: Mwo0) -> Self {
        variant as u8 != 0
    }
}
///Field `MWO0` reader - Device 0 write order setting
pub type Mwo0R = crate::BitReader<Mwo0>;
impl Mwo0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mwo0 {
        match self.bits {
            false => Mwo0::_0,
            true => Mwo0::_1,
        }
    }
    ///Write order is byte0, byte1, byte2, byte3.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mwo0::_0
    }
    ///Write order is byte1, byte0, byte3, byte2.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mwo0::_1
    }
}
///Field `MWO0` writer - Device 0 write order setting
pub type Mwo0W<'a, REG> = crate::BitWriter<'a, REG, Mwo0>;
impl<'a, REG> Mwo0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write order is byte0, byte1, byte2, byte3.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mwo0::_0)
    }
    ///Write order is byte1, byte0, byte3, byte2.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mwo0::_1)
    }
}
///Field `MRAL1` reader - Device 1 read address length setting
pub type Mral1R = crate::FieldReader;
///Field `MRAL1` writer - Device 1 read address length setting
pub type Mral1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MRCL1` reader - Device 1 read command length setting
pub type Mrcl1R = crate::FieldReader;
///Field `MRCL1` writer - Device 1 read command length setting
pub type Mrcl1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Device 1 read order setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mro1 {
    ///0: Read order is byte0, byte1, byte2, byte3.
    _0 = 0,
    ///1: Read order is byte1, byte0, byte3, byte2.
    _1 = 1,
}
impl From<Mro1> for bool {
    #[inline(always)]
    fn from(variant: Mro1) -> Self {
        variant as u8 != 0
    }
}
///Field `MRO1` reader - Device 1 read order setting
pub type Mro1R = crate::BitReader<Mro1>;
impl Mro1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mro1 {
        match self.bits {
            false => Mro1::_0,
            true => Mro1::_1,
        }
    }
    ///Read order is byte0, byte1, byte2, byte3.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mro1::_0
    }
    ///Read order is byte1, byte0, byte3, byte2.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mro1::_1
    }
}
///Field `MRO1` writer - Device 1 read order setting
pub type Mro1W<'a, REG> = crate::BitWriter<'a, REG, Mro1>;
impl<'a, REG> Mro1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read order is byte0, byte1, byte2, byte3.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mro1::_0)
    }
    ///Read order is byte1, byte0, byte3, byte2.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mro1::_1)
    }
}
/**Preamble bit enable for mem1 memory-map read

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pren1 {
    ///0: No check preamble bit
    _0 = 0,
    ///1: Check preamble bit from OctaFlash (if OctaFlash is connected to device 1)
    _1 = 1,
}
impl From<Pren1> for bool {
    #[inline(always)]
    fn from(variant: Pren1) -> Self {
        variant as u8 != 0
    }
}
///Field `PREN1` reader - Preamble bit enable for mem1 memory-map read
pub type Pren1R = crate::BitReader<Pren1>;
impl Pren1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pren1 {
        match self.bits {
            false => Pren1::_0,
            true => Pren1::_1,
        }
    }
    ///No check preamble bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pren1::_0
    }
    ///Check preamble bit from OctaFlash (if OctaFlash is connected to device 1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pren1::_1
    }
}
///Field `PREN1` writer - Preamble bit enable for mem1 memory-map read
pub type Pren1W<'a, REG> = crate::BitWriter<'a, REG, Pren1>;
impl<'a, REG> Pren1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No check preamble bit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pren1::_0)
    }
    ///Check preamble bit from OctaFlash (if OctaFlash is connected to device 1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pren1::_1)
    }
}
///Field `MWAL1` reader - Device 1 write address length setting
pub type Mwal1R = crate::FieldReader;
///Field `MWAL1` writer - Device 1 write address length setting
pub type Mwal1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MWCL1` reader - Device 1 write command length setting
pub type Mwcl1R = crate::FieldReader;
///Field `MWCL1` writer - Device 1 write command length setting
pub type Mwcl1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Device 1 write order setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mwo1 {
    ///0: Write order is byte0, byte1, byte2, byte3.
    _0 = 0,
    ///1: Write order is byte1, byte0, byte3, byte2.
    _1 = 1,
}
impl From<Mwo1> for bool {
    #[inline(always)]
    fn from(variant: Mwo1) -> Self {
        variant as u8 != 0
    }
}
///Field `MWO1` reader - Device 1 write order setting
pub type Mwo1R = crate::BitReader<Mwo1>;
impl Mwo1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mwo1 {
        match self.bits {
            false => Mwo1::_0,
            true => Mwo1::_1,
        }
    }
    ///Write order is byte0, byte1, byte2, byte3.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mwo1::_0
    }
    ///Write order is byte1, byte0, byte3, byte2.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mwo1::_1
    }
}
///Field `MWO1` writer - Device 1 write order setting
pub type Mwo1W<'a, REG> = crate::BitWriter<'a, REG, Mwo1>;
impl<'a, REG> Mwo1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write order is byte0, byte1, byte2, byte3.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mwo1::_0)
    }
    ///Write order is byte1, byte0, byte3, byte2.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mwo1::_1)
    }
}
impl R {
    ///Bits 0:2 - Device 0 read address length setting
    #[inline(always)]
    pub fn mral0(&self) -> Mral0R {
        Mral0R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Device 0 read command length setting
    #[inline(always)]
    pub fn mrcl0(&self) -> Mrcl0R {
        Mrcl0R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - Device 0 read order setting
    #[inline(always)]
    pub fn mro0(&self) -> Mro0R {
        Mro0R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Preamble bit enable for mem0 memory-map read
    #[inline(always)]
    pub fn pren0(&self) -> Pren0R {
        Pren0R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Device 0 write address length setting
    #[inline(always)]
    pub fn mwal0(&self) -> Mwal0R {
        Mwal0R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - Device 0 write command length setting
    #[inline(always)]
    pub fn mwcl0(&self) -> Mwcl0R {
        Mwcl0R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bit 14 - Device 0 write order setting
    #[inline(always)]
    pub fn mwo0(&self) -> Mwo0R {
        Mwo0R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:18 - Device 1 read address length setting
    #[inline(always)]
    pub fn mral1(&self) -> Mral1R {
        Mral1R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - Device 1 read command length setting
    #[inline(always)]
    pub fn mrcl1(&self) -> Mrcl1R {
        Mrcl1R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bit 22 - Device 1 read order setting
    #[inline(always)]
    pub fn mro1(&self) -> Mro1R {
        Mro1R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Preamble bit enable for mem1 memory-map read
    #[inline(always)]
    pub fn pren1(&self) -> Pren1R {
        Pren1R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - Device 1 write address length setting
    #[inline(always)]
    pub fn mwal1(&self) -> Mwal1R {
        Mwal1R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - Device 1 write command length setting
    #[inline(always)]
    pub fn mwcl1(&self) -> Mwcl1R {
        Mwcl1R::new(((self.bits >> 27) & 7) as u8)
    }
    ///Bit 30 - Device 1 write order setting
    #[inline(always)]
    pub fn mwo1(&self) -> Mwo1R {
        Mwo1R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRWCSR")
            .field("mral0", &self.mral0())
            .field("mrcl0", &self.mrcl0())
            .field("mro0", &self.mro0())
            .field("pren0", &self.pren0())
            .field("mwal0", &self.mwal0())
            .field("mwcl0", &self.mwcl0())
            .field("mwo0", &self.mwo0())
            .field("mral1", &self.mral1())
            .field("mrcl1", &self.mrcl1())
            .field("mro1", &self.mro1())
            .field("pren1", &self.pren1())
            .field("mwal1", &self.mwal1())
            .field("mwcl1", &self.mwcl1())
            .field("mwo1", &self.mwo1())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Device 0 read address length setting
    #[inline(always)]
    pub fn mral0(&mut self) -> Mral0W<MrwcsrSpec> {
        Mral0W::new(self, 0)
    }
    ///Bits 3:5 - Device 0 read command length setting
    #[inline(always)]
    pub fn mrcl0(&mut self) -> Mrcl0W<MrwcsrSpec> {
        Mrcl0W::new(self, 3)
    }
    ///Bit 6 - Device 0 read order setting
    #[inline(always)]
    pub fn mro0(&mut self) -> Mro0W<MrwcsrSpec> {
        Mro0W::new(self, 6)
    }
    ///Bit 7 - Preamble bit enable for mem0 memory-map read
    #[inline(always)]
    pub fn pren0(&mut self) -> Pren0W<MrwcsrSpec> {
        Pren0W::new(self, 7)
    }
    ///Bits 8:10 - Device 0 write address length setting
    #[inline(always)]
    pub fn mwal0(&mut self) -> Mwal0W<MrwcsrSpec> {
        Mwal0W::new(self, 8)
    }
    ///Bits 11:13 - Device 0 write command length setting
    #[inline(always)]
    pub fn mwcl0(&mut self) -> Mwcl0W<MrwcsrSpec> {
        Mwcl0W::new(self, 11)
    }
    ///Bit 14 - Device 0 write order setting
    #[inline(always)]
    pub fn mwo0(&mut self) -> Mwo0W<MrwcsrSpec> {
        Mwo0W::new(self, 14)
    }
    ///Bits 16:18 - Device 1 read address length setting
    #[inline(always)]
    pub fn mral1(&mut self) -> Mral1W<MrwcsrSpec> {
        Mral1W::new(self, 16)
    }
    ///Bits 19:21 - Device 1 read command length setting
    #[inline(always)]
    pub fn mrcl1(&mut self) -> Mrcl1W<MrwcsrSpec> {
        Mrcl1W::new(self, 19)
    }
    ///Bit 22 - Device 1 read order setting
    #[inline(always)]
    pub fn mro1(&mut self) -> Mro1W<MrwcsrSpec> {
        Mro1W::new(self, 22)
    }
    ///Bit 23 - Preamble bit enable for mem1 memory-map read
    #[inline(always)]
    pub fn pren1(&mut self) -> Pren1W<MrwcsrSpec> {
        Pren1W::new(self, 23)
    }
    ///Bits 24:26 - Device 1 write address length setting
    #[inline(always)]
    pub fn mwal1(&mut self) -> Mwal1W<MrwcsrSpec> {
        Mwal1W::new(self, 24)
    }
    ///Bits 27:29 - Device 1 write command length setting
    #[inline(always)]
    pub fn mwcl1(&mut self) -> Mwcl1W<MrwcsrSpec> {
        Mwcl1W::new(self, 27)
    }
    ///Bit 30 - Device 1 write order setting
    #[inline(always)]
    pub fn mwo1(&mut self) -> Mwo1W<MrwcsrSpec> {
        Mwo1W::new(self, 30)
    }
}
/**Memory Map Read/Write Setting Register

You can [`read`](crate::Reg::read) this register and get [`mrwcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrwcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MrwcsrSpec;
impl crate::RegisterSpec for MrwcsrSpec {
    type Ux = u32;
}
///`read()` method returns [`mrwcsr::R`](R) reader structure
impl crate::Readable for MrwcsrSpec {}
///`write(|w| ..)` method takes [`mrwcsr::W`](W) writer structure
impl crate::Writable for MrwcsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MRWCSR to value 0
impl crate::Resettable for MrwcsrSpec {}
