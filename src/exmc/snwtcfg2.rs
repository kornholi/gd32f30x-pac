#[doc = "Reader of register SNWTCFG2"]
pub type R = crate::R<u32, super::SNWTCFG2>;
#[doc = "Writer for register SNWTCFG2"]
pub type W = crate::W<u32, super::SNWTCFG2>;
#[doc = "Register SNWTCFG2 `reset()`'s with value 0x0fff_ffff"]
impl crate::ResetValue for super::SNWTCFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_ffff
    }
}
#[doc = "Reader of field `WASYNCMOD`"]
pub type WASYNCMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WASYNCMOD`"]
pub struct WASYNCMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> WASYNCMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `WBUSLAT`"]
pub type WBUSLAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WBUSLAT`"]
pub struct WBUSLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> WBUSLAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WDSET`"]
pub type WDSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDSET`"]
pub struct WDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> WDSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WAHLD`"]
pub type WAHLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAHLD`"]
pub struct WAHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WAHLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `WASET`"]
pub type WASET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WASET`"]
pub struct WASET_W<'a> {
    w: &'a mut W,
}
impl<'a> WASET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    pub fn wasyncmod(&self) -> WASYNCMOD_R {
        WASYNCMOD_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn wbuslat(&self) -> WBUSLAT_R {
        WBUSLAT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn wdset(&self) -> WDSET_R {
        WDSET_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn wahld(&self) -> WAHLD_R {
        WAHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn waset(&self) -> WASET_R {
        WASET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    pub fn wasyncmod(&mut self) -> WASYNCMOD_W {
        WASYNCMOD_W { w: self }
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn wbuslat(&mut self) -> WBUSLAT_W {
        WBUSLAT_W { w: self }
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn wdset(&mut self) -> WDSET_W {
        WDSET_W { w: self }
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn wahld(&mut self) -> WAHLD_W {
        WAHLD_W { w: self }
    }
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn waset(&mut self) -> WASET_W {
        WASET_W { w: self }
    }
}
