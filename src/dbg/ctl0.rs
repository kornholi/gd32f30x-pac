#[doc = "Reader of register CTL0"]
pub type R = crate::R<u32, super::CTL0>;
#[doc = "Writer for register CTL0"]
pub type W = crate::W<u32, super::CTL0>;
#[doc = "Register CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLP_HOLD`"]
pub type SLP_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_HOLD`"]
pub struct SLP_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_HOLD_W<'a> {
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
#[doc = "Reader of field `DSLP_HOLD`"]
pub type DSLP_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSLP_HOLD`"]
pub struct DSLP_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSLP_HOLD_W<'a> {
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
#[doc = "Reader of field `STB_HOLD`"]
pub type STB_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STB_HOLD`"]
pub struct STB_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> STB_HOLD_W<'a> {
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
#[doc = "Reader of field `TRACE_IOEN`"]
pub type TRACE_IOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRACE_IOEN`"]
pub struct TRACE_IOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACE_IOEN_W<'a> {
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
#[doc = "Reader of field `TRACE_MODE`"]
pub type TRACE_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRACE_MODE`"]
pub struct TRACE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `WWDGT_HOLD`"]
pub type WWDGT_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WWDGT_HOLD`"]
pub struct WWDGT_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGT_HOLD_W<'a> {
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
#[doc = "Reader of field `FWDGT_HOLD`"]
pub type FWDGT_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FWDGT_HOLD`"]
pub struct FWDGT_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> FWDGT_HOLD_W<'a> {
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
#[doc = "Reader of field `TIMER0_HOLD`"]
pub type TIMER0_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER0_HOLD`"]
pub struct TIMER0_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_HOLD_W<'a> {
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
#[doc = "Reader of field `TIMER1_HOLD`"]
pub type TIMER1_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER1_HOLD`"]
pub struct TIMER1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_HOLD_W<'a> {
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
#[doc = "Reader of field `TIMER2_HOLD`"]
pub type TIMER2_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER2_HOLD`"]
pub struct TIMER2_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_HOLD_W<'a> {
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
#[doc = "Reader of field `TIMER3_HOLD`"]
pub type TIMER3_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER3_HOLD`"]
pub struct TIMER3_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_HOLD_W<'a> {
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
#[doc = "Reader of field `CAN0_HOLD`"]
pub type CAN0_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN0_HOLD`"]
pub struct CAN0_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0_HOLD_W<'a> {
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
#[doc = "Reader of field `I2C0_HOLD`"]
pub type I2C0_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0_HOLD`"]
pub struct I2C0_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `I2C1_HOLD`"]
pub type I2C1_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1_HOLD`"]
pub struct I2C1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_HOLD_W<'a> {
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
#[doc = "Reader of field `TIMER4_HOLD`"]
pub type TIMER4_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER4_HOLD`"]
pub struct TIMER4_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER4_HOLD_W<'a> {
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
#[doc = "Reader of field `TIMER5_HOLD`"]
pub type TIMER5_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER5_HOLD`"]
pub struct TIMER5_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER5_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TIMER6_HOLD`"]
pub type TIMER6_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER6_HOLD`"]
pub struct TIMER6_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER6_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `TIMER7_HOLD`"]
pub type TIMER7_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER7_HOLD`"]
pub struct TIMER7_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER7_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CAN1_HOLD`"]
pub type CAN1_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN1_HOLD`"]
pub struct CAN1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TIMER11_HOLD`"]
pub type TIMER11_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER11_HOLD`"]
pub struct TIMER11_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER11_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `TIMER12_HOLD`"]
pub type TIMER12_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER12_HOLD`"]
pub struct TIMER12_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER12_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `TIMER13_HOLD`"]
pub type TIMER13_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER13_HOLD`"]
pub struct TIMER13_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER13_HOLD_W<'a> {
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
#[doc = "Reader of field `TIMER8_HOLD`"]
pub type TIMER8_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER8_HOLD`"]
pub struct TIMER8_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER8_HOLD_W<'a> {
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
#[doc = "Reader of field `TIMER9_HOLD`"]
pub type TIMER9_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER9_HOLD`"]
pub struct TIMER9_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER9_HOLD_W<'a> {
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
#[doc = "Reader of field `TIMER10_HOLD`"]
pub type TIMER10_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER10_HOLD`"]
pub struct TIMER10_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER10_HOLD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    pub fn slp_hold(&self) -> SLP_HOLD_R {
        SLP_HOLD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    pub fn dslp_hold(&self) -> DSLP_HOLD_R {
        DSLP_HOLD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    pub fn stb_hold(&self) -> STB_HOLD_R {
        STB_HOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Trace pin allocation enable"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Trace pin allocation mode"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - WWDGT hold bit"]
    #[inline(always)]
    pub fn wwdgt_hold(&self) -> WWDGT_HOLD_R {
        WWDGT_HOLD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FWDGT hold bit"]
    #[inline(always)]
    pub fn fwdgt_hold(&self) -> FWDGT_HOLD_R {
        FWDGT_HOLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TIMER 0 hold bit"]
    #[inline(always)]
    pub fn timer0_hold(&self) -> TIMER0_HOLD_R {
        TIMER0_HOLD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIMER 1 hold bit"]
    #[inline(always)]
    pub fn timer1_hold(&self) -> TIMER1_HOLD_R {
        TIMER1_HOLD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TIMER 2 hold bit"]
    #[inline(always)]
    pub fn timer2_hold(&self) -> TIMER2_HOLD_R {
        TIMER2_HOLD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TIMER 23 hold bit"]
    #[inline(always)]
    pub fn timer3_hold(&self) -> TIMER3_HOLD_R {
        TIMER3_HOLD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CAN0 hold bit"]
    #[inline(always)]
    pub fn can0_hold(&self) -> CAN0_HOLD_R {
        CAN0_HOLD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C0 hold bit"]
    #[inline(always)]
    pub fn i2c0_hold(&self) -> I2C0_HOLD_R {
        I2C0_HOLD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C1 hold bit"]
    #[inline(always)]
    pub fn i2c1_hold(&self) -> I2C1_HOLD_R {
        I2C1_HOLD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIMER4_HOLD"]
    #[inline(always)]
    pub fn timer4_hold(&self) -> TIMER4_HOLD_R {
        TIMER4_HOLD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIMER 5 hold bit"]
    #[inline(always)]
    pub fn timer5_hold(&self) -> TIMER5_HOLD_R {
        TIMER5_HOLD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TIMER 6 hold bit"]
    #[inline(always)]
    pub fn timer6_hold(&self) -> TIMER6_HOLD_R {
        TIMER6_HOLD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TIMER 7 hold bit"]
    #[inline(always)]
    pub fn timer7_hold(&self) -> TIMER7_HOLD_R {
        TIMER7_HOLD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CAN1 hold bit"]
    #[inline(always)]
    pub fn can1_hold(&self) -> CAN1_HOLD_R {
        CAN1_HOLD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TIMER 11 hold bit"]
    #[inline(always)]
    pub fn timer11_hold(&self) -> TIMER11_HOLD_R {
        TIMER11_HOLD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TIMER 12 hold bit"]
    #[inline(always)]
    pub fn timer12_hold(&self) -> TIMER12_HOLD_R {
        TIMER12_HOLD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TIMER 13 hold bit"]
    #[inline(always)]
    pub fn timer13_hold(&self) -> TIMER13_HOLD_R {
        TIMER13_HOLD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TIMER 8 hold bit"]
    #[inline(always)]
    pub fn timer8_hold(&self) -> TIMER8_HOLD_R {
        TIMER8_HOLD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TIMER 9 hold bit"]
    #[inline(always)]
    pub fn timer9_hold(&self) -> TIMER9_HOLD_R {
        TIMER9_HOLD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TIMER 10 hold bit"]
    #[inline(always)]
    pub fn timer10_hold(&self) -> TIMER10_HOLD_R {
        TIMER10_HOLD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    pub fn slp_hold(&mut self) -> SLP_HOLD_W {
        SLP_HOLD_W { w: self }
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    pub fn dslp_hold(&mut self) -> DSLP_HOLD_W {
        DSLP_HOLD_W { w: self }
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    pub fn stb_hold(&mut self) -> STB_HOLD_W {
        STB_HOLD_W { w: self }
    }
    #[doc = "Bit 5 - Trace pin allocation enable"]
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W {
        TRACE_IOEN_W { w: self }
    }
    #[doc = "Bits 6:7 - Trace pin allocation mode"]
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W {
        TRACE_MODE_W { w: self }
    }
    #[doc = "Bit 8 - WWDGT hold bit"]
    #[inline(always)]
    pub fn wwdgt_hold(&mut self) -> WWDGT_HOLD_W {
        WWDGT_HOLD_W { w: self }
    }
    #[doc = "Bit 9 - FWDGT hold bit"]
    #[inline(always)]
    pub fn fwdgt_hold(&mut self) -> FWDGT_HOLD_W {
        FWDGT_HOLD_W { w: self }
    }
    #[doc = "Bit 10 - TIMER 0 hold bit"]
    #[inline(always)]
    pub fn timer0_hold(&mut self) -> TIMER0_HOLD_W {
        TIMER0_HOLD_W { w: self }
    }
    #[doc = "Bit 11 - TIMER 1 hold bit"]
    #[inline(always)]
    pub fn timer1_hold(&mut self) -> TIMER1_HOLD_W {
        TIMER1_HOLD_W { w: self }
    }
    #[doc = "Bit 12 - TIMER 2 hold bit"]
    #[inline(always)]
    pub fn timer2_hold(&mut self) -> TIMER2_HOLD_W {
        TIMER2_HOLD_W { w: self }
    }
    #[doc = "Bit 13 - TIMER 23 hold bit"]
    #[inline(always)]
    pub fn timer3_hold(&mut self) -> TIMER3_HOLD_W {
        TIMER3_HOLD_W { w: self }
    }
    #[doc = "Bit 14 - CAN0 hold bit"]
    #[inline(always)]
    pub fn can0_hold(&mut self) -> CAN0_HOLD_W {
        CAN0_HOLD_W { w: self }
    }
    #[doc = "Bit 15 - I2C0 hold bit"]
    #[inline(always)]
    pub fn i2c0_hold(&mut self) -> I2C0_HOLD_W {
        I2C0_HOLD_W { w: self }
    }
    #[doc = "Bit 16 - I2C1 hold bit"]
    #[inline(always)]
    pub fn i2c1_hold(&mut self) -> I2C1_HOLD_W {
        I2C1_HOLD_W { w: self }
    }
    #[doc = "Bit 17 - TIMER4_HOLD"]
    #[inline(always)]
    pub fn timer4_hold(&mut self) -> TIMER4_HOLD_W {
        TIMER4_HOLD_W { w: self }
    }
    #[doc = "Bit 18 - TIMER 5 hold bit"]
    #[inline(always)]
    pub fn timer5_hold(&mut self) -> TIMER5_HOLD_W {
        TIMER5_HOLD_W { w: self }
    }
    #[doc = "Bit 19 - TIMER 6 hold bit"]
    #[inline(always)]
    pub fn timer6_hold(&mut self) -> TIMER6_HOLD_W {
        TIMER6_HOLD_W { w: self }
    }
    #[doc = "Bit 20 - TIMER 7 hold bit"]
    #[inline(always)]
    pub fn timer7_hold(&mut self) -> TIMER7_HOLD_W {
        TIMER7_HOLD_W { w: self }
    }
    #[doc = "Bit 21 - CAN1 hold bit"]
    #[inline(always)]
    pub fn can1_hold(&mut self) -> CAN1_HOLD_W {
        CAN1_HOLD_W { w: self }
    }
    #[doc = "Bit 25 - TIMER 11 hold bit"]
    #[inline(always)]
    pub fn timer11_hold(&mut self) -> TIMER11_HOLD_W {
        TIMER11_HOLD_W { w: self }
    }
    #[doc = "Bit 26 - TIMER 12 hold bit"]
    #[inline(always)]
    pub fn timer12_hold(&mut self) -> TIMER12_HOLD_W {
        TIMER12_HOLD_W { w: self }
    }
    #[doc = "Bit 27 - TIMER 13 hold bit"]
    #[inline(always)]
    pub fn timer13_hold(&mut self) -> TIMER13_HOLD_W {
        TIMER13_HOLD_W { w: self }
    }
    #[doc = "Bit 28 - TIMER 8 hold bit"]
    #[inline(always)]
    pub fn timer8_hold(&mut self) -> TIMER8_HOLD_W {
        TIMER8_HOLD_W { w: self }
    }
    #[doc = "Bit 29 - TIMER 9 hold bit"]
    #[inline(always)]
    pub fn timer9_hold(&mut self) -> TIMER9_HOLD_W {
        TIMER9_HOLD_W { w: self }
    }
    #[doc = "Bit 30 - TIMER 10 hold bit"]
    #[inline(always)]
    pub fn timer10_hold(&mut self) -> TIMER10_HOLD_W {
        TIMER10_HOLD_W { w: self }
    }
}
