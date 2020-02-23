#[doc = "Reader of register CAR"]
pub type R = crate::R<u32, super::CAR>;
#[doc = "Writer for register CAR"]
pub type W = crate::W<u32, super::CAR>;
#[doc = "Register CAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CARL`"]
pub type CARL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CARL`"]
pub struct CARL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Counter auto reload value"]
    #[inline(always)]
    pub fn carl(&self) -> CARL_R {
        CARL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter auto reload value"]
    #[inline(always)]
    pub fn carl(&mut self) -> CARL_W {
        CARL_W { w: self }
    }
}
