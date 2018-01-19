// -*- mode: rust; -*-
//
// This file is part of curve25519-dalek.
// Copyright (c) 2016-2017 Isis Lovecruft, Henry de Valence
// See LICENSE for licensing information.
//
// Authors:
// - Isis Agora Lovecruft <isis@patternsinthevoid.net>
// - Henry de Valence <hdevalence@hdevalence.ca>

//! This module contains constants used by the AVX2 backend.

use stdsimd::simd::u32x8;

use backend::avx2::field::FieldElement32x4;
use backend::avx2::edwards::ExtendedPoint;

/// The low limbs of (2p, 2p, 2p, 2p), so that
/// ```no_run
/// (2p, 2p, 2p, 2p) = [P_TIMES_2_LO, P_TIMES_2_HI, P_TIMES_2_HI, P_TIMES_2_HI, P_TIMES_2_HI]
/// ```
pub(crate) static P_TIMES_2_LO: u32x8 =
    u32x8::new(67108845 << 1, 67108845 << 1, 33554431 << 1, 33554431 << 1, 67108845 << 1, 67108845 << 1, 33554431 << 1, 33554431 << 1);

/// The high limbs of (2p, 2p, 2p, 2p), so that
/// ```no_run
/// (2p, 2p, 2p, 2p) = [P_TIMES_2_LO, P_TIMES_2_HI, P_TIMES_2_HI, P_TIMES_2_HI, P_TIMES_2_HI]
/// ```
pub(crate) static P_TIMES_2_HI: u32x8 =
    u32x8::new(67108863 << 1, 67108863 << 1, 33554431 << 1, 33554431 << 1, 67108863 << 1, 67108863 << 1, 33554431 << 1, 33554431 << 1);

/// The low limbs of (16p, 16p, 16p, 16p), so that
/// ```no_run
/// (16p, 16p, 16p, 16p) = [P_TIMES_16_LO, P_TIMES_16_HI, P_TIMES_16_HI, P_TIMES_16_HI, P_TIMES_16_HI]
/// ```
pub(crate) static P_TIMES_16_LO: u32x8 =
    u32x8::new(67108845 << 4, 67108845 << 4, 33554431 << 4, 33554431 << 4, 67108845 << 4, 67108845 << 4, 33554431 << 4, 33554431 << 4);

/// The high limbs of (16p, 16p, 16p, 16p), so that
/// ```no_run
/// (16p, 16p, 16p, 16p) = [P_TIMES_16_LO, P_TIMES_16_HI, P_TIMES_16_HI, P_TIMES_16_HI, P_TIMES_16_HI]
/// ```
pub(crate) static P_TIMES_16_HI: u32x8 =
    u32x8::new(67108863 << 4, 67108863 << 4, 33554431 << 4, 33554431 << 4, 67108863 << 4, 67108863 << 4, 33554431 << 4, 33554431 << 4);

pub(crate) static P_TIMES_2_MASKED: FieldElement32x4 = FieldElement32x4([
    u32x8::new(        0, 134217690,        0, 67108862, 134217690,         0, 67108862,        0),
    u32x8::new(        0, 134217726,        0, 67108862, 134217726,         0, 67108862,        0),
    u32x8::new(        0, 134217726,        0, 67108862, 134217726,         0, 67108862,        0),
    u32x8::new(        0, 134217726,        0, 67108862, 134217726,         0, 67108862,        0),
    u32x8::new(        0, 134217726,        0, 67108862, 134217726,         0, 67108862,        0)
]);

