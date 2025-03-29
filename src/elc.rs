#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    elcr: Elcr,
    _reserved1: [u8; 0x01],
    elsegr: (),
    _reserved2: [u8; 0x0e],
    elsr: (),
    _reserved3: [u8; 0x64],
    elcsara: Elcsara,
    _reserved4: [u8; 0x02],
    elcsarb: Elcsarb,
    _reserved5: [u8; 0x02],
    elcsarc: Elcsarc,
}
impl RegisterBlock {
    ///0x00 - Event Link Controller Register
    #[inline(always)]
    pub const fn elcr(&self) -> &Elcr {
        &self.elcr
    }
    ///0x02 - Event Link Software Event Generation Register %s
    #[inline(always)]
    pub const fn elsegr(&self, n: usize) -> &Elsegr {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).add(2 * n).cast() }
    }
    ///Iterator for array of:
    ///0x02 - Event Link Software Event Generation Register %s
    #[inline(always)]
    pub fn elsegr_iter(&self) -> impl Iterator<Item = &Elsegr> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(2).add(2 * n).cast()
            })
    }
    ///0x10..0x36 - Event Link Setting Register %s
    #[inline(always)]
    pub const fn elsr(&self, n: usize) -> &Elsr {
        #[allow(clippy::no_effect)] [(); 19][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x10..0x36 - Event Link Setting Register %s
    #[inline(always)]
    pub fn elsr_iter(&self) -> impl Iterator<Item = &Elsr> {
        (0..19)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(16).add(4 * n).cast()
            })
    }
    ///0x74 - Event Link Controller Security Attribution Register A
    #[inline(always)]
    pub const fn elcsara(&self) -> &Elcsara {
        &self.elcsara
    }
    ///0x78 - Event Link Controller Security Attribution Register B
    #[inline(always)]
    pub const fn elcsarb(&self) -> &Elcsarb {
        &self.elcsarb
    }
    ///0x7c - Event Link Controller Security Attribution Register C
    #[inline(always)]
    pub const fn elcsarc(&self) -> &Elcsarc {
        &self.elcsarc
    }
}
/**ELCR (rw) register accessor: Event Link Controller Register

You can [`read`](crate::Reg::read) this register and get [`elcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@elcr`] module*/
#[doc(alias = "ELCR")]
pub type Elcr = crate::Reg<elcr::ElcrSpec>;
///Event Link Controller Register
pub mod elcr;
/**ELSEGR (rw) register accessor: Event Link Software Event Generation Register %s

You can [`read`](crate::Reg::read) this register and get [`elsegr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsegr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@elsegr`] module*/
#[doc(alias = "ELSEGR")]
pub type Elsegr = crate::Reg<elsegr::ElsegrSpec>;
///Event Link Software Event Generation Register %s
pub mod elsegr;
/**ELSR (rw) register accessor: Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`elsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@elsr`] module*/
#[doc(alias = "ELSR")]
pub type Elsr = crate::Reg<elsr::ElsrSpec>;
///Event Link Setting Register %s
pub mod elsr;
/**ELCSARA (rw) register accessor: Event Link Controller Security Attribution Register A

You can [`read`](crate::Reg::read) this register and get [`elcsara::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elcsara::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@elcsara`] module*/
#[doc(alias = "ELCSARA")]
pub type Elcsara = crate::Reg<elcsara::ElcsaraSpec>;
///Event Link Controller Security Attribution Register A
pub mod elcsara;
/**ELCSARB (rw) register accessor: Event Link Controller Security Attribution Register B

You can [`read`](crate::Reg::read) this register and get [`elcsarb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elcsarb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@elcsarb`] module*/
#[doc(alias = "ELCSARB")]
pub type Elcsarb = crate::Reg<elcsarb::ElcsarbSpec>;
///Event Link Controller Security Attribution Register B
pub mod elcsarb;
/**ELCSARC (rw) register accessor: Event Link Controller Security Attribution Register C

You can [`read`](crate::Reg::read) this register and get [`elcsarc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elcsarc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@elcsarc`] module*/
#[doc(alias = "ELCSARC")]
pub type Elcsarc = crate::Reg<elcsarc::ElcsarcSpec>;
///Event Link Controller Security Attribution Register C
pub mod elcsarc;
