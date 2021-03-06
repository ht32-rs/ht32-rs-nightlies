#[doc = "Reader of register CKCU_AHBCFGR"]
pub type R = crate::R<u32, super::CKCU_AHBCFGR>;
#[doc = "Writer for register CKCU_AHBCFGR"]
pub type W = crate::W<u32, super::CKCU_AHBCFGR>;
#[doc = "Register CKCU_AHBCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CKCU_AHBCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AHBPRE`"]
pub type AHBPRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHBPRE`"]
pub struct AHBPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - AHBPRE"]
    #[inline(always)]
    pub fn ahbpre(&self) -> AHBPRE_R {
        AHBPRE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AHBPRE"]
    #[inline(always)]
    pub fn ahbpre(&mut self) -> AHBPRE_W {
        AHBPRE_W { w: self }
    }
}
