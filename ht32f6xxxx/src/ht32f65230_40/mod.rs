#![doc = "Peripheral access API for HT32F65230_40 microcontrollers (generated using svd2rust v0.29.0 (615f093 2023-06-05))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.29.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
extern "C" {
    fn RTC();
    fn FMC();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_9();
    fn EXTI10_15();
    fn ADCA();
    fn ADCB();
    fn MCTM();
    fn GPTM();
    fn BFTM0();
    fn BFTM1();
    fn CMP0();
    fn I2C();
    fn SPI();
    fn USART();
    fn UART();
    fn PDMACH0_1();
    fn PDMA_CH2_3();
    fn PDMA_CH4_5();
    fn SCTM0();
    fn SCTM1();
    fn SCTM2();
    fn SCTM3();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _reserved: 0 },
    Vector { _handler: RTC },
    Vector { _handler: FMC },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_9 },
    Vector {
        _handler: EXTI10_15,
    },
    Vector { _handler: ADCA },
    Vector { _handler: ADCB },
    Vector { _handler: MCTM },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPTM },
    Vector { _reserved: 0 },
    Vector { _handler: BFTM0 },
    Vector { _handler: BFTM1 },
    Vector { _handler: CMP0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C },
    Vector { _handler: SPI },
    Vector { _handler: USART },
    Vector { _handler: UART },
    Vector {
        _handler: PDMACH0_1,
    },
    Vector {
        _handler: PDMA_CH2_3,
    },
    Vector {
        _handler: PDMA_CH4_5,
    },
    Vector { _handler: SCTM0 },
    Vector { _handler: SCTM1 },
    Vector { _handler: SCTM2 },
    Vector { _handler: SCTM3 },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "1 - RTC interrupt"]
    RTC = 1,
    #[doc = "2 - FMC interrupt"]
    FMC = 2,
    #[doc = "4 - EXTI0_1 interrupt"]
    EXTI0_1 = 4,
    #[doc = "5 - EXTI2_3 interrupt"]
    EXTI2_3 = 5,
    #[doc = "6 - EXTI4_9 interrupt"]
    EXTI4_9 = 6,
    #[doc = "7 - EXTI10_15 interrupt"]
    EXTI10_15 = 7,
    #[doc = "8 - ADCA interrupt"]
    ADCA = 8,
    #[doc = "9 - ADCB interrupt"]
    ADCB = 9,
    #[doc = "10 - MCTM interrupt"]
    MCTM = 10,
    #[doc = "14 - GPTM interrupt"]
    GPTM = 14,
    #[doc = "16 - BFTM0 interrupt"]
    BFTM0 = 16,
    #[doc = "17 - BFTM1 interrupt"]
    BFTM1 = 17,
    #[doc = "18 - CMP0 interrupt"]
    CMP0 = 18,
    #[doc = "21 - I2C interrupt"]
    I2C = 21,
    #[doc = "22 - SPI interrupt"]
    SPI = 22,
    #[doc = "23 - USART interrupt"]
    USART = 23,
    #[doc = "24 - UART interrupt"]
    UART = 24,
    #[doc = "25 - PDMACH0_1 interrupt"]
    PDMACH0_1 = 25,
    #[doc = "26 - PDMA_CH2_3 interrupt"]
    PDMA_CH2_3 = 26,
    #[doc = "27 - PDMA_CH4_5 interrupt"]
    PDMA_CH4_5 = 27,
    #[doc = "28 - SCTM0 interrupt"]
    SCTM0 = 28,
    #[doc = "29 - SCTM1 interrupt"]
    SCTM1 = 29,
    #[doc = "30 - SCTM2 interrupt"]
    SCTM2 = 30,
    #[doc = "31 - SCTM3 interrupt"]
    SCTM3 = 31,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "SysTick"]
