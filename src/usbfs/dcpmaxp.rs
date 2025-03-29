///Register `DCPMAXP` reader
pub type R = crate::R<DcpmaxpSpec>;
///Register `DCPMAXP` writer
pub type W = crate::W<DcpmaxpSpec>;
///Field `MXPS` reader - Maximum Packet Size
pub type MxpsR = crate::FieldReader;
///Field `MXPS` writer - Maximum Packet Size
pub type MxpsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Device Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Devsel {
    ///0: Address 0000b
    _0x0 = 0,
    ///1: Address 0001b
    _0x1 = 1,
    ///2: Address 0010b
    _0x2 = 2,
    ///3: Address 0011b
    _0x3 = 3,
    ///4: Address 0100b
    _0x4 = 4,
    ///5: Address 0101b
    _0x5 = 5,
    ///6: Setting prohibited
    Others = 6,
}
impl From<Devsel> for u8 {
    #[inline(always)]
    fn from(variant: Devsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Devsel {
    type Ux = u8;
}
impl crate::IsEnum for Devsel {}
///Field `DEVSEL` reader - Device Select
pub type DevselR = crate::FieldReader<Devsel>;
impl DevselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Devsel {
        match self.bits {
            0 => Devsel::_0x0,
            1 => Devsel::_0x1,
            2 => Devsel::_0x2,
            3 => Devsel::_0x3,
            4 => Devsel::_0x4,
            5 => Devsel::_0x5,
            _ => Devsel::Others,
        }
    }
    ///Address 0000b
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Devsel::_0x0
    }
    ///Address 0001b
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Devsel::_0x1
    }
    ///Address 0010b
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Devsel::_0x2
    }
    ///Address 0011b
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Devsel::_0x3
    }
    ///Address 0100b
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Devsel::_0x4
    }
    ///Address 0101b
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Devsel::_0x5
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Devsel::Others)
    }
}
///Field `DEVSEL` writer - Device Select
pub type DevselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Devsel, crate::Safe>;
impl<'a, REG> DevselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Address 0000b
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0x0)
    }
    ///Address 0001b
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0x1)
    }
    ///Address 0010b
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0x2)
    }
    ///Address 0011b
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0x3)
    }
    ///Address 0100b
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0x4)
    }
    ///Address 0101b
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0x5)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::Others)
    }
}
impl R {
    ///Bits 0:6 - Maximum Packet Size
    #[inline(always)]
    pub fn mxps(&self) -> MxpsR {
        MxpsR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 12:15 - Device Select
    #[inline(always)]
    pub fn devsel(&self) -> DevselR {
        DevselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCPMAXP")
            .field("mxps", &self.mxps())
            .field("devsel", &self.devsel())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Maximum Packet Size
    #[inline(always)]
    pub fn mxps(&mut self) -> MxpsW<DcpmaxpSpec> {
        MxpsW::new(self, 0)
    }
    ///Bits 12:15 - Device Select
    #[inline(always)]
    pub fn devsel(&mut self) -> DevselW<DcpmaxpSpec> {
        DevselW::new(self, 12)
    }
}
/**DCP Maximum Packet Size Register

You can [`read`](crate::Reg::read) this register and get [`dcpmaxp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpmaxp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcpmaxpSpec;
impl crate::RegisterSpec for DcpmaxpSpec {
    type Ux = u16;
}
///`read()` method returns [`dcpmaxp::R`](R) reader structure
impl crate::Readable for DcpmaxpSpec {}
///`write(|w| ..)` method takes [`dcpmaxp::W`](W) writer structure
impl crate::Writable for DcpmaxpSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCPMAXP to value 0x40
impl crate::Resettable for DcpmaxpSpec {
    const RESET_VALUE: u16 = 0x40;
}
