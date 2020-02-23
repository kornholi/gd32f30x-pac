#[doc = "Reader of register SNTCFG0"]
pub type R = crate::R<u32, super::SNTCFG0>;
#[doc = "Writer for register SNTCFG0"]
pub type W = crate::W<u32, super::SNTCFG0>;
#[doc = "Register SNTCFG0 `reset()`'s with value 0x0fff_ffff"]
impl crate::ResetValue for super::SNTCFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_ffff
    }
}
#[doc = "Reader of field `ASYNCMOD`"]
pub type ASYNCMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ASYNCMOD`"]
pub struct ASYNCMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `DLAT`"]
pub type DLAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLAT`"]
pub struct DLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CKDIV`"]
pub type CKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKDIV`"]
pub struct CKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `BUSLAT`"]
pub type BUSLAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUSLAT`"]
pub struct BUSLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSLAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DSET`"]
pub type DSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSET`"]
pub struct DSET_W<'a> {
    w: &'a mut W,
}
impl<'a> DSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `AHLD`"]
pub type AHLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHLD`"]
pub struct AHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> AHLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ASET`"]
pub type ASET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ASET`"]
pub struct ASET_W<'a> {
    w: &'a mut W,
}
impl<'a> ASET_W<'a> {
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
    pub fn asyncmod(&self) -> ASYNCMOD_R {
        ASYNCMOD_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Data latency for NOR Flash"]
    #[inline(always)]
    pub fn dlat(&self) -> DLAT_R {
        DLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Synchronous clock divide ratio"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&self) -> BUSLAT_R {
        BUSLAT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn dset(&self) -> DSET_R {
        DSET_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn ahld(&self) -> AHLD_R {
        AHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn aset(&self) -> ASET_R {
        ASET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    pub fn asyncmod(&mut self) -> ASYNCMOD_W {
        ASYNCMOD_W { w: self }
    }
    #[doc = "Bits 24:27 - Data latency for NOR Flash"]
    #[inline(always)]
    pub fn dlat(&mut self) -> DLAT_W {
        DLAT_W { w: self }
    }
    #[doc = "Bits 20:23 - Synchronous clock divide ratio"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CKDIV_W {
        CKDIV_W { w: self }
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&mut self) -> BUSLAT_W {
        BUSLAT_W { w: self }
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn dset(&mut self) -> DSET_W {
        DSET_W { w: self }
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn ahld(&mut self) -> AHLD_W {
        AHLD_W { w: self }
    }
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn aset(&mut self) -> ASET_W {
        ASET_W { w: self }
    }
}
