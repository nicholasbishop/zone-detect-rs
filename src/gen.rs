// Note: this code was originally zonedetect.c (see copyright notice
// below). It was automatically converted to Rust with the c2rust
// tool. It has been manually cleaned up quite a bit, so nothing is
// unsafe now, but there's still a lot of weird looking code due to
// the c2rust step.

#![allow(clippy::cognitive_complexity)]

// TODO
use crate::{Database as ZoneDetect, Zone};

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

fn float_to_fixed_point(input: f32, scale: f32, precision: u32) -> i32 {
    let input_scaled: f32 = input / scale;
    (input_scaled
        * ((1 as i32) << precision.wrapping_sub(1 as i32 as u32)) as f32)
        as i32
}

pub fn decode_variable_length_unsigned(
    library: &ZoneDetect,
    index: &mut u32,
    result: &mut u64,
) -> u32 {
    if *index >= library.mapping.len() as u32 {
        return 0 as i32 as u32;
    }
    let mut value: u64 = 0 as i32 as u64;
    let mut i: u32 = 0 as i32 as u32;
    let buffer = &library.mapping[*index as usize..];
    let mut shift: u32 = 0 as i32 as u32;
    for byte in buffer {
        value |= (*byte as u64 & 0x7f as i32 as u64) << shift;
        shift = shift.wrapping_add(7 as u32);
        if *byte as i32 & 0x80 as i32 == 0 {
            break;
        }
        i = i.wrapping_add(1);
    }
    i = i.wrapping_add(1);
    *result = value;
    *index = (*index as u32).wrapping_add(i) as u32 as u32;
    i
}
fn decode_variable_length_unsigned_reverse(
    library: &ZoneDetect,
    index: &mut u32,
    result: &mut u64,
) -> u32 {
    let mut i: u32 = *index;
    if *index >= library.mapping.len() as u32 {
        return 0 as i32 as u32;
    }
    if library.mapping[i as usize] as i32 & 0x80 as i32 != 0 {
        return 0 as i32 as u32;
    }
    if i == 0 {
        return 0 as i32 as u32;
    }
    i = i.wrapping_sub(1);
    while library.mapping[i as usize] as i32 & 0x80 as i32 != 0 {
        if i == 0 {
            return 0 as i32 as u32;
        }
        i = i.wrapping_sub(1)
    }
    *index = i;
    i = i.wrapping_add(1);
    let mut i2: u32 = i;
    decode_variable_length_unsigned(library, &mut i2, result)
}
fn decode_unsigned_to_signed(value: u64) -> i64 {
    if value & 1 as i32 as u64 != 0 {
        -(value.wrapping_div(2 as i32 as u64) as i64)
    } else {
        value.wrapping_div(2 as i32 as u64) as i64
    }
}
fn decode_variable_length_signed(
    library: &ZoneDetect,
    index: &mut u32,
    result: &mut i32,
) -> u32 {
    let mut value: u64 = 0 as i32 as u64;
    let retval: u32 =
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
    let mut remote_str: u32 = 0 as i32 as u32;
    if str_length >= 256 as i32 as u64 {
        str_offset = library
            .metadata_offset
            .wrapping_add(str_length as u32)
            .wrapping_sub(256 as i32 as u32);
        remote_str = 1 as i32 as u32;
        if decode_variable_length_unsigned(
            library,
            &mut str_offset,
            &mut str_length,
        ) == 0
        {
            return None;
        }
        if str_length > 256 as i32 as u64 {
            return None;
        }
    }
    let mut str = Vec::with_capacity(str_length as usize);
    for i in 0..str_length as usize {
        str.push(
            (library.mapping[str_offset as usize + i] as i32 ^ 0x80 as i32)
                as u8,
        );
    }
    if remote_str == 0 {
        *index = (*index as u32).wrapping_add(str_length as u32) as u32 as u32
    }
    Some(str)
}

fn point_in_box(xl: i32, x: i32, xr: i32, yl: i32, y: i32, yr: i32) -> bool {
    (xl <= x && x <= xr || xr <= x && x <= xl)
        && (yl <= y && y <= yr || yr <= y && y <= yl)
}

