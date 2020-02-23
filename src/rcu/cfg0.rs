#[doc = "Reader of register CFG0"]
pub type R = crate::R<u32, super::CFG0>;
#[doc = "Writer for register CFG0"]
pub type W = crate::W<u32, super::CFG0>;
#[doc = "Register CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCS`"]
pub type SCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCS`"]
pub struct SCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SCSS`"]
pub type SCSS_R = crate::R<u8, u8>;
#[doc = "Reader of field `AHBPSC`"]
pub type AHBPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHBPSC`"]
pub struct AHBPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `APB1PSC`"]
pub type APB1PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB1PSC`"]
pub struct APB1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> APB1PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `APB2PSC`"]
pub type APB2PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB2PSC`"]
pub struct APB2PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> APB2PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `ADCPSC_1_0`"]
pub type ADCPSC_1_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCPSC_1_0`"]
pub struct ADCPSC_1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPSC_1_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PLLSEL`"]
pub type PLLSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSEL`"]
pub struct PLLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSEL_W<'a> {
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
#[doc = "Reader of field `PREDV0`"]
pub type PREDV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREDV0`"]
pub struct PREDV0_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDV0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PLLMF_3_0`"]
pub type PLLMF_3_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLMF_3_0`"]
pub struct PLLMF_3_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMF_3_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `USBDPSC_1_0`"]
pub type USBDPSC_1_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBDPSC_1_0`"]
pub struct USBDPSC_1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDPSC_1_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `CKOUT0SEL`"]
pub type CKOUT0SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKOUT0SEL`"]
pub struct CKOUT0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUT0SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `PLLMF_4`"]
pub type PLLMF_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLMF_4`"]
pub struct PLLMF_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMF_4_W<'a> {
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
#[doc = "Reader of field `ADCPSC_2`"]
pub type ADCPSC_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPSC_2`"]
pub struct ADCPSC_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPSC_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `PLLMF_5`"]
pub type PLLMF_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLMF_5`"]
pub struct PLLMF_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMF_5_W<'a> {
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
#[doc = "Reader of field `USBDPSC_2`"]
pub type USBDPSC_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBDPSC_2`"]
pub struct USBDPSC_2_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDPSC_2_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn scss(&self) -> SCSS_R {
        SCSS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&self) -> AHBPSC_R {
        AHBPSC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&self) -> APB1PSC_R {
        APB1PSC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&self) -> APB2PSC_R {
        APB2PSC_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc_1_0(&self) -> ADCPSC_1_0_R {
        ADCPSC_1_0_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&self) -> PLLSEL_R {
        PLLSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PREDV0 division factor"]
    #[inline(always)]
    pub fn predv0(&self) -> PREDV0_R {
        PREDV0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:21 - The PLL clock multiplication factor"]
    #[inline(always)]
    pub fn pllmf_3_0(&self) -> PLLMF_3_0_R {
        PLLMF_3_0_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - USBD clock prescaler selection"]
    #[inline(always)]
    pub fn usbdpsc_1_0(&self) -> USBDPSC_1_0_R {
        USBDPSC_1_0_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - CKOUT0 Clock Source Selection"]
    #[inline(always)]
    pub fn ckout0sel(&self) -> CKOUT0SEL_R {
        CKOUT0SEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Bit 4 of PLLMF"]
    #[inline(always)]
    pub fn pllmf_4(&self) -> PLLMF_4_R {
        PLLMF_4_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Bit 2 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc_2(&self) -> ADCPSC_2_R {
        ADCPSC_2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Bit 5 of PLLMF"]
    #[inline(always)]
    pub fn pllmf_5(&self) -> PLLMF_5_R {
        PLLMF_5_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Bit 2 of USBDPSC"]
    #[inline(always)]
    pub fn usbdpsc_2(&self) -> USBDPSC_2_R {
        USBDPSC_2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&mut self) -> SCS_W {
        SCS_W { w: self }
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&mut self) -> AHBPSC_W {
        AHBPSC_W { w: self }
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&mut self) -> APB1PSC_W {
        APB1PSC_W { w: self }
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&mut self) -> APB2PSC_W {
        APB2PSC_W { w: self }
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc_1_0(&mut self) -> ADCPSC_1_0_W {
        ADCPSC_1_0_W { w: self }
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&mut self) -> PLLSEL_W {
        PLLSEL_W { w: self }
    }
    #[doc = "Bit 17 - PREDV0 division factor"]
    #[inline(always)]
    pub fn predv0(&mut self) -> PREDV0_W {
        PREDV0_W { w: self }
    }
    #[doc = "Bits 18:21 - The PLL clock multiplication factor"]
    #[inline(always)]
    pub fn pllmf_3_0(&mut self) -> PLLMF_3_0_W {
        PLLMF_3_0_W { w: self }
    }
    #[doc = "Bits 22:23 - USBD clock prescaler selection"]
    #[inline(always)]
    pub fn usbdpsc_1_0(&mut self) -> USBDPSC_1_0_W {
        USBDPSC_1_0_W { w: self }
    }
    #[doc = "Bits 24:26 - CKOUT0 Clock Source Selection"]
    #[inline(always)]
    pub fn ckout0sel(&mut self) -> CKOUT0SEL_W {
        CKOUT0SEL_W { w: self }
    }
    #[doc = "Bit 27 - Bit 4 of PLLMF"]
    #[inline(always)]
    pub fn pllmf_4(&mut self) -> PLLMF_4_W {
        PLLMF_4_W { w: self }
    }
    #[doc = "Bit 28 - Bit 2 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc_2(&mut self) -> ADCPSC_2_W {
        ADCPSC_2_W { w: self }
    }
    #[doc = "Bit 30 - Bit 5 of PLLMF"]
    #[inline(always)]
    pub fn pllmf_5(&mut self) -> PLLMF_5_W {
        PLLMF_5_W { w: self }
    }
    #[doc = "Bit 31 - Bit 2 of USBDPSC"]
    #[inline(always)]
    pub fn usbdpsc_2(&mut self) -> USBDPSC_2_W {
        USBDPSC_2_W { w: self }
    }
}
