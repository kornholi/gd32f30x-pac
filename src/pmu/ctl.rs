#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0xc000"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000
    }
}
#[doc = "Reader of field `LDEN`"]
pub type LDEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LDEN`"]
pub struct LDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LDEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `HDS`"]
pub type HDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDS`"]
pub struct HDS_W<'a> {
    w: &'a mut W,
}
impl<'a> HDS_W<'a> {
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
#[doc = "Reader of field `HDEN`"]
pub type HDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDEN`"]
pub struct HDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDEN_W<'a> {
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
#[doc = "Reader of field `LDOVS`"]
pub type LDOVS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LDOVS`"]
pub struct LDOVS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `LDNP`"]
pub type LDNP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDNP`"]
pub struct LDNP_W<'a> {
    w: &'a mut W,
}
impl<'a> LDNP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `LDLP`"]
pub type LDLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDLP`"]
pub struct LDLP_W<'a> {
    w: &'a mut W,
}
impl<'a> LDLP_W<'a> {
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
#[doc = "Reader of field `BKPWEN`"]
pub type BKPWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKPWEN`"]
pub struct BKPWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPWEN_W<'a> {
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
#[doc = "Reader of field `LVDT`"]
pub type LVDT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LVDT`"]
pub struct LVDT_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `LVDEN`"]
pub type LVDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDEN`"]
pub struct LVDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `STBRST`"]
pub type STBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STBRST`"]
pub struct STBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> STBRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `WURST`"]
pub type WURST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WURST`"]
pub struct WURST_W<'a> {
    w: &'a mut W,
}
impl<'a> WURST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `STBMOD`"]
pub type STBMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STBMOD`"]
pub struct STBMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> STBMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `LDOLP`"]
pub type LDOLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDOLP`"]
pub struct LDOLP_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOLP_W<'a> {
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
    #[doc = "Bits 18:19 - Low-driver mode enable in Deep-sleep mode"]
    #[inline(always)]
    pub fn lden(&self) -> LDEN_R {
        LDEN_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - High-driver mode switch"]
    #[inline(always)]
    pub fn hds(&self) -> HDS_R {
        HDS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - High-driver mode enable"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - LDO output voltage select"]
    #[inline(always)]
    pub fn ldovs(&self) -> LDOVS_R {
        LDOVS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Low-driver mode when use normal power LDO"]
    #[inline(always)]
    pub fn ldnp(&self) -> LDNP_R {
        LDNP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Low-driver mode when use low power LDO."]
    #[inline(always)]
    pub fn ldlp(&self) -> LDLP_R {
        LDLP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    pub fn bkpwen(&self) -> BKPWEN_R {
        BKPWEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    pub fn lvdt(&self) -> LVDT_R {
        LVDT_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    pub fn lvden(&self) -> LVDEN_R {
        LVDEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    pub fn stbrst(&self) -> STBRST_R {
        STBRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    pub fn wurst(&self) -> WURST_R {
        WURST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    pub fn stbmod(&self) -> STBMOD_R {
        STBMOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    pub fn ldolp(&self) -> LDOLP_R {
        LDOLP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 18:19 - Low-driver mode enable in Deep-sleep mode"]
    #[inline(always)]
    pub fn lden(&mut self) -> LDEN_W {
        LDEN_W { w: self }
    }
    #[doc = "Bit 17 - High-driver mode switch"]
    #[inline(always)]
    pub fn hds(&mut self) -> HDS_W {
        HDS_W { w: self }
    }
    #[doc = "Bit 16 - High-driver mode enable"]
    #[inline(always)]
    pub fn hden(&mut self) -> HDEN_W {
        HDEN_W { w: self }
    }
    #[doc = "Bits 14:15 - LDO output voltage select"]
    #[inline(always)]
    pub fn ldovs(&mut self) -> LDOVS_W {
        LDOVS_W { w: self }
    }
    #[doc = "Bit 11 - Low-driver mode when use normal power LDO"]
    #[inline(always)]
    pub fn ldnp(&mut self) -> LDNP_W {
        LDNP_W { w: self }
    }
    #[doc = "Bit 10 - Low-driver mode when use low power LDO."]
    #[inline(always)]
    pub fn ldlp(&mut self) -> LDLP_W {
        LDLP_W { w: self }
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    pub fn bkpwen(&mut self) -> BKPWEN_W {
        BKPWEN_W { w: self }
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    pub fn lvdt(&mut self) -> LVDT_W {
        LVDT_W { w: self }
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    pub fn lvden(&mut self) -> LVDEN_W {
        LVDEN_W { w: self }
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    pub fn stbrst(&mut self) -> STBRST_W {
        STBRST_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    pub fn wurst(&mut self) -> WURST_W {
        WURST_W { w: self }
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    pub fn stbmod(&mut self) -> STBMOD_W {
        STBMOD_W { w: self }
    }
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    pub fn ldolp(&mut self) -> LDOLP_W {
        LDOLP_W { w: self }
    }
}
