#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - CSR"]
    pub csr: CSR,
    #[doc = "0x14 - RVR"]
    pub rvr: RVR,
    #[doc = "0x18 - CVR"]
    pub cvr: CVR,
    #[doc = "0x1c - CALIB"]
    pub calib: CALIB,
}
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "CSR"]
pub mod csr;
#[doc = "RVR (rw) register accessor: an alias for `Reg<RVR_SPEC>`"]
pub type RVR = crate::Reg<rvr::RVR_SPEC>;
#[doc = "RVR"]
pub mod rvr;
#[doc = "CVR (rw) register accessor: an alias for `Reg<CVR_SPEC>`"]
pub type CVR = crate::Reg<cvr::CVR_SPEC>;
#[doc = "CVR"]
pub mod cvr;
#[doc = "CALIB (rw) register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "CALIB"]
pub mod calib;
