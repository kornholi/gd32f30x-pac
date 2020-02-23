#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `CCRCERR`"]
pub type CCRCERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTCRCERR`"]
pub type DTCRCERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDTMOUT`"]
pub type CMDTMOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTTMOUT`"]
pub type DTTMOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXURE`"]
pub type TXURE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXORE`"]
pub type RXORE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDRECV`"]
pub type CMDRECV_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDSEND`"]
pub type CMDSEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTEND`"]
pub type DTEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `STBITE`"]
pub type STBITE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTBLKEND`"]
pub type DTBLKEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDRUN`"]
pub type CMDRUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRUN`"]
pub type TXRUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRUN`"]
pub type RXRUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFH`"]
pub type TFH_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFH`"]
pub type RFH_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFF`"]
pub type TFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFF`"]
pub type RFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFE`"]
pub type TFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFE`"]
pub type RFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXDTVAL`"]
pub type TXDTVAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDTVAL`"]
pub type RXDTVAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDIOINT`"]
pub type SDIOINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ATAEND`"]
pub type ATAEND_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Command response received"]
    #[inline(always)]
    pub fn ccrcerr(&self) -> CCRCERR_R {
        CCRCERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received"]
    #[inline(always)]
    pub fn dtcrcerr(&self) -> DTCRCERR_R {
        DTCRCERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command response timeout"]
    #[inline(always)]
    pub fn cmdtmout(&self) -> CMDTMOUT_R {
        CMDTMOUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dttmout(&self) -> DTTMOUT_R {
        DTTMOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error occurs"]
    #[inline(always)]
    pub fn txure(&self) -> TXURE_R {
        TXURE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error occurs"]
    #[inline(always)]
    pub fn rxore(&self) -> RXORE_R {
        RXORE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Command response received"]
    #[inline(always)]
    pub fn cmdrecv(&self) -> CMDRECV_R {
        CMDRECV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command sent"]
    #[inline(always)]
    pub fn cmdsend(&self) -> CMDSEND_R {
        CMDSEND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data end"]
    #[inline(always)]
    pub fn dtend(&self) -> DTEND_R {
        DTEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start bit error in the bus"]
    #[inline(always)]
    pub fn stbite(&self) -> STBITE_R {
        STBITE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received"]
    #[inline(always)]
    pub fn dtblkend(&self) -> DTBLKEND_R {
        DTBLKEND_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Command transmission in progress"]
    #[inline(always)]
    pub fn cmdrun(&self) -> CMDRUN_R {
        CMDRUN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data transmission in progress"]
    #[inline(always)]
    pub fn txrun(&self) -> TXRUN_R {
        TXRUN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Data reception in progress"]
    #[inline(always)]
    pub fn rxrun(&self) -> RXRUN_R {
        RXRUN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO is half empty"]
    #[inline(always)]
    pub fn tfh(&self) -> TFH_R {
        TFH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO is half full"]
    #[inline(always)]
    pub fn rfh(&self) -> RFH_R {
        RFH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO is full"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO is full"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO is empty"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data is valid in transmit FIFO"]
    #[inline(always)]
    pub fn txdtval(&self) -> TXDTVAL_R {
        TXDTVAL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data is valid in receive FIFO"]
    #[inline(always)]
    pub fn rxdtval(&self) -> RXDTVAL_R {
        RXDTVAL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SD I/O interrupt received"]
    #[inline(always)]
    pub fn sdioint(&self) -> SDIOINT_R {
        SDIOINT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received"]
    #[inline(always)]
    pub fn ataend(&self) -> ATAEND_R {
        ATAEND_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
