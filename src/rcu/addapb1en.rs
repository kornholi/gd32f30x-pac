#[doc = "Reader of register ADDAPB1EN"]
pub type R = crate::R<u32, super::ADDAPB1EN>;
#[doc = "Writer for register ADDAPB1EN"]
pub type W = crate::W<u32, super::ADDAPB1EN>;
#[doc = "Register ADDAPB1EN `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDAPB1EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTCEN`"]
pub type CTCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTCEN`"]
pub struct CTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCEN_W<'a> {
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
    #[doc = "Bit 27 - CTC clock enable"]
    #[inline(always)]
    pub fn ctcen(&self) -> CTCEN_R {
        CTCEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC clock enable"]
    #[inline(always)]
    pub fn ctcen(&mut self) -> CTCEN_W {
        CTCEN_W { w: self }
    }
}
