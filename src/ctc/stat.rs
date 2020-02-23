#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `CKOKIF`"]
pub type CKOKIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CKWARNIF`"]
pub type CKWARNIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF`"]
pub type ERRIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `EREFIF`"]
pub type EREFIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CKERR`"]
pub type CKERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `REFMISS`"]
pub type REFMISS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRIMERR`"]
pub type TRIMERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `REFDIR`"]
pub type REFDIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `REFCAP`"]
pub type REFCAP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Clock trim OK interrupt flag"]
    #[inline(always)]
    pub fn ckokif(&self) -> CKOKIF_R {
        CKOKIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock trim warning interrupt flag"]
    #[inline(always)]
    pub fn ckwarnif(&self) -> CKWARNIF_R {
        CKWARNIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&self) -> ERRIF_R {
        ERRIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Expect reference interrupt flag"]
    #[inline(always)]
    pub fn erefif(&self) -> EREFIF_R {
        EREFIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clock trim error bit"]
    #[inline(always)]
    pub fn ckerr(&self) -> CKERR_R {
        CKERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reference sync pulse miss"]
    #[inline(always)]
    pub fn refmiss(&self) -> REFMISS_R {
        REFMISS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Trim value error bit"]
    #[inline(always)]
    pub fn trimerr(&self) -> TRIMERR_R {
        TRIMERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CTC trim counter direction when reference sync pulse"]
    #[inline(always)]
    pub fn refdir(&self) -> REFDIR_R {
        REFDIR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - CTC counter capture when reference sync pulse"]
    #[inline(always)]
    pub fn refcap(&self) -> REFCAP_R {
        REFCAP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
