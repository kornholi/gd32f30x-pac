#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ep0cs: EP0CS,
    #[doc = "0x04 - endpoint 1 register"]
    pub ep1cs: EP1CS,
    #[doc = "0x08 - endpoint 2 register"]
    pub ep2cs: EP2CS,
    #[doc = "0x0c - endpoint 3 register"]
    pub ep3cs: EP3CS,
    #[doc = "0x10 - endpoint 4 register"]
    pub ep4cs: EP4CS,
    #[doc = "0x14 - endpoint 5 register"]
    pub ep5cs: EP5CS,
    #[doc = "0x18 - endpoint 6 register"]
    pub ep6cs: EP6CS,
    #[doc = "0x1c - endpoint 7 register"]
    pub ep7cs: EP7CS,
    _reserved8: [u8; 32usize],
    #[doc = "0x40 - control register"]
    pub ctl: CTL,
    #[doc = "0x44 - interrupt flag register"]
    pub intf: INTF,
    #[doc = "0x48 - Status register"]
    pub stat: STAT,
    #[doc = "0x4c - device address register"]
    pub addr: ADDR,
    #[doc = "0x50 - Buffer address register"]
    pub baddr: BADDR,
    #[doc = "0x54 - USB LPM control and status register"]
    pub lpmcs: LPMCS,
}
#[doc = "endpoint 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0cs](ep0cs) module"]
pub type EP0CS = crate::Reg<u32, _EP0CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0CS;
#[doc = "`read()` method returns [ep0cs::R](ep0cs::R) reader structure"]
impl crate::Readable for EP0CS {}
#[doc = "`write(|w| ..)` method takes [ep0cs::W](ep0cs::W) writer structure"]
impl crate::Writable for EP0CS {}
#[doc = "endpoint 0 register"]
pub mod ep0cs;
#[doc = "endpoint 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1cs](ep1cs) module"]
pub type EP1CS = crate::Reg<u32, _EP1CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1CS;
#[doc = "`read()` method returns [ep1cs::R](ep1cs::R) reader structure"]
impl crate::Readable for EP1CS {}
#[doc = "`write(|w| ..)` method takes [ep1cs::W](ep1cs::W) writer structure"]
impl crate::Writable for EP1CS {}
#[doc = "endpoint 1 register"]
pub mod ep1cs;
#[doc = "endpoint 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2cs](ep2cs) module"]
pub type EP2CS = crate::Reg<u32, _EP2CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2CS;
#[doc = "`read()` method returns [ep2cs::R](ep2cs::R) reader structure"]
impl crate::Readable for EP2CS {}
#[doc = "`write(|w| ..)` method takes [ep2cs::W](ep2cs::W) writer structure"]
impl crate::Writable for EP2CS {}
#[doc = "endpoint 2 register"]
pub mod ep2cs;
#[doc = "endpoint 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3cs](ep3cs) module"]
pub type EP3CS = crate::Reg<u32, _EP3CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3CS;
#[doc = "`read()` method returns [ep3cs::R](ep3cs::R) reader structure"]
impl crate::Readable for EP3CS {}
#[doc = "`write(|w| ..)` method takes [ep3cs::W](ep3cs::W) writer structure"]
impl crate::Writable for EP3CS {}
#[doc = "endpoint 3 register"]
pub mod ep3cs;
#[doc = "endpoint 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4cs](ep4cs) module"]
pub type EP4CS = crate::Reg<u32, _EP4CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4CS;
#[doc = "`read()` method returns [ep4cs::R](ep4cs::R) reader structure"]
impl crate::Readable for EP4CS {}
#[doc = "`write(|w| ..)` method takes [ep4cs::W](ep4cs::W) writer structure"]
impl crate::Writable for EP4CS {}
#[doc = "endpoint 4 register"]
pub mod ep4cs;
#[doc = "endpoint 5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5cs](ep5cs) module"]
pub type EP5CS = crate::Reg<u32, _EP5CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5CS;
#[doc = "`read()` method returns [ep5cs::R](ep5cs::R) reader structure"]
impl crate::Readable for EP5CS {}
#[doc = "`write(|w| ..)` method takes [ep5cs::W](ep5cs::W) writer structure"]
impl crate::Writable for EP5CS {}
#[doc = "endpoint 5 register"]
pub mod ep5cs;
#[doc = "endpoint 6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6cs](ep6cs) module"]
pub type EP6CS = crate::Reg<u32, _EP6CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6CS;
#[doc = "`read()` method returns [ep6cs::R](ep6cs::R) reader structure"]
impl crate::Readable for EP6CS {}
#[doc = "`write(|w| ..)` method takes [ep6cs::W](ep6cs::W) writer structure"]
impl crate::Writable for EP6CS {}
#[doc = "endpoint 6 register"]
pub mod ep6cs;
#[doc = "endpoint 7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7cs](ep7cs) module"]
pub type EP7CS = crate::Reg<u32, _EP7CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7CS;
#[doc = "`read()` method returns [ep7cs::R](ep7cs::R) reader structure"]
impl crate::Readable for EP7CS {}
#[doc = "`write(|w| ..)` method takes [ep7cs::W](ep7cs::W) writer structure"]
impl crate::Writable for EP7CS {}
#[doc = "endpoint 7 register"]
pub mod ep7cs;
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "control register"]
pub mod ctl;
#[doc = "interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](intf) module"]
pub type INTF = crate::Reg<u32, _INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF;
#[doc = "`read()` method returns [intf::R](intf::R) reader structure"]
impl crate::Readable for INTF {}
#[doc = "`write(|w| ..)` method takes [intf::W](intf::W) writer structure"]
impl crate::Writable for INTF {}
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "Status register"]
pub mod stat;
#[doc = "device address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "device address register"]
pub mod addr;
#[doc = "Buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baddr](baddr) module"]
pub type BADDR = crate::Reg<u32, _BADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BADDR;
#[doc = "`read()` method returns [baddr::R](baddr::R) reader structure"]
impl crate::Readable for BADDR {}
#[doc = "`write(|w| ..)` method takes [baddr::W](baddr::W) writer structure"]
impl crate::Writable for BADDR {}
#[doc = "Buffer address register"]
pub mod baddr;
#[doc = "USB LPM control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcs](lpmcs) module"]
pub type LPMCS = crate::Reg<u32, _LPMCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMCS;
#[doc = "`read()` method returns [lpmcs::R](lpmcs::R) reader structure"]
impl crate::Readable for LPMCS {}
#[doc = "`write(|w| ..)` method takes [lpmcs::W](lpmcs::W) writer structure"]
impl crate::Writable for LPMCS {}
#[doc = "USB LPM control and status register"]
pub mod lpmcs;
