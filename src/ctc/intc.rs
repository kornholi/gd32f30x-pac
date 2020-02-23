#[doc = "Writer for register INTC"]
pub type W = crate::W<u32, super::INTC>;
#[doc = "Register INTC `reset()`'s with value 0"]
impl crate::ResetValue for super::INTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CKOKIC`"]
pub struct CKOKIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOKIC_W<'a> {
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
#[doc = "Write proxy for field `CKWARNIC`"]
pub struct CKWARNIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKWARNIC_W<'a> {
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
#[doc = "Write proxy for field `ERRIC`"]
pub struct ERRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIC_W<'a> {
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
#[doc = "Write proxy for field `EREFIC`"]
pub struct EREFIC_W<'a> {
    w: &'a mut W,
}
impl<'a> EREFIC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - CKOKIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckokic(&mut self) -> CKOKIC_W {
        CKOKIC_W { w: self }
    }
    #[doc = "Bit 1 - CKWARNIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckwarnic(&mut self) -> CKWARNIC_W {
        CKWARNIC_W { w: self }
    }
    #[doc = "Bit 2 - ERRIF interrupt clear bit"]
    #[inline(always)]
    pub fn erric(&mut self) -> ERRIC_W {
        ERRIC_W { w: self }
    }
    #[doc = "Bit 3 - EREFIF interrupt clear bit"]
    #[inline(always)]
    pub fn erefic(&mut self) -> EREFIC_W {
        EREFIC_W { w: self }
    }
}
