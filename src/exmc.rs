#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR flash control register 0"]
    pub snctl0: SNCTL0,
    #[doc = "0x04 - SRAM/NOR flash timing configuration register 0"]
    pub sntcfg0: SNTCFG0,
    #[doc = "0x08 - SRAM/NOR flash control register 1"]
    pub snctl1: SNCTL1,
    #[doc = "0x0c - SRAM/NOR flash timing configuration register 1"]
    pub sntcfg1: SNTCFG1,
    #[doc = "0x10 - SRAM/NOR flash control register 2"]
    pub snctl2: SNCTL2,
    #[doc = "0x14 - SRAM/NOR flash timing configuration register 2"]
    pub sntcfg2: SNTCFG2,
    #[doc = "0x18 - SRAM/NOR flash control register 3"]
    pub snctl3: SNCTL3,
    #[doc = "0x1c - SRAM/NOR flash timing configuration register 3"]
    pub sntcfg3: SNTCFG3,
    _reserved8: [u8; 64usize],
    #[doc = "0x60 - NAND flash/PC card control register 1"]
    pub npctl1: NPCTL1,
    #[doc = "0x64 - NAND flash/PC card interrupt enable register 1"]
    pub npinten1: NPINTEN1,
    #[doc = "0x68 - NAND flash/PC card common space timing configuration register 1"]
    pub npctcfg1: NPCTCFG1,
    #[doc = "0x6c - NAND flash/PC card attribute space timing configuration register 1"]
    pub npatcfg1: NPATCFG1,
    _reserved12: [u8; 4usize],
    #[doc = "0x74 - NAND flash ECC register 1"]
    pub necc1: NECC1,
    _reserved13: [u8; 8usize],
    #[doc = "0x80 - NAND flash/PC card control register 2"]
    pub npctl2: NPCTL2,
    #[doc = "0x84 - NAND flash/PC card interrupt enable register 2"]
    pub npinten2: NPINTEN2,
    #[doc = "0x88 - NAND flash/PC card common space timing configuration register 2"]
    pub npctcfg2: NPCTCFG2,
    #[doc = "0x8c - NAND flash/PC card attribute space timing configuration register 2"]
    pub npatcfg2: NPATCFG2,
    _reserved17: [u8; 4usize],
    #[doc = "0x94 - NAND flash ECC register 2"]
    pub necc2: NECC2,
    _reserved18: [u8; 8usize],
    #[doc = "0xa0 - NAND flash/PC card control register 3"]
    pub npctl3: NPCTL3,
    #[doc = "0xa4 - NAND flash/PC card interrupt enable register 3"]
    pub npinten3: NPINTEN3,
    #[doc = "0xa8 - NAND flash/PC card common space timing configuration register 3"]
    pub npctcfg3: NPCTCFG3,
    #[doc = "0xac - NAND flash/PC card attribute space timing configuration register 3"]
    pub npatcfg3: NPATCFG3,
    #[doc = "0xb0 - PC card I/O space timing configuration register"]
    pub piotcfg3: PIOTCFG3,
    _reserved23: [u8; 80usize],
    #[doc = "0x104 - SRAM/NOR flash write timing configuration register 0"]
    pub snwtcfg0: SNWTCFG0,
    _reserved24: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR flash write timing configuration register 1"]
    pub snwtcfg1: SNWTCFG1,
    _reserved25: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR flash write timing configuration register 2"]
    pub snwtcfg2: SNWTCFG2,
    _reserved26: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR flash write timing configuration register 3"]
    pub snwtcfg3: SNWTCFG3,
}
#[doc = "SRAM/NOR flash control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snctl0](snctl0) module"]
pub type SNCTL0 = crate::Reg<u32, _SNCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNCTL0;
#[doc = "`read()` method returns [snctl0::R](snctl0::R) reader structure"]
impl crate::Readable for SNCTL0 {}
#[doc = "`write(|w| ..)` method takes [snctl0::W](snctl0::W) writer structure"]
impl crate::Writable for SNCTL0 {}
#[doc = "SRAM/NOR flash control register 0"]
pub mod snctl0;
#[doc = "SRAM/NOR flash timing configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sntcfg0](sntcfg0) module"]
pub type SNTCFG0 = crate::Reg<u32, _SNTCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNTCFG0;
#[doc = "`read()` method returns [sntcfg0::R](sntcfg0::R) reader structure"]
impl crate::Readable for SNTCFG0 {}
#[doc = "`write(|w| ..)` method takes [sntcfg0::W](sntcfg0::W) writer structure"]
impl crate::Writable for SNTCFG0 {}
#[doc = "SRAM/NOR flash timing configuration register 0"]
pub mod sntcfg0;
#[doc = "SRAM/NOR flash control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snctl1](snctl1) module"]
pub type SNCTL1 = crate::Reg<u32, _SNCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNCTL1;
#[doc = "`read()` method returns [snctl1::R](snctl1::R) reader structure"]
impl crate::Readable for SNCTL1 {}
#[doc = "`write(|w| ..)` method takes [snctl1::W](snctl1::W) writer structure"]
impl crate::Writable for SNCTL1 {}
#[doc = "SRAM/NOR flash control register 1"]
pub mod snctl1;
#[doc = "SRAM/NOR flash timing configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sntcfg1](sntcfg1) module"]
pub type SNTCFG1 = crate::Reg<u32, _SNTCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNTCFG1;
#[doc = "`read()` method returns [sntcfg1::R](sntcfg1::R) reader structure"]
impl crate::Readable for SNTCFG1 {}
#[doc = "`write(|w| ..)` method takes [sntcfg1::W](sntcfg1::W) writer structure"]
impl crate::Writable for SNTCFG1 {}
#[doc = "SRAM/NOR flash timing configuration register 1"]
pub mod sntcfg1;
#[doc = "SRAM/NOR flash control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snctl2](snctl2) module"]
pub type SNCTL2 = crate::Reg<u32, _SNCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNCTL2;
#[doc = "`read()` method returns [snctl2::R](snctl2::R) reader structure"]
impl crate::Readable for SNCTL2 {}
#[doc = "`write(|w| ..)` method takes [snctl2::W](snctl2::W) writer structure"]
impl crate::Writable for SNCTL2 {}
#[doc = "SRAM/NOR flash control register 2"]
pub mod snctl2;
#[doc = "SRAM/NOR flash timing configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sntcfg2](sntcfg2) module"]
pub type SNTCFG2 = crate::Reg<u32, _SNTCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNTCFG2;
#[doc = "`read()` method returns [sntcfg2::R](sntcfg2::R) reader structure"]
impl crate::Readable for SNTCFG2 {}
#[doc = "`write(|w| ..)` method takes [sntcfg2::W](sntcfg2::W) writer structure"]
impl crate::Writable for SNTCFG2 {}
#[doc = "SRAM/NOR flash timing configuration register 2"]
pub mod sntcfg2;
#[doc = "SRAM/NOR flash control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snctl3](snctl3) module"]
pub type SNCTL3 = crate::Reg<u32, _SNCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNCTL3;
#[doc = "`read()` method returns [snctl3::R](snctl3::R) reader structure"]
impl crate::Readable for SNCTL3 {}
#[doc = "`write(|w| ..)` method takes [snctl3::W](snctl3::W) writer structure"]
impl crate::Writable for SNCTL3 {}
#[doc = "SRAM/NOR flash control register 3"]
pub mod snctl3;
#[doc = "SRAM/NOR flash timing configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sntcfg3](sntcfg3) module"]
pub type SNTCFG3 = crate::Reg<u32, _SNTCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNTCFG3;
#[doc = "`read()` method returns [sntcfg3::R](sntcfg3::R) reader structure"]
impl crate::Readable for SNTCFG3 {}
#[doc = "`write(|w| ..)` method takes [sntcfg3::W](sntcfg3::W) writer structure"]
impl crate::Writable for SNTCFG3 {}
#[doc = "SRAM/NOR flash timing configuration register 3"]
pub mod sntcfg3;
#[doc = "SRAM/NOR flash write timing configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snwtcfg0](snwtcfg0) module"]
pub type SNWTCFG0 = crate::Reg<u32, _SNWTCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNWTCFG0;
#[doc = "`read()` method returns [snwtcfg0::R](snwtcfg0::R) reader structure"]
impl crate::Readable for SNWTCFG0 {}
#[doc = "`write(|w| ..)` method takes [snwtcfg0::W](snwtcfg0::W) writer structure"]
impl crate::Writable for SNWTCFG0 {}
#[doc = "SRAM/NOR flash write timing configuration register 0"]
pub mod snwtcfg0;
#[doc = "SRAM/NOR flash write timing configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snwtcfg1](snwtcfg1) module"]
pub type SNWTCFG1 = crate::Reg<u32, _SNWTCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNWTCFG1;
#[doc = "`read()` method returns [snwtcfg1::R](snwtcfg1::R) reader structure"]
impl crate::Readable for SNWTCFG1 {}
#[doc = "`write(|w| ..)` method takes [snwtcfg1::W](snwtcfg1::W) writer structure"]
impl crate::Writable for SNWTCFG1 {}
#[doc = "SRAM/NOR flash write timing configuration register 1"]
pub mod snwtcfg1;
#[doc = "SRAM/NOR flash write timing configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snwtcfg2](snwtcfg2) module"]
pub type SNWTCFG2 = crate::Reg<u32, _SNWTCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNWTCFG2;
#[doc = "`read()` method returns [snwtcfg2::R](snwtcfg2::R) reader structure"]
impl crate::Readable for SNWTCFG2 {}
#[doc = "`write(|w| ..)` method takes [snwtcfg2::W](snwtcfg2::W) writer structure"]
impl crate::Writable for SNWTCFG2 {}
#[doc = "SRAM/NOR flash write timing configuration register 2"]
pub mod snwtcfg2;
#[doc = "SRAM/NOR flash write timing configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snwtcfg3](snwtcfg3) module"]
pub type SNWTCFG3 = crate::Reg<u32, _SNWTCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNWTCFG3;
#[doc = "`read()` method returns [snwtcfg3::R](snwtcfg3::R) reader structure"]
impl crate::Readable for SNWTCFG3 {}
#[doc = "`write(|w| ..)` method takes [snwtcfg3::W](snwtcfg3::W) writer structure"]
impl crate::Writable for SNWTCFG3 {}
#[doc = "SRAM/NOR flash write timing configuration register 3"]
pub mod snwtcfg3;
#[doc = "NAND flash/PC card control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npctl1](npctl1) module"]
pub type NPCTL1 = crate::Reg<u32, _NPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPCTL1;
#[doc = "`read()` method returns [npctl1::R](npctl1::R) reader structure"]
impl crate::Readable for NPCTL1 {}
#[doc = "`write(|w| ..)` method takes [npctl1::W](npctl1::W) writer structure"]
impl crate::Writable for NPCTL1 {}
#[doc = "NAND flash/PC card control register 1"]
pub mod npctl1;
#[doc = "NAND flash/PC card control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npctl2](npctl2) module"]
pub type NPCTL2 = crate::Reg<u32, _NPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPCTL2;
#[doc = "`read()` method returns [npctl2::R](npctl2::R) reader structure"]
impl crate::Readable for NPCTL2 {}
#[doc = "`write(|w| ..)` method takes [npctl2::W](npctl2::W) writer structure"]
impl crate::Writable for NPCTL2 {}
#[doc = "NAND flash/PC card control register 2"]
pub mod npctl2;
#[doc = "NAND flash/PC card control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npctl3](npctl3) module"]
pub type NPCTL3 = crate::Reg<u32, _NPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPCTL3;
#[doc = "`read()` method returns [npctl3::R](npctl3::R) reader structure"]
impl crate::Readable for NPCTL3 {}
#[doc = "`write(|w| ..)` method takes [npctl3::W](npctl3::W) writer structure"]
impl crate::Writable for NPCTL3 {}
#[doc = "NAND flash/PC card control register 3"]
pub mod npctl3;
#[doc = "NAND flash/PC card interrupt enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npinten1](npinten1) module"]
pub type NPINTEN1 = crate::Reg<u32, _NPINTEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPINTEN1;
#[doc = "`read()` method returns [npinten1::R](npinten1::R) reader structure"]
impl crate::Readable for NPINTEN1 {}
#[doc = "`write(|w| ..)` method takes [npinten1::W](npinten1::W) writer structure"]
impl crate::Writable for NPINTEN1 {}
#[doc = "NAND flash/PC card interrupt enable register 1"]
pub mod npinten1;
#[doc = "NAND flash/PC card interrupt enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npinten2](npinten2) module"]
pub type NPINTEN2 = crate::Reg<u32, _NPINTEN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPINTEN2;
#[doc = "`read()` method returns [npinten2::R](npinten2::R) reader structure"]
impl crate::Readable for NPINTEN2 {}
#[doc = "`write(|w| ..)` method takes [npinten2::W](npinten2::W) writer structure"]
impl crate::Writable for NPINTEN2 {}
#[doc = "NAND flash/PC card interrupt enable register 2"]
pub mod npinten2;
#[doc = "NAND flash/PC card interrupt enable register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npinten3](npinten3) module"]
pub type NPINTEN3 = crate::Reg<u32, _NPINTEN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPINTEN3;
#[doc = "`read()` method returns [npinten3::R](npinten3::R) reader structure"]
impl crate::Readable for NPINTEN3 {}
#[doc = "`write(|w| ..)` method takes [npinten3::W](npinten3::W) writer structure"]
impl crate::Writable for NPINTEN3 {}
#[doc = "NAND flash/PC card interrupt enable register 3"]
pub mod npinten3;
#[doc = "NAND flash/PC card common space timing configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npctcfg1](npctcfg1) module"]
pub type NPCTCFG1 = crate::Reg<u32, _NPCTCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPCTCFG1;
#[doc = "`read()` method returns [npctcfg1::R](npctcfg1::R) reader structure"]
impl crate::Readable for NPCTCFG1 {}
#[doc = "`write(|w| ..)` method takes [npctcfg1::W](npctcfg1::W) writer structure"]
impl crate::Writable for NPCTCFG1 {}
#[doc = "NAND flash/PC card common space timing configuration register 1"]
pub mod npctcfg1;
#[doc = "NAND flash/PC card common space timing configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npctcfg2](npctcfg2) module"]
pub type NPCTCFG2 = crate::Reg<u32, _NPCTCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPCTCFG2;
#[doc = "`read()` method returns [npctcfg2::R](npctcfg2::R) reader structure"]
impl crate::Readable for NPCTCFG2 {}
#[doc = "`write(|w| ..)` method takes [npctcfg2::W](npctcfg2::W) writer structure"]
impl crate::Writable for NPCTCFG2 {}
#[doc = "NAND flash/PC card common space timing configuration register 2"]
pub mod npctcfg2;
#[doc = "NAND flash/PC card common space timing configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npctcfg3](npctcfg3) module"]
pub type NPCTCFG3 = crate::Reg<u32, _NPCTCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPCTCFG3;
#[doc = "`read()` method returns [npctcfg3::R](npctcfg3::R) reader structure"]
impl crate::Readable for NPCTCFG3 {}
#[doc = "`write(|w| ..)` method takes [npctcfg3::W](npctcfg3::W) writer structure"]
impl crate::Writable for NPCTCFG3 {}
#[doc = "NAND flash/PC card common space timing configuration register 3"]
pub mod npctcfg3;
#[doc = "NAND flash/PC card attribute space timing configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npatcfg1](npatcfg1) module"]
pub type NPATCFG1 = crate::Reg<u32, _NPATCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPATCFG1;
#[doc = "`read()` method returns [npatcfg1::R](npatcfg1::R) reader structure"]
impl crate::Readable for NPATCFG1 {}
#[doc = "`write(|w| ..)` method takes [npatcfg1::W](npatcfg1::W) writer structure"]
impl crate::Writable for NPATCFG1 {}
#[doc = "NAND flash/PC card attribute space timing configuration register 1"]
pub mod npatcfg1;
#[doc = "NAND flash/PC card attribute space timing configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npatcfg2](npatcfg2) module"]
pub type NPATCFG2 = crate::Reg<u32, _NPATCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPATCFG2;
#[doc = "`read()` method returns [npatcfg2::R](npatcfg2::R) reader structure"]
impl crate::Readable for NPATCFG2 {}
#[doc = "`write(|w| ..)` method takes [npatcfg2::W](npatcfg2::W) writer structure"]
impl crate::Writable for NPATCFG2 {}
#[doc = "NAND flash/PC card attribute space timing configuration register 2"]
pub mod npatcfg2;
#[doc = "NAND flash/PC card attribute space timing configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npatcfg3](npatcfg3) module"]
pub type NPATCFG3 = crate::Reg<u32, _NPATCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NPATCFG3;
#[doc = "`read()` method returns [npatcfg3::R](npatcfg3::R) reader structure"]
impl crate::Readable for NPATCFG3 {}
#[doc = "`write(|w| ..)` method takes [npatcfg3::W](npatcfg3::W) writer structure"]
impl crate::Writable for NPATCFG3 {}
#[doc = "NAND flash/PC card attribute space timing configuration register 3"]
pub mod npatcfg3;
#[doc = "PC card I/O space timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [piotcfg3](piotcfg3) module"]
pub type PIOTCFG3 = crate::Reg<u32, _PIOTCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIOTCFG3;
#[doc = "`read()` method returns [piotcfg3::R](piotcfg3::R) reader structure"]
impl crate::Readable for PIOTCFG3 {}
#[doc = "`write(|w| ..)` method takes [piotcfg3::W](piotcfg3::W) writer structure"]
impl crate::Writable for PIOTCFG3 {}
#[doc = "PC card I/O space timing configuration register"]
pub mod piotcfg3;
#[doc = "NAND flash ECC register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [necc1](necc1) module"]
pub type NECC1 = crate::Reg<u32, _NECC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NECC1;
#[doc = "`read()` method returns [necc1::R](necc1::R) reader structure"]
impl crate::Readable for NECC1 {}
#[doc = "NAND flash ECC register 1"]
pub mod necc1;
#[doc = "NAND flash ECC register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [necc2](necc2) module"]
pub type NECC2 = crate::Reg<u32, _NECC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NECC2;
#[doc = "`read()` method returns [necc2::R](necc2::R) reader structure"]
impl crate::Readable for NECC2 {}
#[doc = "NAND flash ECC register 2"]
pub mod necc2;
