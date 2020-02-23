#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `FCNT`"]
pub type FCNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `SOFLN`"]
pub type SOFLN_R = crate::R<u8, u8>;
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_DM`"]
pub type RX_DM_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_DP`"]
pub type RX_DP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:10 - Frame number counter"]
    #[inline(always)]
    pub fn fcnt(&self) -> FCNT_R {
        FCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - Lost SOF number"]
    #[inline(always)]
    pub fn sofln(&self) -> SOFLN_R {
        SOFLN_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Locked the USB"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive data - line status"]
    #[inline(always)]
    pub fn rx_dm(&self) -> RX_DM_R {
        RX_DM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive data + line status"]
    #[inline(always)]
    pub fn rx_dp(&self) -> RX_DP_R {
        RX_DP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
