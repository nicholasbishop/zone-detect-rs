#![allow(non_snake_case, unused_assignments, clippy::cognitive_complexity)]
#[derive(Clone, Debug)]
#[repr(C)]
pub struct ZoneDetectResult {
    pub lookup_result: LookupResult,
    pub polygon_id: u32,
    pub meta_id: u32,
    // TODO: maybe change this to &str
    pub fields: std::collections::HashMap<String, String>,
}
#[derive(Clone)]
#[repr(C)]
pub struct ZoneDetectOpaque {
    pub mapping: Vec<u8>,
    pub table_type: crate::TableType,
    pub version: u8,
    pub precision: u8,
    pub notice: String,
    pub field_names: Vec<String>,
    pub bbox_offset: u32,
    pub metadata_offset: u32,
    pub data_offset: u32,
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
    pub polygon_index: u32,
    pub num_vertices: u64,
    pub done: u8,
    pub first: u8,
    pub reference_start: u32,
    pub reference_end: u32,
    pub reference_direction: i32,
    pub point_lat: i32,
    pub point_lon: i32,
    pub first_lat: i32,
    pub first_lon: i32,
}

impl<'a> Reader<'a> {
    fn new(library: &'a ZoneDetect, polygon_index: u32) -> Reader {
        Reader {
            library,
            polygon_index,
            num_vertices: 0,
            done: 0,
            first: 1,
            reference_start: 0,
            reference_end: 0,
            reference_direction: 0,
            point_lat: 0,
            point_lon: 0,
            first_lat: 0,
            first_lon: 0,
        }
    }
}

fn float_to_fixed_point(
    input: f32,
    scale: f32,
    precision: libc::c_uint,
) -> i32 {
    let input_scaled: f32 = input / scale;
    (input_scaled
        * ((1 as libc::c_int)
            << precision.wrapping_sub(1 as libc::c_int as libc::c_uint))
            as f32) as i32
}

pub fn decode_variable_length_unsigned(
    library: &ZoneDetect,
    index: &mut u32,
    result: &mut u64,
) -> libc::c_uint {
    if *index >= library.mapping.len() as u32 {
        return 0 as libc::c_int as libc::c_uint;
    }
    let mut value: u64 = 0 as libc::c_int as u64;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let buffer = &library.mapping[*index as usize..];
    let mut shift: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    for byte in buffer {
        value |= (*byte as u64 & 0x7f as libc::c_int as libc::c_ulong) << shift;
        shift = shift.wrapping_add(7 as libc::c_uint);
        if *byte as libc::c_int & 0x80 as libc::c_int == 0 {
            break;
        }
        i = i.wrapping_add(1);
    }
    i = i.wrapping_add(1);
    *result = value;
    *index = (*index as libc::c_uint).wrapping_add(i) as u32 as u32;
    i
}
fn decode_variable_length_unsigned_reverse(
    library: &ZoneDetect,
    index: &mut u32,
    result: &mut u64,
) -> libc::c_uint {
    let mut i: u32 = *index;
    if *index >= library.mapping.len() as u32 {
        return 0 as libc::c_int as libc::c_uint;
    }
    if library.mapping[i as usize] as libc::c_int & 0x80 as libc::c_int != 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    if i == 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    i = i.wrapping_sub(1);
    while library.mapping[i as usize] as libc::c_int & 0x80 as libc::c_int != 0
    {
        if i == 0 {
            return 0 as libc::c_int as libc::c_uint;
        }
        i = i.wrapping_sub(1)
    }
    *index = i;
    i = i.wrapping_add(1);
    let mut i2: u32 = i;
    decode_variable_length_unsigned(library, &mut i2, result)
}
fn decode_unsigned_to_signed(value: u64) -> i64 {
    if value & 1 as libc::c_int as libc::c_ulong != 0 {
        -(value.wrapping_div(2 as libc::c_int as libc::c_ulong) as i64)
    } else {
        value.wrapping_div(2 as libc::c_int as libc::c_ulong) as i64
    }
}
fn decode_variable_length_signed(
    library: &ZoneDetect,
    index: &mut u32,
    result: &mut i32,
) -> libc::c_uint {
    let mut value: u64 = 0 as libc::c_int as u64;
    let retval: libc::c_uint =
        decode_variable_length_unsigned(library, index, &mut value);
    *result = decode_unsigned_to_signed(value) as i32;
    retval
}
pub fn parse_string(library: &ZoneDetect, index: &mut u32) -> Option<Vec<u8>> {
    let mut str_length: u64 = 0;
    if decode_variable_length_unsigned(library, index, &mut str_length) == 0 {
        return None;
    }
    let mut str_offset: u32 = *index;
    let mut remote_str: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if str_length >= 256 as libc::c_int as libc::c_ulong {
        str_offset = library
            .metadata_offset
            .wrapping_add(str_length as u32)
            .wrapping_sub(256 as libc::c_int as libc::c_uint);
        remote_str = 1 as libc::c_int as libc::c_uint;
        if decode_variable_length_unsigned(
            library,
            &mut str_offset,
            &mut str_length,
        ) == 0
        {
            return None;
        }
        if str_length > 256 as libc::c_int as libc::c_ulong {
            return None;
        }
    }
    let mut str = Vec::with_capacity(str_length as usize);
    for i in 0..str_length as usize {
        str.push(
            (library.mapping[str_offset as usize + i] as libc::c_int
                ^ 0x80 as libc::c_int) as u8,
        );
    }
    if remote_str == 0 {
        *index = (*index as libc::c_uint).wrapping_add(str_length as u32) as u32
            as u32
    }
    Some(str)
}

