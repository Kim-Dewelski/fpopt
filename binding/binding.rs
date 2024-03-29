/* automatically generated by rust-bindgen 0.69.4 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const _FENV_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const __GLIBC_USE_C2X_STRTOL: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 39;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const FE_INVALID: u32 = 1;
pub const FE_DIVBYZERO: u32 = 4;
pub const FE_OVERFLOW: u32 = 8;
pub const FE_UNDERFLOW: u32 = 16;
pub const FE_INEXACT: u32 = 32;
pub const FE_ALL_EXCEPT: u32 = 61;
pub const FE_TONEAREST: u32 = 0;
pub const FE_DOWNWARD: u32 = 1024;
pub const FE_UPWARD: u32 = 2048;
pub const FE_TOWARDZERO: u32 = 3072;
pub mod _bindgen_ty_1 {
    pub type Type = ::core::ffi::c_uint;
    pub const FE_INVALID: Type = 1;
    pub const __FE_DENORM: Type = 2;
    pub const FE_DIVBYZERO: Type = 4;
    pub const FE_OVERFLOW: Type = 8;
    pub const FE_UNDERFLOW: Type = 16;
    pub const FE_INEXACT: Type = 32;
}
pub mod _bindgen_ty_2 {
    pub type Type = ::core::ffi::c_uint;
    pub const FE_TONEAREST: Type = 0;
    pub const FE_DOWNWARD: Type = 1024;
    pub const FE_UPWARD: Type = 2048;
    pub const FE_TOWARDZERO: Type = 3072;
}
pub type fexcept_t = ::core::ffi::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fenv_t {
    pub __control_word: ::core::ffi::c_ushort,
    pub __glibc_reserved1: ::core::ffi::c_ushort,
    pub __status_word: ::core::ffi::c_ushort,
    pub __glibc_reserved2: ::core::ffi::c_ushort,
    pub __tags: ::core::ffi::c_ushort,
    pub __glibc_reserved3: ::core::ffi::c_ushort,
    pub __eip: ::core::ffi::c_uint,
    pub __cs_selector: ::core::ffi::c_ushort,
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub __data_offset: ::core::ffi::c_uint,
    pub __data_selector: ::core::ffi::c_ushort,
    pub __glibc_reserved5: ::core::ffi::c_ushort,
    pub __mxcsr: ::core::ffi::c_uint,
}
#[test]
fn bindgen_test_layout_fenv_t() {
    const UNINIT: ::core::mem::MaybeUninit<fenv_t> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<fenv_t>(),
        32usize,
        concat!("Size of: ", stringify!(fenv_t))
    );
    assert_eq!(
        ::core::mem::align_of::<fenv_t>(),
        4usize,
        concat!("Alignment of ", stringify!(fenv_t))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__control_word) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__control_word)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__glibc_reserved1) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__glibc_reserved1)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__status_word) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__status_word)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__glibc_reserved2) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__glibc_reserved2)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__tags) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__tags)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__glibc_reserved3) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__glibc_reserved3)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__eip) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__eip)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__cs_selector) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__cs_selector)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__data_offset) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__data_offset)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__data_selector) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__data_selector)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__glibc_reserved5) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__glibc_reserved5)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__mxcsr) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(fenv_t),
            "::",
            stringify!(__mxcsr)
        )
    );
}
impl fenv_t {
    #[inline]
    pub fn __opcode(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 11u8) as u32) }
    }
    #[inline]
    pub fn set___opcode(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 11u8, val as u64)
        }
    }
    #[inline]
    pub fn __glibc_reserved4(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(11usize, 5u8) as u32) }
    }
    #[inline]
    pub fn set___glibc_reserved4(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(11usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        __opcode: ::core::ffi::c_uint,
        __glibc_reserved4: ::core::ffi::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 11u8, {
            let __opcode: u32 = unsafe { ::core::mem::transmute(__opcode) };
            __opcode as u64
        });
        __bindgen_bitfield_unit.set(11usize, 5u8, {
            let __glibc_reserved4: u32 = unsafe { ::core::mem::transmute(__glibc_reserved4) };
            __glibc_reserved4 as u64
        });
        __bindgen_bitfield_unit
    }
}
extern "C" {
    pub fn feclearexcept(__excepts: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn fegetexceptflag(
        __flagp: *mut fexcept_t,
        __excepts: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn feraiseexcept(__excepts: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn fesetexceptflag(
        __flagp: *const fexcept_t,
        __excepts: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn fetestexcept(__excepts: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn fegetround() -> ::core::ffi::c_int;
    pub fn fesetround(__rounding_direction: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn fegetenv(__envp: *mut fenv_t) -> ::core::ffi::c_int;
    pub fn feholdexcept(__envp: *mut fenv_t) -> ::core::ffi::c_int;
    pub fn fesetenv(__envp: *const fenv_t) -> ::core::ffi::c_int;
    pub fn feupdateenv(__envp: *const fenv_t) -> ::core::ffi::c_int;
}
