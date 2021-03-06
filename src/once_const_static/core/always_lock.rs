
use crate::once_const_static::generic::UnsafeGenericStaticData;
use crate::once_const_static::UnkStaticData;
use crate::once_const_static::generic::GenericStaticData;
use crate::err::IgnoreInitErr;
use crate::err::StaticErr;
use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct AlwaysLockOnce;

impl AlwaysLockOnce {
	#[inline(always)]
	pub const fn new() -> Self {
		AlwaysLockOnce
	}
}

impl<T> UnkStaticData<T, AlwaysLockOnce> {
	#[inline]
	pub const fn new(a: T) -> Self {
		Self {
			data: UnsafeCell::new(a),
			sync_data: AlwaysLockOnce::new(),
		}
	}
}


//UNSAFE
impl<T> UnsafeGenericStaticData<T> for UnkStaticData<T, AlwaysLockOnce> {
	#[inline]
	unsafe fn set_box(&self, v: Box<T>) -> Result<(), StaticErr<Box<T>>> {
		Err(StaticErr::allow(v))
	}
	
	#[inline]
	unsafe fn set_raw(&self, v: T) -> Result<(), StaticErr<T>> {
		Err(StaticErr::allow(v))
	}
}


impl<T> GenericStaticData<T> for UnkStaticData<T, AlwaysLockOnce> {
	#[inline]
	fn set(&self, v: T) -> Result<(), StaticErr<T>> {
		Err(StaticErr::allow(v))
	}
	
	#[inline]
	fn replace(&self, v: T) -> Result<T, StaticErr<T>> {
		Err(StaticErr::allow(v))
	}
	
	#[inline]
	unsafe fn unsafe_replace(&self, v: T) -> T {
		v
	}
	
	fn get<'a>(&'a self) -> &'a T {
		unsafe{ &*self.data.get() }
	}
	
	#[inline]
	fn ignore_initialize(&self) -> Result<(), IgnoreInitErr> {
		Ok( () )
	}
	
	#[inline]
	fn ignore_initialize_dont_result(&self) {
		
	}

	#[inline]
	fn is_init_state(&self) -> bool {
		true
	}
	
	#[inline]
	fn is_noinit_state(&self) -> bool {
		false
	}	
}
