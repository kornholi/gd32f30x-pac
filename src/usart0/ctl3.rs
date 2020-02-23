#[doc = "Reader of register CTL3"]
pub type R = crate::R<u32, super::CTL3>;
#[doc = "Writer for register CTL3"]
pub type W = crate::W<u32, super::CTL3>;
#[doc = "Register CTL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSBF`"]
pub type MSBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSBF`"]
pub struct MSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBF_W<'a> {
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
#[doc = "Reader of field `DINV`"]
pub type DINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DINV`"]
pub struct DINV_W<'a> {
    w: &'a mut W,
}
impl<'a> DINV_W<'a> {
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
#[doc = "Reader of field `TINV`"]
pub type TINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TINV`"]
pub struct TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TINV_W<'a> {
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
#[doc = "Reader of field `RINV`"]
pub type RINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RINV`"]
pub struct RINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RINV_W<'a> {
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
#[doc = "Reader of field `EBIE`"]
pub type EBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBIE`"]
pub struct EBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EBIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTIE`"]
pub type RTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTIE`"]
pub struct RTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIE_W<'a> {
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
#[doc = "Reader of field `SCRTNUM`"]
pub type SCRTNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCRTNUM`"]
pub struct SCRTNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRTNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `RTEN`"]
pub type RTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTEN`"]
pub struct RTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTEN_W<'a> {
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
    #[doc = "Bit 11 - Most significant bit first"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data bit level inversion"]
    #[inline(always)]
    pub fn dinv(&self) -> DINV_R {
        DINV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TX pin level inversion"]
    #[inline(always)]
    pub fn tinv(&self) -> TINV_R {
        TINV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RX pin level inversion"]
    #[inline(always)]
    pub fn rinv(&self) -> RINV_R {
        RINV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable bit of end of block event"]
    #[inline(always)]
    pub fn ebie(&self) -> EBIE_R {
        EBIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable bit of receive timeout event"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Smartcard auto-retry number"]
    #[inline(always)]
    pub fn scrtnum(&self) -> SCRTNUM_R {
        SCRTNUM_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rten(&self) -> RTEN_R {
        RTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Most significant bit first"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MSBF_W {
        MSBF_W { w: self }
    }
    #[doc = "Bit 10 - Data bit level inversion"]
    #[inline(always)]
    pub fn dinv(&mut self) -> DINV_W {
        DINV_W { w: self }
    }
    #[doc = "Bit 9 - TX pin level inversion"]
    #[inline(always)]
    pub fn tinv(&mut self) -> TINV_W {
        TINV_W { w: self }
    }
    #[doc = "Bit 8 - RX pin level inversion"]
    #[inline(always)]
    pub fn rinv(&mut self) -> RINV_W {
        RINV_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt enable bit of end of block event"]
    #[inline(always)]
    pub fn ebie(&mut self) -> EBIE_W {
        EBIE_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt enable bit of receive timeout event"]
    #[inline(always)]
    pub fn rtie(&mut self) -> RTIE_W {
        RTIE_W { w: self }
    }
    #[doc = "Bits 1:3 - Smartcard auto-retry number"]
    #[inline(always)]
    pub fn scrtnum(&mut self) -> SCRTNUM_W {
        SCRTNUM_W { w: self }
    }
    #[doc = "Bit 0 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rten(&mut self) -> RTEN_W {
        RTEN_W { w: self }
    }
}
