#[doc = "Reader of register ADDR"]
pub type R = crate::R<u32, super::ADDR>;
#[doc = "Writer for register ADDR"]
pub type W = crate::W<u32, super::ADDR>;
#[doc = "Register ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBDAR`"]
pub type USBDAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBDAR`"]
pub struct USBDAR_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `USBEN`"]
pub type USBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBEN`"]
pub struct USBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    pub fn usbdar(&self) -> USBDAR_R {
        USBDAR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    pub fn usbdar(&mut self) -> USBDAR_W {
        USBDAR_W { w: self }
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W {
        USBEN_W { w: self }
    }
}
