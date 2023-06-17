#[doc = "Register `SCTM_CNTCFR` reader"]
pub struct R(crate::R<SCTM_CNTCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTM_CNTCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTM_CNTCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTM_CNTCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTM_CNTCFR` writer"]
pub struct W(crate::W<SCTM_CNTCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTM_CNTCFR_SPEC>;
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
impl From<crate::W<SCTM_CNTCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTM_CNTCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEVDIS` reader - UEVDIS"]
pub type UEVDIS_R = crate::BitReader;
#[doc = "Field `UEVDIS` writer - UEVDIS"]
pub type UEVDIS_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_CNTCFR_SPEC, O>;
#[doc = "Field `UGDIS` reader - UGDIS"]
pub type UGDIS_R = crate::BitReader;
#[doc = "Field `UGDIS` writer - UGDIS"]
pub type UGDIS_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_CNTCFR_SPEC, O>;
#[doc = "Field `CKDIV` reader - CKDIV"]
pub type CKDIV_R = crate::FieldReader;
#[doc = "Field `CKDIV` writer - CKDIV"]
pub type CKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, SCTM_CNTCFR_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - UEVDIS"]
    #[inline(always)]
    pub fn uevdis(&self) -> UEVDIS_R {
        UEVDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UGDIS"]
    #[inline(always)]
    pub fn ugdis(&self) -> UGDIS_R {
        UGDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - UEVDIS"]
    #[inline(always)]
    #[must_use]
    pub fn uevdis(&mut self) -> UEVDIS_W<0> {
        UEVDIS_W::new(self)
    }
    #[doc = "Bit 1 - UGDIS"]
    #[inline(always)]
    #[must_use]
    pub fn ugdis(&mut self) -> UGDIS_W<1> {
        UGDIS_W::new(self)
    }
    #[doc = "Bits 8:9 - CKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv(&mut self) -> CKDIV_W<8> {
        CKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCTM_CNTCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctm_cntcfr](index.html) module"]
pub struct SCTM_CNTCFR_SPEC;
impl crate::RegisterSpec for SCTM_CNTCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctm_cntcfr::R](R) reader structure"]
impl crate::Readable for SCTM_CNTCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctm_cntcfr::W](W) writer structure"]
impl crate::Writable for SCTM_CNTCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTM_CNTCFR to value 0"]
impl crate::Resettable for SCTM_CNTCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}