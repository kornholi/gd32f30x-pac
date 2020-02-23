#[doc = "Reader of register TCRC"]
pub type R = crate::R<u32, super::TCRC>;
#[doc = "Reader of field `TCR`"]
pub type TCR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Tx CRC register"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new((self.bits & 0xffff) as u16)
    }
}
