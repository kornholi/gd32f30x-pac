#[doc = "Reader of register PIOTCFG3"]
pub type R = crate::R<u32, super::PIOTCFG3>;
#[doc = "Writer for register PIOTCFG3"]
pub type W = crate::W<u32, super::PIOTCFG3>;
#[doc = "Register PIOTCFG3 `reset()`'s with value 0xfcfc_fcfc"]
impl crate::ResetValue for super::PIOTCFG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfcfc_fcfc
    }
}
#[doc = "Reader of field `IOHIZ`"]
pub type IOHIZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOHIZ`"]
pub struct IOHIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `IOHLD`"]
pub type IOHLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOHLD`"]
pub struct IOHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `IOWAIT`"]
pub type IOWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOWAIT`"]
pub struct IOWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> IOWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `IOSET`"]
pub type IOSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOSET`"]
pub struct IOSET_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - IO space data bus HiZ time"]
    #[inline(always)]
    pub fn iohiz(&self) -> IOHIZ_R {
        IOHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IO space hold time"]
    #[inline(always)]
    pub fn iohld(&self) -> IOHLD_R {
        IOHLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - IO space wait time"]
    #[inline(always)]
    pub fn iowait(&self) -> IOWAIT_R {
        IOWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - IO space setup time"]
    #[inline(always)]
    pub fn ioset(&self) -> IOSET_R {
        IOSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - IO space data bus HiZ time"]
    #[inline(always)]
    pub fn iohiz(&mut self) -> IOHIZ_W {
        IOHIZ_W { w: self }
    }
    #[doc = "Bits 16:23 - IO space hold time"]
    #[inline(always)]
    pub fn iohld(&mut self) -> IOHLD_W {
        IOHLD_W { w: self }
    }
    #[doc = "Bits 8:15 - IO space wait time"]
    #[inline(always)]
    pub fn iowait(&mut self) -> IOWAIT_W {
        IOWAIT_W { w: self }
    }
    #[doc = "Bits 0:7 - IO space setup time"]
    #[inline(always)]
    pub fn ioset(&mut self) -> IOSET_W {
        IOSET_W { w: self }
    }
}
