#[doc = "Reader of register RESP1"]
pub type R = crate::R<u32, super::RESP1>;
#[doc = "Reader of field `RESP1`"]
pub type RESP1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Card state"]
    #[inline(always)]
    pub fn resp1(&self) -> RESP1_R {
        RESP1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
