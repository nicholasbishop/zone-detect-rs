#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
type size_t = libc::c_ulong;
#[derive(Clone, Debug)]
#[repr(C)]
pub struct ZoneDetectResult {
    pub lookupResult: LookupResult,
    pub polygonId: u32,
    pub metaId: u32,
    // TODO: maybe change this to &str
    pub fields: std::collections::HashMap<String, String>,
}
#[derive(Clone)]
#[repr(C)]
pub struct ZoneDetectOpaque {
    pub mapping: Vec<u8>,
    pub tableType: crate::TableType,
    pub version: u8,
    pub precision: u8,
    pub notice: String,
    pub fieldNames: Vec<String>,
    pub bboxOffset: u32,
    pub metadataOffset: u32,
    pub dataOffset: u32,
}
pub type ZoneDetect = ZoneDetectOpaque;
use crate::LookupResult;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Reader<'a> {
    pub library: &'a ZoneDetect,
    pub polygonIndex: u32,
    pub numVertices: u64,
    pub done: u8,
    pub first: u8,
    pub referenceStart: u32,
    pub referenceEnd: u32,
    pub referenceDirection: i32,
    pub pointLat: i32,
    pub pointLon: i32,
    pub firstLat: i32,
    pub firstLon: i32,
}

impl<'a> Reader<'a> {
    fn new(library: &'a ZoneDetect, polygonIndex: u32) -> Reader {
        Reader {
            library,
            polygonIndex,
            numVertices: 0,
            done: 0,
            first: 1,
            referenceStart: 0,
            referenceEnd: 0,
            referenceDirection: 0,
            pointLat: 0,
            pointLon: 0,
            firstLat: 0,
            firstLon: 0,
        }
    }
}

