#[doc = "Reader of register NPCTCFG1"]
pub type R = crate::R<u32, super::NPCTCFG1>;
#[doc = "Writer for register NPCTCFG1"]
pub type W = crate::W<u32, super::NPCTCFG1>;
#[doc = "Register NPCTCFG1 `reset()`'s with value 0xfcfc_fcfc"]
impl crate::ResetValue for super::NPCTCFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfcfc_fcfc
    }
}
#[doc = "Reader of field `COMHIZ`"]
pub type COMHIZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMHIZ`"]
pub struct COMHIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> COMHIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `COMHLD`"]
pub type COMHLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMHLD`"]
pub struct COMHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> COMHLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `COMWAIT`"]
pub type COMWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMWAIT`"]
pub struct COMWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `COMSET`"]
pub type COMSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMSET`"]
pub struct COMSET_W<'a> {
    w: &'a mut W,
}
impl<'a> COMSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Common memory data bus HiZ time"]
    #[inline(always)]
    pub fn comhiz(&self) -> COMHIZ_R {
        COMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Common memory hold time"]
    #[inline(always)]
    pub fn comhld(&self) -> COMHLD_R {
        COMHLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Common memory wait time"]
    #[inline(always)]
    pub fn comwait(&self) -> COMWAIT_R {
        COMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Common memory setup time"]
    #[inline(always)]
    pub fn comset(&self) -> COMSET_R {
        COMSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Common memory data bus HiZ time"]
    #[inline(always)]
    pub fn comhiz(&mut self) -> COMHIZ_W {
        COMHIZ_W { w: self }
    }
    #[doc = "Bits 16:23 - Common memory hold time"]
    #[inline(always)]
    pub fn comhld(&mut self) -> COMHLD_W {
        COMHLD_W { w: self }
    }
    #[doc = "Bits 8:15 - Common memory wait time"]
    #[inline(always)]
    pub fn comwait(&mut self) -> COMWAIT_W {
        COMWAIT_W { w: self }
    }
    #[doc = "Bits 0:7 - Common memory setup time"]
    #[inline(always)]
    pub fn comset(&mut self) -> COMSET_W {
        COMSET_W { w: self }
    }
}
