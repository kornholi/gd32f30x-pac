#[doc = "Reader of register CFG1"]
pub type R = crate::R<u32, super::CFG1>;
#[doc = "Writer for register CFG1"]
pub type W = crate::W<u32, super::CFG1>;
#[doc = "Register CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLLPRESEL`"]
pub type PLLPRESEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLPRESEL`"]
pub struct PLLPRESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPRESEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `ADCPSC_3`"]
pub type ADCPSC_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPSC_3`"]
pub struct ADCPSC_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPSC_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllpresel(&self) -> PLLPRESEL_R {
        PLLPRESEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Bit 3 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc_3(&self) -> ADCPSC_3_R {
        ADCPSC_3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllpresel(&mut self) -> PLLPRESEL_W {
        PLLPRESEL_W { w: self }
    }
    #[doc = "Bit 29 - Bit 3 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc_3(&mut self) -> ADCPSC_3_W {
        ADCPSC_3_W { w: self }
    }
}
