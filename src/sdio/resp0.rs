#[doc = "Reader of register RESP0"]
pub type R = crate::R<u32, super::RESP0>;
#[doc = "Reader of field `RESP0`"]
pub type RESP0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Card state"]
    #[inline(always)]
    pub fn resp0(&self) -> RESP0_R {
        RESP0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
