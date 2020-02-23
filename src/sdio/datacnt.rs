#[doc = "Reader of register DATACNT"]
pub type R = crate::R<u32, super::DATACNT>;
#[doc = "Reader of field `DATACNT`"]
pub type DATACNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn datacnt(&self) -> DATACNT_R {
        DATACNT_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
