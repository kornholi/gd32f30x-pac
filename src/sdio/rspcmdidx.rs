#[doc = "Reader of register RSPCMDIDX"]
pub type R = crate::R<u32, super::RSPCMDIDX>;
#[doc = "Reader of field `RSPCMDIDX`"]
pub type RSPCMDIDX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Last response command index"]
    #[inline(always)]
    pub fn rspcmdidx(&self) -> RSPCMDIDX_R {
        RSPCMDIDX_R::new((self.bits & 0x3f) as u8)
    }
}
