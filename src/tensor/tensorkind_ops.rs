#![allow(unused_variables)]
use tensor::*;
use num::NumCast;

macro_rules! impl_tk_dispatch_self_ref {
    ($key:ident, $var:ident, $action:expr ) => {(
        match * $key {
            TensorKind::FloatTensor(ref $var) => TensorKind::FloatTensor($action) ,
            TensorKind::LongTensor(ref $var) => TensorKind::LongTensor($action) ,
            TensorKind::ByteTensor(ref $var) => TensorKind::ByteTensor($action) ,
        }
    )}
}
macro_rules! impl_tk_dispatch_self_ref_other {
    ($key:ident, $var:ident, $action:expr ) => {(
        match * $key {
            TensorKind::FloatTensor(ref $var) => $action,
            TensorKind::LongTensor(ref $var) => $action,
            TensorKind::ByteTensor(ref $var) => $action,
        }
    )}
}
macro_rules! impl_tk_dispatch_self_mut {
    ($key:ident, $var:ident, $action:expr ) => {(
        match * $key {
            TensorKind::FloatTensor(ref mut $var) => $action,
            TensorKind::LongTensor(ref mut $var) => $action,
            TensorKind::ByteTensor(ref mut $var) => $action,
        }
    )}
}

macro_rules! impl_tk_dispatch_self_mut_value {
    ($key:ident, $var:ident, $value:ident, $action:expr) => {(
		{
        match * $key {
            TensorKind::FloatTensor(ref mut $var) => {
                let $value = <f32 as NumCast>::from($value).unwrap();
                $action;
            }
            TensorKind::LongTensor(ref mut $var) => {
                let $value = <i64 as NumCast>::from($value).unwrap();
                $action;
            }
            TensorKind::ByteTensor(ref mut $var) => {
                let $value = <u8 as NumCast>::from($value).unwrap();
                $action;
            }
        };
		}
    )}
}

macro_rules! impl_tk_dispatch_self_value {
    ($key:ident, $var:ident, $value:ident, $action:expr ) => {(
        {
        match * $key {
            TensorKind::FloatTensor(ref $var) => {
                let $value = <f32 as NumCast>::from($value).unwrap();
                $action
            }
            TensorKind::LongTensor(ref $var) => {
                let $value = <i64 as NumCast>::from($value).unwrap();
                $action
            }
            TensorKind::ByteTensor(ref $var) => {
                let $value = <u8 as NumCast>::from($value).unwrap();
                $action
            }
        }
        }
    )}
}

macro_rules! impl_tk_dispatch_self_mut_value2 {
    ($key:ident, $var:ident, $value1:ident, $value2:ident, $action:expr) => {(
		{
        match * $key {
            TensorKind::FloatTensor(ref mut $var) => {
                let $value1 = <f32 as NumCast>::from($value1).unwrap();
                let $value2 = <f32 as NumCast>::from($value2).unwrap();
                $action;
            }
            TensorKind::LongTensor(ref mut $var) => {
                let $value1 = <i64 as NumCast>::from($value1).unwrap();
                let $value2 = <i64 as NumCast>::from($value2).unwrap();
                $action;
            }
            TensorKind::ByteTensor(ref mut $var) => {
                let $value1 = <u8 as NumCast>::from($value1).unwrap();
                let $value2 = <u8 as NumCast>::from($value2).unwrap();
                $action;
            }
        };
		}
    )}
}

macro_rules! impl_tk_dispatch_self_value2 {
    ($key:ident, $var:ident, $value1:ident, $value2:ident, $action:expr ) => {(
        {
        match * $key {
            TensorKind::FloatTensor(ref $var) => {
                let $value1 = <f32 as NumCast>::from($value1).unwrap();
                let $value2 = <f32 as NumCast>::from($value2).unwrap();
                $action
            }
            TensorKind::LongTensor(ref $var) => {
                let $value1 = <i64 as NumCast>::from($value1).unwrap();
                let $value2 = <i64 as NumCast>::from($value2).unwrap();
                $action
            }
            TensorKind::ByteTensor(ref $var) => {
                let $value1 = <u8 as NumCast>::from($value1).unwrap();
                let $value2 = <u8 as NumCast>::from($value2).unwrap();
                $action
            }
        }
        }
    )}
}


