#[doc = "Reader of register PWRCTL"]
pub type R = crate::R<u32, super::PWRCTL>;
#[doc = "Writer for register PWRCTL"]
pub type W = crate::W<u32, super::PWRCTL>;
#[doc = "Register PWRCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWRCTL`"]
pub type PWRCTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWRCTL`"]
pub struct PWRCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SDIO power control bits"]
    #[inline(always)]
    pub fn pwrctl(&self) -> PWRCTL_R {
        PWRCTL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDIO power control bits"]
    #[inline(always)]
    pub fn pwrctl(&mut self) -> PWRCTL_W {
        PWRCTL_W { w: self }
    }
}