fn point_in_box(xl: i32, x: i32, xr: i32, yl: i32, y: i32, yr: i32) -> bool {
    (xl <= x && x <= xr || xr <= x && x <= xl)
        && (yl <= y && y <= yr || yr <= y && y <= yl)
}

fn unshuffle(mut w: u64) -> u32 {
    w &= 0x5555_5555_5555_5555 as libc::c_long as libc::c_ulong;
    w = (w | w >> 1 as libc::c_int)
        & 0x3333_3333_3333_3333 as libc::c_long as libc::c_ulong;
    w = (w | w >> 2 as libc::c_int)
        & 0x0f0f_0f0f_0f0f_0f0f as libc::c_long as libc::c_ulong;
    w = (w | w >> 4 as libc::c_int)
        & 0xff_00ff_00ff_00ff as libc::c_long as libc::c_ulong;
    w = (w | w >> 8 as libc::c_int)
        & 0xffff_0000_ffff as libc::c_long as libc::c_ulong;
    w = (w | w >> 16 as libc::c_int)
        & 0xffff_ffff as libc::c_uint as libc::c_ulong;
    w as u32
}
fn decode_point(point: u64, lat: &mut i32, lon: &mut i32) {
    *lat = decode_unsigned_to_signed(unshuffle(point) as u64) as i32;
    *lon = decode_unsigned_to_signed(unshuffle(point >> 1 as libc::c_int) as u64)
        as i32;
}

