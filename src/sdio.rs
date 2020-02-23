#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register"]
    pub pwrctl: PWRCTL,
    #[doc = "0x04 - Clock control register"]
    pub clkctl: CLKCTL,
    #[doc = "0x08 - Command argument register"]
    pub cmdagmt: CMDAGMT,
    #[doc = "0x0c - Command control register"]
    pub cmdctl: CMDCTL,
    #[doc = "0x10 - Command index response register"]
    pub rspcmdidx: RSPCMDIDX,
    #[doc = "0x14 - Response register 0"]
    pub resp0: RESP0,
    #[doc = "0x18 - Response register 1"]
    pub resp1: RESP1,
    #[doc = "0x1c - Response register 2"]
    pub resp2: RESP2,
    #[doc = "0x20 - Response register 3"]
    pub resp3: RESP3,
    #[doc = "0x24 - Data timeout register"]
    pub datato: DATATO,
    #[doc = "0x28 - Data length register"]
    pub datalen: DATALEN,
    #[doc = "0x2c - Data control register"]
    pub datactl: DATACTL,
    #[doc = "0x30 - Data counter register"]
    pub datacnt: DATACNT,
    #[doc = "0x34 - Status register"]
    pub stat: STAT,
    #[doc = "0x38 - Interrupt clear register"]
    pub intc: INTC,
    #[doc = "0x3c - Interrupt enable register"]
    pub inten: INTEN,
    _reserved16: [u8; 8usize],
    #[doc = "0x48 - FIFO counter register"]
    pub fifocnt: FIFOCNT,
    _reserved17: [u8; 52usize],
    #[doc = "0x80 - FIFO data register"]
    pub fifo: FIFO,
}
#[doc = "Power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctl](pwrctl) module"]
pub type PWRCTL = crate::Reg<u32, _PWRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCTL;
#[doc = "`read()` method returns [pwrctl::R](pwrctl::R) reader structure"]
impl crate::Readable for PWRCTL {}
#[doc = "`write(|w| ..)` method takes [pwrctl::W](pwrctl::W) writer structure"]
impl crate::Writable for PWRCTL {}
#[doc = "Power control register"]
pub mod pwrctl;
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctl](clkctl) module"]
pub type CLKCTL = crate::Reg<u32, _CLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCTL;
#[doc = "`read()` method returns [clkctl::R](clkctl::R) reader structure"]
impl crate::Readable for CLKCTL {}
#[doc = "`write(|w| ..)` method takes [clkctl::W](clkctl::W) writer structure"]
impl crate::Writable for CLKCTL {}
#[doc = "Clock control register"]
pub mod clkctl;
#[doc = "Command argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdagmt](cmdagmt) module"]
pub type CMDAGMT = crate::Reg<u32, _CMDAGMT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDAGMT;
#[doc = "`read()` method returns [cmdagmt::R](cmdagmt::R) reader structure"]
impl crate::Readable for CMDAGMT {}
#[doc = "`write(|w| ..)` method takes [cmdagmt::W](cmdagmt::W) writer structure"]
impl crate::Writable for CMDAGMT {}
#[doc = "Command argument register"]
pub mod cmdagmt;
#[doc = "Command control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdctl](cmdctl) module"]
pub type CMDCTL = crate::Reg<u32, _CMDCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDCTL;
#[doc = "`read()` method returns [cmdctl::R](cmdctl::R) reader structure"]
impl crate::Readable for CMDCTL {}
#[doc = "`write(|w| ..)` method takes [cmdctl::W](cmdctl::W) writer structure"]
impl crate::Writable for CMDCTL {}
#[doc = "Command control register"]
pub mod cmdctl;
#[doc = "Command index response register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rspcmdidx](rspcmdidx) module"]
pub type RSPCMDIDX = crate::Reg<u32, _RSPCMDIDX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSPCMDIDX;
#[doc = "`read()` method returns [rspcmdidx::R](rspcmdidx::R) reader structure"]
impl crate::Readable for RSPCMDIDX {}
#[doc = "Command index response register"]
pub mod rspcmdidx;
#[doc = "Response register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp0](resp0) module"]
pub type RESP0 = crate::Reg<u32, _RESP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP0;
#[doc = "`read()` method returns [resp0::R](resp0::R) reader structure"]
impl crate::Readable for RESP0 {}
#[doc = "Response register 0"]
pub mod resp0;
#[doc = "Response register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp1](resp1) module"]
pub type RESP1 = crate::Reg<u32, _RESP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP1;
#[doc = "`read()` method returns [resp1::R](resp1::R) reader structure"]
impl crate::Readable for RESP1 {}
#[doc = "Response register 1"]
pub mod resp1;
#[doc = "Response register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp2](resp2) module"]
pub type RESP2 = crate::Reg<u32, _RESP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP2;
#[doc = "`read()` method returns [resp2::R](resp2::R) reader structure"]
impl crate::Readable for RESP2 {}
#[doc = "Response register 2"]
pub mod resp2;
#[doc = "Response register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp3](resp3) module"]
pub type RESP3 = crate::Reg<u32, _RESP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP3;
#[doc = "`read()` method returns [resp3::R](resp3::R) reader structure"]
impl crate::Readable for RESP3 {}
#[doc = "Response register 3"]
pub mod resp3;
#[doc = "Data timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datato](datato) module"]
pub type DATATO = crate::Reg<u32, _DATATO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATATO;
#[doc = "`read()` method returns [datato::R](datato::R) reader structure"]
impl crate::Readable for DATATO {}
#[doc = "`write(|w| ..)` method takes [datato::W](datato::W) writer structure"]
impl crate::Writable for DATATO {}
#[doc = "Data timeout register"]
pub mod datato;
#[doc = "Data length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datalen](datalen) module"]
pub type DATALEN = crate::Reg<u32, _DATALEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATALEN;
#[doc = "`read()` method returns [datalen::R](datalen::R) reader structure"]
impl crate::Readable for DATALEN {}
#[doc = "`write(|w| ..)` method takes [datalen::W](datalen::W) writer structure"]
impl crate::Writable for DATALEN {}
#[doc = "Data length register"]
pub mod datalen;
#[doc = "Data control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datactl](datactl) module"]
pub type DATACTL = crate::Reg<u32, _DATACTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATACTL;
#[doc = "`read()` method returns [datactl::R](datactl::R) reader structure"]
impl crate::Readable for DATACTL {}
#[doc = "`write(|w| ..)` method takes [datactl::W](datactl::W) writer structure"]
impl crate::Writable for DATACTL {}
#[doc = "Data control register"]
pub mod datactl;
#[doc = "Data counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datacnt](datacnt) module"]
pub type DATACNT = crate::Reg<u32, _DATACNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATACNT;
#[doc = "`read()` method returns [datacnt::R](datacnt::R) reader structure"]
impl crate::Readable for DATACNT {}
#[doc = "Data counter register"]
pub mod datacnt;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "Status register"]
pub mod stat;
#[doc = "Interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](intc) module"]
pub type INTC = crate::Reg<u32, _INTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTC;
#[doc = "`write(|w| ..)` method takes [intc::W](intc::W) writer structure"]
impl crate::Writable for INTC {}
#[doc = "Interrupt clear register"]
pub mod intc;
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "FIFO counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocnt](fifocnt) module"]
pub type FIFOCNT = crate::Reg<u32, _FIFOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCNT;
#[doc = "`read()` method returns [fifocnt::R](fifocnt::R) reader structure"]
impl crate::Readable for FIFOCNT {}
#[doc = "FIFO counter register"]
pub mod fifocnt;
#[doc = "FIFO data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "FIFO data register"]
pub mod fifo;
