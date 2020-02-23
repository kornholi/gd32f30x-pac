#[doc = "Reader of register CPSCTL"]
pub type R = crate::R<u32, super::CPSCTL>;
#[doc = "Writer for register CPSCTL"]
pub type W = crate::W<u32, super::CPSCTL>;
#[doc = "Register CPSCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CPSCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPS_RDY`"]
pub type CPS_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPS_RDY`"]
pub struct CPS_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CPS_RDY_W<'a> {
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
#[doc = "Reader of field `CPS_EN`"]
pub type CPS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPS_EN`"]
pub struct CPS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPS_EN_W<'a> {
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
    #[doc = "Bit 8 - I/O compensation cell is really or not"]
    #[inline(always)]
    pub fn cps_rdy(&self) -> CPS_RDY_R {
        CPS_RDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - I/O compensation cell enable"]
    #[inline(always)]
    pub fn cps_en(&self) -> CPS_EN_R {
        CPS_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - I/O compensation cell is really or not"]
    #[inline(always)]
    pub fn cps_rdy(&mut self) -> CPS_RDY_W {
        CPS_RDY_W { w: self }
    }
    #[doc = "Bit 0 - I/O compensation cell enable"]
    #[inline(always)]
    pub fn cps_en(&mut self) -> CPS_EN_W {
        CPS_EN_W { w: self }
    }
}
