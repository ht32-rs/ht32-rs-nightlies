#[doc = "Reader of register PDMA_CH10DADR"]
pub type R = crate::R<u32, super::PDMA_CH10DADR>;
#[doc = "Writer for register PDMA_CH10DADR"]
pub type W = crate::W<u32, super::PDMA_CH10DADR>;
#[doc = "Register PDMA_CH10DADR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMA_CH10DADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DADR`"]
pub type DADR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DADR`"]
pub struct DADR_W<'a> {
    w: &'a mut W,
}
impl<'a> DADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DADR"]
    #[inline(always)]
    pub fn dadr(&self) -> DADR_R {
        DADR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DADR"]
    #[inline(always)]
    pub fn dadr(&mut self) -> DADR_W {
        DADR_W { w: self }
    }
}
