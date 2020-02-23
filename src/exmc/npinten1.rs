#[doc = "Reader of register NPINTEN1"]
pub type R = crate::R<u32, super::NPINTEN1>;
#[doc = "Writer for register NPINTEN1"]
pub type W = crate::W<u32, super::NPINTEN1>;
#[doc = "Register NPINTEN1 `reset()`'s with value 0x40"]
impl crate::ResetValue for super::NPINTEN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `FFEPT`"]
pub type FFEPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFEPT`"]
pub struct FFEPT_W<'a> {
    w: &'a mut W,
}
impl<'a> FFEPT_W<'a> {
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
#[doc = "Reader of field `INTFEN`"]
pub type INTFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTFEN`"]
pub struct INTFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTFEN_W<'a> {
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
#[doc = "Reader of field `INTHEN`"]
pub type INTHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTHEN`"]
pub struct INTHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTHEN_W<'a> {
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
#[doc = "Reader of field `INTREN`"]
pub type INTREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTREN`"]
pub struct INTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTREN_W<'a> {
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
#[doc = "Reader of field `INTFS`"]
pub type INTFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTFS`"]
pub struct INTFS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `INTHS`"]
pub type INTHS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTHS`"]
pub struct INTHS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTHS_W<'a> {
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
#[doc = "Reader of field `INTRS`"]
pub type INTRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTRS`"]
pub struct INTRS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTRS_W<'a> {
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
    #[doc = "Bit 6 - FIFO empty flag"]
    #[inline(always)]
    pub fn ffept(&self) -> FFEPT_R {
        FFEPT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable"]
    #[inline(always)]
    pub fn intfen(&self) -> INTFEN_R {
        INTFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable"]
    #[inline(always)]
    pub fn inthen(&self) -> INTHEN_R {
        INTHEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn intren(&self) -> INTREN_R {
        INTREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt falling edge status"]
    #[inline(always)]
    pub fn intfs(&self) -> INTFS_R {
        INTFS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status"]
    #[inline(always)]
    pub fn inths(&self) -> INTHS_R {
        INTHS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Interrupt rising edge status"]
    #[inline(always)]
    pub fn intrs(&self) -> INTRS_R {
        INTRS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - FIFO empty flag"]
    #[inline(always)]
    pub fn ffept(&mut self) -> FFEPT_W {
        FFEPT_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable"]
    #[inline(always)]
    pub fn intfen(&mut self) -> INTFEN_W {
        INTFEN_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable"]
    #[inline(always)]
    pub fn inthen(&mut self) -> INTHEN_W {
        INTHEN_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn intren(&mut self) -> INTREN_W {
        INTREN_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt falling edge status"]
    #[inline(always)]
    pub fn intfs(&mut self) -> INTFS_W {
        INTFS_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt high-level status"]
    #[inline(always)]
    pub fn inths(&mut self) -> INTHS_W {
        INTHS_W { w: self }
    }
    #[doc = "Bit 0 - Interrupt rising edge status"]
    #[inline(always)]
    pub fn intrs(&mut self) -> INTRS_W {
        INTRS_W { w: self }
    }
}
