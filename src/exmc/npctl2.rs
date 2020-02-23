#[doc = "Reader of register NPCTL2"]
pub type R = crate::R<u32, super::NPCTL2>;
#[doc = "Writer for register NPCTL2"]
pub type W = crate::W<u32, super::NPCTL2>;
#[doc = "Register NPCTL2 `reset()`'s with value 0x18"]
impl crate::ResetValue for super::NPCTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x18
    }
}
#[doc = "Reader of field `ECCSZ`"]
pub type ECCSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ECCSZ`"]
pub struct ECCSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `ATR`"]
pub type ATR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATR`"]
pub struct ATR_W<'a> {
    w: &'a mut W,
}
impl<'a> ATR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | (((value as u32) & 0x0f) << 13);
        self.w
    }
}
#[doc = "Reader of field `CTR`"]
pub type CTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTR`"]
pub struct CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
#[doc = "Reader of field `ECCEN`"]
pub type ECCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCEN`"]
pub struct ECCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `NDW`"]
pub type NDW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NDW`"]
pub struct NDW_W<'a> {
    w: &'a mut W,
}
impl<'a> NDW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `NDTP`"]
pub type NDTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NDTP`"]
pub struct NDTP_W<'a> {
    w: &'a mut W,
}
impl<'a> NDTP_W<'a> {
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
#[doc = "Reader of field `NDBKEN`"]
pub type NDBKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NDBKEN`"]
pub struct NDBKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NDBKEN_W<'a> {
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
#[doc = "Reader of field `NDWTEN`"]
pub type NDWTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NDWTEN`"]
pub struct NDWTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NDWTEN_W<'a> {
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
impl R {
    #[doc = "Bits 17:19 - ECC size"]
    #[inline(always)]
    pub fn eccsz(&self) -> ECCSZ_R {
        ECCSZ_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    pub fn atr(&self) -> ATR_R {
        ATR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - NAND bank memory data bus width"]
    #[inline(always)]
    pub fn ndw(&self) -> NDW_R {
        NDW_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - NAND bank memory type"]
    #[inline(always)]
    pub fn ndtp(&self) -> NDTP_R {
        NDTP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NAND bank enable"]
    #[inline(always)]
    pub fn ndbken(&self) -> NDBKEN_R {
        NDBKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    pub fn ndwten(&self) -> NDWTEN_R {
        NDWTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 17:19 - ECC size"]
    #[inline(always)]
    pub fn eccsz(&mut self) -> ECCSZ_W {
        ECCSZ_W { w: self }
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    pub fn atr(&mut self) -> ATR_W {
        ATR_W { w: self }
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CTR_W {
        CTR_W { w: self }
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W {
        ECCEN_W { w: self }
    }
    #[doc = "Bits 4:5 - NAND bank memory data bus width"]
    #[inline(always)]
    pub fn ndw(&mut self) -> NDW_W {
        NDW_W { w: self }
    }
    #[doc = "Bit 3 - NAND bank memory type"]
    #[inline(always)]
    pub fn ndtp(&mut self) -> NDTP_W {
        NDTP_W { w: self }
    }
    #[doc = "Bit 2 - NAND bank enable"]
    #[inline(always)]
    pub fn ndbken(&mut self) -> NDBKEN_W {
        NDBKEN_W { w: self }
    }
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    pub fn ndwten(&mut self) -> NDWTEN_W {
        NDWTEN_W { w: self }
    }
}