fn reader_get_point(
    reader: &mut Reader,
    point_lat: &mut i32,
    point_lon: &mut i32,
) -> libc::c_int {
    let mut reference_done: u8 = 0;
    let mut diff_lat: i32 = 0 as libc::c_int;
    let mut diff_lon: i32 = 0 as libc::c_int;
    loop {
        if reader.done as libc::c_int > 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        if reader.first as libc::c_int != 0
            && (*reader.library).version as libc::c_int == 0 as libc::c_int
        {
            if decode_variable_length_unsigned(
                reader.library,
                &mut reader.polygon_index,
                &mut reader.num_vertices,
            ) == 0
            {
                return -(1 as libc::c_int);
            }
            if reader.num_vertices == 0 {
                return -(1 as libc::c_int);
            }
        }
        reference_done = 0 as libc::c_int as u8;
        if (*reader.library).version as libc::c_int == 1 as libc::c_int {
            let mut point: u64 = 0 as libc::c_int as u64;
            if reader.reference_direction == 0 {
                if decode_variable_length_unsigned(
                    reader.library,
                    &mut reader.polygon_index,
                    &mut point,
                ) == 0
                {
                    return -(1 as libc::c_int);
                }
            } else if reader.reference_direction > 0 as libc::c_int {
                /* Read reference forward */
                if decode_variable_length_unsigned(
                    reader.library,
                    &mut reader.reference_start,
                    &mut point,
                ) == 0
                {
                    return -(1 as libc::c_int);
                }
                if reader.reference_start >= reader.reference_end {
                    reference_done = 1 as libc::c_int as u8
                }
            } else if reader.reference_direction < 0 as libc::c_int {
                /* Read reference backwards */
                if decode_variable_length_unsigned_reverse(
                    reader.library,
                    &mut reader.reference_start,
                    &mut point,
                ) == 0
                {
                    return -(1 as libc::c_int);
                }
                if reader.reference_start <= reader.reference_end {
                    reference_done = 1 as libc::c_int as u8
                }
            }
            if point == 0 {
                /* This is a special marker, it is not allowed in reference mode */
                if reader.reference_direction != 0 {
                    return -(1 as libc::c_int);
                }
                let mut value: u64 = 0;
                if decode_variable_length_unsigned(
                    reader.library,
                    &mut reader.polygon_index,
                    &mut value,
                ) == 0
                {
                    return -(1 as libc::c_int);
                }
                if value == 0 as libc::c_int as libc::c_ulong {
                    reader.done = 2 as libc::c_int as u8
                } else if value == 1 as libc::c_int as libc::c_ulong {
                    let mut diff: i32 = 0;
                    let mut start: u64 = 0;
                    if decode_variable_length_unsigned(
                        reader.library,
                        &mut reader.polygon_index,
                        &mut start,
                    ) == 0
                    {
                        return -(1 as libc::c_int);
                    }
                    if decode_variable_length_signed(
                        reader.library,
                        &mut reader.polygon_index,
                        &mut diff,
                    ) == 0
                    {
                        return -(1 as libc::c_int);
                    }
                    reader.reference_start =
                        (*reader.library).data_offset.wrapping_add(start as u32);
                    reader.reference_end =
                        (*reader.library).data_offset.wrapping_add(
                            (start as i64 + diff as libc::c_long) as u32,
                        );
                    reader.reference_direction = diff;
                    if diff < 0 as libc::c_int {
                        reader.reference_start =
                            reader.reference_start.wrapping_sub(1);
                        reader.reference_end =
                            reader.reference_end.wrapping_sub(1)
                    }
                    continue;
                }
            } else {
                decode_point(point, &mut diff_lat, &mut diff_lon);
                if reader.reference_direction < 0 as libc::c_int {
                    diff_lat = -diff_lat;
                    diff_lon = -diff_lon
                }
            }
        }
        if (*reader.library).version as libc::c_int == 0 as libc::c_int {
            if decode_variable_length_signed(
                reader.library,
                &mut reader.polygon_index,
                &mut diff_lat,
            ) == 0
            {
                return -(1 as libc::c_int);
            }
            if decode_variable_length_signed(
                reader.library,
                &mut reader.polygon_index,
                &mut diff_lon,
            ) == 0
            {
                return -(1 as libc::c_int);
            }
        }
        if reader.done == 0 {
            reader.point_lat += diff_lat;
            reader.point_lon += diff_lon;
            if reader.first != 0 {
                reader.first_lat = reader.point_lat;
                reader.first_lon = reader.point_lon
            }
        } else {
            /* Close the polygon (the closing point is not encoded) */
            reader.point_lat = reader.first_lat;
            reader.point_lon = reader.first_lon;
            reader.done = 2 as libc::c_int as u8
        }
        reader.first = 0 as libc::c_int as u8;
        if reader.library.version != 0 {
            break;
        }
        reader.num_vertices = reader.num_vertices.wrapping_sub(1);
        if reader.num_vertices == 0 {
            reader.done = 1 as libc::c_int as u8
        }
        if !(diff_lat == 0 && diff_lon == 0) {
            break;
        }
    }
    if reference_done != 0 {
        reader.reference_direction = 0 as libc::c_int
    }
    *point_lat = reader.point_lat;
    *point_lon = reader.point_lon;
    1 as libc::c_int
}

