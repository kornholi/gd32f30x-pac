#[doc = "Reader of register CTL1"]
pub type R = crate::R<u32, super::CTL1>;
#[doc = "Writer for register CTL1"]
pub type W = crate::W<u32, super::CTL1>;
#[doc = "Register CTL1 `reset()`'s with value 0x2022_bb7f"]
impl crate::ResetValue for super::CTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2022_bb7f
    }
}
#[doc = "Reader of field `REFPOL`"]
pub type REFPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFPOL`"]
pub struct REFPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `REFPSC`"]
pub type REFPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFPSC`"]
pub struct REFPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> REFPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `CKLIM`"]
pub type CKLIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKLIM`"]
pub struct CKLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CKLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RLVALUE`"]
pub type RLVALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RLVALUE`"]
pub struct RLVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RLVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Reference signal source polarity"]
    #[inline(always)]
    pub fn refpol(&self) -> REFPOL_R {
        REFPOL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Reference signal source selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - Reference signal source prescaler"]
    #[inline(always)]
    pub fn refpsc(&self) -> REFPSC_R {
        REFPSC_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Clock trim base limit value"]
    #[inline(always)]
    pub fn cklim(&self) -> CKLIM_R {
        CKLIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - CTC counter reload value"]
    #[inline(always)]
    pub fn rlvalue(&self) -> RLVALUE_R {
        RLVALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Reference signal source polarity"]
    #[inline(always)]
    pub fn refpol(&mut self) -> REFPOL_W {
        REFPOL_W { w: self }
    }
    #[doc = "Bits 28:29 - Reference signal source selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bits 24:26 - Reference signal source prescaler"]
    #[inline(always)]
    pub fn refpsc(&mut self) -> REFPSC_W {
        REFPSC_W { w: self }
    }
    #[doc = "Bits 16:23 - Clock trim base limit value"]
    #[inline(always)]
    pub fn cklim(&mut self) -> CKLIM_W {
        CKLIM_W { w: self }
    }
    #[doc = "Bits 0:15 - CTC counter reload value"]
    #[inline(always)]
    pub fn rlvalue(&mut self) -> RLVALUE_W {
        RLVALUE_W { w: self }
    }
}
