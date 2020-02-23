#[doc = "Reader of register CLKCTL"]
pub type R = crate::R<u32, super::CLKCTL>;
#[doc = "Writer for register CLKCTL"]
pub type W = crate::W<u32, super::CLKCTL>;
#[doc = "Register CLKCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIV_8`"]
pub type DIV_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIV_8`"]
pub struct DIV_8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_8_W<'a> {
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
#[doc = "Reader of field `HWCLKEN`"]
pub type HWCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HWCLKEN`"]
pub struct HWCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HWCLKEN_W<'a> {
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
#[doc = "Reader of field `CLKEDGE`"]
pub type CLKEDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEDGE`"]
pub struct CLKEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `BUSMODE`"]
pub type BUSMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUSMODE`"]
pub struct BUSMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `CLKBYP`"]
pub type CLKBYP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKBYP`"]
pub struct CLKBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CLKPWRSAV`"]
pub type CLKPWRSAV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKPWRSAV`"]
pub struct CLKPWRSAV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPWRSAV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CLKEN`"]
pub type CLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN`"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
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
#[doc = "Reader of field `DIV_0_7`"]
pub type DIV_0_7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_0_7`"]
pub struct DIV_0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_0_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - MSB of Clock division"]
    #[inline(always)]
    pub fn div_8(&self) -> DIV_8_R {
        DIV_8_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Hardware Clock Control enable bit"]
    #[inline(always)]
    pub fn hwclken(&self) -> HWCLKEN_R {
        HWCLKEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SDIO_CLK clock edge selection bit"]
    #[inline(always)]
    pub fn clkedge(&self) -> CLKEDGE_R {
        CLKEDGE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - SDIO card bus mode control bit"]
    #[inline(always)]
    pub fn busmode(&self) -> BUSMODE_R {
        BUSMODE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Clock bypass enable bit"]
    #[inline(always)]
    pub fn clkbyp(&self) -> CLKBYP_R {
        CLKBYP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SDIO_CLK clock dynamic switch on/off for power saving"]
    #[inline(always)]
    pub fn clkpwrsav(&self) -> CLKPWRSAV_R {
        CLKPWRSAV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SDIO_CLK clock output enable bit"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    pub fn div_0_7(&self) -> DIV_0_7_R {
        DIV_0_7_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - MSB of Clock division"]
    #[inline(always)]
    pub fn div_8(&mut self) -> DIV_8_W {
        DIV_8_W { w: self }
    }
    #[doc = "Bit 14 - Hardware Clock Control enable bit"]
    #[inline(always)]
    pub fn hwclken(&mut self) -> HWCLKEN_W {
        HWCLKEN_W { w: self }
    }
    #[doc = "Bit 13 - SDIO_CLK clock edge selection bit"]
    #[inline(always)]
    pub fn clkedge(&mut self) -> CLKEDGE_W {
        CLKEDGE_W { w: self }
    }
    #[doc = "Bits 11:12 - SDIO card bus mode control bit"]
    #[inline(always)]
    pub fn busmode(&mut self) -> BUSMODE_W {
        BUSMODE_W { w: self }
    }
    #[doc = "Bit 10 - Clock bypass enable bit"]
    #[inline(always)]
    pub fn clkbyp(&mut self) -> CLKBYP_W {
        CLKBYP_W { w: self }
    }
    #[doc = "Bit 9 - SDIO_CLK clock dynamic switch on/off for power saving"]
    #[inline(always)]
    pub fn clkpwrsav(&mut self) -> CLKPWRSAV_W {
        CLKPWRSAV_W { w: self }
    }
    #[doc = "Bit 8 - SDIO_CLK clock output enable bit"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    pub fn div_0_7(&mut self) -> DIV_0_7_W {
        DIV_0_7_W { w: self }
    }
}