pub struct SYS_TICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS_TICK {}
impl SYS_TICK {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sys_tick::RegisterBlock = 0xe000_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_tick::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYS_TICK {
    type Target = sys_tick::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYS_TICK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_TICK").finish()
    }
}
#[doc = "SysTick"]
pub mod sys_tick;
#[doc = "EXTI"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const exti::RegisterBlock = 0x4002_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EXTI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI").finish()
    }
}
#[doc = "EXTI"]
pub mod exti;
#[doc = "PDMA"]
pub struct PDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA {}
impl PDMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pdma::RegisterBlock = 0x4009_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PDMA {
    type Target = pdma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PDMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDMA").finish()
    }
}
#[doc = "PDMA"]
pub mod pdma;
#[doc = "USART"]
pub struct USART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART {}
impl USART {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART {
    type Target = usart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART").finish()
    }
}
#[doc = "USART"]
pub mod usart;
#[doc = "UART"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart::RegisterBlock = 0x4000_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART").finish()
    }
}
#[doc = "UART"]
pub mod uart;
#[doc = "SPI"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi::RegisterBlock = 0x4000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI").finish()
    }
}
#[doc = "SPI"]
pub mod spi;
#[doc = "ADCA"]
pub struct ADCA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADCA {}
impl ADCA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adca::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adca::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADCA {
    type Target = adca::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADCA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCA").finish()
    }
}
#[doc = "ADCA"]
pub mod adca;
#[doc = "OPA0"]
pub struct OPA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OPA0 {}
impl OPA0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const opa0::RegisterBlock = 0x4001_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const opa0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for OPA0 {
    type Target = opa0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for OPA0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPA0").finish()
    }
}
#[doc = "OPA0"]
pub mod opa0;
#[doc = "AFIO"]
pub struct AFIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFIO {}
impl AFIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const afio::RegisterBlock = 0x4002_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const afio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AFIO {
    type Target = afio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AFIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFIO").finish()
    }
}
#[doc = "AFIO"]
pub mod afio;
#[doc = "MCTM"]
pub struct MCTM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCTM {}
impl MCTM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const mctm::RegisterBlock = 0x4002_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mctm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MCTM {
    type Target = mctm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MCTM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCTM").finish()
    }
}
#[doc = "MCTM"]
pub mod mctm;
#[doc = "SCTM0"]
pub struct SCTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCTM0 {}
impl SCTM0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sctm0::RegisterBlock = 0x4003_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sctm0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCTM0 {
    type Target = sctm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCTM0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCTM0").finish()
    }
}
#[doc = "SCTM0"]
pub mod sctm0;
#[doc = "SCTM2"]
pub struct SCTM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCTM2 {}
impl SCTM2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sctm2::RegisterBlock = 0x4003_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sctm2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCTM2 {
    type Target = sctm2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCTM2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCTM2").finish()
    }
}
#[doc = "SCTM2"]
pub mod sctm2;
#[doc = "I2C"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c::RegisterBlock = 0x4004_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C").finish()
    }
}
#[doc = "I2C"]
pub mod i2c;
#[doc = "ADCB"]
pub struct ADCB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADCB {}
impl ADCB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adcb::RegisterBlock = 0x4005_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adcb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADCB {
    type Target = adcb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADCB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCB").finish()
    }
}
#[doc = "ADCB"]
pub mod adcb;
#[doc = "CMP0"]
pub struct CMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP0 {}
impl CMP0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cmp0::RegisterBlock = 0x4005_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CMP0 {
    type Target = cmp0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CMP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP0").finish()
    }
}
#[doc = "CMP0"]
pub mod cmp0;
#[doc = "WDT"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdt::RegisterBlock = 0x4006_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT").finish()
    }
}
#[doc = "WDT"]
pub mod wdt;
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x4006_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "PWRCU"]
pub struct PWRCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWRCU {}
impl PWRCU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwrcu::RegisterBlock = 0x4006_a100 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwrcu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWRCU {
    type Target = pwrcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWRCU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRCU").finish()
    }
}
#[doc = "PWRCU"]
pub mod pwrcu;
#[doc = "GPTM"]
pub struct GPTM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTM {}
impl GPTM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gptm::RegisterBlock = 0x4006_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gptm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPTM {
    type Target = gptm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPTM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPTM").finish()
    }
}
#[doc = "GPTM"]
pub mod gptm;
#[doc = "SCTM1"]
pub struct SCTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCTM1 {}
impl SCTM1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sctm1::RegisterBlock = 0x4007_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sctm1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCTM1 {
    type Target = sctm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCTM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCTM1").finish()
    }
}
#[doc = "SCTM1"]
pub mod sctm1;
#[doc = "SCTM3"]
pub struct SCTM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCTM3 {}
impl SCTM3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sctm3::RegisterBlock = 0x4007_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sctm3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCTM3 {
    type Target = sctm3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCTM3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCTM3").finish()
    }
}
#[doc = "SCTM3"]
pub mod sctm3;
#[doc = "BFTM0"]
pub struct BFTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BFTM0 {}
impl BFTM0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bftm0::RegisterBlock = 0x4007_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bftm0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BFTM0 {
    type Target = bftm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BFTM0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BFTM0").finish()
    }
}
#[doc = "BFTM0"]
pub mod bftm0;
#[doc = "BFTM1"]
pub struct BFTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BFTM1 {}
impl BFTM1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bftm1::RegisterBlock = 0x4007_7000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bftm1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BFTM1 {
    type Target = bftm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BFTM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BFTM1").finish()
    }
}
#[doc = "BFTM1"]
pub mod bftm1;
#[doc = "FMC"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const fmc::RegisterBlock = 0x4008_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FMC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC").finish()
    }
}
#[doc = "FMC"]
pub mod fmc;
#[doc = "CKCU"]
pub struct CKCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CKCU {}
impl CKCU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ckcu::RegisterBlock = 0x4008_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ckcu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CKCU {
    type Target = ckcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CKCU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKCU").finish()
    }
}
#[doc = "CKCU"]
pub mod ckcu;
#[doc = "RSTCU"]
pub struct RSTCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTCU {}
impl RSTCU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rstcu::RegisterBlock = 0x4008_8100 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstcu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RSTCU {
    type Target = rstcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RSTCU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTCU").finish()
    }
}
#[doc = "RSTCU"]
pub mod rstcu;
#[doc = "CRC"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crc::RegisterBlock = 0x4008_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC").finish()
    }
}
#[doc = "CRC"]
pub mod crc;
#[doc = "GPIOA"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpioa::RegisterBlock = 0x400b_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOA").finish()
    }
}
#[doc = "GPIOA"]
pub mod gpioa;
#[doc = "GPIOB"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiob::RegisterBlock = 0x400b_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOB").finish()
    }
}
#[doc = "GPIOB"]
pub mod gpiob;
#[doc = "GPIOC"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpioc::RegisterBlock = 0x400b_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOC").finish()
    }
}
#[doc = "GPIOC"]
pub mod gpioc;
#[doc = "DIV"]
pub struct DIV {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DIV {}
impl DIV {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const div::RegisterBlock = 0x400c_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const div::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DIV {
    type Target = div::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIV").finish()
    }
}
#[doc = "DIV"]
pub mod div;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SYS_TICK"]
    pub SYS_TICK: SYS_TICK,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "PDMA"]
    pub PDMA: PDMA,
    #[doc = "USART"]
    pub USART: USART,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "ADCA"]
    pub ADCA: ADCA,
    #[doc = "OPA0"]
    pub OPA0: OPA0,
    #[doc = "AFIO"]
    pub AFIO: AFIO,
    #[doc = "MCTM"]
    pub MCTM: MCTM,
    #[doc = "SCTM0"]
    pub SCTM0: SCTM0,
    #[doc = "SCTM2"]
    pub SCTM2: SCTM2,
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "ADCB"]
    pub ADCB: ADCB,
    #[doc = "CMP0"]
    pub CMP0: CMP0,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "PWRCU"]
    pub PWRCU: PWRCU,
    #[doc = "GPTM"]
    pub GPTM: GPTM,
    #[doc = "SCTM1"]
    pub SCTM1: SCTM1,
    #[doc = "SCTM3"]
    pub SCTM3: SCTM3,
    #[doc = "BFTM0"]
    pub BFTM0: BFTM0,
    #[doc = "BFTM1"]
    pub BFTM1: BFTM1,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "CKCU"]
    pub CKCU: CKCU,
    #[doc = "RSTCU"]
    pub RSTCU: RSTCU,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "DIV"]
    pub DIV: DIV,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            SYS_TICK: SYS_TICK {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            PDMA: PDMA {
                _marker: PhantomData,
            },
            USART: USART {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            ADCA: ADCA {
                _marker: PhantomData,
            },
            OPA0: OPA0 {
                _marker: PhantomData,
            },
            AFIO: AFIO {
                _marker: PhantomData,
            },
            MCTM: MCTM {
                _marker: PhantomData,
            },
            SCTM0: SCTM0 {
                _marker: PhantomData,
            },
            SCTM2: SCTM2 {
                _marker: PhantomData,
            },
            I2C: I2C {
                _marker: PhantomData,
            },
            ADCB: ADCB {
                _marker: PhantomData,
            },
            CMP0: CMP0 {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            PWRCU: PWRCU {
                _marker: PhantomData,
            },
            GPTM: GPTM {
                _marker: PhantomData,
            },
            SCTM1: SCTM1 {
                _marker: PhantomData,
            },
            SCTM3: SCTM3 {
                _marker: PhantomData,
            },
            BFTM0: BFTM0 {
                _marker: PhantomData,
            },
            BFTM1: BFTM1 {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            CKCU: CKCU {
                _marker: PhantomData,
            },
            RSTCU: RSTCU {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            DIV: DIV {
                _marker: PhantomData,
            },
        }
    }
}
