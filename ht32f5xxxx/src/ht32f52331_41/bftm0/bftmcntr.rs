#[doc = "Register `BFTMCNTR` reader"]
pub struct R(crate::R<BFTMCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFTMCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFTMCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFTMCNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFTMCNTR` writer"]
pub struct W(crate::W<BFTMCNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFTMCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BFTMCNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFTMCNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - CNT"]
pub type CNT_R = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - CNT"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, BFTMCNTR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNT"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BFTMCNTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bftmcntr](index.html) module"]
pub struct BFTMCNTR_SPEC;
impl crate::RegisterSpec for BFTMCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bftmcntr::R](R) reader structure"]
impl crate::Readable for BFTMCNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bftmcntr::W](W) writer structure"]
impl crate::Writable for BFTMCNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFTMCNTR to value 0"]
impl crate::Resettable for BFTMCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}