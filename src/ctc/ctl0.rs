#[doc = "Reader of register CTL0"]
pub type R = crate::R<u32, super::CTL0>;
#[doc = "Writer for register CTL0"]
pub type W = crate::W<u32, super::CTL0>;
#[doc = "Register CTL0 `reset()`'s with value 0x2000"]
impl crate::ResetValue for super::CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000
    }
}
#[doc = "Reader of field `CKOKIE`"]
pub type CKOKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKOKIE`"]
pub struct CKOKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOKIE_W<'a> {
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
#[doc = "Reader of field `CKWARNIE`"]
pub type CKWARNIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKWARNIE`"]
pub struct CKWARNIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKWARNIE_W<'a> {
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
#[doc = "Reader of field `ERRIE`"]
pub type ERRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRIE`"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
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
#[doc = "Reader of field `EREFIE`"]
pub type EREFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EREFIE`"]
pub struct EREFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EREFIE_W<'a> {
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
#[doc = "Reader of field `CNTEN`"]
pub type CNTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTEN`"]
pub struct CNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEN_W<'a> {
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
#[doc = "Reader of field `AUTOTRIM`"]
pub type AUTOTRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOTRIM`"]
pub struct AUTOTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOTRIM_W<'a> {
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
#[doc = "Reader of field `SWREFPUL`"]
pub type SWREFPUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWREFPUL`"]
pub struct SWREFPUL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREFPUL_W<'a> {
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
#[doc = "Reader of field `TRIMVALUE`"]
pub type TRIMVALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMVALUE`"]
pub struct TRIMVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock trim ok interrupt enable"]
    #[inline(always)]
    pub fn ckokie(&self) -> CKOKIE_R {
        CKOKIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock trim warning interrupt enable"]
    #[inline(always)]
    pub fn ckwarnie(&self) -> CKWARNIE_R {
        CKWARNIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EREFIF interrupt enable"]
    #[inline(always)]
    pub fn erefie(&self) -> EREFIE_R {
        EREFIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CTC counter enable"]
    #[inline(always)]
    pub fn cnten(&self) -> CNTEN_R {
        CNTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Hardware automatically trim mode"]
    #[inline(always)]
    pub fn autotrim(&self) -> AUTOTRIM_R {
        AUTOTRIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software reference source sync pulse"]
    #[inline(always)]
    pub fn swrefpul(&self) -> SWREFPUL_R {
        SWREFPUL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - IRC48M trim value"]
    #[inline(always)]
    pub fn trimvalue(&self) -> TRIMVALUE_R {
        TRIMVALUE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock trim ok interrupt enable"]
    #[inline(always)]
    pub fn ckokie(&mut self) -> CKOKIE_W {
        CKOKIE_W { w: self }
    }
    #[doc = "Bit 1 - Clock trim warning interrupt enable"]
    #[inline(always)]
    pub fn ckwarnie(&mut self) -> CKWARNIE_W {
        CKWARNIE_W { w: self }
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 3 - EREFIF interrupt enable"]
    #[inline(always)]
    pub fn erefie(&mut self) -> EREFIE_W {
        EREFIE_W { w: self }
    }
    #[doc = "Bit 5 - CTC counter enable"]
    #[inline(always)]
    pub fn cnten(&mut self) -> CNTEN_W {
        CNTEN_W { w: self }
    }
    #[doc = "Bit 6 - Hardware automatically trim mode"]
    #[inline(always)]
    pub fn autotrim(&mut self) -> AUTOTRIM_W {
        AUTOTRIM_W { w: self }
    }
    #[doc = "Bit 7 - Software reference source sync pulse"]
    #[inline(always)]
    pub fn swrefpul(&mut self) -> SWREFPUL_W {
        SWREFPUL_W { w: self }
    }
    #[doc = "Bits 8:13 - IRC48M trim value"]
    #[inline(always)]
    pub fn trimvalue(&mut self) -> TRIMVALUE_W {
        TRIMVALUE_W { w: self }
    }
}
