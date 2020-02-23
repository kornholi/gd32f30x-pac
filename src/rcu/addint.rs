#[doc = "Reader of register ADDINT"]
pub type R = crate::R<u32, super::ADDINT>;
#[doc = "Writer for register ADDINT"]
pub type W = crate::W<u32, super::ADDINT>;
#[doc = "Register ADDINT `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRC48MSTBIF`"]
pub type IRC48MSTBIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRC48MSTBIE`"]
pub type IRC48MSTBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRC48MSTBIE`"]
pub struct IRC48MSTBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC48MSTBIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `IRC48MSTBIC`"]
pub struct IRC48MSTBIC_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC48MSTBIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - IRC48M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc48mstbif(&self) -> IRC48MSTBIF_R {
        IRC48MSTBIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc48mstbie(&self) -> IRC48MSTBIE_R {
        IRC48MSTBIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc48mstbie(&mut self) -> IRC48MSTBIE_W {
        IRC48MSTBIE_W { w: self }
    }
    #[doc = "Bit 22 - Internal 48 MHz RC oscillator Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc48mstbic(&mut self) -> IRC48MSTBIC_W {
        IRC48MSTBIC_W { w: self }
    }
}
