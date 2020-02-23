#[doc = "Reader of register RESP3"]
pub type R = crate::R<u32, super::RESP3>;
#[doc = "Reader of field `RESP3`"]
pub type RESP3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Response register 3"]
    #[inline(always)]
    pub fn resp3(&self) -> RESP3_R {
        RESP3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
