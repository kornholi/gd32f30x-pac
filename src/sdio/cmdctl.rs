#[doc = "Reader of register CMDCTL"]
pub type R = crate::R<u32, super::CMDCTL>;
#[doc = "Writer for register CMDCTL"]
pub type W = crate::W<u32, super::CMDCTL>;
#[doc = "Register CMDCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDIDX`"]
pub type CMDIDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDIDX`"]
pub struct CMDIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CMDRESP`"]
pub type CMDRESP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDRESP`"]
pub struct CMDRESP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRESP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `INTWAIT`"]
pub type INTWAIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTWAIT`"]
pub struct INTWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTWAIT_W<'a> {
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
#[doc = "Reader of field `WAITDEND`"]
pub type WAITDEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAITDEND`"]
pub struct WAITDEND_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITDEND_W<'a> {
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
#[doc = "Reader of field `CSMEN`"]
pub type CSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSMEN`"]
pub struct CSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSMEN_W<'a> {
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
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPEND`"]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
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
#[doc = "Reader of field `ENCMDC`"]
pub type ENCMDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENCMDC`"]
pub struct ENCMDC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCMDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `NINTEN`"]
pub type NINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NINTEN`"]
pub struct NINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NINTEN_W<'a> {
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
#[doc = "Reader of field `ATAEN`"]
pub type ATAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATAEN`"]
pub struct ATAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATAEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Command response type bits"]
    #[inline(always)]
    pub fn cmdresp(&self) -> CMDRESP_R {
        CMDRESP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Interrupt wait instead of timeout"]
    #[inline(always)]
    pub fn intwait(&self) -> INTWAIT_R {
        INTWAIT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Waits for ends of data transfer"]
    #[inline(always)]
    pub fn waitdend(&self) -> WAITDEND_R {
        WAITDEND_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Command state machine (CSM) enable bit"]
    #[inline(always)]
    pub fn csmen(&self) -> CSMEN_R {
        CSMEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SD I/O suspend command(SD I/O only)"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CMD completion signal enabled (CE-ATA only)"]
    #[inline(always)]
    pub fn encmdc(&self) -> ENCMDC_R {
        ENCMDC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - No CE-ATA Interrupt (CE-ATA only)"]
    #[inline(always)]
    pub fn ninten(&self) -> NINTEN_R {
        NINTEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CE-ATA command enable(CE-ATA only)"]
    #[inline(always)]
    pub fn ataen(&self) -> ATAEN_R {
        ATAEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdidx(&mut self) -> CMDIDX_W {
        CMDIDX_W { w: self }
    }
    #[doc = "Bits 6:7 - Command response type bits"]
    #[inline(always)]
    pub fn cmdresp(&mut self) -> CMDRESP_W {
        CMDRESP_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt wait instead of timeout"]
    #[inline(always)]
    pub fn intwait(&mut self) -> INTWAIT_W {
        INTWAIT_W { w: self }
    }
    #[doc = "Bit 9 - Waits for ends of data transfer"]
    #[inline(always)]
    pub fn waitdend(&mut self) -> WAITDEND_W {
        WAITDEND_W { w: self }
    }
    #[doc = "Bit 10 - Command state machine (CSM) enable bit"]
    #[inline(always)]
    pub fn csmen(&mut self) -> CSMEN_W {
        CSMEN_W { w: self }
    }
    #[doc = "Bit 11 - SD I/O suspend command(SD I/O only)"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bit 12 - CMD completion signal enabled (CE-ATA only)"]
    #[inline(always)]
    pub fn encmdc(&mut self) -> ENCMDC_W {
        ENCMDC_W { w: self }
    }
    #[doc = "Bit 13 - No CE-ATA Interrupt (CE-ATA only)"]
    #[inline(always)]
    pub fn ninten(&mut self) -> NINTEN_W {
        NINTEN_W { w: self }
    }
    #[doc = "Bit 14 - CE-ATA command enable(CE-ATA only)"]
    #[inline(always)]
    pub fn ataen(&mut self) -> ATAEN_W {
        ATAEN_W { w: self }
    }
}