fn point_in_polygon(
    library: &ZoneDetect,
    polygon_index: u32,
    latFixedPoint: i32,
    lonFixedPoint: i32,
    // TODO: it seems like these could be combined into an
    // Option<&mut u64>, but I coudln't figure out how to make
    // that compile
    calcDistanceSqrMin: bool,
    distanceSqrMin: &mut u64,
) -> LookupResult {
    let mut point_lat: i32 = 0;
    let mut point_lon: i32 = 0;
    let mut prevLat: i32 = 0 as libc::c_int;
    let mut prevLon: i32 = 0 as libc::c_int;
    let mut prevQuadrant: libc::c_int = 0 as libc::c_int;
    let mut winding: libc::c_int = 0 as libc::c_int;
    let mut first: u8 = 1 as libc::c_int as u8;
    let mut reader = Reader::new(library, polygon_index);
    loop {
        let result: libc::c_int =
            reader_get_point(&mut reader, &mut point_lat, &mut point_lon);
        if result < 0 as libc::c_int {
            return LookupResult::ParseError;
        } else {
            if result == 0 as libc::c_int {
                break;
            }
            /* Check if point is ON the border */
            if point_lat == latFixedPoint && point_lon == lonFixedPoint {
                if calcDistanceSqrMin {
                    *distanceSqrMin = 0 as libc::c_int as u64
                }
                return LookupResult::OnBorderVertex;
            }
            /* Find quadrant */
            let mut quadrant: libc::c_int = 0;
            if point_lat >= latFixedPoint {
                if point_lon >= lonFixedPoint {
                    quadrant = 0 as libc::c_int
                } else {
                    quadrant = 1 as libc::c_int
                }
            } else if point_lon >= lonFixedPoint {
                quadrant = 3 as libc::c_int
            } else {
                quadrant = 2 as libc::c_int
            }
            if first == 0 {
                let mut windingNeedCompare: libc::c_int = 0 as libc::c_int;
                let mut lineIsStraight: libc::c_int = 0 as libc::c_int;
                let mut a: f32 = 0 as libc::c_int as f32;
                let mut b: f32 = 0 as libc::c_int as f32;
                /* Calculate winding number */
                if quadrant != prevQuadrant {
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
                if point_lon == prevLon || point_lat == prevLat {
                    lineIsStraight = 1 as libc::c_int
                }
                /* Calculate the parameters of y=ax+b if needed */
                if lineIsStraight == 0
                    && (calcDistanceSqrMin || windingNeedCompare != 0)
                {
                    a = (point_lat as f32 - prevLat as f32)
                        / (point_lon as f32 - prevLon as f32);
                    b = point_lat as f32 - a * point_lon as f32
                }
                let onStraight = point_in_box(
                    point_lat,
                    latFixedPoint,
                    prevLat,
                    point_lon,
                    lonFixedPoint,
                    prevLon,
                );
                if lineIsStraight != 0
                    && (onStraight || windingNeedCompare != 0)
                {
                    if calcDistanceSqrMin {
                        *distanceSqrMin = 0 as libc::c_int as u64
                    }
                    return LookupResult::OnBorderSegment;
                }
                /* Jumped two quadrants. */
                if windingNeedCompare != 0 {
                    /* Check if the target is on the border */
                    let intersectLon: i32 =
                        ((latFixedPoint as f32 - b) / a) as i32;
                    if intersectLon >= lonFixedPoint - 1 as libc::c_int
                        && intersectLon <= lonFixedPoint + 1 as libc::c_int
                    {
                        if calcDistanceSqrMin {
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
                if calcDistanceSqrMin {
                    let mut closestLon: f32 = 0.;
                    let mut closestLat: f32 = 0.;
                    if lineIsStraight == 0 {
                        closestLon = (lonFixedPoint as f32
                            + a * latFixedPoint as f32
                            - a * b)
                            / (a * a + 1 as libc::c_int as f32);
                        closestLat = (a
                            * (lonFixedPoint as f32 + a * latFixedPoint as f32)
                            + b)
                            / (a * a + 1 as libc::c_int as f32)
                    } else if point_lon == prevLon {
                        closestLon = point_lon as f32;
                        closestLat = latFixedPoint as f32
                    } else {
                        closestLon = lonFixedPoint as f32;
                        closestLat = point_lat as f32
                    }
                    let closestInBox = point_in_box(
                        point_lon,
                        closestLon as i32,
                        prevLon,
                        point_lat,
                        closestLat as i32,
                        prevLat,
                    );
                    let mut diff_lat: i64 = 0;
                    let mut diff_lon: i64 = 0;
                    if closestInBox {
                        /* Calculate squared distance to segment. */
                        diff_lat = (closestLat - latFixedPoint as f32) as i64;
                        diff_lon = (closestLon - lonFixedPoint as f32) as i64
                    } else {
                        /*
                         * Calculate squared distance to vertices
                         * It is enough to check the current point since the polygon is closed.
                         */
                        diff_lat = (point_lat - latFixedPoint) as i64;
                        diff_lon = (point_lon - lonFixedPoint) as i64
                    }
                    /* Note: lon has half scale */
                    let distanceSqr: u64 = ((diff_lat * diff_lat) as u64)
                        .wrapping_add(
                            ((diff_lon * diff_lon) as u64).wrapping_mul(
                                4 as libc::c_int as libc::c_ulong,
                            ),
                        );
                    if distanceSqr < *distanceSqrMin {
                        *distanceSqrMin = distanceSqr
                    }
                }
            }
            prevQuadrant = quadrant;
            prevLat = point_lat;
            prevLon = point_lon;
            first = 0 as libc::c_int as u8
        }
    }
    if winding == -(4 as libc::c_int) {
        return LookupResult::InZone;
    } else if winding == 4 as libc::c_int {
        return LookupResult::InExcludedZone;
    } else if winding == 0 as libc::c_int {
        return LookupResult::NotInZone;
    }
    /* Should not happen */
    if calcDistanceSqrMin {
        *distanceSqrMin = 0 as libc::c_int as u64
    }
    LookupResult::OnBorderSegment
}

pub fn lookup(
    library: &ZoneDetect,
    location: crate::Location,
    safezone: Option<&mut f32>,
) -> Vec<ZoneDetectResult> {
    let latFixedPoint: i32 = float_to_fixed_point(
        location.latitude,
        90 as libc::c_int as f32,
        library.precision as libc::c_uint,
    );
    let lonFixedPoint: i32 = float_to_fixed_point(
        location.longitude,
        180 as libc::c_int as f32,
        library.precision as libc::c_uint,
    );
    let mut distanceSqrMin: u64 = -(1 as libc::c_int) as u64;
    /* Parse the header */
    /* Iterate over all polygons */
    let mut bboxIndex: u32 = library.bbox_offset;
    let mut metadataIndex: u32 = 0 as libc::c_int as u32;
    let mut polygon_index: u32 = 0 as libc::c_int as u32;
    let mut results = Vec::new();
    let mut polygon_id: u32 = 0 as libc::c_int as u32;
    while bboxIndex < library.metadata_offset {
        let mut minLat: i32 = 0;
        let mut minLon: i32 = 0;
        let mut maxLat: i32 = 0;
        let mut maxLon: i32 = 0;
        let mut metadataIndexDelta: i32 = 0;
        let mut polygon_indexDelta: u64 = 0;
        if decode_variable_length_signed(library, &mut bboxIndex, &mut minLat)
            == 0
        {
            break;
        }
        if decode_variable_length_signed(library, &mut bboxIndex, &mut minLon)
            == 0
        {
            break;
        }
        if decode_variable_length_signed(library, &mut bboxIndex, &mut maxLat)
            == 0
        {
            break;
        }
        if decode_variable_length_signed(library, &mut bboxIndex, &mut maxLon)
            == 0
        {
            break;
        }
        if decode_variable_length_signed(
            library,
            &mut bboxIndex,
            &mut metadataIndexDelta,
        ) == 0
        {
            break;
        }
        if decode_variable_length_unsigned(
            library,
            &mut bboxIndex,
            &mut polygon_indexDelta,
        ) == 0
        {
            break;
        }
        metadataIndex = (metadataIndex as libc::c_uint)
            .wrapping_add(metadataIndexDelta as u32)
            as u32 as u32;
        polygon_index = (polygon_index as libc::c_uint)
            .wrapping_add(polygon_indexDelta as u32)
            as u32 as u32;
        if latFixedPoint < minLat {
            break;
        }
        if latFixedPoint <= maxLat
            && lonFixedPoint >= minLon
            && lonFixedPoint <= maxLon
        {
            let lookup_result = point_in_polygon(
                library,
                library.data_offset.wrapping_add(polygon_index),
                latFixedPoint,
                lonFixedPoint,
                safezone.is_some(),
                &mut distanceSqrMin,
            );
            if lookup_result == LookupResult::ParseError {
                break;
            }
            if lookup_result != LookupResult::NotInZone {
                results.push(ZoneDetectResult {
                    polygon_id,
                    meta_id: metadataIndex,
                    fields: std::collections::HashMap::with_capacity(
                        library.field_names.len(),
                    ),
                    lookup_result,
                });
            }
        }
        polygon_id = polygon_id.wrapping_add(1)
    }
    /* Clean up results */
    for i in 0..results.len() {
        let mut insideSum: libc::c_int = 0 as libc::c_int;
        let mut overrideResult = LookupResult::Ignore;
        for j in i..results.len() {
            if results[i as usize].meta_id == results[j as usize].meta_id {
                let tmpResult = results[j as usize].lookup_result;
                results[j as usize].lookup_result = LookupResult::Ignore;
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
        }
        if overrideResult != LookupResult::Ignore {
            results[i as usize].lookup_result = overrideResult
        } else if insideSum != 0 {
            results[i as usize].lookup_result = LookupResult::InZone
        }
    }
    /* Remove zones to ignore */
    results.retain(|r| r.lookup_result != LookupResult::Ignore);
    /* Lookup metadata */
    for result in &mut results {
        let mut tmpIndex: u32 =
            library.metadata_offset.wrapping_add(result.meta_id);

        for j in 0..library.field_names.len() {
            let key = library.field_names[j].clone();
            let value = crate::parse_string(&*library, &mut tmpIndex)
                .expect("failed to get field data");
            result.fields.insert(key, value);
        }
    }

    if let Some(safezone) = safezone {
        let den = (1 << (library.precision - 1)) as f32;
        *safezone = (distanceSqrMin as f32).sqrt() * 90f32 / den;
    }

    results
}
