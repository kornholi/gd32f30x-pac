#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `SETRST`"]
pub type SETRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETRST`"]
pub struct SETRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SETRST_W<'a> {
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
#[doc = "Reader of field `CLOSE`"]
pub type CLOSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLOSE`"]
pub struct CLOSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOSE_W<'a> {
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
#[doc = "Reader of field `LOWM`"]
pub type LOWM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOWM`"]
pub struct LOWM_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWM_W<'a> {
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
#[doc = "Reader of field `SETSPS`"]
pub type SETSPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETSPS`"]
pub struct SETSPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SETSPS_W<'a> {
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
#[doc = "Reader of field `RSREQ`"]
pub type RSREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSREQ`"]
pub struct RSREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RSREQ_W<'a> {
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
#[doc = "Reader of field `L1RSREQ`"]
pub type L1RSREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L1RSREQ`"]
pub struct L1RSREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> L1RSREQ_W<'a> {
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
#[doc = "Reader of field `L1REQIE`"]
pub type L1REQIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L1REQIE`"]
pub struct L1REQIE_W<'a> {
    w: &'a mut W,
}
impl<'a> L1REQIE_W<'a> {
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
#[doc = "Reader of field `ESOFIE`"]
pub type ESOFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESOFIE`"]
pub struct ESOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOFIE_W<'a> {
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
#[doc = "Reader of field `SOFIE`"]
pub type SOFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFIE`"]
pub struct SOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIE_W<'a> {
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
#[doc = "Reader of field `RSTIE`"]
pub type RSTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTIE`"]
pub struct RSTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIE_W<'a> {
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
#[doc = "Reader of field `SPSIE`"]
pub type SPSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPSIE`"]
pub struct SPSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPSIE_W<'a> {
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
#[doc = "Reader of field `WKUPIE`"]
pub type WKUPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPIE`"]
pub struct WKUPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPIE_W<'a> {
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
#[doc = "Reader of field `ERRIE`"]
pub type ERRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRIE`"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
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
#[doc = "Reader of field `PMOUIE`"]
pub type PMOUIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMOUIE`"]
pub struct PMOUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMOUIE_W<'a> {
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
#[doc = "Reader of field `STIE`"]
pub type STIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STIE`"]
pub struct STIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Set reset"]
    #[inline(always)]
    pub fn setrst(&self) -> SETRST_R {
        SETRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Close state"]
    #[inline(always)]
    pub fn close(&self) -> CLOSE_R {
        CLOSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lowm(&self) -> LOWM_R {
        LOWM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set suspend"]
    #[inline(always)]
    pub fn setsps(&self) -> SETSPS_R {
        SETSPS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    pub fn rsreq(&self) -> RSREQ_R {
        RSREQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPM L1 resume request"]
    #[inline(always)]
    pub fn l1rsreq(&self) -> L1RSREQ_R {
        L1RSREQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt enable"]
    #[inline(always)]
    pub fn l1reqie(&self) -> L1REQIE_R {
        L1REQIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt enable"]
    #[inline(always)]
    pub fn esofie(&self) -> ESOFIE_R {
        ESOFIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn spsie(&self) -> SPSIE_R {
        SPSIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt enable"]
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt enable"]
    #[inline(always)]
    pub fn pmouie(&self) -> PMOUIE_R {
        PMOUIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Successful transfer interrupt enable"]
    #[inline(always)]
    pub fn stie(&self) -> STIE_R {
        STIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set reset"]
    #[inline(always)]
    pub fn setrst(&mut self) -> SETRST_W {
        SETRST_W { w: self }
    }
    #[doc = "Bit 1 - Close state"]
    #[inline(always)]
    pub fn close(&mut self) -> CLOSE_W {
        CLOSE_W { w: self }
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lowm(&mut self) -> LOWM_W {
        LOWM_W { w: self }
    }
    #[doc = "Bit 3 - Set suspend"]
    #[inline(always)]
    pub fn setsps(&mut self) -> SETSPS_W {
        SETSPS_W { w: self }
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    pub fn rsreq(&mut self) -> RSREQ_W {
        RSREQ_W { w: self }
    }
    #[doc = "Bit 5 - LPM L1 resume request"]
    #[inline(always)]
    pub fn l1rsreq(&mut self) -> L1RSREQ_W {
        L1RSREQ_W { w: self }
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt enable"]
    #[inline(always)]
    pub fn l1reqie(&mut self) -> L1REQIE_W {
        L1REQIE_W { w: self }
    }
    #[doc = "Bit 8 - Expected start of frame interrupt enable"]
    #[inline(always)]
    pub fn esofie(&mut self) -> ESOFIE_W {
        ESOFIE_W { w: self }
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W {
        SOFIE_W { w: self }
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    pub fn rstie(&mut self) -> RSTIE_W {
        RSTIE_W { w: self }
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn spsie(&mut self) -> SPSIE_W {
        SPSIE_W { w: self }
    }
    #[doc = "Bit 12 - Wakeup interrupt enable"]
    #[inline(always)]
    pub fn wkupie(&mut self) -> WKUPIE_W {
        WKUPIE_W { w: self }
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt enable"]
    #[inline(always)]
    pub fn pmouie(&mut self) -> PMOUIE_W {
        PMOUIE_W { w: self }
    }
    #[doc = "Bit 15 - Successful transfer interrupt enable"]
    #[inline(always)]
    pub fn stie(&mut self) -> STIE_W {
        STIE_W { w: self }
    }
}