/// Odd multiples of the Ed25519 basepoint:
pub static ODD_MULTIPLES_OF_BASEPOINT: [ExtendedPoint; 8] = [
    ExtendedPoint(FieldElement32x4([
        u32x8::new(52811034, 40265304, 25909283, 26843545,        1, 28827043,        0, 27438313),
        u32x8::new(16144682, 13421772, 17082669, 20132659,        0, 39759291,        0,   244362), 
        u32x8::new(27570973, 26843545, 30858332,  6710886,        0,  8635006,        0, 11264893), 
        u32x8::new(40966398, 53687091,  8378388, 13421772,        0, 19351346,        0, 13413597),
        u32x8::new(20764389, 40265318,  8758491, 26843545,        0, 16611511,        0, 27139452),
    ])),
    ExtendedPoint(FieldElement32x4([
        u32x8::new(63703867, 19156774,   608100,  2486757, 12685460,  3173753, 21649412, 16313381),
        u32x8::new(52397038, 65858675, 26775664, 16661035, 14269998,  9080558,  1059463, 28938752),
        u32x8::new( 5461635, 28034025, 23358301,  1245198,  1367765, 20288887, 31111942, 18395221),
        u32x8::new( 1886934, 32436996,   681756, 18977693,  8129860, 40112764, 25764567, 11876840),
        u32x8::new(63042604, 52399761, 22087481, 29829870,  8565820, 33723612, 28645162,  8502864),
    ])),
    ExtendedPoint(FieldElement32x4([
        u32x8::new(14879397, 3951036,   9454671, 16606238, 23529732, 44147004, 11890541, 17067526), 
        u32x8::new(58509479, 57216664,  9671992, 32001147, 60966207, 11801823, 10808378, 15115613), 
        u32x8::new(54854992, 39210911,  8112050,  1353604,  1337416, 35520540, 32967851, 17786030), 
        u32x8::new(59007462, 40864509, 26240923, 30403852, 28456403, 21546582, 32732450, 21005910), 
        u32x8::new(40711675, 22446613,  9664668, 12483629, 26142305, 56254715, 15439904,   214849),
    ])),
    ExtendedPoint(FieldElement32x4([
        u32x8::new(52231579, 51632644,   173613,  7677257, 26374424, 45994428,  5303371,  1425942), 
        u32x8::new(38126791, 48854506, 23252518, 30611978, 49977504, 66706952,  1076178, 27100873),
        u32x8::new(26349427, 63077566, 20258199,  3884787, 33226507,  2371423,  5787271, 18628170), 
        u32x8::new(15005754, 22729577,  4978944,  2522289,  1404784, 56367795, 22517039, 29271243), 
        u32x8::new(22748934, 35977548, 25561257, 31734126, 22775284, 32000077,   927866,  2278697),
    ])),
    ExtendedPoint(FieldElement32x4([
        u32x8::new(66090281, 61980626, 23780289,  6519561, 62542590, 47174086, 28818882, 15661068), 
        u32x8::new(17433715, 12931425, 12232056,  7885877, 44179512, 35590146, 32787344, 22631048), 
        u32x8::new(43729883, 6870635,  15782399, 11810556,  2652935, 31800505, 23683367, 13638649), 
        u32x8::new(64007953, 40242373, 32810277, 20180235, 20399465, 48133835, 32913956, 19094667), 
        u32x8::new(56562708, 40269142, 18953105,  9027935, 35700921, 12896915, 14757156, 22773619),
    ])),
    ExtendedPoint(FieldElement32x4([
        u32x8::new(65129016, 34709402, 25132940, 13788431,  3661652, 16914498, 27409409, 18941039), 
        u32x8::new(42488074, 49427602,  6177212, 20812339, 41644653,  2977316, 12162542,  5293661), 
        u32x8::new( 7981168, 12223605,  6239200, 20403609, 20710415,  4828170, 11627702,  4431044), 
        u32x8::new(65817142,    96824, 25021652, 16364722, 50410869, 24651857,  6979034, 33176209), 
        u32x8::new(33008344,  8687253, 27859668, 28796356, 30192014, 11975680, 11991047, 27710707),
    ])), 
    ExtendedPoint(FieldElement32x4([
        u32x8::new(14676653, 50945941, 13489249, 31456262, 47726639, 21761847,  3324839,  7843947), 
        u32x8::new(53352326,  8688989, 12944061, 12994004, 50113821, 37990636,  1537898, 20483689), 
        u32x8::new(46786852, 15572264, 24004728,  7566233, 32596174, 34437796, 23201722,  3431551), 
        u32x8::new(49025674, 52497128, 13273618, 10266201, 66795206,  2887684, 30966565, 33449990), 
        u32x8::new(53210238, 65839385, 15458877, 18409918, 24777464, 25586795, 15335748, 12323382),
    ])),
    ExtendedPoint(FieldElement32x4([
        u32x8::new(57816016, 23106045, 24948505, 27413507, 32551424, 26145165, 22632568, 27527446), 
        u32x8::new(53022711, 40974949, 14110533, 30646997, 51399118, 53289754, 32528560, 15822835), 
        u32x8::new(23810949, 51779690, 17532625, 21326637, 60314333, 43761996,  4852905,  3474945), 
        u32x8::new(13323962, 10752742, 16431634, 26425049, 24258356, 53260846, 19756601, 19546842), 
        u32x8::new(17403634, 52199608, 32323720,  5313255, 48522162, 33376516, 31903659, 15291466),
    ])),
];