fn unshuffle(mut w: u64) -> u32 {
    w &= 0x5555_5555_5555_5555 as i64 as u64;
    w = (w | w >> 1 as i32) & 0x3333_3333_3333_3333 as i64 as u64;
    w = (w | w >> 2 as i32) & 0x0f0f_0f0f_0f0f_0f0f as i64 as u64;
    w = (w | w >> 4 as i32) & 0xff_00ff_00ff_00ff as i64 as u64;
    w = (w | w >> 8 as i32) & 0xffff_0000_ffff as i64 as u64;
    w = (w | w >> 16 as i32) & 0xffff_ffff as u32 as u64;
    w as u32
}
fn decode_point(point: u64, lat: &mut i32, lon: &mut i32) {
    *lat = decode_unsigned_to_signed(unshuffle(point) as u64) as i32;
    *lon =
        decode_unsigned_to_signed(unshuffle(point >> 1 as i32) as u64) as i32;
}

fn reader_get_point(
    reader: &mut Reader,
    point_lat: &mut i32,
    point_lon: &mut i32,
) -> i32 {
    let mut reference_done;
    let mut diff_lat: i32 = 0 as i32;
    let mut diff_lon: i32 = 0 as i32;
    loop {
        if reader.done as i32 > 1 as i32 {
            return 0 as i32;
        }
        if reader.first as i32 != 0
            && (*reader.library).version as i32 == 0 as i32
        {
            if decode_variable_length_unsigned(
                reader.library,
                &mut reader.polygon_index,
                &mut reader.num_vertices,
            ) == 0
            {
                return -(1 as i32);
            }
            if reader.num_vertices == 0 {
                return -(1 as i32);
            }
        }
        reference_done = 0 as i32 as u8;
        if (*reader.library).version as i32 == 1 as i32 {
            let mut point: u64 = 0 as i32 as u64;
            if reader.reference_direction == 0 {
                if decode_variable_length_unsigned(
                    reader.library,
                    &mut reader.polygon_index,
                    &mut point,
                ) == 0
                {
                    return -(1 as i32);
                }
            } else if reader.reference_direction > 0 as i32 {
                /* Read reference forward */
                if decode_variable_length_unsigned(
                    reader.library,
                    &mut reader.reference_start,
                    &mut point,
                ) == 0
                {
                    return -(1 as i32);
                }
                if reader.reference_start >= reader.reference_end {
                    reference_done = 1 as i32 as u8
                }
            } else if reader.reference_direction < 0 as i32 {
                /* Read reference backwards */
                if decode_variable_length_unsigned_reverse(
                    reader.library,
                    &mut reader.reference_start,
                    &mut point,
                ) == 0
                {
                    return -(1 as i32);
                }
                if reader.reference_start <= reader.reference_end {
                    reference_done = 1 as i32 as u8
                }
            }
            if point == 0 {
                /* This is a special marker, it is not allowed in reference mode */
                if reader.reference_direction != 0 {
                    return -(1 as i32);
                }
                let mut value: u64 = 0;
                if decode_variable_length_unsigned(
                    reader.library,
                    &mut reader.polygon_index,
                    &mut value,
                ) == 0
                {
                    return -(1 as i32);
                }
                if value == 0 as i32 as u64 {
                    reader.done = 2 as i32 as u8
                } else if value == 1 as i32 as u64 {
                    let mut diff: i32 = 0;
                    let mut start: u64 = 0;
                    if decode_variable_length_unsigned(
                        reader.library,
                        &mut reader.polygon_index,
                        &mut start,
                    ) == 0
                    {
                        return -(1 as i32);
                    }
                    if decode_variable_length_signed(
                        reader.library,
                        &mut reader.polygon_index,
                        &mut diff,
                    ) == 0
                    {
                        return -(1 as i32);
                    }
                    reader.reference_start = (*reader.library)
                        .data_offset
                        .wrapping_add(start as u32);
                    reader.reference_end = (*reader.library)
                        .data_offset
                        .wrapping_add((start as i64 + diff as i64) as u32);
                    reader.reference_direction = diff;
                    if diff < 0 as i32 {
                        reader.reference_start =
                            reader.reference_start.wrapping_sub(1);
                        reader.reference_end =
                            reader.reference_end.wrapping_sub(1)
                    }
                    continue;
                }
            } else {
                decode_point(point, &mut diff_lat, &mut diff_lon);
                if reader.reference_direction < 0 as i32 {
                    diff_lat = -diff_lat;
                    diff_lon = -diff_lon
                }
            }
        }
        if (*reader.library).version as i32 == 0 as i32 {
            if decode_variable_length_signed(
                reader.library,
                &mut reader.polygon_index,
                &mut diff_lat,
            ) == 0
            {
                return -(1 as i32);
            }
            if decode_variable_length_signed(
                reader.library,
                &mut reader.polygon_index,
                &mut diff_lon,
            ) == 0
            {
                return -(1 as i32);
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
            reader.done = 2 as i32 as u8
        }
        reader.first = 0 as i32 as u8;
        if reader.library.version != 0 {
            break;
        }
        reader.num_vertices = reader.num_vertices.wrapping_sub(1);
        if reader.num_vertices == 0 {
            reader.done = 1 as i32 as u8
        }
        if !(diff_lat == 0 && diff_lon == 0) {
            break;
        }
    }
    if reference_done != 0 {
        reader.reference_direction = 0 as i32
    }
    *point_lat = reader.point_lat;
    *point_lon = reader.point_lon;
    1 as i32
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointLookupResult {
    Ignore,
    ParseError,
    NotInZone,
    InZone,
    InExcludedZone,
    OnBorderVertex,
    OnBorderSegment,
}

fn point_in_polygon(
    library: &ZoneDetect,
    polygon_index: u32,
    lat_fixed_point: i32,
    lon_fixed_point: i32,
    // TODO: it seems like these could be combined into an
    // Option<&mut u64>, but I coudln't figure out how to make
    // that compile
    calc_distance_sqr_min: bool,
    distance_sqr_min: &mut u64,
) -> PointLookupResult {
    let mut point_lat: i32 = 0;
    let mut point_lon: i32 = 0;
    let mut prev_lat: i32 = 0 as i32;
    let mut prev_lon: i32 = 0 as i32;
    let mut prev_quadrant: i32 = 0 as i32;
    let mut winding: i32 = 0 as i32;
    let mut first: u8 = 1 as i32 as u8;
    let mut reader = Reader::new(library, polygon_index);
    loop {
        let result: i32 =
            reader_get_point(&mut reader, &mut point_lat, &mut point_lon);
        if result < 0 as i32 {
            return PointLookupResult::ParseError;
        } else {
            if result == 0 as i32 {
                break;
            }
            /* Check if point is ON the border */
            if point_lat == lat_fixed_point && point_lon == lon_fixed_point {
                if calc_distance_sqr_min {
                    *distance_sqr_min = 0 as i32 as u64
                }
                return PointLookupResult::OnBorderVertex;
            }
            /* Find quadrant */
            let quadrant: i32;
            if point_lat >= lat_fixed_point {
                if point_lon >= lon_fixed_point {
                    quadrant = 0 as i32
                } else {
                    quadrant = 1 as i32
                }
            } else if point_lon >= lon_fixed_point {
                quadrant = 3 as i32
            } else {
                quadrant = 2 as i32
            }
            if first == 0 {
                let mut winding_need_compare: i32 = 0 as i32;
                let mut line_is_straight: i32 = 0 as i32;
                let mut a: f32 = 0 as i32 as f32;
                let mut b: f32 = 0 as i32 as f32;
                /* Calculate winding number */
                if quadrant != prev_quadrant {
                    if quadrant == (prev_quadrant + 1 as i32) % 4 as i32 {
                        winding += 1
                    } else if (quadrant + 1 as i32) % 4 as i32 == prev_quadrant
                    {
                        winding -= 1
                    } else {
                        winding_need_compare = 1 as i32
                    }
                }
                /* Avoid horizontal and vertical lines */
                if point_lon == prev_lon || point_lat == prev_lat {
                    line_is_straight = 1 as i32
                }
                /* Calculate the parameters of y=ax+b if needed */
                if line_is_straight == 0
                    && (calc_distance_sqr_min || winding_need_compare != 0)
                {
                    a = (point_lat as f32 - prev_lat as f32)
                        / (point_lon as f32 - prev_lon as f32);
                    b = point_lat as f32 - a * point_lon as f32
                }
                let on_straight = point_in_box(
                    point_lat,
                    lat_fixed_point,
                    prev_lat,
                    point_lon,
                    lon_fixed_point,
                    prev_lon,
                );
                if line_is_straight != 0
                    && (on_straight || winding_need_compare != 0)
                {
                    if calc_distance_sqr_min {
                        *distance_sqr_min = 0 as i32 as u64
                    }
                    return PointLookupResult::OnBorderSegment;
                }
                /* Jumped two quadrants. */
                if winding_need_compare != 0 {
                    /* Check if the target is on the border */
                    let intersect_lon: i32 =
                        ((lat_fixed_point as f32 - b) / a) as i32;
                    if intersect_lon >= lon_fixed_point - 1 as i32
                        && intersect_lon <= lon_fixed_point + 1 as i32
                    {
                        if calc_distance_sqr_min {
                            *distance_sqr_min = 0 as i32 as u64
                        }
                        return PointLookupResult::OnBorderSegment;
                    }
                    /* Ok, it's not. In which direction did we go round the target? */
                    let sign: i32 = if intersect_lon < lon_fixed_point {
                        2 as i32
                    } else {
                        -(2 as i32)
                    };
                    if quadrant == 2 as i32 || quadrant == 3 as i32 {
                        winding += sign
                    } else {
                        winding -= sign
                    }
                }
                /* Calculate closest point on line (if needed) */
                if calc_distance_sqr_min {
                    let closest_lon: f32;
                    let closest_lat: f32;
                    if line_is_straight == 0 {
                        closest_lon = (lon_fixed_point as f32
                            + a * lat_fixed_point as f32
                            - a * b)
                            / (a * a + 1 as i32 as f32);
                        closest_lat = (a
                            * (lon_fixed_point as f32
                                + a * lat_fixed_point as f32)
                            + b)
                            / (a * a + 1 as i32 as f32)
                    } else if point_lon == prev_lon {
                        closest_lon = point_lon as f32;
                        closest_lat = lat_fixed_point as f32
                    } else {
                        closest_lon = lon_fixed_point as f32;
                        closest_lat = point_lat as f32
                    }
                    let closest_in_box = point_in_box(
                        point_lon,
                        closest_lon as i32,
                        prev_lon,
                        point_lat,
                        closest_lat as i32,
                        prev_lat,
                    );
                    let diff_lat: i64;
                    let diff_lon: i64;
                    if closest_in_box {
                        /* Calculate squared distance to segment. */
                        diff_lat =
                            (closest_lat - lat_fixed_point as f32) as i64;
                        diff_lon = (closest_lon - lon_fixed_point as f32) as i64
                    } else {
                        /*
                         * Calculate squared distance to vertices
                         * It is enough to check the current point since the polygon is closed.
                         */
                        diff_lat = (point_lat - lat_fixed_point) as i64;
                        diff_lon = (point_lon - lon_fixed_point) as i64
                    }
                    /* Note: lon has half scale */
                    let distance_sqr: u64 = ((diff_lat * diff_lat) as u64)
                        .wrapping_add(
                            ((diff_lon * diff_lon) as u64)
                                .wrapping_mul(4 as i32 as u64),
                        );
                    if distance_sqr < *distance_sqr_min {
                        *distance_sqr_min = distance_sqr
                    }
                }
            }
            prev_quadrant = quadrant;
            prev_lat = point_lat;
            prev_lon = point_lon;
            first = 0 as i32 as u8
        }
    }
    if winding == -(4 as i32) {
        return PointLookupResult::InZone;
    } else if winding == 4 as i32 {
        return PointLookupResult::InExcludedZone;
    } else if winding == 0 as i32 {
        return PointLookupResult::NotInZone;
    }
    /* Should not happen */
    if calc_distance_sqr_min {
        *distance_sqr_min = 0 as i32 as u64
    }
    PointLookupResult::OnBorderSegment
}

pub struct ZoneDetectResult {
    pub result: PointLookupResult,
    pub zone: Zone,
}

pub fn lookup(
    library: &ZoneDetect,
    location: crate::Location,
    safezone: Option<&mut f32>,
) -> Vec<ZoneDetectResult> {
    let lat_fixed_point: i32 = float_to_fixed_point(
        location.latitude,
        90 as i32 as f32,
        library.precision as u32,
    );
    let lon_fixed_point: i32 = float_to_fixed_point(
        location.longitude,
        180 as i32 as f32,
        library.precision as u32,
    );
    let mut distance_sqr_min: u64 = -(1 as i32) as u64;
    /* Parse the header */
    /* Iterate over all polygons */
    let mut bbox_index: u32 = library.bbox_offset;
    let mut metadata_index: u32 = 0 as i32 as u32;
    let mut polygon_index: u32 = 0 as i32 as u32;
    let mut results = Vec::new();
    let mut polygon_id: u32 = 0 as i32 as u32;
    while bbox_index < library.metadata_offset {
        let mut min_lat: i32 = 0;
        let mut min_lon: i32 = 0;
        let mut max_lat: i32 = 0;
        let mut max_lon: i32 = 0;
        let mut metadata_index_delta: i32 = 0;
        let mut polygon_index_delta: u64 = 0;
        if decode_variable_length_signed(library, &mut bbox_index, &mut min_lat)
            == 0
        {
            break;
        }
        if decode_variable_length_signed(library, &mut bbox_index, &mut min_lon)
            == 0
        {
            break;
        }
        if decode_variable_length_signed(library, &mut bbox_index, &mut max_lat)
            == 0
        {
            break;
        }
        if decode_variable_length_signed(library, &mut bbox_index, &mut max_lon)
            == 0
        {
            break;
        }
        if decode_variable_length_signed(
            library,
            &mut bbox_index,
            &mut metadata_index_delta,
        ) == 0
        {
            break;
        }
        if decode_variable_length_unsigned(
            library,
            &mut bbox_index,
            &mut polygon_index_delta,
        ) == 0
        {
            break;
        }
        metadata_index = (metadata_index as u32)
            .wrapping_add(metadata_index_delta as u32)
            as u32 as u32;
        polygon_index = (polygon_index as u32)
            .wrapping_add(polygon_index_delta as u32)
            as u32 as u32;
        if lat_fixed_point < min_lat {
            break;
        }
        if lat_fixed_point <= max_lat
            && lon_fixed_point >= min_lon
            && lon_fixed_point <= max_lon
        {
            let lookup_result = point_in_polygon(
                library,
                library.data_offset.wrapping_add(polygon_index),
                lat_fixed_point,
                lon_fixed_point,
                safezone.is_some(),
                &mut distance_sqr_min,
            );
            if lookup_result == PointLookupResult::ParseError {
                break;
            }
            if lookup_result != PointLookupResult::NotInZone {
                results.push(ZoneDetectResult {
                    zone: Zone {
                        polygon_id,
                        meta_id: metadata_index,
                        fields: std::collections::HashMap::with_capacity(
                            library.field_names.len(),
                        ),
                    },
                    result: lookup_result,
                });
            }
        }
        polygon_id = polygon_id.wrapping_add(1)
    }
    /* Clean up results */
    for i in 0..results.len() {
        let mut inside_sum: i32 = 0 as i32;
        let mut override_result = PointLookupResult::Ignore;
        for j in i..results.len() {
            if results[i as usize].zone.meta_id
                == results[j as usize].zone.meta_id
            {
                let tmp_result = results[j as usize].result;
                results[j as usize].result = PointLookupResult::Ignore;
                /* This is the same result. Is it an exclusion zone? */
                if tmp_result == PointLookupResult::InZone {
                    inside_sum += 1
                } else if tmp_result == PointLookupResult::InExcludedZone {
                    inside_sum -= 1
                } else {
                    /* If on the bodrder then the final result is on the border */
                    override_result = tmp_result
                }
            }
        }
        if override_result != PointLookupResult::Ignore {
            results[i as usize].result = override_result
        } else if inside_sum != 0 {
            results[i as usize].result = PointLookupResult::InZone
        }
    }
    /* Remove zones to ignore */
    results.retain(|r| r.result != PointLookupResult::Ignore);
    /* Lookup metadata */
    for result in &mut results {
        let mut tmp_index: u32 =
            library.metadata_offset.wrapping_add(result.zone.meta_id);

        for j in 0..library.field_names.len() {
            let key = library.field_names[j].clone();
            let value = crate::parse_string(&*library, &mut tmp_index)
                .expect("failed to get field data");
            result.zone.fields.insert(key, value);
        }
    }

    if let Some(safezone) = safezone {
        let den = (1 << (library.precision - 1)) as f32;
        *safezone = (distance_sqr_min as f32).sqrt() * 90f32 / den;
    }

    results
}
