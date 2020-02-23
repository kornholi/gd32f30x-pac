#[doc = "Reader of register CMDAGMT"]
pub type R = crate::R<u32, super::CMDAGMT>;
#[doc = "Writer for register CMDAGMT"]
pub type W = crate::W<u32, super::CMDAGMT>;
#[doc = "Register CMDAGMT `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDAGMT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDAGMT`"]
pub type CMDAGMT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CMDAGMT`"]
pub struct CMDAGMT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDAGMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SDIO card command argument"]
    #[inline(always)]
    pub fn cmdagmt(&self) -> CMDAGMT_R {
        CMDAGMT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SDIO card command argument"]
    #[inline(always)]
    pub fn cmdagmt(&mut self) -> CMDAGMT_W {
        CMDAGMT_W { w: self }
    }
}