impl TensorKind {
    pub fn abs<T: NumLimits>(&self) -> Self {
        (self.into(): &Tensor<T>).abs().into()
    }
    pub fn abs_<T: NumLimits>(&mut self) -> &mut Self {
        (self.into(): &mut Tensor<T>).abs_();
        self
    }
    pub fn acos<T: NumLimits>(&self) -> Self {
        (self.into(): &Tensor<T>).acos().into()
    }
    pub fn acos_<T: NumLimits>(&mut self) -> &mut Self {
        (self.into(): &mut Tensor<T>).acos_();
        self
    }
    pub fn add<T: NumLimits>(&self, rhs: T) -> Self {
        (self.into(): &Tensor<T>).add(rhs).into()
    }
    pub fn add_<T: NumLimits>(&mut self, rhs: T) -> &mut Self {
        (self.into(): &mut Tensor<T>).add_(rhs);
        self
    }
    pub fn addt<T: NumLimits>(&self, val: T, rhs: &Self) -> Self {
        impl_tk_dispatch_self_value!(self, t, val, t.addt(val, rhs.into()).into())
    }
    pub fn addt_<T: NumLimits>(&mut self, val: T, rhs: &Self) -> &mut Self {
        impl_tk_dispatch_self_mut_value!(self, t, val, {t.addt(val, rhs.into());});
        self
    }
    pub fn addbmm<T: NumLimits>(&self, beta: T, alpha: T, tensor1: &Self, tensor2: &Self) -> Self {
        unimplemented!()
    }
    pub fn addbmm_<T: NumLimits>(&mut self,
                                 beta: T,
                                 alpha: T,
                                 tensor1: &Self,
                                 tensor2: &Self)
                                 -> &mut Self {
        unimplemented!()
    }
    pub fn addcdiv<T: NumLimits>(&self, value: T, tensor1: &Self, tensor2: &Self) -> Self {
        unimplemented!()
    }
    pub fn addcdiv_<T: NumLimits>(&mut self,
                                  value: T,
                                  tensor1: &Self,
                                  tensor2: &Self)
                                  -> &mut Self {
        unimplemented!()
    }
    pub fn addcmul<T: NumLimits>(&self, value: T, tensor1: &Self, tensor2: &Self) -> Self {
        unimplemented!()
    }
    pub fn addcmul_<T: NumLimits>(&mut self,
                                  value: T,
                                  tensor1: &Self,
                                  tensor2: &Self)
                                  -> &mut Self {
        unimplemented!()
    }
    pub fn addmm<T: NumLimits>(&self, beta: T, alpha: T, mat1: &Self, mat2: &Self) -> Self {
        impl_tk_dispatch_self_value2!(self, t, beta, alpha,
			t.addmm(beta, alpha, mat1.into(), mat2.into()).into())
    }
    pub fn addmm_<T: NumLimits>(&mut self,
                                beta: T,
                                alpha: T,
                                mat1: &Self,
                                mat2: &Self)
                                -> &mut Self {
        impl_tk_dispatch_self_mut_value2!(self, t, beta, alpha,
			{t.addmm_(beta, alpha, mat1.into(), mat2.into());});
        self
    }
    pub fn addmv<T: NumLimits>(&self, beta: T, alpha: T, tensor1: &Self, vec: &Self) -> Self {
        unimplemented!()
    }
    pub fn addmv_<T: NumLimits>(&mut self,
                                beta: T,
                                alpha: T,
                                tensor1: &Self,
                                vec: &Self)
                                -> &mut Self {
        unimplemented!()
    }
    pub fn addr<T: NumLimits>(&self, beta: T, alpha: T, vec1: &Self, vec2: &Self) -> Self {
        unimplemented!()
    }
    pub fn addr_<T: NumLimits>(&mut self,
                               beta: T,
                               alpha: T,
                               vec1: &Self,
                               vec2: &Self)
                               -> &mut Self {
        unimplemented!()
    }
    pub fn asin<T: NumLimits>(&self) -> Self {
        unimplemented!()
    }
    pub fn asin_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn atan<T: NumLimits>(&self) -> Self {
        unimplemented!()
    }
    pub fn atan2<T: NumLimits>(&self) -> Self {
        unimplemented!()
    }
    pub fn atan2_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn baddbmm<T: NumLimits>(&self, beta: T, alpha: T, tensor1: &Self, tensor2: &Self) -> Self {
        unimplemented!()
    }
    pub fn baddbmm_<T: NumLimits>(&mut self,
                                  beta: T,
                                  alpha: T,
                                  tensor1: &Self,
                                  tensor2: &Self)
                                  -> &mut Self {
        unimplemented!()
    }
    pub fn bernoulli(&self, p: f64) -> Self {
        impl_tk_dispatch_self_ref!(self, t, t.bernoulli(p).into())
    }
    pub fn bernoulli_(&mut self, p: f64) -> &mut Self {
        impl_tk_dispatch_self_mut!(self, v, {v.bernoulli(p);});
        self
    }
    pub fn bmm<T: NumLimits>(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn byte(&mut self) -> &mut Self {
        unimplemented!()
    }
    //
    // cauchy_
    //
    pub fn ceil(&self) -> Self {
        impl_tk_dispatch_self_ref!(self, t, t.ceil())
    }
    pub fn ceil_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn char(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn chunk<T: NumLimits>(&self, n_chunks: usize, dim: usize) -> Vec<Self> {
        unimplemented!()
    }
    pub fn clamp<T: NumLimits>(&self, min: T, max: T) -> Self {
        unimplemented!()
    }
    pub fn clamp_<T: NumLimits>(&mut self, min: T, max: T) -> &mut Self {
        unimplemented!()
    }
    pub fn contiguous(&self) -> Self {
        println!("XXX implement contiguous");
        self.clone()
    }
    // perform deep copy
    pub fn copy(&self) -> Self {
        impl_tk_dispatch_self_ref!(self, t, t.copy().into())
    }
    pub fn copy_(&mut self, src: &Self) -> &mut Self {
        impl_tk_dispatch_self_mut!(self, t, {t.copy_(src.into());});
        self
    }
    pub fn copy_async_(&mut self, src: &Self) -> &mut Self {
        impl_tk_dispatch_self_mut!(self, t, {t.copy_async_(src.into());});
        self
    }
    pub fn cos(&self) -> Self {
        unimplemented!()
    }
    pub fn cos_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn cosh(&self) -> Self {
        unimplemented!()
    }
    pub fn cosh_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn cpu(&self) -> Self {
        unimplemented!()
    }
    pub fn cross<T: NumLimits>(&self, dim: Option<i32>) -> Self {
        unimplemented!()
    }
    pub fn cuda(&self, device: Option<i32>) -> Self {
        unimplemented!()
    }
    pub fn cuda_async(&self, device: Option<i32>) -> Self {
        unimplemented!()
    }
    pub fn diag<T: NumLimits>(&self, diag: u32) -> Self {
        unimplemented!()
    }
    pub fn dim(&self) -> i32 {
        impl_tk_dispatch_self_ref_other!(self, t, t.dim())
    }
    pub fn dist<T: NumLimits>(&self, other: &Self, p: u32) -> f32 {
        unimplemented!()
    }
    pub fn div<T: NumLimits>(&self, value: T) -> Self {
        impl_tk_dispatch_self_value!(self, t, value, t.div(value).into())
    }
    pub fn div_<T: NumLimits>(&mut self, value: T) -> &mut Self {
        impl_tk_dispatch_self_mut_value!(self, t, value, t.div_(value));
        self
    }
    pub fn divt<T: NumLimits>(&self, value: &Self) -> Self {
        impl_tk_dispatch_self_ref_other!(self, t, t.divt(value.into()).into())
    }
    pub fn divt_(&mut self, value: &Self) -> &mut Self {
        impl_tk_dispatch_self_mut!(self, t, {t.divt_(value.into());});
        self
    }
    pub fn dot(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn double(&self) -> Self {
        unimplemented!()
    }
    pub fn eig(&self, eigenvectors: bool) -> (Self, Self) {
        unimplemented!()
    }
    pub fn element_size(&self) -> i32 {
        unimplemented!()
    }
    pub fn eq_tensor(&self, other: &Self) -> Tensor<u8> {
        unimplemented!()
    }
    pub fn eq_tensor_(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn exp<T: NumLimits>(&self) -> Self {
        unimplemented!()
    }
    pub fn exp_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn expand<D>(&self, dims: D) -> Self
        where D: AsRef<[usize]>
    {
        impl_tk_dispatch_self_ref!(self, t, t.expand(dims).into())
    }
    pub fn expand_as(&self, tensor: &Self) -> Self {
        impl_tk_dispatch_self_ref!(self, t, t.expand_as(tensor.into()).into())
    }
    pub fn fill_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn float(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn floor(&self) -> Self {
        unimplemented!()
    }
    pub fn floor_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn fmod<T: NumLimits>(&self, divisor: T) -> Self {
        (self.into(): &Tensor<T>).fmod(divisor).into()
    }
    pub fn fmod_<T: NumLimits>(&mut self, divisor: T) -> &mut Self {
        (self.into(): &mut Tensor<T>).fmod_(divisor);
        self
    }
    pub fn frac(&self) -> Self {
        unimplemented!()
    }
    pub fn frac_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn gather(&self, dim: i32, index: Tensor<i64>) {
        unimplemented!()
    }
    pub fn ge_tensor(&self, other: &Self) -> Tensor<u8> {
        unimplemented!()
    }
    pub fn ge_tensor_(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn gels(&self, other: &Self) -> Self {
        unimplemented!();
    }
    pub fn gt_tensor(&self, other: &Self) -> Tensor<u8> {
        unimplemented!()
    }
    pub fn gt_tensor_(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn half(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn id(&self) -> usize {
        impl_tk_dispatch_self_ref_other!(self, t, t.id())
    }
    pub fn index_masked(&self, m: &Tensor<u8>) -> Self {
        unimplemented!()
    }
    pub fn index_add_(&mut self, dim: i32, index: Tensor<i64>, tensor: &Self) -> &mut Self {
        unimplemented!()
    }
    pub fn index_copy_(&mut self, dim: i32, index: Tensor<i64>, tensor: &Self) -> &mut Self {
        unimplemented!()
    }
    pub fn index_fill_(&mut self, dim: i32, index: Tensor<i64>, val: f32) -> &mut Self {
        unimplemented!()
    }
    pub fn index_select(&self, dim: i32, index: Tensor<i64>) -> Self {
        unimplemented!()
    }
    pub fn inner(&self) -> *mut ::std::os::raw::c_void {
        impl_tk_dispatch_self_ref_other!(self, t, t.inner())
    }
    pub fn int(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn is_cuda(&self) -> bool {
        unimplemented!()
    }
    pub fn is_pinned(&self) -> bool {
        unimplemented!()
    }
    pub fn is_set_to(&self, tensor: &Self) -> bool {
        unimplemented!()
    }
    pub fn is_signed(&self) -> bool {
        unimplemented!()
    }
    pub fn kthvalue(&self, k: i32, dim: Option<i32>) -> (Self, Tensor<i64>) {
        unimplemented!()
    }
    pub fn le_tensor(&self, other: &Self) -> Tensor<u8> {
        unimplemented!()
    }
    pub fn le_tensor_(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn le_value<T: NumLimits>(&self, value: T) -> Tensor<u8> {
        impl_tk_dispatch_self_value!(self, t, value, t.le_value(value))
    }
    pub fn lerp(&self, start: &Self, end: &Self, weight: f32) -> Self {
        unimplemented!()
    }
    pub fn lerp_(&self, start: &Self, end: &Self, weight: f32) -> Self {
        unimplemented!()
    }
    pub fn log(&self) -> Self {
        unimplemented!()
    }
    pub fn log_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn log1p(&self) -> Self {
        unimplemented!()
    }
    pub fn log1p_(&mut self) -> &mut Self {
        unimplemented!()
    }
    //
    // log_normal(...)
    //
    pub fn long(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn lt_tensor(&self, other: &Self) -> Tensor<u8> {
        unimplemented!()
    }
    pub fn lt_tensor_(&mut self, other: &Self) -> &mut Self {
        unimplemented!()
    }
    //
    // map_
    //
    pub fn masked_copy_(&mut self, mask: Tensor<u8>, source: &Self) -> &mut Self {
        unimplemented!()
    }
    pub fn masked_fill_<T: NumLimits>(&mut self, mask: Tensor<u8>, value: T) -> &mut Self {
        impl_tk_dispatch_self_mut_value!(self, t, value, t.masked_fill_(mask, value));
        self
    }
    pub fn masked_fill<T: NumLimits>(&self, mask: Tensor<u8>, value: T) -> Self {
        impl_tk_dispatch_self_value!(self, t, value, t.masked_fill(mask, value).into())
    }
    pub fn masked_select(&self, mask: Tensor<u8>) -> Self {
        unimplemented!()
    }
    pub fn max<T: NumLimits>(&self) -> T {
        unimplemented!()
    }
    pub fn max_reduce(&self, dim: i32) -> (Self, Tensor<i64>) {
        unimplemented!()
    }
    pub fn mean<T: NumLimits>(&self) -> T {
        unimplemented!()
    }
    pub fn mean_reduce(&self, dim: i32) -> (Self, Tensor<i64>) {
        unimplemented!()
    }
    //
    // median
    //
    pub fn min<T: NumLimits>(&self) -> T {
        unimplemented!()
    }
    pub fn min_reduce(&self, dim: i32) -> (Self, Tensor<i64>) {
        unimplemented!()
    }
    pub fn mm(&self, rhs: &Self) -> Self {
        impl_tk_dispatch_self_ref!(self, t, t.mm(rhs.into()).into())
    }
    //
    // mode
    //
    pub fn mul<T: NumLimits>(&self, rhs: T) -> Self {
        (self.into(): &Tensor<T>).mul(rhs).into()
    }
    pub fn mul_<T: NumLimits>(&mut self, rhs: T) -> &mut Self {
        (self.into(): &mut Tensor<T>).div(rhs);
        self
    }
    pub fn mult(&self, rhs: &Self) -> Self {
        impl_tk_dispatch_self_ref!(self, t, t.mult(rhs.into()).into())
    }
    pub fn mult_(&mut self, rhs: &Self) -> &mut Self {
        impl_tk_dispatch_self_mut!(self, t, {t.mult(rhs.into());});
        self
    }
    //
    // multinomial
    //
    pub fn mv(&self, vec: &Self) -> Self {
        unimplemented!()
    }
    pub fn narrow(&self, dim: i32, start: i32, length: i32) -> Self {
        unimplemented!()
    }
    pub fn ndimension(&self) -> i32 {
        unimplemented!()
    }
    pub fn ne_tensor(&self, other: &Self) -> Tensor<u8> {
        unimplemented!()
    }
    pub fn ne_tensor_(&mut self, other: &Self) -> &mut Self {
        unimplemented!()
    }
    pub fn neg(&self) -> Self {
        unimplemented!()
    }
    pub fn neg_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn nonzero(&self) -> Tensor<i64> {
        unimplemented!()
    }
    pub fn norm(&self, p: i32) -> f32 {
        unimplemented!()
    }
    //
    // normal_
    //
    pub fn numel(&self) -> i32 {
        unimplemented!()
    }
    //
    // numpy() (need native tensor equivalent - rust-ndarray?)
    //
    //
    // orgqr
    //
    // ormqr
    //
    pub fn permute(&self, dims: &[u32]) -> Self {
        unimplemented!()
    }
    pub fn pin_memory(&mut self) -> &mut Self {
        unimplemented!()
    }
    //
    // potrf
    //
    //
    // potri
    //
    //
    // potrs
    //
    pub fn pow(&self) -> Self {
        unimplemented!()
    }
    pub fn pow_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn prod<T: NumLimits>(&self) -> T {
        unimplemented!()
    }
    //
    // pstrf
    //
    //
    // qr
    //
    //
    // random_
    //
    pub fn reciprocal(&self) -> Self {
        unimplemented!()
    }
    pub fn reciprocal_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn remainder<T: NumLimits>(&self, divisor: T) -> Self {
        unimplemented!()
    }
    pub fn remainder_<T: NumLimits>(&mut self, divisor: T) -> &mut Self {
        unimplemented!()
    }
    //
    // renorm
    //
    //
    // renorm_
    //
    pub fn repeat(&self, sizes: &[i32]) -> Self {
        // NB: copies data
        unimplemented!()
    }
    pub fn resize_<D>(&mut self, sizes: D) -> Self
        where D: AsRef<[usize]>
    {
        impl_tk_dispatch_self_mut!(self, t, t.resize_(sizes).into())
    }
    pub fn resize_as_(&mut self, tensor: &Self) -> Self {
        impl_tk_dispatch_self_mut!(self, t, t.resize_(tensor.size()).into())
    }
    pub fn round(&self) -> Self {
        unimplemented!()
    }
    pub fn round_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn rsqrt(&self) -> Self {
        unimplemented!()
    }
    pub fn rsqrt_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn scatter_(&mut self, dim: i32, index: Tensor<i64>, src: &Self) -> &mut Self {
        unimplemented!()
    }
    pub fn select(&self, dim: i32, index: i32) -> Self {
        unimplemented!()
    }
    //
    // set_
    //
    //
    // share_memory_
    //
    pub fn short(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn sigmoid(&self) -> Self {
        unimplemented!()
    }
    pub fn sigmoid_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn sign(&self) -> Self {
        unimplemented!()
    }
    pub fn sign_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn sin(&self) -> Self {
        unimplemented!()
    }
    pub fn sin_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn sinh(&self) -> Self {
        unimplemented!()
    }
    pub fn sinh_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn size(&self) -> Vec<usize> {
        impl_tk_dispatch_self_ref_other!(self, t, t.size())
    }
    pub fn sort(&self, dim: Option<i32>, descending: bool) -> (Self, Tensor<i64>) {
        unimplemented!()
    }
    pub fn sqrt(&self) -> Self {
        unimplemented!()
    }
    pub fn sqrt_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn squeeze(&self, dim: Option<usize>) -> Self {
        impl_tk_dispatch_self_ref!(self, t, t.squeeze(dim))
    }
    pub fn squeeze_(&mut self, dim: Option<usize>) -> &mut Self {
        impl_tk_dispatch_self_mut!(self, t, {t.squeeze_(dim);});
        self
    }
    pub fn std<T: NumLimits>(&self) -> T {
        unimplemented!()
    }
    //
    // storage
    //
    //
    // storage_offset
    //
    pub fn stride(&self) -> Vec<i32> {
        unimplemented!()
    }
    pub fn sub(&self, rhs: &Self) -> Self {
        unimplemented!()
    }
    pub fn sub_(&mut self, rhs: &Self) -> &mut Self {
        unimplemented!()
    }
    pub fn sum<T: NumLimits>(&self) -> T {
        impl_tk_dispatch_self_ref_other!(self, t, t.sum())
    }
    pub fn sum_reduce(&self, dim: usize, keepdim: bool) -> Self {
        impl_tk_dispatch_self_ref!(self, t, t.sum_reduce(dim, keepdim).into())
    }
    pub fn svd(&self, some: bool) -> (Self, Self, Self) {
        unimplemented!()
    }
    //
    // symeig
    //
    pub fn t(&self) -> Self {
        impl_tk_dispatch_self_ref_other!(self, t, t.t().into())
    }
    pub fn t_(&mut self) -> &mut Self {
        impl_tk_dispatch_self_mut!(self, t, {t.t_();});
        self
    }
    pub fn tan(&self) -> Self {
        unimplemented!()
    }
    pub fn tan_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn tanh(&self) -> Self {
        unimplemented!()
    }
    pub fn tanh_(&mut self) -> &mut Self {
        unimplemented!()
    }
    //
    // tolist
    //
    pub fn topk(k: i32, dim: Option<i32>, largest: bool, sorted: bool) -> (Self, Tensor<i64>) {
        unimplemented!()
    }
    pub fn trace(&self) -> Self {
        unimplemented!()
    }
    pub fn transpose(&self, dim0: i32, dim1: i32) -> Self {
        unimplemented!()
    }
    pub fn transpose_(&self, dim0: i32, dim1: i32) -> Self {
        unimplemented!()
    }
    //
    // tril
    //
    //
    // tril_
    //
    //
    // triu
    //
    //
    // tril_
    //
    //
    // trtrs
    //
    pub fn trunc(&self) -> Self {
        unimplemented!()
    }
    pub fn trunc_(&mut self) -> &mut Self {
        unimplemented!()
    }
    pub fn type_as(&self, tensor: &Self) -> Self {
        unimplemented!()
    }
    pub fn typecast(&self, new_type: TensorType, async: bool) -> Self {
        unimplemented!()
    }
    pub fn unfold(&self, dim: i32, size: i32, step: i32) -> Self {
        unimplemented!()
    }
    pub fn uniform_(&mut self, range: (f64, f64)) -> &mut Self {
        impl_tk_dispatch_self_mut!(self, v, {v.uniform_(range);});
        self
    }
    pub fn unsqueeze(&self, dim: usize) -> Self {
        impl_tk_dispatch_self_ref!(self, t, t.unsqueeze(dim))
    }
    pub fn unsqueeze_(&mut self, dim: usize) -> &mut Self {
        impl_tk_dispatch_self_mut!(self, t, {t.unsqueeze(dim);});
        self
    }
    pub fn validate(&self, arg: &str) {
        impl_tk_dispatch_self_ref_other!(self, t, {t.validate(arg);});
    }
    pub fn var<T: NumLimits>(&self) -> T {
        unimplemented!()
    }
    pub fn view<D>(&self, dims: D) -> Self
        where D: AsRef<[isize]>
    {
        impl_tk_dispatch_self_ref!(self, t, (t.view(dims)))
    }
    pub fn view_as(&self, tensor: &Self) -> Self {
        unimplemented!()
    }
    pub fn zero_(&mut self) -> &mut Self {
        impl_tk_dispatch_self_mut!(self, t, {t.zero_();});
        self
    }
}
