#[doc = "Reader of register LPMCS"]
pub type R = crate::R<u32, super::LPMCS>;
#[doc = "Writer for register LPMCS"]
pub type W = crate::W<u32, super::LPMCS>;
#[doc = "Register LPMCS `reset()`'s with value 0"]
impl crate::ResetValue for super::LPMCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLSTAT`"]
pub type BLSTAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLSTAT`"]
pub struct BLSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `REMWK`"]
pub type REMWK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REMWK`"]
pub struct REMWK_W<'a> {
    w: &'a mut W,
}
impl<'a> REMWK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `LPMACK`"]
pub type LPMACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPMACK`"]
pub struct LPMACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `LPMEN`"]
pub type LPMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPMEN`"]
pub struct LPMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - bLinkState value"]
    #[inline(always)]
    pub fn blstat(&self) -> BLSTAT_R {
        BLSTAT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwk(&self) -> REMWK_R {
        REMWK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - bLinkState value"]
    #[inline(always)]
    pub fn blstat(&mut self) -> BLSTAT_W {
        BLSTAT_W { w: self }
    }
    #[doc = "Bit 3 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwk(&mut self) -> REMWK_W {
        REMWK_W { w: self }
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W {
        LPMACK_W { w: self }
    }
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W {
        LPMEN_W { w: self }
    }
}
