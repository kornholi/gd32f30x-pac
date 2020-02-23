#[doc = "Reader of register EP7CS"]
pub type R = crate::R<u32, super::EP7CS>;
#[doc = "Writer for register EP7CS"]
pub type W = crate::W<u32, super::EP7CS>;
#[doc = "Register EP7CS `reset()`'s with value 0"]
impl crate::ResetValue for super::EP7CS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP_AR`"]
pub type EP_AR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP_AR`"]
pub struct EP_AR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_AR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TX_STA`"]
pub type TX_STA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_STA`"]
pub struct TX_STA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_STA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `TX_DTG`"]
pub type TX_DTG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DTG`"]
pub struct TX_DTG_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DTG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TX_ST`"]
pub type TX_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_ST`"]
pub struct TX_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `EP_KCTL`"]
pub type EP_KCTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP_KCTL`"]
pub struct EP_KCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_KCTL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EP_CTL`"]
pub type EP_CTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP_CTL`"]
pub struct EP_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_CTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `SETUP`"]
pub type SETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUP`"]
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RX_STA`"]
pub type RX_STA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_STA`"]
pub struct RX_STA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_STA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `RX_DTG`"]
pub type RX_DTG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DTG`"]
pub struct RX_DTG_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DTG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RX_ST`"]
pub type RX_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_ST`"]
pub struct RX_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ep_ar(&self) -> EP_AR_R {
        EP_AR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn tx_sta(&self) -> TX_STA_R {
        TX_STA_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    pub fn tx_dtg(&self) -> TX_DTG_R {
        TX_DTG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    pub fn tx_st(&self) -> TX_ST_R {
        TX_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    pub fn ep_kctl(&self) -> EP_KCTL_R {
        EP_KCTL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    pub fn ep_ctl(&self) -> EP_CTL_R {
        EP_CTL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    pub fn rx_sta(&self) -> RX_STA_R {
        RX_STA_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    pub fn rx_dtg(&self) -> RX_DTG_R {
        RX_DTG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    pub fn rx_st(&self) -> RX_ST_R {
        RX_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ep_ar(&mut self) -> EP_AR_W {
        EP_AR_W { w: self }
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn tx_sta(&mut self) -> TX_STA_W {
        TX_STA_W { w: self }
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    pub fn tx_dtg(&mut self) -> TX_DTG_W {
        TX_DTG_W { w: self }
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    pub fn tx_st(&mut self) -> TX_ST_W {
        TX_ST_W { w: self }
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    pub fn ep_kctl(&mut self) -> EP_KCTL_W {
        EP_KCTL_W { w: self }
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    pub fn ep_ctl(&mut self) -> EP_CTL_W {
        EP_CTL_W { w: self }
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    pub fn rx_sta(&mut self) -> RX_STA_W {
        RX_STA_W { w: self }
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    pub fn rx_dtg(&mut self) -> RX_DTG_W {
        RX_DTG_W { w: self }
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    pub fn rx_st(&mut self) -> RX_ST_W {
        RX_ST_W { w: self }
    }
}
