#[doc = "Reader of register RCRC"]
pub type R = crate::R<u32, super::RCRC>;
#[doc = "Reader of field `RCR`"]
pub type RCR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - RX CRC register"]
    #[inline(always)]
    pub fn rcr(&self) -> RCR_R {
        RCR_R::new((self.bits & 0xffff) as u16)
    }
}
