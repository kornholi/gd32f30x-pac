#[doc = "Reader of register QCTL"]
pub type R = crate::R<u32, super::QCTL>;
#[doc = "Writer for register QCTL"]
pub type W = crate::W<u32, super::QCTL>;
#[doc = "Register QCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::QCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IO23_DRV`"]
pub type IO23_DRV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO23_DRV`"]
pub struct IO23_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> IO23_DRV_W<'a> {
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
#[doc = "Reader of field `QRD`"]
pub type QRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QRD`"]
pub struct QRD_W<'a> {
    w: &'a mut W,
}
impl<'a> QRD_W<'a> {
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
#[doc = "Reader of field `QMOD`"]
pub type QMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QMOD`"]
pub struct QMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> QMOD_W<'a> {
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
    #[doc = "Bit 2 - Drive IO2 and IO3 enable"]
    #[inline(always)]
    pub fn io23_drv(&self) -> IO23_DRV_R {
        IO23_DRV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Quad-SPI mode read select"]
    #[inline(always)]
    pub fn qrd(&self) -> QRD_R {
        QRD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Quad-SPI mode enable"]
    #[inline(always)]
    pub fn qmod(&self) -> QMOD_R {
        QMOD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Drive IO2 and IO3 enable"]
    #[inline(always)]
    pub fn io23_drv(&mut self) -> IO23_DRV_W {
        IO23_DRV_W { w: self }
    }
    #[doc = "Bit 1 - Quad-SPI mode read select"]
    #[inline(always)]
    pub fn qrd(&mut self) -> QRD_W {
        QRD_W { w: self }
    }
    #[doc = "Bit 0 - Quad-SPI mode enable"]
    #[inline(always)]
    pub fn qmod(&mut self) -> QMOD_W {
        QMOD_W { w: self }
    }
}