unsafe extern "C" fn ZDFloatToFixedPoint(
    mut input: libc::c_float,
    mut scale: libc::c_float,
    mut precision: libc::c_uint,
) -> i32 {
    let inputScaled: libc::c_float = input / scale;
    (inputScaled
        * ((1 as libc::c_int)
            << precision.wrapping_sub(1 as libc::c_int as libc::c_uint))
            as libc::c_float) as i32
}
unsafe extern "C" fn ZDFixedPointToFloat(
    mut input: i32,
    mut scale: libc::c_float,
    mut precision: libc::c_uint,
) -> libc::c_float {
    let value: libc::c_float = input as libc::c_float
        / ((1 as libc::c_int)
            << precision.wrapping_sub(1 as libc::c_int as libc::c_uint))
            as libc::c_float;
    value * scale
}
pub unsafe extern "C" fn ZDDecodeVariableLengthUnsigned(
    library: &ZoneDetect,
    mut index: *mut u32,
    mut result: *mut u64,
) -> libc::c_uint {
    if *index >= (*library).mapping.len() as u32 {
        return 0 as libc::c_int as libc::c_uint;
    }
    let mut value: u64 = 0 as libc::c_int as u64;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mapping: *const u8 = (*library).mapping.as_ptr();
    let buffer: *const u8 = mapping.offset(*index as isize);
    let bufferEnd: *const u8 = mapping
        .offset((*library).mapping.len() as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut shift: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        value |= (*buffer.offset(i as isize) as u64
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
    *index = (*index as libc::c_uint).wrapping_add(i) as u32 as u32;
    i
}
unsafe extern "C" fn ZDDecodeVariableLengthUnsignedReverse(
    library: &ZoneDetect,
    mut index: *mut u32,
    mut result: *mut u64,
) -> libc::c_uint {
    let mut i: u32 = *index;
    if *index >= (*library).mapping.len() as u32 {
        return 0 as libc::c_int as libc::c_uint;
    }
    let mapping: *const u8 = (*library).mapping.as_ptr();
    if *mapping.offset(i as isize) as libc::c_int & 0x80 as libc::c_int != 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    if i == 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    i = i.wrapping_sub(1);
    while *mapping.offset(i as isize) as libc::c_int & 0x80 as libc::c_int != 0
    {
        if i == 0 {
            return 0 as libc::c_int as libc::c_uint;
        }
        i = i.wrapping_sub(1)
    }
    *index = i;
    i = i.wrapping_add(1);
    let mut i2: u32 = i;
    ZDDecodeVariableLengthUnsigned(library, &mut i2, result)
}
unsafe extern "C" fn ZDDecodeUnsignedToSigned(mut value: u64) -> i64 {
    if value & 1 as libc::c_int as libc::c_ulong != 0 {
        -(value.wrapping_div(2 as libc::c_int as libc::c_ulong) as i64)
    } else {
        value.wrapping_div(2 as libc::c_int as libc::c_ulong) as i64
    }
}
unsafe extern "C" fn ZDDecodeVariableLengthSigned(
    library: &ZoneDetect,
    mut index: *mut u32,
    mut result: *mut i32,
) -> libc::c_uint {
    let mut value: u64 = 0 as libc::c_int as u64;
    let retVal: libc::c_uint =
        ZDDecodeVariableLengthUnsigned(library, index, &mut value);
    *result = ZDDecodeUnsignedToSigned(value) as i32;
    retVal
}
pub unsafe extern "C" fn ZDParseString(
    library: &ZoneDetect,
    mut index: *mut u32,
) -> *mut libc::c_char {
    let mut strLength: u64 = 0;
    if ZDDecodeVariableLengthUnsigned(library, index, &mut strLength) == 0 {
        return 0 as *mut libc::c_char;
    }
    let mut strOffset: u32 = *index;
    let mut remoteStr: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if strLength >= 256 as libc::c_int as libc::c_ulong {
        strOffset = (*library)
            .metadataOffset
            .wrapping_add(strLength as u32)
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
    let mapping: *const u8 = (*library).mapping.as_ptr();
    if !str.is_null() {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < strLength {
            *str.offset(i as isize) = (*mapping
                .offset((strOffset as libc::c_ulong).wrapping_add(i) as isize)
                as libc::c_int
                ^ 0x80 as libc::c_int)
                as libc::c_char;
            i = i.wrapping_add(1)
        }
        *str.offset(strLength as isize) = 0 as libc::c_int as libc::c_char
    }
    if remoteStr == 0 {
        *index = (*index as libc::c_uint).wrapping_add(strLength as u32) as u32
            as u32
    }
    str
}

unsafe extern "C" fn ZDPointInBox(
    mut xl: i32,
    mut x: i32,
    mut xr: i32,
    mut yl: i32,
    mut y: i32,
    mut yr: i32,
) -> libc::c_int {
    if xl <= x && x <= xr || xr <= x && x <= xl {
        if yl <= y && y <= yr || yr <= y && y <= yl {
            return 1 as libc::c_int;
        }
    }
    0 as libc::c_int
}
unsafe extern "C" fn ZDUnshuffle(mut w: u64) -> u32 {
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
    w as u32
}
unsafe extern "C" fn ZDDecodePoint(
    mut point: u64,
    mut lat: *mut i32,
    mut lon: *mut i32,
) {
    *lat = ZDDecodeUnsignedToSigned(ZDUnshuffle(point) as u64) as i32;
    *lon =
        ZDDecodeUnsignedToSigned(ZDUnshuffle(point >> 1 as libc::c_int) as u64)
            as i32;
}

unsafe extern "C" fn ZDReaderGetPoint(
    mut reader: *mut Reader,
    mut pointLat: *mut i32,
    mut pointLon: *mut i32,
) -> libc::c_int {
    let mut referenceDone: u8 = 0;
    let mut diffLat: i32 = 0 as libc::c_int;
    let mut diffLon: i32 = 0 as libc::c_int;
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
        referenceDone = 0 as libc::c_int as u8;
        if (*(*reader).library).version as libc::c_int == 1 as libc::c_int {
            let mut point: u64 = 0 as libc::c_int as u64;
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
                    referenceDone = 1 as libc::c_int as u8
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
                    referenceDone = 1 as libc::c_int as u8
                }
            }
            if point == 0 {
                /* This is a special marker, it is not allowed in reference mode */
                if (*reader).referenceDirection != 0 {
                    return -(1 as libc::c_int);
                }
                let mut value: u64 = 0;
                if ZDDecodeVariableLengthUnsigned(
                    (*reader).library,
                    &mut (*reader).polygonIndex,
                    &mut value,
                ) == 0
                {
                    return -(1 as libc::c_int);
                }
                if value == 0 as libc::c_int as libc::c_ulong {
                    (*reader).done = 2 as libc::c_int as u8
                } else if value == 1 as libc::c_int as libc::c_ulong {
                    let mut diff: i32 = 0;
                    let mut start: i64 = 0;
                    if ZDDecodeVariableLengthUnsigned(
                        (*reader).library,
                        &mut (*reader).polygonIndex,
                        &mut start as *mut i64 as *mut u64,
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
                        .wrapping_add(start as u32);
                    (*reader).referenceEnd = (*(*reader).library)
                        .dataOffset
                        .wrapping_add((start + diff as libc::c_long) as u32);
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
            (*reader).done = 2 as libc::c_int as u8
        }
        (*reader).first = 0 as libc::c_int as u8;
        if !((*(*reader).library).version as libc::c_int == 0 as libc::c_int) {
            break;
        }
        (*reader).numVertices = (*reader).numVertices.wrapping_sub(1);
        if (*reader).numVertices == 0 {
            (*reader).done = 1 as libc::c_int as u8
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
    1 as libc::c_int
}
unsafe extern "C" fn ZDFindPolygon(
    library: &ZoneDetect,
    mut wantedId: u32,
    mut metadataIndexPtr: *mut u32,
    mut polygonIndexPtr: *mut u32,
) -> libc::c_int {
    let mut polygonId: u32 = 0 as libc::c_int as u32;
    let mut bboxIndex: u32 = (*library).bboxOffset;
    let mut metadataIndex: u32 = 0 as libc::c_int as u32;
    let mut polygonIndex: u32 = 0 as libc::c_int as u32;
    while bboxIndex < (*library).metadataOffset {
        let mut polygonIndexDelta: u64 = 0;
        let mut metadataIndexDelta: i32 = 0;
        let mut tmp: i32 = 0;
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
            .wrapping_add(metadataIndexDelta as u32)
            as u32 as u32;
        polygonIndex = (polygonIndex as libc::c_uint)
            .wrapping_add(polygonIndexDelta as u32)
            as u32 as u32;
        if polygonId == wantedId {
            if !metadataIndexPtr.is_null() {
                metadataIndex = (metadataIndex as libc::c_uint)
                    .wrapping_add((*library).metadataOffset)
                    as u32 as u32;
                *metadataIndexPtr = metadataIndex
            }
            if !polygonIndexPtr.is_null() {
                polygonIndex = (polygonIndex as libc::c_uint)
                    .wrapping_add((*library).dataOffset)
                    as u32 as u32;
                *polygonIndexPtr = polygonIndex
            }
            return 1 as libc::c_int;
        }
        polygonId = polygonId.wrapping_add(1)
    }
    0 as libc::c_int
}
fn ZDPolygonToListInternal(
    library: &ZoneDetect,
    mut polygonIndex: u32,
) -> Option<Vec<i32>> {
    let mut reader = Reader::new(library, polygonIndex);
    let mut list = Vec::new();
    loop {
        let mut pointLat: i32 = 0;
        let mut pointLon: i32 = 0;
        let mut result: libc::c_int = unsafe {
            ZDReaderGetPoint(&mut reader, &mut pointLat, &mut pointLon)
        };
        if result < 0 as libc::c_int {
            return None;
        }
        if result == 0 as libc::c_int {
            return Some(list);
        } else {
            list.push(pointLat);
            list.push(pointLon);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZDPolygonToList(
    library: &ZoneDetect,
    mut polygonId: u32,
    mut lengthPtr: *mut size_t,
) -> *mut libc::c_float {
    let mut length: size_t = 0;
    let mut polygonIndex: u32 = 0;
    let mut flData: *mut libc::c_float = 0 as *mut libc::c_float;
    if !(ZDFindPolygon(library, polygonId, 0 as *mut u32, &mut polygonIndex)
        == 0)
    {
        if let Some(data) = ZDPolygonToListInternal(library, polygonIndex) {
            flData = malloc(
                (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                    .wrapping_mul(length),
            ) as *mut libc::c_float;
            if !flData.is_null() {
                let mut i: size_t = 0 as libc::c_int as size_t;
                while i < length {
                    let mut lat: i32 = data[i as usize];
                    let mut lon: i32 = data[i
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as usize];
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
    if !flData.is_null() {
        free(flData as *mut libc::c_void);
    }
    0 as *mut libc::c_float
}
unsafe extern "C" fn ZDPointInPolygon(
    mut library: &ZoneDetect,
    mut polygonIndex: u32,
    mut latFixedPoint: i32,
    mut lonFixedPoint: i32,
    mut distanceSqrMin: *mut u64,
) -> LookupResult {
    let mut pointLat: i32 = 0;
    let mut pointLon: i32 = 0;
    let mut prevLat: i32 = 0 as libc::c_int;
    let mut prevLon: i32 = 0 as libc::c_int;
    let mut prevQuadrant: libc::c_int = 0 as libc::c_int;
    let mut winding: libc::c_int = 0 as libc::c_int;
    let mut first: u8 = 1 as libc::c_int as u8;
    let mut reader = Reader::new(library, polygonIndex);
    loop {
        let mut result: libc::c_int =
            ZDReaderGetPoint(&mut reader, &mut pointLat, &mut pointLon);
        if result < 0 as libc::c_int {
            return LookupResult::ParseError;
        } else {
            if result == 0 as libc::c_int {
                break;
            }
            /* Check if point is ON the border */
            if pointLat == latFixedPoint && pointLon == lonFixedPoint {
                if !distanceSqrMin.is_null() {
                    *distanceSqrMin = 0 as libc::c_int as u64
                }
                return LookupResult::OnBorderVertex;
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
                        *distanceSqrMin = 0 as libc::c_int as u64
                    }
                    return LookupResult::OnBorderSegment;
                }
                /* Jumped two quadrants. */
                if windingNeedCompare != 0 {
                    /* Check if the target is on the border */
                    let intersectLon: i32 =
                        ((latFixedPoint as libc::c_float - b) / a) as i32;
                    if intersectLon >= lonFixedPoint - 1 as libc::c_int
                        && intersectLon <= lonFixedPoint + 1 as libc::c_int
                    {
                        if !distanceSqrMin.is_null() {
                            *distanceSqrMin = 0 as libc::c_int as u64
                        }
                        return LookupResult::OnBorderSegment;
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
                        closestLon as i32,
                        prevLon,
                        pointLat,
                        closestLat as i32,
                        prevLat,
                    );
                    let mut diffLat: i64 = 0;
                    let mut diffLon: i64 = 0;
                    if closestInBox != 0 {
                        /* Calculate squared distance to segment. */
                        diffLat = (closestLat - latFixedPoint as libc::c_float)
                            as i64;
                        diffLon =
                            (closestLon - lonFixedPoint as libc::c_float) as i64
                    } else {
                        /*
                         * Calculate squared distance to vertices
                         * It is enough to check the current point since the polygon is closed.
                         */
                        diffLat = (pointLat - latFixedPoint) as i64;
                        diffLon = (pointLon - lonFixedPoint) as i64
                    }
                    /* Note: lon has half scale */
                    let mut distanceSqr: u64 = ((diffLat * diffLat) as u64)
                        .wrapping_add(
                            ((diffLon * diffLon) as u64).wrapping_mul(
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
            first = 0 as libc::c_int as u8
        }
    }
    if winding == -(4 as libc::c_int) {
        return LookupResult::InZone;
    } else {
        if winding == 4 as libc::c_int {
            return LookupResult::InExcludedZone;
        } else {
            if winding == 0 as libc::c_int {
                return LookupResult::NotInZone;
            }
        }
    }
    /* Should not happen */
    if !distanceSqrMin.is_null() {
        *distanceSqrMin = 0 as libc::c_int as u64
    }
    LookupResult::OnBorderSegment
}

#[no_mangle]
pub unsafe extern "C" fn ZDLookup(
    mut library: &ZoneDetect,
    mut lat: f32,
    mut lon: f32,
    mut safezone: Option<&mut f32>,
) -> Vec<ZoneDetectResult> {
    let latFixedPoint: i32 = ZDFloatToFixedPoint(
        lat,
        90 as libc::c_int as libc::c_float,
        (*library).precision as libc::c_uint,
    );
    let lonFixedPoint: i32 = ZDFloatToFixedPoint(
        lon,
        180 as libc::c_int as libc::c_float,
        (*library).precision as libc::c_uint,
    );
    let mut distanceSqrMin: u64 = -(1 as libc::c_int) as u64;
    /* Parse the header */
    /* Iterate over all polygons */
    let mut bboxIndex: u32 = (*library).bboxOffset;
    let mut metadataIndex: u32 = 0 as libc::c_int as u32;
    let mut polygonIndex: u32 = 0 as libc::c_int as u32;
    let mut results = Vec::new();
    let mut polygonId: u32 = 0 as libc::c_int as u32;
    while bboxIndex < (*library).metadataOffset {
        let mut minLat: i32 = 0;
        let mut minLon: i32 = 0;
        let mut maxLat: i32 = 0;
        let mut maxLon: i32 = 0;
        let mut metadataIndexDelta: i32 = 0;
        let mut polygonIndexDelta: u64 = 0;
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
            .wrapping_add(metadataIndexDelta as u32)
            as u32 as u32;
        polygonIndex = (polygonIndex as libc::c_uint)
            .wrapping_add(polygonIndexDelta as u32)
            as u32 as u32;
        if !(latFixedPoint >= minLat) {
            break;
        }
        if latFixedPoint <= maxLat
            && lonFixedPoint >= minLon
            && lonFixedPoint <= maxLon
        {
            let lookupResult = ZDPointInPolygon(
                library,
                (*library).dataOffset.wrapping_add(polygonIndex),
                latFixedPoint,
                lonFixedPoint,
                if safezone.is_some() {
                    &mut distanceSqrMin
                } else {
                    0 as *mut u64
                },
            );
            if lookupResult == LookupResult::ParseError {
                break;
            }
            if lookupResult != LookupResult::NotInZone {
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
        let mut overrideResult = LookupResult::Ignore;
        let mut j: size_t = i;
        while j < results.len() as u64 {
            if results[i as usize].metaId == results[j as usize].metaId {
                let mut tmpResult = results[j as usize].lookupResult;
                results[j as usize].lookupResult = LookupResult::Ignore;
                /* This is the same result. Is it an exclusion zone? */
                if tmpResult == LookupResult::InZone {
                    insideSum += 1
                } else if tmpResult == LookupResult::InExcludedZone {
                    insideSum -= 1
                } else {
                    /* If on the bodrder then the final result is on the border */
                    overrideResult = tmpResult
                }
            }
            j = j.wrapping_add(1)
        }
        if overrideResult != LookupResult::Ignore {
            results[i as usize].lookupResult = overrideResult
        } else if insideSum != 0 {
            results[i as usize].lookupResult = LookupResult::InZone
        }
        i = i.wrapping_add(1)
    }
    /* Remove zones to ignore */
    let mut newNumResults: size_t = 0 as libc::c_int as size_t;
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    while i_0 < results.len() as u64 {
        if results[i_0 as usize].lookupResult != LookupResult::Ignore {
            results[newNumResults as usize] = results[i_0 as usize].clone();
            newNumResults = newNumResults.wrapping_add(1)
        }
        i_0 = i_0.wrapping_add(1)
    }
    /* Lookup metadata */
    let mut i_1: size_t = 0 as libc::c_int as size_t;
    while i_1 < results.len() as u64 {
        let mut tmpIndex: u32 = (*library)
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

    if let Some(safezone) = safezone {
        *safezone = (distanceSqrMin as f32).sqrt()
            * 90 as libc::c_int as libc::c_float
            / ((1 as libc::c_int)
                << (*library).precision as libc::c_int - 1 as libc::c_int)
                as libc::c_float
    }

    // TODO: we've removed the end marker, so the length is probably off by one
    results
}
