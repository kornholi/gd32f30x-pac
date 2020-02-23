#[doc = "Reader of register ADDAPB1RST"]
pub type R = crate::R<u32, super::ADDAPB1RST>;
#[doc = "Writer for register ADDAPB1RST"]
pub type W = crate::W<u32, super::ADDAPB1RST>;
#[doc = "Register ADDAPB1RST `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDAPB1RST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTCRST`"]
pub type CTCRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTCRST`"]
pub struct CTCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - CTC reset"]
    #[inline(always)]
    pub fn ctcrst(&self) -> CTCRST_R {
        CTCRST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC reset"]
    #[inline(always)]
    pub fn ctcrst(&mut self) -> CTCRST_W {
        CTCRST_W { w: self }
    }
}
