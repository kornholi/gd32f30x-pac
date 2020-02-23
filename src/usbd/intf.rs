#[doc = "Reader of register INTF"]
pub type R = crate::R<u32, super::INTF>;
#[doc = "Writer for register INTF"]
pub type W = crate::W<u32, super::INTF>;
#[doc = "Register INTF `reset()`'s with value 0"]
impl crate::ResetValue for super::INTF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPNUM`"]
pub type EPNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPNUM`"]
pub struct EPNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `L1REQ`"]
pub type L1REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L1REQ`"]
pub struct L1REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> L1REQ_W<'a> {
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
#[doc = "Reader of field `ESOFIF`"]
pub type ESOFIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESOFIF`"]
pub struct ESOFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOFIF_W<'a> {
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
#[doc = "Reader of field `SOFIF`"]
pub type SOFIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFIF`"]
pub struct SOFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RSTIF`"]
pub type RSTIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTIF`"]
pub struct RSTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPSIF`"]
pub type SPSIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPSIF`"]
pub struct SPSIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPSIF_W<'a> {
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
#[doc = "Reader of field `WKUPIF`"]
pub type WKUPIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPIF`"]
pub struct WKUPIF_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ERRIF`"]
pub type ERRIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRIF`"]
pub struct ERRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PMOUIF`"]
pub type PMOUIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMOUIF`"]
pub struct PMOUIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PMOUIF_W<'a> {
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
#[doc = "Reader of field `STIF`"]
pub type STIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STIF`"]
pub struct STIF_W<'a> {
    w: &'a mut W,
}
impl<'a> STIF_W<'a> {
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
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPM L1 transaction is successful"]
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt flag"]
    #[inline(always)]
    pub fn esofif(&self) -> ESOFIF_R {
        ESOFIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - start of frame interrupt flag"]
    #[inline(always)]
    pub fn sofif(&self) -> SOFIF_R {
        SOFIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - reset interrupt flag"]
    #[inline(always)]
    pub fn rstif(&self) -> RSTIF_R {
        RSTIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt flag"]
    #[inline(always)]
    pub fn spsif(&self) -> SPSIF_R {
        SPSIF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt flag"]
    #[inline(always)]
    pub fn wkupif(&self) -> WKUPIF_R {
        WKUPIF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&self) -> ERRIF_R {
        ERRIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt flag"]
    #[inline(always)]
    pub fn pmouif(&self) -> PMOUIF_R {
        PMOUIF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Successful transfer interrupt flag"]
    #[inline(always)]
    pub fn stif(&self) -> STIF_R {
        STIF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W {
        EPNUM_W { w: self }
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 7 - LPM L1 transaction is successful"]
    #[inline(always)]
    pub fn l1req(&mut self) -> L1REQ_W {
        L1REQ_W { w: self }
    }
    #[doc = "Bit 8 - Expected start of frame interrupt flag"]
    #[inline(always)]
    pub fn esofif(&mut self) -> ESOFIF_W {
        ESOFIF_W { w: self }
    }
    #[doc = "Bit 9 - start of frame interrupt flag"]
    #[inline(always)]
    pub fn sofif(&mut self) -> SOFIF_W {
        SOFIF_W { w: self }
    }
    #[doc = "Bit 10 - reset interrupt flag"]
    #[inline(always)]
    pub fn rstif(&mut self) -> RSTIF_W {
        RSTIF_W { w: self }
    }
    #[doc = "Bit 11 - Suspend mode interrupt flag"]
    #[inline(always)]
    pub fn spsif(&mut self) -> SPSIF_W {
        SPSIF_W { w: self }
    }
    #[doc = "Bit 12 - Wakeup interrupt flag"]
    #[inline(always)]
    pub fn wkupif(&mut self) -> WKUPIF_W {
        WKUPIF_W { w: self }
    }
    #[doc = "Bit 13 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&mut self) -> ERRIF_W {
        ERRIF_W { w: self }
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt flag"]
    #[inline(always)]
    pub fn pmouif(&mut self) -> PMOUIF_W {
        PMOUIF_W { w: self }
    }
    #[doc = "Bit 15 - Successful transfer interrupt flag"]
    #[inline(always)]
    pub fn stif(&mut self) -> STIF_W {
        STIF_W { w: self }
    }
}
