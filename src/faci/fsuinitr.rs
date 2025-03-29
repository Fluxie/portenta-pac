///Register `FSUINITR` reader
pub type R = crate::R<FsuinitrSpec>;
///Register `FSUINITR` writer
pub type W = crate::W<FsuinitrSpec>;
/**Set-Up Initialization

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suinit {
    ///0: The FSADDR, FEADDR, FBPROT0, FBPROT1, FENTRYR, FBCCNT, and FCPSR flash sequencer setup registers keep their current values
    _0 = 0,
    ///1: The FSADDR, FEADDR, FBPROT0, FBRPOT1, FENTRYR, FBCCNT, and FCPSR flash sequencer setup registers are initialized.
    _1 = 1,
}
impl From<Suinit> for bool {
    #[inline(always)]
    fn from(variant: Suinit) -> Self {
        variant as u8 != 0
    }
}
///Field `SUINIT` reader - Set-Up Initialization
pub type SuinitR = crate::BitReader<Suinit>;
impl SuinitR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Suinit {
        match self.bits {
            false => Suinit::_0,
            true => Suinit::_1,
        }
    }
    ///The FSADDR, FEADDR, FBPROT0, FBPROT1, FENTRYR, FBCCNT, and FCPSR flash sequencer setup registers keep their current values
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Suinit::_0
    }
    ///The FSADDR, FEADDR, FBPROT0, FBRPOT1, FENTRYR, FBCCNT, and FCPSR flash sequencer setup registers are initialized.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Suinit::_1
    }
}
///Field `SUINIT` writer - Set-Up Initialization
pub type SuinitW<'a, REG> = crate::BitWriter<'a, REG, Suinit>;
impl<'a, REG> SuinitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The FSADDR, FEADDR, FBPROT0, FBPROT1, FENTRYR, FBCCNT, and FCPSR flash sequencer setup registers keep their current values
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Suinit::_0)
    }
    ///The FSADDR, FEADDR, FBPROT0, FBRPOT1, FENTRYR, FBCCNT, and FCPSR flash sequencer setup registers are initialized.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Suinit::_1)
    }
}
///Field `KEY` writer - Key Code
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Set-Up Initialization
    #[inline(always)]
    pub fn suinit(&self) -> SuinitR {
        SuinitR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSUINITR").field("suinit", &self.suinit()).finish()
    }
}
impl W {
    ///Bit 0 - Set-Up Initialization
    #[inline(always)]
    pub fn suinit(&mut self) -> SuinitW<FsuinitrSpec> {
        SuinitW::new(self, 0)
    }
    ///Bits 8:15 - Key Code
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<FsuinitrSpec> {
        KeyW::new(self, 8)
    }
}
/**Flash Sequencer Setup Initialization Register

You can [`read`](crate::Reg::read) this register and get [`fsuinitr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsuinitr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FsuinitrSpec;
impl crate::RegisterSpec for FsuinitrSpec {
    type Ux = u16;
}
///`read()` method returns [`fsuinitr::R`](R) reader structure
impl crate::Readable for FsuinitrSpec {}
///`write(|w| ..)` method takes [`fsuinitr::W`](W) writer structure
impl crate::Writable for FsuinitrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FSUINITR to value 0
impl crate::Resettable for FsuinitrSpec {}
