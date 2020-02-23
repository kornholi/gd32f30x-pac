#[doc = "Reader of register DATACTL"]
pub type R = crate::R<u32, super::DATACTL>;
#[doc = "Writer for register DATACTL"]
pub type W = crate::W<u32, super::DATACTL>;
#[doc = "Register DATACTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DATACTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATAEN`"]
pub type DATAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAEN`"]
pub struct DATAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAEN_W<'a> {
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
#[doc = "Reader of field `DATADIR`"]
pub type DATADIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATADIR`"]
pub struct DATADIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATADIR_W<'a> {
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
#[doc = "Reader of field `TRANSMOD`"]
pub type TRANSMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANSMOD`"]
pub struct TRANSMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSMOD_W<'a> {
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
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
#[doc = "Reader of field `BLKSZ`"]
pub type BLKSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLKSZ`"]
pub struct BLKSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RWEN`"]
pub type RWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWEN`"]
pub struct RWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RWEN_W<'a> {
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
#[doc = "Reader of field `RWSTOP`"]
pub type RWSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWSTOP`"]
pub struct RWSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> RWSTOP_W<'a> {
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
#[doc = "Reader of field `RWTYPE`"]
pub type RWTYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWTYPE`"]
pub struct RWTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RWTYPE_W<'a> {
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
#[doc = "Reader of field `IOEN`"]
pub type IOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOEN`"]
pub struct IOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Data transfer enabled bit"]
    #[inline(always)]
    pub fn dataen(&self) -> DATAEN_R {
        DATAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn datadir(&self) -> DATADIR_R {
        DATADIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn transmod(&self) -> TRANSMOD_R {
        TRANSMOD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    pub fn blksz(&self) -> BLKSZ_R {
        BLKSZ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Read wait mode enabled"]
    #[inline(always)]
    pub fn rwen(&self) -> RWEN_R {
        RWEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Read wait type"]
    #[inline(always)]
    pub fn rwtype(&self) -> RWTYPE_R {
        RWTYPE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SD I/O specific function enable"]
    #[inline(always)]
    pub fn ioen(&self) -> IOEN_R {
        IOEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data transfer enabled bit"]
    #[inline(always)]
    pub fn dataen(&mut self) -> DATAEN_W {
        DATAEN_W { w: self }
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn datadir(&mut self) -> DATADIR_W {
        DATADIR_W { w: self }
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn transmod(&mut self) -> TRANSMOD_W {
        TRANSMOD_W { w: self }
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    pub fn blksz(&mut self) -> BLKSZ_W {
        BLKSZ_W { w: self }
    }
    #[doc = "Bit 8 - Read wait mode enabled"]
    #[inline(always)]
    pub fn rwen(&mut self) -> RWEN_W {
        RWEN_W { w: self }
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    pub fn rwstop(&mut self) -> RWSTOP_W {
        RWSTOP_W { w: self }
    }
    #[doc = "Bit 10 - Read wait type"]
    #[inline(always)]
    pub fn rwtype(&mut self) -> RWTYPE_W {
        RWTYPE_W { w: self }
    }
    #[doc = "Bit 11 - SD I/O specific function enable"]
    #[inline(always)]
    pub fn ioen(&mut self) -> IOEN_W {
        IOEN_W { w: self }
    }
}
