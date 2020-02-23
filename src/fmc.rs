#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - wait state counter register"]
    pub ws: WS,
    #[doc = "0x04 - Unlock key register"]
    pub key: KEY,
    #[doc = "0x08 - Option byte unlock key register"]
    pub obkey: OBKEY,
    #[doc = "0x0c - Status register 0"]
    pub stat0: STAT0,
    #[doc = "0x10 - Control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x14 - Address register 0"]
    pub addr0: ADDR0,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - Option byte control register"]
    pub obctl: OBCTL,
    #[doc = "0x20 - Erase/Program Protection register"]
    pub wpr: WPR,
    _reserved8: [u8; 32usize],
    #[doc = "0x44 - Unlock key register 1"]
    pub key1: KEY1,
    _reserved9: [u8; 4usize],
    #[doc = "0x4c - Status register 1"]
    pub stat1: STAT1,
    #[doc = "0x50 - Control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x54 - Address register 1"]
    pub addr1: ADDR1,
    _reserved12: [u8; 164usize],
    #[doc = "0xfc - Wait state enable register"]
    pub wsen: WSEN,
    #[doc = "0x100 - Product ID register"]
    pub pid: PID,
}
#[doc = "wait state counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ws](ws) module"]
pub type WS = crate::Reg<u32, _WS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WS;
#[doc = "`read()` method returns [ws::R](ws::R) reader structure"]
impl crate::Readable for WS {}
#[doc = "`write(|w| ..)` method takes [ws::W](ws::W) writer structure"]
impl crate::Writable for WS {}
#[doc = "wait state counter register"]
pub mod ws;
#[doc = "Unlock key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key](key) module"]
pub type KEY = crate::Reg<u32, _KEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY;
#[doc = "`write(|w| ..)` method takes [key::W](key::W) writer structure"]
impl crate::Writable for KEY {}
#[doc = "Unlock key register"]
pub mod key;
#[doc = "Option byte unlock key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obkey](obkey) module"]
pub type OBKEY = crate::Reg<u32, _OBKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBKEY;
#[doc = "`write(|w| ..)` method takes [obkey::W](obkey::W) writer structure"]
impl crate::Writable for OBKEY {}
#[doc = "Option byte unlock key register"]
pub mod obkey;
#[doc = "Status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat0](stat0) module"]
pub type STAT0 = crate::Reg<u32, _STAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT0;
#[doc = "`read()` method returns [stat0::R](stat0::R) reader structure"]
impl crate::Readable for STAT0 {}
#[doc = "`write(|w| ..)` method takes [stat0::W](stat0::W) writer structure"]
impl crate::Writable for STAT0 {}
#[doc = "Status register 0"]
pub mod stat0;
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u32, _CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL0;
#[doc = "`read()` method returns [ctl0::R](ctl0::R) reader structure"]
impl crate::Readable for CTL0 {}
#[doc = "`write(|w| ..)` method takes [ctl0::W](ctl0::W) writer structure"]
impl crate::Writable for CTL0 {}
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "Address register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr0](addr0) module"]
pub type ADDR0 = crate::Reg<u32, _ADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR0;
#[doc = "`write(|w| ..)` method takes [addr0::W](addr0::W) writer structure"]
impl crate::Writable for ADDR0 {}
#[doc = "Address register 0"]
pub mod addr0;
#[doc = "Option byte control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obctl](obctl) module"]
pub type OBCTL = crate::Reg<u32, _OBCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBCTL;
#[doc = "`read()` method returns [obctl::R](obctl::R) reader structure"]
impl crate::Readable for OBCTL {}
#[doc = "Option byte control register"]
pub mod obctl;
#[doc = "Erase/Program Protection register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpr](wpr) module"]
pub type WPR = crate::Reg<u32, _WPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPR;
#[doc = "`read()` method returns [wpr::R](wpr::R) reader structure"]
impl crate::Readable for WPR {}
#[doc = "Erase/Program Protection register"]
pub mod wpr;
#[doc = "Unlock key register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1](key1) module"]
pub type KEY1 = crate::Reg<u32, _KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1;
#[doc = "`write(|w| ..)` method takes [key1::W](key1::W) writer structure"]
impl crate::Writable for KEY1 {}
#[doc = "Unlock key register 1"]
pub mod key1;
#[doc = "Status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat1](stat1) module"]
pub type STAT1 = crate::Reg<u32, _STAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT1;
#[doc = "`read()` method returns [stat1::R](stat1::R) reader structure"]
impl crate::Readable for STAT1 {}
#[doc = "`write(|w| ..)` method takes [stat1::W](stat1::W) writer structure"]
impl crate::Writable for STAT1 {}
#[doc = "Status register 1"]
pub mod stat1;
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](ctl1) module"]
pub type CTL1 = crate::Reg<u32, _CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL1;
#[doc = "`read()` method returns [ctl1::R](ctl1::R) reader structure"]
impl crate::Readable for CTL1 {}
#[doc = "`write(|w| ..)` method takes [ctl1::W](ctl1::W) writer structure"]
impl crate::Writable for CTL1 {}
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "Address register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr1](addr1) module"]
pub type ADDR1 = crate::Reg<u32, _ADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR1;
#[doc = "`write(|w| ..)` method takes [addr1::W](addr1::W) writer structure"]
impl crate::Writable for ADDR1 {}
#[doc = "Address register 1"]
pub mod addr1;
#[doc = "Wait state enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wsen](wsen) module"]
pub type WSEN = crate::Reg<u32, _WSEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WSEN;
#[doc = "`read()` method returns [wsen::R](wsen::R) reader structure"]
impl crate::Readable for WSEN {}
#[doc = "`write(|w| ..)` method takes [wsen::W](wsen::W) writer structure"]
impl crate::Writable for WSEN {}
#[doc = "Wait state enable register"]
pub mod wsen;
#[doc = "Product ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid](pid) module"]
pub type PID = crate::Reg<u32, _PID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID;
#[doc = "`read()` method returns [pid::R](pid::R) reader structure"]
impl crate::Readable for PID {}
#[doc = "Product ID register"]
pub mod pid;
