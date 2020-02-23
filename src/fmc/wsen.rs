#[doc = "Reader of register WSEN"]
pub type R = crate::R<u32, super::WSEN>;
#[doc = "Writer for register WSEN"]
pub type W = crate::W<u32, super::WSEN>;
#[doc = "Register WSEN `reset()`'s with value 0"]
impl crate::ResetValue for super::WSEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WSEN`"]
pub type WSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WSEN`"]
pub struct WSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WSEN_W<'a> {
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
    #[doc = "Bit 0 - FMC wait state enable register"]
    #[inline(always)]
    pub fn wsen(&self) -> WSEN_R {
        WSEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMC wait state enable register"]
    #[inline(always)]
    pub fn wsen(&mut self) -> WSEN_W {
        WSEN_W { w: self }
    }
}
