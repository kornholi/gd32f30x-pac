#[doc = "Writer for register INTC"]
pub type W = crate::W<u32, super::INTC>;
#[doc = "Register INTC `reset()`'s with value 0"]
impl crate::ResetValue for super::INTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CCRCERRC`"]
pub struct CCRCERRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRCERRC_W<'a> {
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
#[doc = "Write proxy for field `DTCRCERRC`"]
pub struct DTCRCERRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCRCERRC_W<'a> {
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
#[doc = "Write proxy for field `CMDTMOUTC`"]
pub struct CMDTMOUTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTMOUTC_W<'a> {
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
#[doc = "Write proxy for field `DTTMOUTC`"]
pub struct DTTMOUTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTTMOUTC_W<'a> {
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
#[doc = "Write proxy for field `TXUREC`"]
pub struct TXUREC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUREC_W<'a> {
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
#[doc = "Write proxy for field `RXOREC`"]
pub struct RXOREC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOREC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `CMDRECVC`"]
pub struct CMDRECVC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRECVC_W<'a> {
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
#[doc = "Write proxy for field `CMDSENDC`"]
pub struct CMDSENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSENDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `DTENDC`"]
pub struct DTENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTENDC_W<'a> {
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
#[doc = "Write proxy for field `STBITEC`"]
pub struct STBITEC_W<'a> {
    w: &'a mut W,
}
impl<'a> STBITEC_W<'a> {
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
#[doc = "Write proxy for field `DTBLKENDC`"]
pub struct DTBLKENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBLKENDC_W<'a> {
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
#[doc = "Write proxy for field `SDIOINTC`"]
pub struct SDIOINTC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOINTC_W<'a> {
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
#[doc = "Write proxy for field `ATAENDC`"]
pub struct ATAENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> ATAENDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - CCRCERR flag clear bit"]
    #[inline(always)]
    pub fn ccrcerrc(&mut self) -> CCRCERRC_W {
        CCRCERRC_W { w: self }
    }
    #[doc = "Bit 1 - DTCRCERR flag clear bit"]
    #[inline(always)]
    pub fn dtcrcerrc(&mut self) -> DTCRCERRC_W {
        DTCRCERRC_W { w: self }
    }
    #[doc = "Bit 2 - CMDTMOUT flag clear bit"]
    #[inline(always)]
    pub fn cmdtmoutc(&mut self) -> CMDTMOUTC_W {
        CMDTMOUTC_W { w: self }
    }
    #[doc = "Bit 3 - DTTMOUT flag clear bit"]
    #[inline(always)]
    pub fn dttmoutc(&mut self) -> DTTMOUTC_W {
        DTTMOUTC_W { w: self }
    }
    #[doc = "Bit 4 - TXURE flag clear bit"]
    #[inline(always)]
    pub fn txurec(&mut self) -> TXUREC_W {
        TXUREC_W { w: self }
    }
    #[doc = "Bit 5 - RXORE flag clear bit"]
    #[inline(always)]
    pub fn rxorec(&mut self) -> RXOREC_W {
        RXOREC_W { w: self }
    }
    #[doc = "Bit 6 - CMDRECV flag clear bit"]
    #[inline(always)]
    pub fn cmdrecvc(&mut self) -> CMDRECVC_W {
        CMDRECVC_W { w: self }
    }
    #[doc = "Bit 7 - CMDSEND flag clear bit"]
    #[inline(always)]
    pub fn cmdsendc(&mut self) -> CMDSENDC_W {
        CMDSENDC_W { w: self }
    }
    #[doc = "Bit 8 - DTEND flag clear bit"]
    #[inline(always)]
    pub fn dtendc(&mut self) -> DTENDC_W {
        DTENDC_W { w: self }
    }
    #[doc = "Bit 9 - STBITE flag clear bit"]
    #[inline(always)]
    pub fn stbitec(&mut self) -> STBITEC_W {
        STBITEC_W { w: self }
    }
    #[doc = "Bit 10 - DTBLKEND flag clear bit"]
    #[inline(always)]
    pub fn dtblkendc(&mut self) -> DTBLKENDC_W {
        DTBLKENDC_W { w: self }
    }
    #[doc = "Bit 22 - SDIOINT flag clear bit"]
    #[inline(always)]
    pub fn sdiointc(&mut self) -> SDIOINTC_W {
        SDIOINTC_W { w: self }
    }
    #[doc = "Bit 23 - ATAEND flag clear bit"]
    #[inline(always)]
    pub fn ataendc(&mut self) -> ATAENDC_W {
        ATAENDC_W { w: self }
    }
}
