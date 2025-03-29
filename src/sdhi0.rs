#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    sd_cmd: SdCmd,
    _reserved1: [u8; 0x04],
    sd_arg: SdArg,
    sd_arg1: SdArg1,
    sd_stop: SdStop,
    sd_seccnt: SdSeccnt,
    sd_rsp10: SdRsp10,
    sd_rsp1: SdRsp1,
    sd_rsp32: SdRsp32,
    sd_rsp3: SdRsp3,
    sd_rsp54: SdRsp54,
    sd_rsp5: SdRsp5,
    sd_rsp76: SdRsp76,
    sd_rsp7: SdRsp7,
    sd_info1: SdInfo1,
    sd_info2: SdInfo2,
    sd_info1_mask: SdInfo1Mask,
    sd_info2_mask: SdInfo2Mask,
    sd_clk_ctrl: SdClkCtrl,
    sd_size: SdSize,
    sd_option: SdOption,
    _reserved20: [u8; 0x04],
    sd_err_sts1: SdErrSts1,
    sd_err_sts2: SdErrSts2,
    sd_buf0: SdBuf0,
    _reserved23: [u8; 0x04],
    sdio_mode: SdioMode,
    sdio_info1: SdioInfo1,
    sdio_info1_mask: SdioInfo1Mask,
    _reserved26: [u8; 0x013c],
    sd_dmaen: SdDmaen,
    _reserved27: [u8; 0x0c],
    soft_rst: SoftRst,
    _reserved28: [u8; 0x08],
    sdif_mode: SdifMode,
    _reserved29: [u8; 0x10],
    ext_swap: ExtSwap,
}
impl RegisterBlock {
    ///0x00 - Command Type Register
    #[inline(always)]
    pub const fn sd_cmd(&self) -> &SdCmd {
        &self.sd_cmd
    }
    ///0x08 - SD Command Argument Register
    #[inline(always)]
    pub const fn sd_arg(&self) -> &SdArg {
        &self.sd_arg
    }
    ///0x0c - SD Command Argument Register 1
    #[inline(always)]
    pub const fn sd_arg1(&self) -> &SdArg1 {
        &self.sd_arg1
    }
    ///0x10 - Data Stop Register
    #[inline(always)]
    pub const fn sd_stop(&self) -> &SdStop {
        &self.sd_stop
    }
    ///0x14 - Block Count Register
    #[inline(always)]
    pub const fn sd_seccnt(&self) -> &SdSeccnt {
        &self.sd_seccnt
    }
    ///0x18 - SD Card Response Register 10
    #[inline(always)]
    pub const fn sd_rsp10(&self) -> &SdRsp10 {
        &self.sd_rsp10
    }
    ///0x1c - SD Card Response Register 1
    #[inline(always)]
    pub const fn sd_rsp1(&self) -> &SdRsp1 {
        &self.sd_rsp1
    }
    ///0x20 - SD Card Response Register 32
    #[inline(always)]
    pub const fn sd_rsp32(&self) -> &SdRsp32 {
        &self.sd_rsp32
    }
    ///0x24 - SD Card Response Register 3
    #[inline(always)]
    pub const fn sd_rsp3(&self) -> &SdRsp3 {
        &self.sd_rsp3
    }
    ///0x28 - SD Card Response Register 54
    #[inline(always)]
    pub const fn sd_rsp54(&self) -> &SdRsp54 {
        &self.sd_rsp54
    }
    ///0x2c - SD Card Response Register 5
    #[inline(always)]
    pub const fn sd_rsp5(&self) -> &SdRsp5 {
        &self.sd_rsp5
    }
    ///0x30 - SD Card Response Register 76
    #[inline(always)]
    pub const fn sd_rsp76(&self) -> &SdRsp76 {
        &self.sd_rsp76
    }
    ///0x34 - SD Card Response Register 7
    #[inline(always)]
    pub const fn sd_rsp7(&self) -> &SdRsp7 {
        &self.sd_rsp7
    }
    ///0x38 - SD Card Interrupt Flag Register 1
    #[inline(always)]
    pub const fn sd_info1(&self) -> &SdInfo1 {
        &self.sd_info1
    }
    ///0x3c - SD Card Interrupt Flag Register 2
    #[inline(always)]
    pub const fn sd_info2(&self) -> &SdInfo2 {
        &self.sd_info2
    }
    ///0x40 - SD INFO1 Interrupt Mask Register
    #[inline(always)]
    pub const fn sd_info1_mask(&self) -> &SdInfo1Mask {
        &self.sd_info1_mask
    }
    ///0x44 - SD INFO2 Interrupt Mask Register
    #[inline(always)]
    pub const fn sd_info2_mask(&self) -> &SdInfo2Mask {
        &self.sd_info2_mask
    }
    ///0x48 - SD Clock Control Register
    #[inline(always)]
    pub const fn sd_clk_ctrl(&self) -> &SdClkCtrl {
        &self.sd_clk_ctrl
    }
    ///0x4c - Transfer Data Length Register
    #[inline(always)]
    pub const fn sd_size(&self) -> &SdSize {
        &self.sd_size
    }
    ///0x50 - SD Card Access Control Option Register
    #[inline(always)]
    pub const fn sd_option(&self) -> &SdOption {
        &self.sd_option
    }
    ///0x58 - SD Error Status Register 1
    #[inline(always)]
    pub const fn sd_err_sts1(&self) -> &SdErrSts1 {
        &self.sd_err_sts1
    }
    ///0x5c - SD Error Status Register 2
    #[inline(always)]
    pub const fn sd_err_sts2(&self) -> &SdErrSts2 {
        &self.sd_err_sts2
    }
    ///0x60 - SD Buffer Register
    #[inline(always)]
    pub const fn sd_buf0(&self) -> &SdBuf0 {
        &self.sd_buf0
    }
    ///0x68 - SDIO Mode Control Register
    #[inline(always)]
    pub const fn sdio_mode(&self) -> &SdioMode {
        &self.sdio_mode
    }
    ///0x6c - SDIO Interrupt Flag Register
    #[inline(always)]
    pub const fn sdio_info1(&self) -> &SdioInfo1 {
        &self.sdio_info1
    }
    ///0x70 - SDIO INFO1 Interrupt Mask Register
    #[inline(always)]
    pub const fn sdio_info1_mask(&self) -> &SdioInfo1Mask {
        &self.sdio_info1_mask
    }
    ///0x1b0 - DMA Mode Enable Register
    #[inline(always)]
    pub const fn sd_dmaen(&self) -> &SdDmaen {
        &self.sd_dmaen
    }
    ///0x1c0 - Software Reset Register
    #[inline(always)]
    pub const fn soft_rst(&self) -> &SoftRst {
        &self.soft_rst
    }
    ///0x1cc - SD Interface Mode Setting Register
    #[inline(always)]
    pub const fn sdif_mode(&self) -> &SdifMode {
        &self.sdif_mode
    }
    ///0x1e0 - Swap Control Register
    #[inline(always)]
    pub const fn ext_swap(&self) -> &ExtSwap {
        &self.ext_swap
    }
}
/**SD_CMD (rw) register accessor: Command Type Register

You can [`read`](crate::Reg::read) this register and get [`sd_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_cmd`] module*/
#[doc(alias = "SD_CMD")]
pub type SdCmd = crate::Reg<sd_cmd::SdCmdSpec>;
///Command Type Register
pub mod sd_cmd;
/**SD_ARG (rw) register accessor: SD Command Argument Register

You can [`read`](crate::Reg::read) this register and get [`sd_arg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_arg`] module*/
#[doc(alias = "SD_ARG")]
pub type SdArg = crate::Reg<sd_arg::SdArgSpec>;
///SD Command Argument Register
pub mod sd_arg;
/**SD_ARG1 (rw) register accessor: SD Command Argument Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_arg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_arg1`] module*/
#[doc(alias = "SD_ARG1")]
pub type SdArg1 = crate::Reg<sd_arg1::SdArg1Spec>;
///SD Command Argument Register 1
pub mod sd_arg1;
/**SD_STOP (rw) register accessor: Data Stop Register

You can [`read`](crate::Reg::read) this register and get [`sd_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_stop`] module*/
#[doc(alias = "SD_STOP")]
pub type SdStop = crate::Reg<sd_stop::SdStopSpec>;
///Data Stop Register
pub mod sd_stop;
/**SD_SECCNT (rw) register accessor: Block Count Register

You can [`read`](crate::Reg::read) this register and get [`sd_seccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_seccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_seccnt`] module*/
#[doc(alias = "SD_SECCNT")]
pub type SdSeccnt = crate::Reg<sd_seccnt::SdSeccntSpec>;
///Block Count Register
pub mod sd_seccnt;
/**SD_RSP10 (rw) register accessor: SD Card Response Register 10

You can [`read`](crate::Reg::read) this register and get [`sd_rsp10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_rsp10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp10`] module*/
#[doc(alias = "SD_RSP10")]
pub type SdRsp10 = crate::Reg<sd_rsp10::SdRsp10Spec>;
///SD Card Response Register 10
pub mod sd_rsp10;
/**SD_RSP1 (r) register accessor: SD Card Response Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_rsp1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp1`] module*/
#[doc(alias = "SD_RSP1")]
pub type SdRsp1 = crate::Reg<sd_rsp1::SdRsp1Spec>;
///SD Card Response Register 1
pub mod sd_rsp1;
/**SD_RSP32 (rw) register accessor: SD Card Response Register 32

You can [`read`](crate::Reg::read) this register and get [`sd_rsp32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_rsp32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp32`] module*/
#[doc(alias = "SD_RSP32")]
pub type SdRsp32 = crate::Reg<sd_rsp32::SdRsp32Spec>;
///SD Card Response Register 32
pub mod sd_rsp32;
/**SD_RSP3 (r) register accessor: SD Card Response Register 3

You can [`read`](crate::Reg::read) this register and get [`sd_rsp3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp3`] module*/
#[doc(alias = "SD_RSP3")]
pub type SdRsp3 = crate::Reg<sd_rsp3::SdRsp3Spec>;
///SD Card Response Register 3
pub mod sd_rsp3;
/**SD_RSP54 (rw) register accessor: SD Card Response Register 54

You can [`read`](crate::Reg::read) this register and get [`sd_rsp54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_rsp54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp54`] module*/
#[doc(alias = "SD_RSP54")]
pub type SdRsp54 = crate::Reg<sd_rsp54::SdRsp54Spec>;
///SD Card Response Register 54
pub mod sd_rsp54;
/**SD_RSP5 (r) register accessor: SD Card Response Register 5

You can [`read`](crate::Reg::read) this register and get [`sd_rsp5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp5`] module*/
#[doc(alias = "SD_RSP5")]
pub type SdRsp5 = crate::Reg<sd_rsp5::SdRsp5Spec>;
///SD Card Response Register 5
pub mod sd_rsp5;
/**SD_RSP76 (r) register accessor: SD Card Response Register 76

You can [`read`](crate::Reg::read) this register and get [`sd_rsp76::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp76`] module*/
#[doc(alias = "SD_RSP76")]
pub type SdRsp76 = crate::Reg<sd_rsp76::SdRsp76Spec>;
///SD Card Response Register 76
pub mod sd_rsp76;
/**SD_RSP7 (r) register accessor: SD Card Response Register 7

You can [`read`](crate::Reg::read) this register and get [`sd_rsp7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp7`] module*/
#[doc(alias = "SD_RSP7")]
pub type SdRsp7 = crate::Reg<sd_rsp7::SdRsp7Spec>;
///SD Card Response Register 7
pub mod sd_rsp7;
/**SD_INFO1 (rw) register accessor: SD Card Interrupt Flag Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_info1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_info1`] module*/
#[doc(alias = "SD_INFO1")]
pub type SdInfo1 = crate::Reg<sd_info1::SdInfo1Spec>;
///SD Card Interrupt Flag Register 1
pub mod sd_info1;
/**SD_INFO2 (rw) register accessor: SD Card Interrupt Flag Register 2

You can [`read`](crate::Reg::read) this register and get [`sd_info2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_info2`] module*/
#[doc(alias = "SD_INFO2")]
pub type SdInfo2 = crate::Reg<sd_info2::SdInfo2Spec>;
///SD Card Interrupt Flag Register 2
pub mod sd_info2;
/**SD_INFO1_MASK (rw) register accessor: SD INFO1 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sd_info1_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info1_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_info1_mask`] module*/
#[doc(alias = "SD_INFO1_MASK")]
pub type SdInfo1Mask = crate::Reg<sd_info1_mask::SdInfo1MaskSpec>;
///SD INFO1 Interrupt Mask Register
pub mod sd_info1_mask;
/**SD_INFO2_MASK (rw) register accessor: SD INFO2 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sd_info2_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info2_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_info2_mask`] module*/
#[doc(alias = "SD_INFO2_MASK")]
pub type SdInfo2Mask = crate::Reg<sd_info2_mask::SdInfo2MaskSpec>;
///SD INFO2 Interrupt Mask Register
pub mod sd_info2_mask;
/**SD_CLK_CTRL (rw) register accessor: SD Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`sd_clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_clk_ctrl`] module*/
#[doc(alias = "SD_CLK_CTRL")]
pub type SdClkCtrl = crate::Reg<sd_clk_ctrl::SdClkCtrlSpec>;
///SD Clock Control Register
pub mod sd_clk_ctrl;
/**SD_SIZE (rw) register accessor: Transfer Data Length Register

You can [`read`](crate::Reg::read) this register and get [`sd_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_size`] module*/
#[doc(alias = "SD_SIZE")]
pub type SdSize = crate::Reg<sd_size::SdSizeSpec>;
///Transfer Data Length Register
pub mod sd_size;
/**SD_OPTION (rw) register accessor: SD Card Access Control Option Register

You can [`read`](crate::Reg::read) this register and get [`sd_option::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_option::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_option`] module*/
#[doc(alias = "SD_OPTION")]
pub type SdOption = crate::Reg<sd_option::SdOptionSpec>;
///SD Card Access Control Option Register
pub mod sd_option;
/**SD_ERR_STS1 (r) register accessor: SD Error Status Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_err_sts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_err_sts1`] module*/
#[doc(alias = "SD_ERR_STS1")]
pub type SdErrSts1 = crate::Reg<sd_err_sts1::SdErrSts1Spec>;
///SD Error Status Register 1
pub mod sd_err_sts1;
/**SD_ERR_STS2 (r) register accessor: SD Error Status Register 2

You can [`read`](crate::Reg::read) this register and get [`sd_err_sts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_err_sts2`] module*/
#[doc(alias = "SD_ERR_STS2")]
pub type SdErrSts2 = crate::Reg<sd_err_sts2::SdErrSts2Spec>;
///SD Error Status Register 2
pub mod sd_err_sts2;
/**SD_BUF0 (rw) register accessor: SD Buffer Register

You can [`read`](crate::Reg::read) this register and get [`sd_buf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_buf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_buf0`] module*/
#[doc(alias = "SD_BUF0")]
pub type SdBuf0 = crate::Reg<sd_buf0::SdBuf0Spec>;
///SD Buffer Register
pub mod sd_buf0;
/**SDIO_MODE (rw) register accessor: SDIO Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sdio_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_mode`] module*/
#[doc(alias = "SDIO_MODE")]
pub type SdioMode = crate::Reg<sdio_mode::SdioModeSpec>;
///SDIO Mode Control Register
pub mod sdio_mode;
/**SDIO_INFO1 (rw) register accessor: SDIO Interrupt Flag Register

You can [`read`](crate::Reg::read) this register and get [`sdio_info1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_info1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_info1`] module*/
#[doc(alias = "SDIO_INFO1")]
pub type SdioInfo1 = crate::Reg<sdio_info1::SdioInfo1Spec>;
///SDIO Interrupt Flag Register
pub mod sdio_info1;
/**SDIO_INFO1_MASK (rw) register accessor: SDIO INFO1 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sdio_info1_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_info1_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_info1_mask`] module*/
#[doc(alias = "SDIO_INFO1_MASK")]
pub type SdioInfo1Mask = crate::Reg<sdio_info1_mask::SdioInfo1MaskSpec>;
///SDIO INFO1 Interrupt Mask Register
pub mod sdio_info1_mask;
/**SD_DMAEN (rw) register accessor: DMA Mode Enable Register

You can [`read`](crate::Reg::read) this register and get [`sd_dmaen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dmaen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_dmaen`] module*/
#[doc(alias = "SD_DMAEN")]
pub type SdDmaen = crate::Reg<sd_dmaen::SdDmaenSpec>;
///DMA Mode Enable Register
pub mod sd_dmaen;
/**SOFT_RST (rw) register accessor: Software Reset Register

You can [`read`](crate::Reg::read) this register and get [`soft_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@soft_rst`] module*/
#[doc(alias = "SOFT_RST")]
pub type SoftRst = crate::Reg<soft_rst::SoftRstSpec>;
///Software Reset Register
pub mod soft_rst;
/**SDIF_MODE (rw) register accessor: SD Interface Mode Setting Register

You can [`read`](crate::Reg::read) this register and get [`sdif_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdif_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdif_mode`] module*/
#[doc(alias = "SDIF_MODE")]
pub type SdifMode = crate::Reg<sdif_mode::SdifModeSpec>;
///SD Interface Mode Setting Register
pub mod sdif_mode;
/**EXT_SWAP (rw) register accessor: Swap Control Register

You can [`read`](crate::Reg::read) this register and get [`ext_swap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_swap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_swap`] module*/
#[doc(alias = "EXT_SWAP")]
pub type ExtSwap = crate::Reg<ext_swap::ExtSwapSpec>;
///Swap Control Register
pub mod ext_swap;
