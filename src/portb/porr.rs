///Register `PORR` writer
pub type W = crate::W<PorrSpec>;
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr00 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr00> for bool {
    #[inline(always)]
    fn from(variant: Porr00) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR00` writer - Pmn Output Reset
pub type Porr00W<'a, REG> = crate::BitWriter<'a, REG, Porr00>;
impl<'a, REG> Porr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr00::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr00::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr01 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr01> for bool {
    #[inline(always)]
    fn from(variant: Porr01) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR01` writer - Pmn Output Reset
pub type Porr01W<'a, REG> = crate::BitWriter<'a, REG, Porr01>;
impl<'a, REG> Porr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr01::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr01::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr02 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr02> for bool {
    #[inline(always)]
    fn from(variant: Porr02) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR02` writer - Pmn Output Reset
pub type Porr02W<'a, REG> = crate::BitWriter<'a, REG, Porr02>;
impl<'a, REG> Porr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr02::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr02::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr03 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr03> for bool {
    #[inline(always)]
    fn from(variant: Porr03) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR03` writer - Pmn Output Reset
pub type Porr03W<'a, REG> = crate::BitWriter<'a, REG, Porr03>;
impl<'a, REG> Porr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr03::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr03::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr04 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr04> for bool {
    #[inline(always)]
    fn from(variant: Porr04) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR04` writer - Pmn Output Reset
pub type Porr04W<'a, REG> = crate::BitWriter<'a, REG, Porr04>;
impl<'a, REG> Porr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr04::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr04::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr05 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr05> for bool {
    #[inline(always)]
    fn from(variant: Porr05) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR05` writer - Pmn Output Reset
pub type Porr05W<'a, REG> = crate::BitWriter<'a, REG, Porr05>;
impl<'a, REG> Porr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr05::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr05::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr06 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr06> for bool {
    #[inline(always)]
    fn from(variant: Porr06) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR06` writer - Pmn Output Reset
pub type Porr06W<'a, REG> = crate::BitWriter<'a, REG, Porr06>;
impl<'a, REG> Porr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr06::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr06::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr07 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr07> for bool {
    #[inline(always)]
    fn from(variant: Porr07) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR07` writer - Pmn Output Reset
pub type Porr07W<'a, REG> = crate::BitWriter<'a, REG, Porr07>;
impl<'a, REG> Porr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr07::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr07::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr08 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr08> for bool {
    #[inline(always)]
    fn from(variant: Porr08) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR08` writer - Pmn Output Reset
pub type Porr08W<'a, REG> = crate::BitWriter<'a, REG, Porr08>;
impl<'a, REG> Porr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr08::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr08::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr09 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr09> for bool {
    #[inline(always)]
    fn from(variant: Porr09) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR09` writer - Pmn Output Reset
pub type Porr09W<'a, REG> = crate::BitWriter<'a, REG, Porr09>;
impl<'a, REG> Porr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr09::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr09::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr10 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr10> for bool {
    #[inline(always)]
    fn from(variant: Porr10) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR10` writer - Pmn Output Reset
pub type Porr10W<'a, REG> = crate::BitWriter<'a, REG, Porr10>;
impl<'a, REG> Porr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr10::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr10::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr11 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr11> for bool {
    #[inline(always)]
    fn from(variant: Porr11) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR11` writer - Pmn Output Reset
pub type Porr11W<'a, REG> = crate::BitWriter<'a, REG, Porr11>;
impl<'a, REG> Porr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr11::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr11::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr12 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr12> for bool {
    #[inline(always)]
    fn from(variant: Porr12) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR12` writer - Pmn Output Reset
pub type Porr12W<'a, REG> = crate::BitWriter<'a, REG, Porr12>;
impl<'a, REG> Porr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr12::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr12::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr13 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr13> for bool {
    #[inline(always)]
    fn from(variant: Porr13) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR13` writer - Pmn Output Reset
pub type Porr13W<'a, REG> = crate::BitWriter<'a, REG, Porr13>;
impl<'a, REG> Porr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr13::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr13::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr14 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr14> for bool {
    #[inline(always)]
    fn from(variant: Porr14) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR14` writer - Pmn Output Reset
pub type Porr14W<'a, REG> = crate::BitWriter<'a, REG, Porr14>;
impl<'a, REG> Porr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr14::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr14::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr15 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Porr15> for bool {
    #[inline(always)]
    fn from(variant: Porr15) -> Self {
        variant as u8 != 0
    }
}
///Field `PORR15` writer - Pmn Output Reset
pub type Porr15W<'a, REG> = crate::BitWriter<'a, REG, Porr15>;
impl<'a, REG> Porr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr15::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr15::_1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<PorrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Pmn Output Reset
    #[inline(always)]
    pub fn porr00(&mut self) -> Porr00W<PorrSpec> {
        Porr00W::new(self, 0)
    }
    ///Bit 1 - Pmn Output Reset
    #[inline(always)]
    pub fn porr01(&mut self) -> Porr01W<PorrSpec> {
        Porr01W::new(self, 1)
    }
    ///Bit 2 - Pmn Output Reset
    #[inline(always)]
    pub fn porr02(&mut self) -> Porr02W<PorrSpec> {
        Porr02W::new(self, 2)
    }
    ///Bit 3 - Pmn Output Reset
    #[inline(always)]
    pub fn porr03(&mut self) -> Porr03W<PorrSpec> {
        Porr03W::new(self, 3)
    }
    ///Bit 4 - Pmn Output Reset
    #[inline(always)]
    pub fn porr04(&mut self) -> Porr04W<PorrSpec> {
        Porr04W::new(self, 4)
    }
    ///Bit 5 - Pmn Output Reset
    #[inline(always)]
    pub fn porr05(&mut self) -> Porr05W<PorrSpec> {
        Porr05W::new(self, 5)
    }
    ///Bit 6 - Pmn Output Reset
    #[inline(always)]
    pub fn porr06(&mut self) -> Porr06W<PorrSpec> {
        Porr06W::new(self, 6)
    }
    ///Bit 7 - Pmn Output Reset
    #[inline(always)]
    pub fn porr07(&mut self) -> Porr07W<PorrSpec> {
        Porr07W::new(self, 7)
    }
    ///Bit 8 - Pmn Output Reset
    #[inline(always)]
    pub fn porr08(&mut self) -> Porr08W<PorrSpec> {
        Porr08W::new(self, 8)
    }
    ///Bit 9 - Pmn Output Reset
    #[inline(always)]
    pub fn porr09(&mut self) -> Porr09W<PorrSpec> {
        Porr09W::new(self, 9)
    }
    ///Bit 10 - Pmn Output Reset
    #[inline(always)]
    pub fn porr10(&mut self) -> Porr10W<PorrSpec> {
        Porr10W::new(self, 10)
    }
    ///Bit 11 - Pmn Output Reset
    #[inline(always)]
    pub fn porr11(&mut self) -> Porr11W<PorrSpec> {
        Porr11W::new(self, 11)
    }
    ///Bit 12 - Pmn Output Reset
    #[inline(always)]
    pub fn porr12(&mut self) -> Porr12W<PorrSpec> {
        Porr12W::new(self, 12)
    }
    ///Bit 13 - Pmn Output Reset
    #[inline(always)]
    pub fn porr13(&mut self) -> Porr13W<PorrSpec> {
        Porr13W::new(self, 13)
    }
    ///Bit 14 - Pmn Output Reset
    #[inline(always)]
    pub fn porr14(&mut self) -> Porr14W<PorrSpec> {
        Porr14W::new(self, 14)
    }
    ///Bit 15 - Pmn Output Reset
    #[inline(always)]
    pub fn porr15(&mut self) -> Porr15W<PorrSpec> {
        Porr15W::new(self, 15)
    }
}
/**Port Control Register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PorrSpec;
impl crate::RegisterSpec for PorrSpec {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`porr::W`](W) writer structure
impl crate::Writable for PorrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PORR to value 0
impl crate::Resettable for PorrSpec {}
