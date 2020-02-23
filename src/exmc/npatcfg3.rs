#[doc = "Reader of register NPATCFG3"]
pub type R = crate::R<u32, super::NPATCFG3>;
#[doc = "Writer for register NPATCFG3"]
pub type W = crate::W<u32, super::NPATCFG3>;
#[doc = "Register NPATCFG3 `reset()`'s with value 0xfcfc_fcfc"]
impl crate::ResetValue for super::NPATCFG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfcfc_fcfc
    }
}
#[doc = "Reader of field `ATTHIZ`"]
pub type ATTHIZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATTHIZ`"]
pub struct ATTHIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `ATTHLD`"]
pub type ATTHLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATTHLD`"]
pub struct ATTHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ATTWAIT`"]
pub type ATTWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATTWAIT`"]
pub struct ATTWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ATTSET`"]
pub type ATTSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATTSET`"]
pub struct ATTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Attribute memory data bus HiZ time"]
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Attribute memory hold time"]
    #[inline(always)]
    pub fn atthld(&self) -> ATTHLD_R {
        ATTHLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Attribute memory wait time"]
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Attribute memory setup time"]
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Attribute memory data bus HiZ time"]
    #[inline(always)]
    pub fn atthiz(&mut self) -> ATTHIZ_W {
        ATTHIZ_W { w: self }
    }
    #[doc = "Bits 16:23 - Attribute memory hold time"]
    #[inline(always)]
    pub fn atthld(&mut self) -> ATTHLD_W {
        ATTHLD_W { w: self }
    }
    #[doc = "Bits 8:15 - Attribute memory wait time"]
    #[inline(always)]
    pub fn attwait(&mut self) -> ATTWAIT_W {
        ATTWAIT_W { w: self }
    }
    #[doc = "Bits 0:7 - Attribute memory setup time"]
    #[inline(always)]
    pub fn attset(&mut self) -> ATTSET_W {
        ATTSET_W { w: self }
    }
}
