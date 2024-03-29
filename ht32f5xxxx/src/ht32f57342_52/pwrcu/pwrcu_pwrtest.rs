#[doc = "Register `PWRCU_PWRTEST` reader"]
pub struct R(crate::R<PWRCU_PWRTEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCU_PWRTEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCU_PWRTEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCU_PWRTEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCU_PWRTEST` writer"]
pub struct W(crate::W<PWRCU_PWRTEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCU_PWRTEST_SPEC>;
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
impl From<crate::W<PWRCU_PWRTEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCU_PWRTEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRTEST` reader - PWRTEST"]
pub type PWRTEST_R = crate::FieldReader;
#[doc = "Field `PWRTEST` writer - PWRTEST"]
pub type PWRTEST_W<'a, const O: u8> = crate::FieldWriter<'a, PWRCU_PWRTEST_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - PWRTEST"]
    #[inline(always)]
    pub fn pwrtest(&self) -> PWRTEST_R {
        PWRTEST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PWRTEST"]
    #[inline(always)]
    #[must_use]
    pub fn pwrtest(&mut self) -> PWRTEST_W<0> {
        PWRTEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWRCU_PWRTEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_pwrtest](index.html) module"]
pub struct PWRCU_PWRTEST_SPEC;
impl crate::RegisterSpec for PWRCU_PWRTEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcu_pwrtest::R](R) reader structure"]
impl crate::Readable for PWRCU_PWRTEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcu_pwrtest::W](W) writer structure"]
impl crate::Writable for PWRCU_PWRTEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCU_PWRTEST to value 0"]
impl crate::Resettable for PWRCU_PWRTEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
