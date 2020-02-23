#[doc = "Reader of register PCF1"]
pub type R = crate::R<u32, super::PCF1>;
#[doc = "Writer for register PCF1"]
pub type W = crate::W<u32, super::PCF1>;
#[doc = "Register PCF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTC_REMAP`"]
pub type CTC_REMAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTC_REMAP`"]
pub struct CTC_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTC_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `FSMC_NADV`"]
pub type FSMC_NADV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSMC_NADV`"]
pub struct FSMC_NADV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMC_NADV_W<'a> {
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
#[doc = "Reader of field `TIMER13_REMAP`"]
pub type TIMER13_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER13_REMAP`"]
pub struct TIMER13_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER13_REMAP_W<'a> {
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
#[doc = "Reader of field `TIMER12_REMAP`"]
pub type TIMER12_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER12_REMAP`"]
pub struct TIMER12_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER12_REMAP_W<'a> {
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
#[doc = "Reader of field `TIMER10_REMAP`"]
pub type TIMER10_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER10_REMAP`"]
pub struct TIMER10_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER10_REMAP_W<'a> {
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
#[doc = "Reader of field `TIMER9_REMAP`"]
pub type TIMER9_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER9_REMAP`"]
pub struct TIMER9_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER9_REMAP_W<'a> {
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
#[doc = "Reader of field `TIMER8_REMAP`"]
pub type TIMER8_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER8_REMAP`"]
pub struct TIMER8_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER8_REMAP_W<'a> {
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
impl R {
    #[doc = "Bits 11:12 - CTC remapping"]
    #[inline(always)]
    pub fn ctc_remap(&self) -> CTC_REMAP_R {
        CTC_REMAP_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10 - FSMC_NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FSMC_NADV_R {
        FSMC_NADV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TIMER13 remapping"]
    #[inline(always)]
    pub fn timer13_remap(&self) -> TIMER13_REMAP_R {
        TIMER13_REMAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIMER12 remapping"]
    #[inline(always)]
    pub fn timer12_remap(&self) -> TIMER12_REMAP_R {
        TIMER12_REMAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIMER10 remapping"]
    #[inline(always)]
    pub fn timer10_remap(&self) -> TIMER10_REMAP_R {
        TIMER10_REMAP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TIMER9 remapping"]
    #[inline(always)]
    pub fn timer9_remap(&self) -> TIMER9_REMAP_R {
        TIMER9_REMAP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIMER8 remapping"]
    #[inline(always)]
    pub fn timer8_remap(&self) -> TIMER8_REMAP_R {
        TIMER8_REMAP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:12 - CTC remapping"]
    #[inline(always)]
    pub fn ctc_remap(&mut self) -> CTC_REMAP_W {
        CTC_REMAP_W { w: self }
    }
    #[doc = "Bit 10 - FSMC_NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&mut self) -> FSMC_NADV_W {
        FSMC_NADV_W { w: self }
    }
    #[doc = "Bit 9 - TIMER13 remapping"]
    #[inline(always)]
    pub fn timer13_remap(&mut self) -> TIMER13_REMAP_W {
        TIMER13_REMAP_W { w: self }
    }
    #[doc = "Bit 8 - TIMER12 remapping"]
    #[inline(always)]
    pub fn timer12_remap(&mut self) -> TIMER12_REMAP_W {
        TIMER12_REMAP_W { w: self }
    }
    #[doc = "Bit 7 - TIMER10 remapping"]
    #[inline(always)]
    pub fn timer10_remap(&mut self) -> TIMER10_REMAP_W {
        TIMER10_REMAP_W { w: self }
    }
    #[doc = "Bit 6 - TIMER9 remapping"]
    #[inline(always)]
    pub fn timer9_remap(&mut self) -> TIMER9_REMAP_W {
        TIMER9_REMAP_W { w: self }
    }
    #[doc = "Bit 5 - TIMER8 remapping"]
    #[inline(always)]
    pub fn timer8_remap(&mut self) -> TIMER8_REMAP_W {
        TIMER8_REMAP_W { w: self }
    }
}
