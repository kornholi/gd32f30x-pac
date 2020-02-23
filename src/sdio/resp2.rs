#[doc = "Reader of register RESP2"]
pub type R = crate::R<u32, super::RESP2>;
#[doc = "Reader of field `RESP2`"]
pub type RESP2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Card state"]
    #[inline(always)]
    pub fn resp2(&self) -> RESP2_R {
        RESP2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
