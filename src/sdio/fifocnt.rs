#[doc = "Reader of register FIFOCNT"]
pub type R = crate::R<u32, super::FIFOCNT>;
#[doc = "Reader of field `FIFOCNT`"]
pub type FIFOCNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - FIFO counter"]
    #[inline(always)]
    pub fn fifocnt(&self) -> FIFOCNT_R {
        FIFOCNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
