#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    #[no_mangle]
    fn strcat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn open(
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn lseek(
        __fd: libc::c_int,
        __offset: __off_t,
        __whence: libc::c_int,
    ) -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type off_t = __off_t;
pub type ZDLookupResult = libc::c_int;
pub const ZD_LOOKUP_ON_BORDER_SEGMENT: ZDLookupResult = 4;
pub const ZD_LOOKUP_ON_BORDER_VERTEX: ZDLookupResult = 3;
pub const ZD_LOOKUP_IN_EXCLUDED_ZONE: ZDLookupResult = 2;
pub const ZD_LOOKUP_IN_ZONE: ZDLookupResult = 1;
pub const ZD_LOOKUP_NOT_IN_ZONE: ZDLookupResult = 0;
pub const ZD_LOOKUP_PARSE_ERROR: ZDLookupResult = -1;
pub const ZD_LOOKUP_END: ZDLookupResult = -2;
pub const ZD_LOOKUP_IGNORE: ZDLookupResult = -3;
#[derive(Clone, Debug)]
#[repr(C)]
pub struct ZoneDetectResult {
    pub lookupResult: ZDLookupResult,
    pub polygonId: uint32_t,
    pub metaId: uint32_t,
    // TODO: maybe change this to &str
    // TODO: maybe combine these two fields into a hashmap
    pub fields: std::collections::HashMap<String, String>,
}
#[derive(Clone)]
#[repr(C)]
pub struct ZoneDetectOpaque {
    pub fd: libc::c_int,
    pub length: off_t,
    pub mapping: *const uint8_t,
    pub tableType: crate::TableType,
    pub version: uint8_t,
    pub precision: uint8_t,
    pub notice: *mut libc::c_char,
    pub fieldNames: Vec<String>,
    pub bboxOffset: uint32_t,
    pub metadataOffset: uint32_t,
    pub dataOffset: uint32_t,
}
pub type ZoneDetect = ZoneDetectOpaque;
/*
 * Copyright (c) 2018, Bertold Van den Bergh (vandenbergh@bertold.org)
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *     * Neither the name of the author nor the
 *       names of its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR DISTRIBUTOR BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
pub type ZDInternalError = libc::c_uint;
pub const ZD_E_PARSE_HEADER: ZDInternalError = 6;
pub const ZD_E_DB_CLOSE: ZDInternalError = 5;
pub const ZD_E_DB_MUNMAP: ZDInternalError = 4;
pub const ZD_E_DB_MMAP: ZDInternalError = 3;
pub const ZD_E_DB_SEEK: ZDInternalError = 2;
pub const ZD_E_DB_OPEN: ZDInternalError = 1;
pub const ZD_OK: ZDInternalError = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Reader {
    pub library: *const ZoneDetect,
    pub polygonIndex: uint32_t,
    pub numVertices: uint64_t,
    pub done: uint8_t,
    pub first: uint8_t,
    pub referenceStart: uint32_t,
    pub referenceEnd: uint32_t,
    pub referenceDirection: int32_t,
    pub pointLat: int32_t,
    pub pointLon: int32_t,
    pub firstLat: int32_t,
    pub firstLon: int32_t,
}
static mut zdErrorHandler: Option<
    unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> (),
> = None;
unsafe extern "C" fn zdError(
    mut errZD: ZDInternalError,
    mut errNative: libc::c_int,
) {
    if zdErrorHandler.is_some() {
        zdErrorHandler.expect("non-null function pointer")(
            errZD as libc::c_int,
            errNative,
        );
    };
}
unsafe extern "C" fn ZDFloatToFixedPoint(
    mut input: libc::c_float,
    mut scale: libc::c_float,
    mut precision: libc::c_uint,
) -> int32_t {
    let inputScaled: libc::c_float = input / scale;
    return (inputScaled
        * ((1 as libc::c_int)
            << precision.wrapping_sub(1 as libc::c_int as libc::c_uint))
            as libc::c_float) as int32_t;
}
unsafe extern "C" fn ZDFixedPointToFloat(
    mut input: int32_t,
    mut scale: libc::c_float,
    mut precision: libc::c_uint,
) -> libc::c_float {
    let value: libc::c_float = input as libc::c_float
        / ((1 as libc::c_int)
            << precision.wrapping_sub(1 as libc::c_int as libc::c_uint))
            as libc::c_float;
    return value * scale;
}
unsafe extern "C" fn ZDDecodeVariableLengthUnsigned(
    mut library: *const ZoneDetect,
    mut index: *mut uint32_t,
    mut result: *mut uint64_t,
) -> libc::c_uint {
    if *index >= (*library).length as uint32_t {
        return 0 as libc::c_int as libc::c_uint;
    }
    let mut value: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let buffer: *const uint8_t = (*library).mapping.offset(*index as isize);
    let bufferEnd: *const uint8_t = (*library)
        .mapping
        .offset((*library).length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut shift: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        value |= (*buffer.offset(i as isize) as uint64_t
            & 0x7f as libc::c_int as libc::c_ulong)
            << shift;
        shift = shift.wrapping_add(7 as libc::c_uint);
        if *buffer.offset(i as isize) as libc::c_int & 0x80 as libc::c_int == 0
        {
            break;
        }
        i = i.wrapping_add(1);
        if buffer.offset(i as isize) > bufferEnd {
            return 0 as libc::c_int as libc::c_uint;
        }
    }
    i = i.wrapping_add(1);
    *result = value;
    *index = (*index as libc::c_uint).wrapping_add(i) as uint32_t as uint32_t;
    return i;
}
unsafe extern "C" fn ZDDecodeVariableLengthUnsignedReverse(
    mut library: *const ZoneDetect,
    mut index: *mut uint32_t,
    mut result: *mut uint64_t,
) -> libc::c_uint {
    let mut i: uint32_t = *index;
    if *index >= (*library).length as uint32_t {
        return 0 as libc::c_int as libc::c_uint;
    }
    if *(*library).mapping.offset(i as isize) as libc::c_int
        & 0x80 as libc::c_int
        != 0
    {
        return 0 as libc::c_int as libc::c_uint;
    }
    if i == 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    i = i.wrapping_sub(1);
    while *(*library).mapping.offset(i as isize) as libc::c_int
        & 0x80 as libc::c_int
        != 0
    {
        if i == 0 {
            return 0 as libc::c_int as libc::c_uint;
        }
        i = i.wrapping_sub(1)
    }
    *index = i;
    i = i.wrapping_add(1);
    let mut i2: uint32_t = i;
    return ZDDecodeVariableLengthUnsigned(library, &mut i2, result);
}
unsafe extern "C" fn ZDDecodeUnsignedToSigned(mut value: uint64_t) -> int64_t {
    return if value & 1 as libc::c_int as libc::c_ulong != 0 {
        -(value.wrapping_div(2 as libc::c_int as libc::c_ulong) as int64_t)
    } else {
        value.wrapping_div(2 as libc::c_int as libc::c_ulong) as int64_t
    };
}
unsafe extern "C" fn ZDDecodeVariableLengthSigned(
    mut library: *const ZoneDetect,
    mut index: *mut uint32_t,
    mut result: *mut int32_t,
) -> libc::c_uint {
    let mut value: uint64_t = 0 as libc::c_int as uint64_t;
    let retVal: libc::c_uint =
        ZDDecodeVariableLengthUnsigned(library, index, &mut value);
    *result = ZDDecodeUnsignedToSigned(value) as int32_t;
    return retVal;
}
pub unsafe extern "C" fn ZDParseString(
    mut library: *const ZoneDetect,
    mut index: *mut uint32_t,
) -> *mut libc::c_char {
    let mut strLength: uint64_t = 0;
    if ZDDecodeVariableLengthUnsigned(library, index, &mut strLength) == 0 {
        return 0 as *mut libc::c_char;
    }
    let mut strOffset: uint32_t = *index;
    let mut remoteStr: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if strLength >= 256 as libc::c_int as libc::c_ulong {
        strOffset = (*library)
            .metadataOffset
            .wrapping_add(strLength as uint32_t)
            .wrapping_sub(256 as libc::c_int as libc::c_uint);
        remoteStr = 1 as libc::c_int as libc::c_uint;
        if ZDDecodeVariableLengthUnsigned(
            library,
            &mut strOffset,
            &mut strLength,
        ) == 0
        {
            return 0 as *mut libc::c_char;
        }
        if strLength > 256 as libc::c_int as libc::c_ulong {
            return 0 as *mut libc::c_char;
        }
    }
    let str: *mut libc::c_char =
        malloc(strLength.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    if !str.is_null() {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < strLength {
            *str.offset(i as isize) =
                (*(*library).mapping.offset(
                    (strOffset as libc::c_ulong).wrapping_add(i) as isize,
                ) as libc::c_int
                    ^ 0x80 as libc::c_int) as libc::c_char;
            i = i.wrapping_add(1)
        }
        *str.offset(strLength as isize) = 0 as libc::c_int as libc::c_char
    }
    if remoteStr == 0 {
        *index = (*index as libc::c_uint).wrapping_add(strLength as uint32_t)
            as uint32_t as uint32_t
    }
    return str;
}

unsafe extern "C" fn ZDPointInBox(
    mut xl: int32_t,
    mut x: int32_t,
    mut xr: int32_t,
    mut yl: int32_t,
    mut y: int32_t,
    mut yr: int32_t,
) -> libc::c_int {
    if xl <= x && x <= xr || xr <= x && x <= xl {
        if yl <= y && y <= yr || yr <= y && y <= yl {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ZDUnshuffle(mut w: uint64_t) -> uint32_t {
    w &= 0x5555555555555555 as libc::c_long as libc::c_ulong;
    w = (w | w >> 1 as libc::c_int)
        & 0x3333333333333333 as libc::c_long as libc::c_ulong;
    w = (w | w >> 2 as libc::c_int)
        & 0xf0f0f0f0f0f0f0f as libc::c_long as libc::c_ulong;
    w = (w | w >> 4 as libc::c_int)
        & 0xff00ff00ff00ff as libc::c_long as libc::c_ulong;
    w = (w | w >> 8 as libc::c_int)
        & 0xffff0000ffff as libc::c_long as libc::c_ulong;
    w = (w | w >> 16 as libc::c_int)
        & 0xffffffff as libc::c_uint as libc::c_ulong;
    return w as uint32_t;
}
unsafe extern "C" fn ZDDecodePoint(
    mut point: uint64_t,
    mut lat: *mut int32_t,
    mut lon: *mut int32_t,
) {
    *lat = ZDDecodeUnsignedToSigned(ZDUnshuffle(point) as uint64_t) as int32_t;
    *lon = ZDDecodeUnsignedToSigned(
        ZDUnshuffle(point >> 1 as libc::c_int) as uint64_t
    ) as int32_t;
}
unsafe extern "C" fn ZDReaderInit(
    mut reader: *mut Reader,
    mut library: *const ZoneDetect,
    mut polygonIndex: uint32_t,
) {
    memset(
        reader as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Reader>() as libc::c_ulong,
    );
    (*reader).library = library;
    (*reader).polygonIndex = polygonIndex;
    (*reader).first = 1 as libc::c_int as uint8_t;
}
unsafe extern "C" fn ZDReaderGetPoint(
    mut reader: *mut Reader,
    mut pointLat: *mut int32_t,
    mut pointLon: *mut int32_t,
) -> libc::c_int {
    let mut referenceDone: uint8_t = 0;
    let mut diffLat: int32_t = 0 as libc::c_int;
    let mut diffLon: int32_t = 0 as libc::c_int;
    loop {
        if (*reader).done as libc::c_int > 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        if (*reader).first as libc::c_int != 0
            && (*(*reader).library).version as libc::c_int == 0 as libc::c_int
        {
            if ZDDecodeVariableLengthUnsigned(
                (*reader).library,
                &mut (*reader).polygonIndex,
                &mut (*reader).numVertices,
            ) == 0
            {
                return -(1 as libc::c_int);
            }
            if (*reader).numVertices == 0 {
                return -(1 as libc::c_int);
            }
        }
        referenceDone = 0 as libc::c_int as uint8_t;
        if (*(*reader).library).version as libc::c_int == 1 as libc::c_int {
            let mut point: uint64_t = 0 as libc::c_int as uint64_t;
            if (*reader).referenceDirection == 0 {
                if ZDDecodeVariableLengthUnsigned(
                    (*reader).library,
                    &mut (*reader).polygonIndex,
                    &mut point,
                ) == 0
                {
                    return -(1 as libc::c_int);
                }
            } else if (*reader).referenceDirection > 0 as libc::c_int {
                /* Read reference forward */
                if ZDDecodeVariableLengthUnsigned(
                    (*reader).library,
                    &mut (*reader).referenceStart,
                    &mut point,
                ) == 0
                {
                    return -(1 as libc::c_int);
                }
                if (*reader).referenceStart >= (*reader).referenceEnd {
                    referenceDone = 1 as libc::c_int as uint8_t
                }
            } else if (*reader).referenceDirection < 0 as libc::c_int {
                /* Read reference backwards */
                if ZDDecodeVariableLengthUnsignedReverse(
                    (*reader).library,
                    &mut (*reader).referenceStart,
                    &mut point,
                ) == 0
                {
                    return -(1 as libc::c_int);
                }
                if (*reader).referenceStart <= (*reader).referenceEnd {
                    referenceDone = 1 as libc::c_int as uint8_t
                }
            }
            if point == 0 {
                /* This is a special marker, it is not allowed in reference mode */
                if (*reader).referenceDirection != 0 {
                    return -(1 as libc::c_int);
                }
                let mut value: uint64_t = 0;
                if ZDDecodeVariableLengthUnsigned(
                    (*reader).library,
                    &mut (*reader).polygonIndex,
                    &mut value,
                ) == 0
                {
                    return -(1 as libc::c_int);
                }
                if value == 0 as libc::c_int as libc::c_ulong {
                    (*reader).done = 2 as libc::c_int as uint8_t
                } else if value == 1 as libc::c_int as libc::c_ulong {
                    let mut diff: int32_t = 0;
                    let mut start: int64_t = 0;
                    if ZDDecodeVariableLengthUnsigned(
                        (*reader).library,
                        &mut (*reader).polygonIndex,
                        &mut start as *mut int64_t as *mut uint64_t,
                    ) == 0
                    {
                        return -(1 as libc::c_int);
                    }
                    if ZDDecodeVariableLengthSigned(
                        (*reader).library,
                        &mut (*reader).polygonIndex,
                        &mut diff,
                    ) == 0
                    {
                        return -(1 as libc::c_int);
                    }
                    (*reader).referenceStart = (*(*reader).library)
                        .dataOffset
                        .wrapping_add(start as uint32_t);
                    (*reader).referenceEnd =
                        (*(*reader).library).dataOffset.wrapping_add(
                            (start + diff as libc::c_long) as uint32_t,
                        );
                    (*reader).referenceDirection = diff;
                    if diff < 0 as libc::c_int {
                        (*reader).referenceStart =
                            (*reader).referenceStart.wrapping_sub(1);
                        (*reader).referenceEnd =
                            (*reader).referenceEnd.wrapping_sub(1)
                    }
                    continue;
                }
            } else {
                ZDDecodePoint(point, &mut diffLat, &mut diffLon);
                if (*reader).referenceDirection < 0 as libc::c_int {
                    diffLat = -diffLat;
                    diffLon = -diffLon
                }
            }
        }
        if (*(*reader).library).version as libc::c_int == 0 as libc::c_int {
            if ZDDecodeVariableLengthSigned(
                (*reader).library,
                &mut (*reader).polygonIndex,
                &mut diffLat,
            ) == 0
            {
                return -(1 as libc::c_int);
            }
            if ZDDecodeVariableLengthSigned(
                (*reader).library,
                &mut (*reader).polygonIndex,
                &mut diffLon,
            ) == 0
            {
                return -(1 as libc::c_int);
            }
        }
        if (*reader).done == 0 {
            (*reader).pointLat += diffLat;
            (*reader).pointLon += diffLon;
            if (*reader).first != 0 {
                (*reader).firstLat = (*reader).pointLat;
                (*reader).firstLon = (*reader).pointLon
            }
        } else {
            /* Close the polygon (the closing point is not encoded) */
            (*reader).pointLat = (*reader).firstLat;
            (*reader).pointLon = (*reader).firstLon;
            (*reader).done = 2 as libc::c_int as uint8_t
        }
        (*reader).first = 0 as libc::c_int as uint8_t;
        if !((*(*reader).library).version as libc::c_int == 0 as libc::c_int) {
            break;
        }
        (*reader).numVertices = (*reader).numVertices.wrapping_sub(1);
        if (*reader).numVertices == 0 {
            (*reader).done = 1 as libc::c_int as uint8_t
        }
        if !(diffLat == 0 && diffLon == 0) {
            break;
        }
    }
    if referenceDone != 0 {
        (*reader).referenceDirection = 0 as libc::c_int
    }
    if !pointLat.is_null() {
        *pointLat = (*reader).pointLat
    }
    if !pointLon.is_null() {
        *pointLon = (*reader).pointLon
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ZDFindPolygon(
    mut library: *const ZoneDetect,
    mut wantedId: uint32_t,
    mut metadataIndexPtr: *mut uint32_t,
    mut polygonIndexPtr: *mut uint32_t,
) -> libc::c_int {
    let mut polygonId: uint32_t = 0 as libc::c_int as uint32_t;
    let mut bboxIndex: uint32_t = (*library).bboxOffset;
    let mut metadataIndex: uint32_t = 0 as libc::c_int as uint32_t;
    let mut polygonIndex: uint32_t = 0 as libc::c_int as uint32_t;
    while bboxIndex < (*library).metadataOffset {
        let mut polygonIndexDelta: uint64_t = 0;
        let mut metadataIndexDelta: int32_t = 0;
        let mut tmp: int32_t = 0;
        if ZDDecodeVariableLengthSigned(library, &mut bboxIndex, &mut tmp) == 0
        {
            break;
        }
        if ZDDecodeVariableLengthSigned(library, &mut bboxIndex, &mut tmp) == 0
        {
            break;
        }
        if ZDDecodeVariableLengthSigned(library, &mut bboxIndex, &mut tmp) == 0
        {
            break;
        }
        if ZDDecodeVariableLengthSigned(library, &mut bboxIndex, &mut tmp) == 0
        {
            break;
        }
        if ZDDecodeVariableLengthSigned(
            library,
            &mut bboxIndex,
            &mut metadataIndexDelta,
        ) == 0
        {
            break;
        }
        if ZDDecodeVariableLengthUnsigned(
            library,
            &mut bboxIndex,
            &mut polygonIndexDelta,
        ) == 0
        {
            break;
        }
        metadataIndex = (metadataIndex as libc::c_uint)
            .wrapping_add(metadataIndexDelta as uint32_t)
            as uint32_t as uint32_t;
        polygonIndex = (polygonIndex as libc::c_uint)
            .wrapping_add(polygonIndexDelta as uint32_t)
            as uint32_t as uint32_t;
        if polygonId == wantedId {
            if !metadataIndexPtr.is_null() {
                metadataIndex = (metadataIndex as libc::c_uint)
                    .wrapping_add((*library).metadataOffset)
                    as uint32_t as uint32_t;
                *metadataIndexPtr = metadataIndex
            }
            if !polygonIndexPtr.is_null() {
                polygonIndex = (polygonIndex as libc::c_uint)
                    .wrapping_add((*library).dataOffset)
                    as uint32_t as uint32_t;
                *polygonIndexPtr = polygonIndex
            }
            return 1 as libc::c_int;
        }
        polygonId = polygonId.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ZDPolygonToListInternal(
    mut library: *const ZoneDetect,
    mut polygonIndex: uint32_t,
    mut length: *mut size_t,
) -> *mut int32_t {
    let mut current_block: u64;
    let mut reader: Reader = Reader {
        library: 0 as *const ZoneDetect,
        polygonIndex: 0,
        numVertices: 0,
        done: 0,
        first: 0,
        referenceStart: 0,
        referenceEnd: 0,
        referenceDirection: 0,
        pointLat: 0,
        pointLon: 0,
        firstLat: 0,
        firstLon: 0,
    };
    ZDReaderInit(&mut reader, library, polygonIndex);
    let mut listLength: size_t =
        (2 as libc::c_int * 100 as libc::c_int) as size_t;
    let mut listIndex: size_t = 0 as libc::c_int as size_t;
    let mut list: *mut int32_t = malloc(
        (::std::mem::size_of::<int32_t>() as libc::c_ulong)
            .wrapping_mul(listLength),
    ) as *mut int32_t;
    if list.is_null() {
        current_block = 982749321299142201;
    } else {
        current_block = 6873731126896040597;
    }
    loop {
        match current_block {
            982749321299142201 => {
                if !list.is_null() {
                    free(list as *mut libc::c_void);
                }
                return 0 as *mut int32_t;
            }
            _ => {
                let mut pointLat: int32_t = 0;
                let mut pointLon: int32_t = 0;
                let mut result: libc::c_int =
                    ZDReaderGetPoint(&mut reader, &mut pointLat, &mut pointLon);
                if result < 0 as libc::c_int {
                    current_block = 982749321299142201;
                    continue;
                }
                if result == 0 as libc::c_int {
                    if !length.is_null() {
                        *length = listIndex
                    }
                    return list;
                } else {
                    if listIndex >= listLength {
                        listLength = (listLength as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            as size_t
                            as size_t;
                        if listLength >= 1048576 as libc::c_int as libc::c_ulong
                        {
                            current_block = 982749321299142201;
                            continue;
                        }
                        list = realloc(
                            list as *mut libc::c_void,
                            (::std::mem::size_of::<int32_t>() as libc::c_ulong)
                                .wrapping_mul(listLength),
                        ) as *mut int32_t;
                        if list.is_null() {
                            current_block = 982749321299142201;
                            continue;
                        }
                    }
                    let fresh1 = listIndex;
                    listIndex = listIndex.wrapping_add(1);
                    *list.offset(fresh1 as isize) = pointLat;
                    let fresh2 = listIndex;
                    listIndex = listIndex.wrapping_add(1);
                    *list.offset(fresh2 as isize) = pointLon;
                    current_block = 6873731126896040597;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZDPolygonToList(
    mut library: *const ZoneDetect,
    mut polygonId: uint32_t,
    mut lengthPtr: *mut size_t,
) -> *mut libc::c_float {
    let mut length: size_t = 0;
    let mut polygonIndex: uint32_t = 0;
    let mut data: *mut int32_t = 0 as *mut int32_t;
    let mut flData: *mut libc::c_float = 0 as *mut libc::c_float;
    if !(ZDFindPolygon(
        library,
        polygonId,
        0 as *mut uint32_t,
        &mut polygonIndex,
    ) == 0)
    {
        length = 0 as libc::c_int as size_t;
        data = ZDPolygonToListInternal(library, polygonIndex, &mut length);
        if !data.is_null() {
            flData = malloc(
                (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                    .wrapping_mul(length),
            ) as *mut libc::c_float;
            if !flData.is_null() {
                let mut i: size_t = 0 as libc::c_int as size_t;
                while i < length {
                    let mut lat: int32_t = *data.offset(i as isize);
                    let mut lon: int32_t = *data.offset(
                        i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    );
                    *flData.offset(i as isize) = ZDFixedPointToFloat(
                        lat,
                        90 as libc::c_int as libc::c_float,
                        (*library).precision as libc::c_uint,
                    );
                    *flData.offset(
                        i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) = ZDFixedPointToFloat(
                        lon,
                        180 as libc::c_int as libc::c_float,
                        (*library).precision as libc::c_uint,
                    );
                    i = (i as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                        as size_t as size_t
                }
                if !lengthPtr.is_null() {
                    *lengthPtr = length
                }
                return flData;
            }
        }
    }
    if !data.is_null() {
        free(data as *mut libc::c_void);
    }
    if !flData.is_null() {
        free(flData as *mut libc::c_void);
    }
    return 0 as *mut libc::c_float;
}
unsafe extern "C" fn ZDPointInPolygon(
    mut library: *const ZoneDetect,
    mut polygonIndex: uint32_t,
    mut latFixedPoint: int32_t,
    mut lonFixedPoint: int32_t,
    mut distanceSqrMin: *mut uint64_t,
) -> ZDLookupResult {
    let mut pointLat: int32_t = 0;
    let mut pointLon: int32_t = 0;
    let mut prevLat: int32_t = 0 as libc::c_int;
    let mut prevLon: int32_t = 0 as libc::c_int;
    let mut prevQuadrant: libc::c_int = 0 as libc::c_int;
    let mut winding: libc::c_int = 0 as libc::c_int;
    let mut first: uint8_t = 1 as libc::c_int as uint8_t;
    let mut reader: Reader = Reader {
        library: 0 as *const ZoneDetect,
        polygonIndex: 0,
        numVertices: 0,
        done: 0,
        first: 0,
        referenceStart: 0,
        referenceEnd: 0,
        referenceDirection: 0,
        pointLat: 0,
        pointLon: 0,
        firstLat: 0,
        firstLon: 0,
    };
    ZDReaderInit(&mut reader, library, polygonIndex);
    loop {
        let mut result: libc::c_int =
            ZDReaderGetPoint(&mut reader, &mut pointLat, &mut pointLon);
        if result < 0 as libc::c_int {
            return ZD_LOOKUP_PARSE_ERROR;
        } else {
            if result == 0 as libc::c_int {
                break;
            }
            /* Check if point is ON the border */
            if pointLat == latFixedPoint && pointLon == lonFixedPoint {
                if !distanceSqrMin.is_null() {
                    *distanceSqrMin = 0 as libc::c_int as uint64_t
                }
                return ZD_LOOKUP_ON_BORDER_VERTEX;
            }
            /* Find quadrant */
            let mut quadrant: libc::c_int = 0;
            if pointLat >= latFixedPoint {
                if pointLon >= lonFixedPoint {
                    quadrant = 0 as libc::c_int
                } else {
                    quadrant = 1 as libc::c_int
                }
            } else if pointLon >= lonFixedPoint {
                quadrant = 3 as libc::c_int
            } else {
                quadrant = 2 as libc::c_int
            }
            if first == 0 {
                let mut windingNeedCompare: libc::c_int = 0 as libc::c_int;
                let mut lineIsStraight: libc::c_int = 0 as libc::c_int;
                let mut a: libc::c_float = 0 as libc::c_int as libc::c_float;
                let mut b: libc::c_float = 0 as libc::c_int as libc::c_float;
                /* Calculate winding number */
                if !(quadrant == prevQuadrant) {
                    if quadrant
                        == (prevQuadrant + 1 as libc::c_int) % 4 as libc::c_int
                    {
                        winding += 1
                    } else if (quadrant + 1 as libc::c_int) % 4 as libc::c_int
                        == prevQuadrant
                    {
                        winding -= 1
                    } else {
                        windingNeedCompare = 1 as libc::c_int
                    }
                }
                /* Avoid horizontal and vertical lines */
                if pointLon == prevLon || pointLat == prevLat {
                    lineIsStraight = 1 as libc::c_int
                }
                /* Calculate the parameters of y=ax+b if needed */
                if lineIsStraight == 0
                    && (!distanceSqrMin.is_null() || windingNeedCompare != 0)
                {
                    a = (pointLat as libc::c_float - prevLat as libc::c_float)
                        / (pointLon as libc::c_float
                            - prevLon as libc::c_float);
                    b = pointLat as libc::c_float
                        - a * pointLon as libc::c_float
                }
                let mut onStraight: libc::c_int = ZDPointInBox(
                    pointLat,
                    latFixedPoint,
                    prevLat,
                    pointLon,
                    lonFixedPoint,
                    prevLon,
                );
                if lineIsStraight != 0
                    && (onStraight != 0 || windingNeedCompare != 0)
                {
                    if !distanceSqrMin.is_null() {
                        *distanceSqrMin = 0 as libc::c_int as uint64_t
                    }
                    return ZD_LOOKUP_ON_BORDER_SEGMENT;
                }
                /* Jumped two quadrants. */
                if windingNeedCompare != 0 {
                    /* Check if the target is on the border */
                    let intersectLon: int32_t =
                        ((latFixedPoint as libc::c_float - b) / a) as int32_t;
                    if intersectLon >= lonFixedPoint - 1 as libc::c_int
                        && intersectLon <= lonFixedPoint + 1 as libc::c_int
                    {
                        if !distanceSqrMin.is_null() {
                            *distanceSqrMin = 0 as libc::c_int as uint64_t
                        }
                        return ZD_LOOKUP_ON_BORDER_SEGMENT;
                    }
                    /* Ok, it's not. In which direction did we go round the target? */
                    let sign: libc::c_int = if intersectLon < lonFixedPoint {
                        2 as libc::c_int
                    } else {
                        -(2 as libc::c_int)
                    };
                    if quadrant == 2 as libc::c_int
                        || quadrant == 3 as libc::c_int
                    {
                        winding += sign
                    } else {
                        winding -= sign
                    }
                }
                /* Calculate closest point on line (if needed) */
                if !distanceSqrMin.is_null() {
                    let mut closestLon: libc::c_float = 0.;
                    let mut closestLat: libc::c_float = 0.;
                    if lineIsStraight == 0 {
                        closestLon = (lonFixedPoint as libc::c_float
                            + a * latFixedPoint as libc::c_float
                            - a * b)
                            / (a * a + 1 as libc::c_int as libc::c_float);
                        closestLat = (a
                            * (lonFixedPoint as libc::c_float
                                + a * latFixedPoint as libc::c_float)
                            + b)
                            / (a * a + 1 as libc::c_int as libc::c_float)
                    } else if pointLon == prevLon {
                        closestLon = pointLon as libc::c_float;
                        closestLat = latFixedPoint as libc::c_float
                    } else {
                        closestLon = lonFixedPoint as libc::c_float;
                        closestLat = pointLat as libc::c_float
                    }
                    let closestInBox: libc::c_int = ZDPointInBox(
                        pointLon,
                        closestLon as int32_t,
                        prevLon,
                        pointLat,
                        closestLat as int32_t,
                        prevLat,
                    );
                    let mut diffLat: int64_t = 0;
                    let mut diffLon: int64_t = 0;
                    if closestInBox != 0 {
                        /* Calculate squared distance to segment. */
                        diffLat = (closestLat - latFixedPoint as libc::c_float)
                            as int64_t;
                        diffLon = (closestLon - lonFixedPoint as libc::c_float)
                            as int64_t
                    } else {
                        /*
                         * Calculate squared distance to vertices
                         * It is enough to check the current point since the polygon is closed.
                         */
                        diffLat = (pointLat - latFixedPoint) as int64_t;
                        diffLon = (pointLon - lonFixedPoint) as int64_t
                    }
                    /* Note: lon has half scale */
                    let mut distanceSqr: uint64_t =
                        ((diffLat * diffLat) as uint64_t).wrapping_add(
                            ((diffLon * diffLon) as uint64_t).wrapping_mul(
                                4 as libc::c_int as libc::c_ulong,
                            ),
                        );
                    if distanceSqr < *distanceSqrMin {
                        *distanceSqrMin = distanceSqr
                    }
                }
            }
            prevQuadrant = quadrant;
            prevLat = pointLat;
            prevLon = pointLon;
            first = 0 as libc::c_int as uint8_t
        }
    }
    if winding == -(4 as libc::c_int) {
        return ZD_LOOKUP_IN_ZONE;
    } else {
        if winding == 4 as libc::c_int {
            return ZD_LOOKUP_IN_EXCLUDED_ZONE;
        } else {
            if winding == 0 as libc::c_int {
                return ZD_LOOKUP_NOT_IN_ZONE;
            }
        }
    }
    /* Should not happen */
    if !distanceSqrMin.is_null() {
        *distanceSqrMin = 0 as libc::c_int as uint64_t
    }
    return ZD_LOOKUP_ON_BORDER_SEGMENT;
}
#[no_mangle]
pub unsafe extern "C" fn ZDCloseDatabase(mut library: *mut ZoneDetect) {
    if !library.is_null() {
        if !(*library).notice.is_null() {
            free((*library).notice as *mut libc::c_void);
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn ZDLookup(
    mut library: *const ZoneDetect,
    mut lat: libc::c_float,
    mut lon: libc::c_float,
    mut safezone: *mut libc::c_float,
) -> Vec<ZoneDetectResult> {
    let latFixedPoint: int32_t = ZDFloatToFixedPoint(
        lat,
        90 as libc::c_int as libc::c_float,
        (*library).precision as libc::c_uint,
    );
    let lonFixedPoint: int32_t = ZDFloatToFixedPoint(
        lon,
        180 as libc::c_int as libc::c_float,
        (*library).precision as libc::c_uint,
    );
    let mut distanceSqrMin: uint64_t = -(1 as libc::c_int) as uint64_t;
    /* Parse the header */
    /* Iterate over all polygons */
    let mut bboxIndex: uint32_t = (*library).bboxOffset;
    let mut metadataIndex: uint32_t = 0 as libc::c_int as uint32_t;
    let mut polygonIndex: uint32_t = 0 as libc::c_int as uint32_t;
    let mut results = Vec::new();
    let mut polygonId: uint32_t = 0 as libc::c_int as uint32_t;
    while bboxIndex < (*library).metadataOffset {
        let mut minLat: int32_t = 0;
        let mut minLon: int32_t = 0;
        let mut maxLat: int32_t = 0;
        let mut maxLon: int32_t = 0;
        let mut metadataIndexDelta: int32_t = 0;
        let mut polygonIndexDelta: uint64_t = 0;
        if ZDDecodeVariableLengthSigned(library, &mut bboxIndex, &mut minLat)
            == 0
        {
            break;
        }
        if ZDDecodeVariableLengthSigned(library, &mut bboxIndex, &mut minLon)
            == 0
        {
            break;
        }
        if ZDDecodeVariableLengthSigned(library, &mut bboxIndex, &mut maxLat)
            == 0
        {
            break;
        }
        if ZDDecodeVariableLengthSigned(library, &mut bboxIndex, &mut maxLon)
            == 0
        {
            break;
        }
        if ZDDecodeVariableLengthSigned(
            library,
            &mut bboxIndex,
            &mut metadataIndexDelta,
        ) == 0
        {
            break;
        }
        if ZDDecodeVariableLengthUnsigned(
            library,
            &mut bboxIndex,
            &mut polygonIndexDelta,
        ) == 0
        {
            break;
        }
        metadataIndex = (metadataIndex as libc::c_uint)
            .wrapping_add(metadataIndexDelta as uint32_t)
            as uint32_t as uint32_t;
        polygonIndex = (polygonIndex as libc::c_uint)
            .wrapping_add(polygonIndexDelta as uint32_t)
            as uint32_t as uint32_t;
        if !(latFixedPoint >= minLat) {
            break;
        }
        if latFixedPoint <= maxLat
            && lonFixedPoint >= minLon
            && lonFixedPoint <= maxLon
        {
            let lookupResult: ZDLookupResult = ZDPointInPolygon(
                library,
                (*library).dataOffset.wrapping_add(polygonIndex),
                latFixedPoint,
                lonFixedPoint,
                if !safezone.is_null() {
                    &mut distanceSqrMin
                } else {
                    0 as *mut uint64_t
                },
            );
            if lookupResult as libc::c_int
                == ZD_LOOKUP_PARSE_ERROR as libc::c_int
            {
                break;
            }
            if lookupResult as libc::c_int
                != ZD_LOOKUP_NOT_IN_ZONE as libc::c_int
            {
                results.push(ZoneDetectResult {
                    polygonId,
                    metaId: metadataIndex,
                    fields: std::collections::HashMap::with_capacity(
                        (*library).fieldNames.len(),
                    ),
                    lookupResult,
                });
            }
        }
        polygonId = polygonId.wrapping_add(1)
    }
    /* Clean up results */
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < results.len() as u64 {
        let mut insideSum: libc::c_int = 0 as libc::c_int;
        let mut overrideResult: ZDLookupResult = ZD_LOOKUP_IGNORE;
        let mut j: size_t = i;
        while j < results.len() as u64 {
            if results[i as usize].metaId == results[j as usize].metaId {
                let mut tmpResult: ZDLookupResult =
                    results[j as usize].lookupResult;
                results[j as usize].lookupResult = ZD_LOOKUP_IGNORE;
                /* This is the same result. Is it an exclusion zone? */
                if tmpResult as libc::c_int == ZD_LOOKUP_IN_ZONE as libc::c_int
                {
                    insideSum += 1
                } else if tmpResult as libc::c_int
                    == ZD_LOOKUP_IN_EXCLUDED_ZONE as libc::c_int
                {
                    insideSum -= 1
                } else {
                    /* If on the bodrder then the final result is on the border */
                    overrideResult = tmpResult
                }
            }
            j = j.wrapping_add(1)
        }
        if overrideResult as libc::c_int != ZD_LOOKUP_IGNORE as libc::c_int {
            results[i as usize].lookupResult = overrideResult
        } else if insideSum != 0 {
            results[i as usize].lookupResult = ZD_LOOKUP_IN_ZONE
        }
        i = i.wrapping_add(1)
    }
    /* Remove zones to ignore */
    let mut newNumResults: size_t = 0 as libc::c_int as size_t;
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    while i_0 < results.len() as u64 {
        if results[i_0 as usize].lookupResult as libc::c_int
            != ZD_LOOKUP_IGNORE as libc::c_int
        {
            results[newNumResults as usize] = results[i_0 as usize].clone();
            newNumResults = newNumResults.wrapping_add(1)
        }
        i_0 = i_0.wrapping_add(1)
    }
    /* Lookup metadata */
    let mut i_1: size_t = 0 as libc::c_int as size_t;
    while i_1 < results.len() as u64 {
        let mut tmpIndex: uint32_t = (*library)
            .metadataOffset
            .wrapping_add(results[i_1 as usize].metaId);

        let mut j_0: size_t = 0 as libc::c_int as size_t;
        while j_0 < (*library).fieldNames.len() as libc::c_ulong {
            let key = (*library).fieldNames[j_0 as usize].clone();
            let value = crate::parse_string(&*library, &mut tmpIndex)
                .expect("failed to get field data");
            results[i_1 as usize].fields.insert(key, value);
            j_0 = j_0.wrapping_add(1)
        }
        i_1 = i_1.wrapping_add(1)
    }

    if !safezone.is_null() {
        *safezone = sqrtf(distanceSqrMin as libc::c_float)
            * 90 as libc::c_int as libc::c_float
            / ((1 as libc::c_int)
                << (*library).precision as libc::c_int - 1 as libc::c_int)
                as libc::c_float
    }

    // TODO: we've removed the end marker, so the length is probably off by one
    return results;
}

#[no_mangle]
pub unsafe extern "C" fn ZDGetNotice(
    mut library: *const ZoneDetect,
) -> *const libc::c_char {
    return (*library).notice;
}
#[no_mangle]
pub unsafe extern "C" fn ZDLookupResultToString(
    mut result: ZDLookupResult,
) -> *const libc::c_char {
    match result as libc::c_int {
        -3 => return b"Ignore\x00" as *const u8 as *const libc::c_char,
        -2 => return b"End\x00" as *const u8 as *const libc::c_char,
        -1 => return b"Parsing error\x00" as *const u8 as *const libc::c_char,
        0 => return b"Not in zone\x00" as *const u8 as *const libc::c_char,
        1 => return b"In zone\x00" as *const u8 as *const libc::c_char,
        2 => {
            return b"In excluded zone\x00" as *const u8 as *const libc::c_char
        }
        3 => {
            return b"Target point is border vertex\x00" as *const u8
                as *const libc::c_char
        }
        4 => {
            return b"Target point is on border\x00" as *const u8
                as *const libc::c_char
        }
        _ => {}
    }
    return b"Unknown\x00" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ZDGetErrorString(
    mut errZD: libc::c_int,
) -> *const libc::c_char {
    match errZD as ZDInternalError as libc::c_uint {
        0 => {}
        1 => {
            return b"could not open database file\x00" as *const u8
                as *const libc::c_char
        }
        2 => {
            return b"could not retrieve database file size\x00" as *const u8
                as *const libc::c_char
        }
        3 => {
            return b"could not map database file to system memory\x00"
                as *const u8 as *const libc::c_char
        }
        4 => {
            return b"could not unmap database\x00" as *const u8
                as *const libc::c_char
        }
        5 => {
            return b"could not close database file\x00" as *const u8
                as *const libc::c_char
        }
        6 => {
            return b"could not parse database header\x00" as *const u8
                as *const libc::c_char
        }
        _ => {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"/home/nbishop/src/tzlookup/c/zonedetect.c\x00" as *const u8
                    as *const libc::c_char,
                1095 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"const char *ZDGetErrorString(int)\x00",
                ))
                .as_ptr(),
            );
        }
    }
    return b"\x00" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ZDSetErrorHandler(
    mut handler: Option<
        unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> (),
    >,
) -> libc::c_int {
    zdErrorHandler = handler;
    return 0 as libc::c_int;
}
