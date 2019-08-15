use libc;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
#[no_mangle]
pub unsafe extern "C" fn calc_crc_16_ccitt(mut src: *const uint8_t,
                                           mut nbytes: size_t,
                                           mut crc: uint16_t) -> uint16_t {
    let mut ibyte: size_t = 0;
    let mut ibit: uint32_t = 0;
    let mut crcwk: uint32_t = ((crc as libc::c_int) << 8i32) as uint32_t;
    ibyte = 0i32 as size_t;
    while ibyte < nbytes {
        let fresh0 = src;
        src = src.offset(1);
        crcwk |= *fresh0 as libc::c_uint;
        ibit = 0i32 as uint32_t;
        while ibit < 8i32 as libc::c_uint {
            crcwk <<= 1i32;
            if 0 != crcwk & 0x1000000i32 as libc::c_uint {
                crcwk =
                    (crcwk as libc::c_ulong ^ 0x11021u64 << 8i32) as uint32_t
            }
            ibit = ibit.wrapping_add(1)
        }
        ibyte = ibyte.wrapping_add(1)
    }
    return (crcwk >> 8i32) as uint16_t;
}