#[doc = "Reader of register ADDCTL"]
pub type R = crate::R<u32, super::ADDCTL>;
#[doc = "Writer for register ADDCTL"]
pub type W = crate::W<u32, super::ADDCTL>;
#[doc = "Register ADDCTL `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::ADDCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `CK48MSEL`"]
pub type CK48MSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CK48MSEL`"]
pub struct CK48MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CK48MSEL_W<'a> {
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
#[doc = "Reader of field `IRC48MEN`"]
pub type IRC48MEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRC48MEN`"]
pub struct IRC48MEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC48MEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `IRC48MSTB`"]
pub type IRC48MSTB_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRC48MCALIB`"]
pub type IRC48MCALIB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - 48MHz clock selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Internal 48MHz RC oscillator enable"]
    #[inline(always)]
    pub fn irc48men(&self) -> IRC48MEN_R {
        IRC48MEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Internal 48MHz RC oscillator clock stabilization Flag"]
    #[inline(always)]
    pub fn irc48mstb(&self) -> IRC48MSTB_R {
        IRC48MSTB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Internal 48MHz RC oscillator calibration value register"]
    #[inline(always)]
    pub fn irc48mcalib(&self) -> IRC48MCALIB_R {
        IRC48MCALIB_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 48MHz clock selection"]
    #[inline(always)]
    pub fn ck48msel(&mut self) -> CK48MSEL_W {
        CK48MSEL_W { w: self }
    }
    #[doc = "Bit 16 - Internal 48MHz RC oscillator enable"]
    #[inline(always)]
    pub fn irc48men(&mut self) -> IRC48MEN_W {
        IRC48MEN_W { w: self }
    }
}
