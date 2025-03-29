#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    r64cnt: R64cnt,
    _reserved1: [u8; 0x01],
    rseccnt: Rseccnt,
    _reserved2: [u8; 0x01],
    rmincnt: Rmincnt,
    _reserved3: [u8; 0x01],
    rhrcnt: Rhrcnt,
    _reserved4: [u8; 0x01],
    rwkcnt: Rwkcnt,
    _reserved5: [u8; 0x01],
    rdaycnt: Rdaycnt,
    _reserved6: [u8; 0x01],
    rmoncnt: Rmoncnt,
    _reserved7: [u8; 0x01],
    ryrcnt: Ryrcnt,
    rsecar: Rsecar,
    _reserved9: [u8; 0x01],
    rminar: Rminar,
    _reserved10: [u8; 0x01],
    rhrar: Rhrar,
    _reserved11: [u8; 0x01],
    rwkar: Rwkar,
    _reserved12: [u8; 0x01],
    rdayar: Rdayar,
    _reserved13: [u8; 0x01],
    rmonar: Rmonar,
    _reserved14: [u8; 0x01],
    ryrar: Ryrar,
    ryraren: Ryraren,
    _reserved16: [u8; 0x03],
    rcr1: Rcr1,
    _reserved17: [u8; 0x01],
    rcr2: Rcr2,
    _reserved18: [u8; 0x03],
    rcr4: Rcr4,
    _reserved19: [u8; 0x01],
    rfrh: Rfrh,
    rfrl: Rfrl,
    radj: Radj,
    _reserved22: [u8; 0x11],
    rtccr: (),
    _reserved23: [u8; 0x12],
    rseccp: (),
    _reserved24: [u8; 0x02],
    rmincp: (),
    _reserved25: [u8; 0x02],
    rhrcp: (),
    _reserved26: [u8; 0x04],
    rdaycp: (),
    _reserved27: [u8; 0x02],
    rmoncp: (),
}
impl RegisterBlock {
    ///0x00 - 64-Hz Counter
    #[inline(always)]
    pub const fn r64cnt(&self) -> &R64cnt {
        &self.r64cnt
    }
    ///0x02 - Second Counter (in Calendar Count Mode)
    #[inline(always)]
    pub const fn rseccnt(&self) -> &Rseccnt {
        &self.rseccnt
    }
    ///0x04 - Minute Counter (in Calendar Count Mode)
    #[inline(always)]
    pub const fn rmincnt(&self) -> &Rmincnt {
        &self.rmincnt
    }
    ///0x06 - Hour Counter (in Calendar Count Mode)
    #[inline(always)]
    pub const fn rhrcnt(&self) -> &Rhrcnt {
        &self.rhrcnt
    }
    ///0x08 - Day-of-Week Counter (in Calendar Count Mode)
    #[inline(always)]
    pub const fn rwkcnt(&self) -> &Rwkcnt {
        &self.rwkcnt
    }
    ///0x0a - Day Counter
    #[inline(always)]
    pub const fn rdaycnt(&self) -> &Rdaycnt {
        &self.rdaycnt
    }
    ///0x0c - Month Counter
    #[inline(always)]
    pub const fn rmoncnt(&self) -> &Rmoncnt {
        &self.rmoncnt
    }
    ///0x0e - Year Counter
    #[inline(always)]
    pub const fn ryrcnt(&self) -> &Ryrcnt {
        &self.ryrcnt
    }
    ///0x10 - Second Alarm Register (in Calendar Count Mode)
    #[inline(always)]
    pub const fn rsecar(&self) -> &Rsecar {
        &self.rsecar
    }
    ///0x12 - Minute Alarm Register (in Calendar Count Mode)
    #[inline(always)]
    pub const fn rminar(&self) -> &Rminar {
        &self.rminar
    }
    ///0x14 - Hour Alarm Register (in Calendar Count Mode)
    #[inline(always)]
    pub const fn rhrar(&self) -> &Rhrar {
        &self.rhrar
    }
    ///0x16 - Day-of-Week Alarm Register (in Calendar Count Mode)
    #[inline(always)]
    pub const fn rwkar(&self) -> &Rwkar {
        &self.rwkar
    }
    ///0x18 - Date Alarm Register (in Calendar Count Mode)
    #[inline(always)]
    pub const fn rdayar(&self) -> &Rdayar {
        &self.rdayar
    }
    ///0x1a - Month Alarm Register (in Calendar Count Mode)
    #[inline(always)]
    pub const fn rmonar(&self) -> &Rmonar {
        &self.rmonar
    }
    ///0x1c - Year Alarm Register (in Calendar Count Mode)
    #[inline(always)]
    pub const fn ryrar(&self) -> &Ryrar {
        &self.ryrar
    }
    ///0x1e - Year Alarm Enable Register (in Calendar Count Mode)
    #[inline(always)]
    pub const fn ryraren(&self) -> &Ryraren {
        &self.ryraren
    }
    ///0x22 - RTC Control Register 1
    #[inline(always)]
    pub const fn rcr1(&self) -> &Rcr1 {
        &self.rcr1
    }
    ///0x24 - RTC Control Register 2 (in Calendar Count Mode)
    #[inline(always)]
    pub const fn rcr2(&self) -> &Rcr2 {
        &self.rcr2
    }
    ///0x28 - RTC Control Register 4
    #[inline(always)]
    pub const fn rcr4(&self) -> &Rcr4 {
        &self.rcr4
    }
    ///0x2a - Frequency Register H
    #[inline(always)]
    pub const fn rfrh(&self) -> &Rfrh {
        &self.rfrh
    }
    ///0x2c - Frequency Register L
    #[inline(always)]
    pub const fn rfrl(&self) -> &Rfrl {
        &self.rfrl
    }
    ///0x2e - Time Error Adjustment Register
    #[inline(always)]
    pub const fn radj(&self) -> &Radj {
        &self.radj
    }
    ///0x40 - Time Capture Control Register %s
    #[inline(always)]
    pub const fn rtccr(&self, n: usize) -> &Rtccr {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).add(2 * n).cast() }
    }
    ///Iterator for array of:
    ///0x40 - Time Capture Control Register %s
    #[inline(always)]
    pub fn rtccr_iter(&self) -> impl Iterator<Item = &Rtccr> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(64).add(2 * n).cast()
            })
    }
    ///0x52 - Second Capture Register %s
    #[inline(always)]
    pub const fn rseccp(&self, n: usize) -> &Rseccp {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(82).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x52 - Second Capture Register %s
    #[inline(always)]
    pub fn rseccp_iter(&self) -> impl Iterator<Item = &Rseccp> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(82).add(16 * n).cast()
            })
    }
    ///0x54 - Minute Capture Register %s
    #[inline(always)]
    pub const fn rmincp(&self, n: usize) -> &Rmincp {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x54 - Minute Capture Register %s
    #[inline(always)]
    pub fn rmincp_iter(&self) -> impl Iterator<Item = &Rmincp> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(84).add(16 * n).cast()
            })
    }
    ///0x56 - Hour Capture Register %s
    #[inline(always)]
    pub const fn rhrcp(&self, n: usize) -> &Rhrcp {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(86).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x56 - Hour Capture Register %s
    #[inline(always)]
    pub fn rhrcp_iter(&self) -> impl Iterator<Item = &Rhrcp> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(86).add(16 * n).cast()
            })
    }
    ///0x5a - Date Capture Register %s
    #[inline(always)]
    pub const fn rdaycp(&self, n: usize) -> &Rdaycp {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(90).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x5a - Date Capture Register %s
    #[inline(always)]
    pub fn rdaycp_iter(&self) -> impl Iterator<Item = &Rdaycp> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(90).add(16 * n).cast()
            })
    }
    ///0x5c - Month Capture Register %s
    #[inline(always)]
    pub const fn rmoncp(&self, n: usize) -> &Rmoncp {
        #[allow(clippy::no_effect)] [(); 3][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x5c - Month Capture Register %s
    #[inline(always)]
    pub fn rmoncp_iter(&self) -> impl Iterator<Item = &Rmoncp> {
        (0..3)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(92).add(16 * n).cast()
            })
    }
}
/**R64CNT (r) register accessor: 64-Hz Counter

You can [`read`](crate::Reg::read) this register and get [`r64cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@r64cnt`] module*/
#[doc(alias = "R64CNT")]
pub type R64cnt = crate::Reg<r64cnt::R64cntSpec>;
///64-Hz Counter
pub mod r64cnt;
/**RSECCNT (rw) register accessor: Second Counter (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rseccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rseccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rseccnt`] module*/
#[doc(alias = "RSECCNT")]
pub type Rseccnt = crate::Reg<rseccnt::RseccntSpec>;
///Second Counter (in Calendar Count Mode)
pub mod rseccnt;
/**RMINCNT (rw) register accessor: Minute Counter (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rmincnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmincnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmincnt`] module*/
#[doc(alias = "RMINCNT")]
pub type Rmincnt = crate::Reg<rmincnt::RmincntSpec>;
///Minute Counter (in Calendar Count Mode)
pub mod rmincnt;
/**RHRCNT (rw) register accessor: Hour Counter (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rhrcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rhrcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rhrcnt`] module*/
#[doc(alias = "RHRCNT")]
pub type Rhrcnt = crate::Reg<rhrcnt::RhrcntSpec>;
///Hour Counter (in Calendar Count Mode)
pub mod rhrcnt;
/**RWKCNT (rw) register accessor: Day-of-Week Counter (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rwkcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwkcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rwkcnt`] module*/
#[doc(alias = "RWKCNT")]
pub type Rwkcnt = crate::Reg<rwkcnt::RwkcntSpec>;
///Day-of-Week Counter (in Calendar Count Mode)
pub mod rwkcnt;
/**RDAYCNT (rw) register accessor: Day Counter

You can [`read`](crate::Reg::read) this register and get [`rdaycnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdaycnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdaycnt`] module*/
#[doc(alias = "RDAYCNT")]
pub type Rdaycnt = crate::Reg<rdaycnt::RdaycntSpec>;
///Day Counter
pub mod rdaycnt;
/**RMONCNT (rw) register accessor: Month Counter

You can [`read`](crate::Reg::read) this register and get [`rmoncnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmoncnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmoncnt`] module*/
#[doc(alias = "RMONCNT")]
pub type Rmoncnt = crate::Reg<rmoncnt::RmoncntSpec>;
///Month Counter
pub mod rmoncnt;
/**RYRCNT (rw) register accessor: Year Counter

You can [`read`](crate::Reg::read) this register and get [`ryrcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryrcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ryrcnt`] module*/
#[doc(alias = "RYRCNT")]
pub type Ryrcnt = crate::Reg<ryrcnt::RyrcntSpec>;
///Year Counter
pub mod ryrcnt;
/**RSECAR (rw) register accessor: Second Alarm Register (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rsecar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsecar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rsecar`] module*/
#[doc(alias = "RSECAR")]
pub type Rsecar = crate::Reg<rsecar::RsecarSpec>;
///Second Alarm Register (in Calendar Count Mode)
pub mod rsecar;
/**RMINAR (rw) register accessor: Minute Alarm Register (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rminar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rminar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rminar`] module*/
#[doc(alias = "RMINAR")]
pub type Rminar = crate::Reg<rminar::RminarSpec>;
///Minute Alarm Register (in Calendar Count Mode)
pub mod rminar;
/**RHRAR (rw) register accessor: Hour Alarm Register (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rhrar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rhrar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rhrar`] module*/
#[doc(alias = "RHRAR")]
pub type Rhrar = crate::Reg<rhrar::RhrarSpec>;
///Hour Alarm Register (in Calendar Count Mode)
pub mod rhrar;
/**RWKAR (rw) register accessor: Day-of-Week Alarm Register (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rwkar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwkar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rwkar`] module*/
#[doc(alias = "RWKAR")]
pub type Rwkar = crate::Reg<rwkar::RwkarSpec>;
///Day-of-Week Alarm Register (in Calendar Count Mode)
pub mod rwkar;
/**RDAYAR (rw) register accessor: Date Alarm Register (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rdayar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdayar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdayar`] module*/
#[doc(alias = "RDAYAR")]
pub type Rdayar = crate::Reg<rdayar::RdayarSpec>;
///Date Alarm Register (in Calendar Count Mode)
pub mod rdayar;
/**RMONAR (rw) register accessor: Month Alarm Register (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rmonar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmonar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmonar`] module*/
#[doc(alias = "RMONAR")]
pub type Rmonar = crate::Reg<rmonar::RmonarSpec>;
///Month Alarm Register (in Calendar Count Mode)
pub mod rmonar;
/**RYRAR (rw) register accessor: Year Alarm Register (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`ryrar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryrar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ryrar`] module*/
#[doc(alias = "RYRAR")]
pub type Ryrar = crate::Reg<ryrar::RyrarSpec>;
///Year Alarm Register (in Calendar Count Mode)
pub mod ryrar;
/**RYRAREN (rw) register accessor: Year Alarm Enable Register (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`ryraren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryraren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ryraren`] module*/
#[doc(alias = "RYRAREN")]
pub type Ryraren = crate::Reg<ryraren::RyrarenSpec>;
///Year Alarm Enable Register (in Calendar Count Mode)
pub mod ryraren;
/**RCR1 (rw) register accessor: RTC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`rcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rcr1`] module*/
#[doc(alias = "RCR1")]
pub type Rcr1 = crate::Reg<rcr1::Rcr1Spec>;
///RTC Control Register 1
pub mod rcr1;
/**RCR2 (rw) register accessor: RTC Control Register 2 (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rcr2`] module*/
#[doc(alias = "RCR2")]
pub type Rcr2 = crate::Reg<rcr2::Rcr2Spec>;
///RTC Control Register 2 (in Calendar Count Mode)
pub mod rcr2;
/**RCR4 (rw) register accessor: RTC Control Register 4

You can [`read`](crate::Reg::read) this register and get [`rcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rcr4`] module*/
#[doc(alias = "RCR4")]
pub type Rcr4 = crate::Reg<rcr4::Rcr4Spec>;
///RTC Control Register 4
pub mod rcr4;
/**RFRH (rw) register accessor: Frequency Register H

You can [`read`](crate::Reg::read) this register and get [`rfrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rfrh`] module*/
#[doc(alias = "RFRH")]
pub type Rfrh = crate::Reg<rfrh::RfrhSpec>;
///Frequency Register H
pub mod rfrh;
/**RFRL (rw) register accessor: Frequency Register L

You can [`read`](crate::Reg::read) this register and get [`rfrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rfrl`] module*/
#[doc(alias = "RFRL")]
pub type Rfrl = crate::Reg<rfrl::RfrlSpec>;
///Frequency Register L
pub mod rfrl;
/**RADJ (rw) register accessor: Time Error Adjustment Register

You can [`read`](crate::Reg::read) this register and get [`radj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@radj`] module*/
#[doc(alias = "RADJ")]
pub type Radj = crate::Reg<radj::RadjSpec>;
///Time Error Adjustment Register
pub mod radj;
/**RTCCR (rw) register accessor: Time Capture Control Register %s

You can [`read`](crate::Reg::read) this register and get [`rtccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccr`] module*/
#[doc(alias = "RTCCR")]
pub type Rtccr = crate::Reg<rtccr::RtccrSpec>;
///Time Capture Control Register %s
pub mod rtccr;
/**RSECCP (r) register accessor: Second Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rseccp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rseccp`] module*/
#[doc(alias = "RSECCP")]
pub type Rseccp = crate::Reg<rseccp::RseccpSpec>;
///Second Capture Register %s
pub mod rseccp;
/**RMINCP (r) register accessor: Minute Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rmincp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmincp`] module*/
#[doc(alias = "RMINCP")]
pub type Rmincp = crate::Reg<rmincp::RmincpSpec>;
///Minute Capture Register %s
pub mod rmincp;
/**RHRCP (r) register accessor: Hour Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rhrcp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rhrcp`] module*/
#[doc(alias = "RHRCP")]
pub type Rhrcp = crate::Reg<rhrcp::RhrcpSpec>;
///Hour Capture Register %s
pub mod rhrcp;
/**RDAYCP (r) register accessor: Date Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rdaycp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdaycp`] module*/
#[doc(alias = "RDAYCP")]
pub type Rdaycp = crate::Reg<rdaycp::RdaycpSpec>;
///Date Capture Register %s
pub mod rdaycp;
/**RMONCP (r) register accessor: Month Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rmoncp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmoncp`] module*/
#[doc(alias = "RMONCP")]
pub type Rmoncp = crate::Reg<rmoncp::RmoncpSpec>;
///Month Capture Register %s
pub mod rmoncp;
