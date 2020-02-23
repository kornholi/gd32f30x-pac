#[doc = "Reader of register SNCTL3"]
pub type R = crate::R<u32, super::SNCTL3>;
#[doc = "Writer for register SNCTL3"]
pub type W = crate::W<u32, super::SNCTL3>;
#[doc = "Register SNCTL3 `reset()`'s with value 0x30d2"]
impl crate::ResetValue for super::SNCTL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30d2
    }
}
#[doc = "Reader of field `SYNCWR`"]
pub type SYNCWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCWR`"]
pub struct SYNCWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCWR_W<'a> {
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
#[doc = "Reader of field `CPS`"]
pub type CPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPS`"]
pub struct CPS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `ASYNCWAIT`"]
pub type ASYNCWAIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASYNCWAIT`"]
pub struct ASYNCWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCWAIT_W<'a> {
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
#[doc = "Reader of field `EXMODEN`"]
pub type EXMODEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXMODEN`"]
pub struct EXMODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXMODEN_W<'a> {
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
#[doc = "Reader of field `NRWTEN`"]
pub type NRWTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NRWTEN`"]
pub struct NRWTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NRWTEN_W<'a> {
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
#[doc = "Reader of field `WREN`"]
pub type WREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WREN`"]
pub struct WREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WREN_W<'a> {
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
#[doc = "Reader of field `NRWTCFG`"]
pub type NRWTCFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NRWTCFG`"]
pub struct NRWTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> NRWTCFG_W<'a> {
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
#[doc = "Reader of field `WRAPEN`"]
pub type WRAPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRAPEN`"]
pub struct WRAPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAPEN_W<'a> {
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
#[doc = "Reader of field `NRWTPOL`"]
pub type NRWTPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NRWTPOL`"]
pub struct NRWTPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> NRWTPOL_W<'a> {
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
#[doc = "Reader of field `SBRSTEN`"]
pub type SBRSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SBRSTEN`"]
pub struct SBRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SBRSTEN_W<'a> {
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
#[doc = "Reader of field `NREN`"]
pub type NREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NREN`"]
pub struct NREN_W<'a> {
    w: &'a mut W,
}
impl<'a> NREN_W<'a> {
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
#[doc = "Reader of field `NRW`"]
pub type NRW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NRW`"]
pub struct NRW_W<'a> {
    w: &'a mut W,
}
impl<'a> NRW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `NRTP`"]
pub type NRTP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NRTP`"]
pub struct NRTP_W<'a> {
    w: &'a mut W,
}
impl<'a> NRTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `NRMUX`"]
pub type NRMUX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NRMUX`"]
pub struct NRMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> NRMUX_W<'a> {
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
#[doc = "Reader of field `NRBKEN`"]
pub type NRBKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NRBKEN`"]
pub struct NRBKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NRBKEN_W<'a> {
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
    #[doc = "Bit 19 - Synchronous write"]
    #[inline(always)]
    pub fn syncwr(&self) -> SYNCWR_R {
        SYNCWR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - CRAM page size"]
    #[inline(always)]
    pub fn cps(&self) -> CPS_R {
        CPS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Asynchronous wait"]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Extended mode enable"]
    #[inline(always)]
    pub fn exmoden(&self) -> EXMODEN_R {
        EXMODEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NWAIT signal enable"]
    #[inline(always)]
    pub fn nrwten(&self) -> NRWTEN_R {
        NRWTEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - NWAIT signal configuration, only work in synchronous mode"]
    #[inline(always)]
    pub fn nrwtcfg(&self) -> NRWTCFG_R {
        NRWTCFG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Wrapped burst mode enable"]
    #[inline(always)]
    pub fn wrapen(&self) -> WRAPEN_R {
        WRAPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - NWAIT signal polarity"]
    #[inline(always)]
    pub fn nrwtpol(&self) -> NRWTPOL_R {
        NRWTPOL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Synchronous burst enable"]
    #[inline(always)]
    pub fn sbrsten(&self) -> SBRSTEN_R {
        SBRSTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NOR Flash access enable"]
    #[inline(always)]
    pub fn nren(&self) -> NREN_R {
        NREN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - NOR bank memory data bus width"]
    #[inline(always)]
    pub fn nrw(&self) -> NRW_R {
        NRW_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - NOR bank memory type"]
    #[inline(always)]
    pub fn nrtp(&self) -> NRTP_R {
        NRTP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - NOR bank memory address/data multiplexing"]
    #[inline(always)]
    pub fn nrmux(&self) -> NRMUX_R {
        NRMUX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - NOR bank enable"]
    #[inline(always)]
    pub fn nrbken(&self) -> NRBKEN_R {
        NRBKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - Synchronous write"]
    #[inline(always)]
    pub fn syncwr(&mut self) -> SYNCWR_W {
        SYNCWR_W { w: self }
    }
    #[doc = "Bits 16:18 - CRAM page size"]
    #[inline(always)]
    pub fn cps(&mut self) -> CPS_W {
        CPS_W { w: self }
    }
    #[doc = "Bit 15 - Asynchronous wait"]
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W {
        ASYNCWAIT_W { w: self }
    }
    #[doc = "Bit 14 - Extended mode enable"]
    #[inline(always)]
    pub fn exmoden(&mut self) -> EXMODEN_W {
        EXMODEN_W { w: self }
    }
    #[doc = "Bit 13 - NWAIT signal enable"]
    #[inline(always)]
    pub fn nrwten(&mut self) -> NRWTEN_W {
        NRWTEN_W { w: self }
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W { w: self }
    }
    #[doc = "Bit 11 - NWAIT signal configuration, only work in synchronous mode"]
    #[inline(always)]
    pub fn nrwtcfg(&mut self) -> NRWTCFG_W {
        NRWTCFG_W { w: self }
    }
    #[doc = "Bit 10 - Wrapped burst mode enable"]
    #[inline(always)]
    pub fn wrapen(&mut self) -> WRAPEN_W {
        WRAPEN_W { w: self }
    }
    #[doc = "Bit 9 - NWAIT signal polarity"]
    #[inline(always)]
    pub fn nrwtpol(&mut self) -> NRWTPOL_W {
        NRWTPOL_W { w: self }
    }
    #[doc = "Bit 8 - Synchronous burst enable"]
    #[inline(always)]
    pub fn sbrsten(&mut self) -> SBRSTEN_W {
        SBRSTEN_W { w: self }
    }
    #[doc = "Bit 6 - NOR Flash access enable"]
    #[inline(always)]
    pub fn nren(&mut self) -> NREN_W {
        NREN_W { w: self }
    }
    #[doc = "Bits 4:5 - NOR bank memory data bus width"]
    #[inline(always)]
    pub fn nrw(&mut self) -> NRW_W {
        NRW_W { w: self }
    }
    #[doc = "Bits 2:3 - NOR bank memory type"]
    #[inline(always)]
    pub fn nrtp(&mut self) -> NRTP_W {
        NRTP_W { w: self }
    }
    #[doc = "Bit 1 - NOR bank memory address/data multiplexing"]
    #[inline(always)]
    pub fn nrmux(&mut self) -> NRMUX_W {
        NRMUX_W { w: self }
    }
    #[doc = "Bit 0 - NOR bank enable"]
    #[inline(always)]
    pub fn nrbken(&mut self) -> NRBKEN_W {
        NRBKEN_W { w: self }
    }
}
