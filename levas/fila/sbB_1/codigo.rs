// --- sbB_1: the board's confidence in the manager, the sentence of the board report (manager_manager.cpp) ---

/// `build_board_confidence_text` before 30 days in charge: the template by the wealth band (−2..2).
const CONFIDENCE_FIRST_DAYS: [u32; 5] = [0xa0160c, 0xa016c8, 0xa0179c, 0xa01818, 0xa018bc];

/// `build_board_confidence_text` from 30 to 59 days in charge: [wealth band −2..2][board band 0..6][run kind 1..5].
const CONFIDENCE_EARLY_DAYS: [[[u32; 5]; 7]; 5] = [
    [ // wealth -2, deep in debt
        [0xa0036c, 0xa0036c, 0xa0036c, 0xa0036c, 0xa0036c], // board band 0
        [0xa00290, 0xa00290, 0xa00290, 0xa00290, 0xa00290], // board band 1
        [0x9fff00, 0x9fff00, 0x9fff00, 0x9fff00, 0x9fff00], // board band 2
        [0xa001dc, 0xa001dc, 0xa001dc, 0xa001dc, 0xa001dc], // board band 3
        [0xa00130, 0x9ffe3c, 0x9fff00, 0x9fff00, 0x9ffe3c], // board band 4
        [0x9fffb4, 0xa00064, 0x9fff00, 0x9fff00, 0xa00064], // board band 5
        [0x9ffd8c, 0x9ffe3c, 0x9fff00, 0x9fff00, 0x9ffe3c], // board band 6
    ],
    [ // wealth -1, in debt
        [0xa00910, 0xa00910, 0xa00910, 0xa00910, 0xa00910], // board band 0
        [0xa0081c, 0xa0081c, 0xa0081c, 0xa0081c, 0xa0081c], // board band 1
        [0xa00684, 0xa00684, 0xa00684, 0xa00684, 0xa00684], // board band 2
        [0xa00750, 0xa00750, 0xa00750, 0xa00750, 0xa00750], // board band 3
        [0xa005c0, 0xa005c0, 0xa00684, 0xa00684, 0xa00684], // board band 4
        [0xa004f8, 0xa004f8, 0xa00684, 0xa00684, 0xa00684], // board band 5
        [0xa00438, 0xa00438, 0xa00684, 0xa00684, 0xa00684], // board band 6
    ],
    [ // wealth 0, steady
        [0xa00cb0, 0xa00cb0, 0xa00cb0, 0xa00cb0, 0xa00cb0], // board band 0
        [0xa00c18, 0xa00c18, 0xa00c18, 0xa00c18, 0xa00c18], // board band 1
        [0xa00b34, 0xa00b34, 0xa00b34, 0xa00b34, 0xa00b34], // board band 2
        [0xa00ba4, 0xa00ba4, 0xa00ba4, 0xa00ba4, 0xa00ba4], // board band 3
        [0xa00acc, 0xa00acc, 0xa00b34, 0xa00b34, 0xa00b34], // board band 4
        [0xa00a60, 0xa00a60, 0xa00b34, 0xa00b34, 0xa00b34], // board band 5
        [0xa009f4, 0xa009f4, 0xa00b34, 0xa00b34, 0xa00b34], // board band 6
    ],
    [ // wealth 1, comfortable
        [0xa01194, 0xa010e4, 0xa010e4, 0xa010e4, 0xa010e4], // board band 0
        [0xa01194, 0xa01024, 0xa01024, 0xa01024, 0xa01024], // board band 1
        [0xa00e60, 0xa00e60, 0xa00e60, 0xa00e60, 0xa00e60], // board band 2
        [0xa00f88, 0xa00f88, 0xa00f88, 0xa00f88, 0xa00f88], // board band 3
        [0xa00ef8, 0xa00ef8, 0xa00e60, 0xa00e60, 0xa00e60], // board band 4
        [0xa00dcc, 0xa00dcc, 0xa00e60, 0xa00e60, 0xa00e60], // board band 5
        [0xa00d38, 0xa00d38, 0xa00e60, 0xa00e60, 0xa00e60], // board band 6
    ],
    [ // wealth 2, rich
        [0xa01194, 0xa01558, 0xa01558, 0xa01558, 0xa01558], // board band 0
        [0xa01194, 0xa01494, 0xa01494, 0xa01494, 0xa01494], // board band 1
        [0xa01194, 0xa01194, 0xa01194, 0xa01194, 0xa01194], // board band 2
        [0xa013f4, 0xa013f4, 0xa013f4, 0xa013f4, 0xa013f4], // board band 3
        [0xa01360, 0xa01360, 0xa01194, 0xa01194, 0xa01194], // board band 4
        [0xa012c8, 0xa012c8, 0xa01194, 0xa01194, 0xa01194], // board band 5
        [0xa01230, 0xa01230, 0xa01194, 0xa01194, 0xa01194], // board band 6
    ],
];

/// `build_board_confidence_text` from 60 days in charge: [wealth band −2..2][league band 0, 1, 2, 3..6][board band
/// 0..6][run kind 1..5].
const CONFIDENCE_LATER: [[[[u32; 5]; 7]; 4]; 5] = [
    [ // wealth -2, deep in debt
        [ // league band 0
            [0x9f47e4, 0x9f47e4, 0x9f47e4, 0x9f47e4, 0x9f47e4], // board band 0
            [0x9f45e4, 0x9f4704, 0x9f4704, 0x9f4704, 0x9f4704], // board band 1
            [0x9f43d8, 0x9f44f4, 0x9f44f4, 0x9f44f4, 0x9f44f4], // board band 2
            [0x9f4190, 0x9f42c0, 0x9f42c0, 0x9f42c0, 0x9f42c0], // board band 3
            [0x9f3f88, 0x9f4098, 0x9f4098, 0x9f4098, 0x9f4098], // board band 4
            [0x9f3d6c, 0x9f3e90, 0x9f3e90, 0x9f3e90, 0x9f3e90], // board band 5
            [0x9f3b58, 0x9f3c78, 0x9f3c78, 0x9f3c78, 0x9f3c78], // board band 6
        ],
        [ // league band 1
            [0x9f47e4, 0x9f47e4, 0x9f47e4, 0x9f47e4, 0x9f47e4], // board band 0
            [0x9f3968, 0x9f3a7c, 0x9f3a7c, 0x9f3a7c, 0x9f3a7c], // board band 1
            [0x9f376c, 0x9f387c, 0x9f387c, 0x9f387c, 0x9f387c], // board band 2
            [0x9f3548, 0x9f3668, 0x9f3668, 0x9f3668, 0x9f3668], // board band 3
            [0x9f3344, 0x9f3454, 0x9f3454, 0x9f3454, 0x9f3454], // board band 4
            [0x9f3230, 0x9f3e90, 0x9f3e90, 0x9f3e90, 0x9f3e90], // board band 5
            [0x9f3048, 0x9f3148, 0x9f3148, 0x9f3148, 0x9f3148], // board band 6
        ],
        [ // league band 2
            [0x9f47e4, 0x9f47e4, 0x9f47e4, 0x9f47e4, 0x9f47e4], // board band 0
            [0x9f2694, 0x9f2694, 0x9f2694, 0x9f2694, 0x9f2694], // board band 1
            [0x9f2e4c, 0x9f2f58, 0x9f2f58, 0x9f2f58, 0x9f2f58], // board band 2
            [0x9f2c2c, 0x9f2d48, 0x9f2d48, 0x9f2d48, 0x9f2d48], // board band 3
            [0x9f2b78, 0x9f2b78, 0x9f2a84, 0x9f2b78, 0x9f2b78], // board band 4
            [0x9f29d4, 0x9f29d4, 0x9f28e8, 0x9f29d4, 0x9f29d4], // board band 5
            [0x9f2834, 0x9f2834, 0x9f2744, 0x9f2834, 0x9f2834], // board band 6
        ],
        [ // league bands 3..6
            [0x9f47e4, 0x9f47e4, 0x9f47e4, 0x9f47e4, 0x9f47e4], // board band 0
            [0x9f2694, 0x9f2694, 0x9f2694, 0x9f2694, 0x9f2694], // board band 1
            [0x9f25dc, 0x9f25dc, 0x9f25dc, 0x9f25dc, 0x9f25dc], // board band 2
            [0x9f252c, 0x9f252c, 0x9f252c, 0x9f252c, 0x9f252c], // board band 3
            [0x9f29d4, 0x9f29d4, 0x9f2444, 0x9f29d4, 0x9f29d4], // board band 4
            [0x9f2390, 0x9f2390, 0x9f22a4, 0x9f2390, 0x9f2390], // board band 5
            [0x9f21f4, 0x9f21f4, 0x9f210c, 0x9f21f4, 0x9f21f4], // board band 6
        ],
    ],
    [ // wealth -1, in debt
        [ // league band 0
            [0x9f6988, 0x9f6988, 0x9f6988, 0x9f6988, 0x9f6988], // board band 0
            [0x9f67cc, 0x9f68bc, 0x9f68bc, 0x9f68bc, 0x9f68bc], // board band 1
            [0x9f65fc, 0x9f66f0, 0x9f66f0, 0x9f66f0, 0x9f66f0], // board band 2
            [0x9f6400, 0x9f6510, 0x9f6510, 0x9f6510, 0x9f6510], // board band 3
            [0x9f6224, 0x9f6324, 0x9f6324, 0x9f6324, 0x9f6324], // board band 4
            [0x9f6048, 0x9f6148, 0x9f6148, 0x9f6148, 0x9f6148], // board band 5
            [0x9f5e8c, 0x9f5f70, 0x9f5f70, 0x9f5f70, 0x9f5f70], // board band 6
        ],
        [ // league band 1
            [0x9f6988, 0x9f6988, 0x9f6988, 0x9f6988, 0x9f6988], // board band 0
            [0x9f5ce8, 0x9f5dc4, 0x9f5dc4, 0x9f5dc4, 0x9f5dc4], // board band 1
            [0x9f5b30, 0x9f5c20, 0x9f5c20, 0x9f5c20, 0x9f5c20], // board band 2
            [0x9f5978, 0x9f5a70, 0x9f5a70, 0x9f5a70, 0x9f5978], // board band 3
            [0x9f57cc, 0x9f58bc, 0x9f58bc, 0x9f58bc, 0x9f57cc], // board band 4
            [0x9f55f8, 0x9f56f0, 0x9f56f0, 0x9f56f0, 0x9f56f0], // board band 5
            [0x9f54fc, 0x9f56f0, 0x9f56f0, 0x9f56f0, 0x9f56f0], // board band 6
        ],
        [ // league band 2
            [0x9f6988, 0x9f6988, 0x9f6988, 0x9f6988, 0x9f6988], // board band 0
            [0x9f5460, 0x9f5460, 0x9f5460, 0x9f5460, 0x9f5460], // board band 1
            [0x9f5288, 0x9f5384, 0x9f5384, 0x9f5384, 0x9f5384], // board band 2
            [0x9f509c, 0x9f519c, 0x9f519c, 0x9f519c, 0x9f519c], // board band 3
            [0x9f4ff4, 0x9f4ff4, 0x9f4ff4, 0x9f4ff4, 0x9f4ff4], // board band 4
            [0x9f4f50, 0x9f4f50, 0x9f4f50, 0x9f4f50, 0x9f4f50], // board band 5
            [0x9f4eac, 0x9f4eac, 0x9f4eac, 0x9f4eac, 0x9f4eac], // board band 6
        ],
        [ // league bands 3..6
            [0x9f6988, 0x9f6988, 0x9f6988, 0x9f6988, 0x9f6988], // board band 0
            [0x9f5460, 0x9f5460, 0x9f5460, 0x9f5460, 0x9f5460], // board band 1
            [0x9f4e08, 0x9f4e08, 0x9f4e08, 0x9f4e08, 0x9f4e08], // board band 2
            [0x9f4d64, 0x9f4d64, 0x9f4c84, 0x9f4d64, 0x9f4d64], // board band 3
            [0x9f4f50, 0x9f4f50, 0x9f4ba8, 0x9f4f50, 0x9f4f50], // board band 4
            [0x9f4b00, 0x9f4b00, 0x9f4a1c, 0x9f4b00, 0x9f4b00], // board band 5
            [0x9f4978, 0x9f4978, 0x9f4898, 0x9f4978, 0x9f4978], // board band 6
        ],
    ],
    [ // wealth 0, steady
        [ // league band 0
            [0x9f9788, 0x9f9850, 0x9f9920, 0x9f9920, 0x9f9920], // board band 0
            [0x9f9508, 0x9f95fc, 0x9f96f4, 0x9f96f4, 0x9f96f4], // board band 1
            [0x9f9214, 0x9f933c, 0x9f9464, 0x9f9464, 0x9f9464], // board band 2
            [0x9f8ee0, 0x9f901c, 0x9f9158, 0x9f9158, 0x9f9158], // board band 3
            [0x9f8bd4, 0x9f8d04, 0x9f8e34, 0x9f8e34, 0x9f8e34], // board band 4
            [0x9f88cc, 0x9f89f8, 0x9f8b24, 0x9f8b24, 0x9f8b24], // board band 5
            [0x9f8620, 0x9f871c, 0x9f8824, 0x9f8824, 0x9f8824], // board band 6
        ],
        [ // league band 1
            [0x9f9920, 0x9f9920, 0x9f9920, 0x9f9920, 0x9f9920], // board band 0
            [0x9f8568, 0x9f8568, 0x9f8490, 0x9f83a8, 0x9f8318], // board band 1
            [0x9f8258, 0x9f8258, 0x9f8170, 0x9f8078, 0x9f7fd8], // board band 2
            [0x9f7e10, 0x9f7e10, 0x9f7ecc, 0x9f7d58, 0x9f7d58], // board band 3
            [0x9f7b04, 0x9f7cb0, 0x9f7bb4, 0x9f7cb0, 0x9f7cb0], // board band 4
            [0x9f77f0, 0x9f78a4, 0x9f7958, 0x9f7a58, 0x9f7a58], // board band 5
            [0x9f75c4, 0x9f774c, 0x9f7660, 0x9f774c, 0x9f774c], // board band 6
        ],
        [ // league band 2
            [0x9f9920, 0x9f9920, 0x9f9920, 0x9f9920, 0x9f9920], // board band 0
            [0x9f7560, 0x9f7560, 0x9f7560, 0x9f7560, 0x9f7560], // board band 1
            [0x9f73fc, 0x9f74bc, 0x9f74bc, 0x9f74bc, 0x9f74bc], // board band 2
            [0x9f7278, 0x9f7344, 0x9f7344, 0x9f7344, 0x9f7344], // board band 3
            [0x9f7208, 0x9f7208, 0x9f715c, 0x9f7208, 0x9f7208], // board band 4
            [0x9f70f4, 0x9f70f4, 0x9f7050, 0x9f70f4, 0x9f70f4], // board band 5
            [0x9f6fe4, 0x9f6fe4, 0x9f6f3c, 0x9f6fe4, 0x9f6fe4], // board band 6
        ],
        [ // league bands 3..6
            [0x9f6e8c, 0x9f9920, 0x9f9920, 0x9f9920, 0x9f9920], // board band 0
            [0x9f6de0, 0x9f7560, 0x9f7560, 0x9f7560, 0x9f7560], // board band 1
            [0x9f6cbc, 0x9f6d70, 0x9f6d70, 0x9f6d70, 0x9f6d70], // board band 2
            [0x9f6c50, 0x9f6c50, 0x9f6c50, 0x9f6c50, 0x9f6c50], // board band 3
            [0x9f70f4, 0x9f70f4, 0x9f7050, 0x9f70f4, 0x9f70f4], // board band 4
            [0x9f6be4, 0x9f6be4, 0x9f6b3c, 0x9f6be4, 0x9f6be4], // board band 5
            [0x9f6ad0, 0x9f6ad0, 0x9f6a28, 0x9f6ad0, 0x9f6ad0], // board band 6
        ],
    ],
    [ // wealth 1, comfortable
        [ // league band 0
            [0x9fca0c, 0x9fcb00, 0x9fcbf8, 0x9fcbf8, 0x9fcbf8], // board band 0
            [0x9fc70c, 0x9fc828, 0x9fc94c, 0x9fc94c, 0x9fc94c], // board band 1
            [0x9fc394, 0x9fc4e8, 0x9fc63c, 0x9fc63c, 0x9fc63c], // board band 2
            [0x9fbfc8, 0x9fc138, 0x9fc2a8, 0x9fc2a8, 0x9fc2a8], // board band 3
            [0x9fbc2c, 0x9fbd8c, 0x9fbeec, 0x9fbeec, 0x9fbeec], // board band 4
            [0x9fb890, 0x9fb9ec, 0x9fbb48, 0x9fbb48, 0x9fbb48], // board band 5
            [0x9fb550, 0x9fb680, 0x9fb7b8, 0x9fb7b8, 0x9fb7b8], // board band 6
        ],
        [ // league band 1
            [0x9fcbf8, 0x9fcbf8, 0x9fcbf8, 0x9fcbf8, 0x9fcbf8], // board band 0
            [0x9fb470, 0x9fb470, 0x9fb36c, 0x9fb258, 0x9fb19c], // board band 1
            [0x9fb0b4, 0x9fb0b4, 0x9fafa0, 0x9fae7c, 0x9fadb0], // board band 2
            [0x9fb0b4, 0x9fb0b4, 0x9fac74, 0x9fab8c, 0x9fab8c], // board band 3
            [0x9fa8a4, 0x9faab4, 0x9fa988, 0x9faab4, 0x9faab4], // board band 4
            [0x9fa4c8, 0x9fa5ac, 0x9fa690, 0x9fa7c4, 0x9fa7c4], // board band 5
            [0x9fa208, 0x9fa3f4, 0x9fa2d8, 0x9fa3f4, 0x9fa3f4], // board band 6
        ],
        [ // league band 2
            [0x9fcbf8, 0x9fcbf8, 0x9fcbf8, 0x9fcbf8, 0x9fcbf8], // board band 0
            [0x9fa178, 0x9fa178, 0x9fa178, 0x9fa178, 0x9fa178], // board band 1
            [0x9f9fbc, 0x9fa0a8, 0x9fa0a8, 0x9fa0a8, 0x9fa0a8], // board band 2
            [0x9f9dd8, 0x9f9ed4, 0x9f9ed4, 0x9f9ed4, 0x9f9ed4], // board band 3
            [0x9f9d38, 0x9f9d38, 0x9f9d38, 0x9f9d38, 0x9f9d38], // board band 4
            [0x9f9c9c, 0x9f9c9c, 0x9f9c9c, 0x9f9c9c, 0x9f9c9c], // board band 5
            [0x9f9c00, 0x9f9c00, 0x9f9c00, 0x9f9c00, 0x9f9c00], // board band 6
        ],
        [ // league bands 3..6
            [0x9fcbf8, 0x9fcbf8, 0x9fcbf8, 0x9fcbf8, 0x9fcbf8], // board band 0
            [0x9fa178, 0x9fa178, 0x9fa178, 0x9fa178, 0x9fa178], // board band 1
            [0x9f9b68, 0x9f9b68, 0x9f9b68, 0x9f9b68, 0x9f9b68], // board band 2
            [0x9f9ac8, 0x9f9ac8, 0x9f9ac8, 0x9f9ac8, 0x9f9ac8], // board band 3
            [0x9f9c9c, 0x9f9c9c, 0x9f9c9c, 0x9f9c9c, 0x9f9c9c], // board band 4
            [0x9f9a28, 0x9f9a28, 0x9f9a28, 0x9f9a28, 0x9f9a28], // board band 5
            [0x9f998c, 0x9f998c, 0x9f998c, 0x9f998c, 0x9f998c], // board band 6
        ],
    ],
    [ // wealth 2, rich
        [ // league band 0
            [0x9ffae8, 0x9ffbec, 0x9ffcf4, 0x9ffcf4, 0x9ffcf4], // board band 0
            [0x9ff7c8, 0x9ff8f4, 0x9ffa28, 0x9ffa28, 0x9ffa28], // board band 1
            [0x9ff450, 0x9ff5a4, 0x9ff6f8, 0x9ff6f8, 0x9ff6f8], // board band 2
            [0x9ff084, 0x9ff1f4, 0x9ff364, 0x9ff364, 0x9ff364], // board band 3
            [0x9fece4, 0x9fee44, 0x9fefa4, 0x9fefa4, 0x9fefa4], // board band 4
            [0x9fe948, 0x9feaa4, 0x9fec00, 0x9fec00, 0x9fec00], // board band 5
            [0x9fe5f8, 0x9fe72c, 0x9fe86c, 0x9fe86c, 0x9fe86c], // board band 6
        ],
        [ // league band 1
            [0x9ffcf4, 0x9ffcf4, 0x9ffcf4, 0x9ffcf4, 0x9ffcf4], // board band 0
            [0x9fe514, 0x9fe514, 0x9fe410, 0x9fe2fc, 0x9fe240], // board band 1
            [0x9fe154, 0x9fe154, 0x9fe040, 0x9fdf1c, 0x9fde50], // board band 2
            [0x9fe154, 0x9fe154, 0x9fdd14, 0x9fdc2c, 0x9fdc2c], // board band 3
            [0x9fd93c, 0x9fdb50, 0x9fda20, 0x9fdb50, 0x9fdb50], // board band 4
            [0x9fd768, 0x9fd768, 0x9fd808, 0x9fd688, 0x9fd688], // board band 5
            [0x9fd5ec, 0x9fd5ec, 0x9fd4cc, 0x9fd3f8, 0x9fd5ec], // board band 6
        ],
        [ // league band 2
            [0x9ffcf4, 0x9ffcf4, 0x9ffcf4, 0x9ffcf4, 0x9ffcf4], // board band 0
            [0x9fd368, 0x9fd368, 0x9fd368, 0x9fd368, 0x9fd368], // board band 1
            [0x9fd1ac, 0x9fd298, 0x9fd298, 0x9fd298, 0x9fd298], // board band 2
            [0x9fcfc4, 0x9fd0c0, 0x9fd0c0, 0x9fd0c0, 0x9fd0c0], // board band 3
            [0x9fcf24, 0x9fcf24, 0x9fcf24, 0x9fcf24, 0x9fcf24], // board band 4
            [0x9fce88, 0x9fce88, 0x9fce88, 0x9fce88, 0x9fce88], // board band 5
            [0x9fcde8, 0x9fcde8, 0x9fcde8, 0x9fcde8, 0x9fcde8], // board band 6
        ],
        [ // league bands 3..6
            [0x9ffcf4, 0x9ffcf4, 0x9ffcf4, 0x9ffcf4, 0x9ffcf4], // board band 0
            [0x9fd368, 0x9fd368, 0x9fd368, 0x9fd368, 0x9fd368], // board band 1
            [0x9fcd4c, 0x9fcd4c, 0x9fcd4c, 0x9fcd4c, 0x9fcd4c], // board band 2
            [0x9fccac, 0x9fccac, 0x9fccac, 0x9fccac, 0x9fccac], // board band 3
            [0x9fce88, 0x9fce88, 0x9fce88, 0x9fce88, 0x9fce88], // board band 4
            [0x9fd768, 0x9fd768, 0x9fd768, 0x9fd768, 0x9fd768], // board band 5
            [0x9fd5ec, 0x9fd5ec, 0x9fd5ec, 0x9fd5ec, 0x9fd5ec], // board band 6
        ],
    ],
];

/// The FA's verdict on a national team manager by the board confidence (above 9500, 8500, 6500, 4275, 3500, else).
const NATIONAL_CONFIDENCE_TEXTS: [(i16, u32); 6] =
    [(9500, 0x9f205c), (8500, 0x9f1fe8), (6500, 0x9f1f7c), (4275, 0x9f1f0c), (3500, 0x9f1e9c), (i16::MIN, 0x9f1e24)];

/// Writes the board's (or the FA's) opinion of the club's manager into `buf`: the sentence of the board confidence
/// news (`news_board_confidence_update_text`) and of the board confidence screen.
///
/// `MANAGER_MANAGER::build_board_confidence_text(club, days, buf, size)` (`days` the manager's days in charge; the size
/// is not used). Reads the league confidence band (`get_league_confidence_band`), the board confidence band
/// (`get_board_confidence_band`) and the club's current run (`get_current_run`: kind 1 won, 2 unbeaten, 3 lost, 4
/// without a win, 5 none, and its length). A club sentence is a language format with the club's gender twice and its
/// name, and the run's length for the templates that count it. By the days in charge:
/// - under 30: one sentence by the wealth band (`exe::finance_get_wealth_band` of the club's finance in
///   `G_FINANCE_TABLE`) straight into `buf` (`CONFIDENCE_FIRST_DAYS`); done;
/// - under 60: one sentence by the wealth band, the board band and the run kind straight into `buf`
///   (`CONFIDENCE_EARLY_DAYS`); done;
/// - from 60: the sentence by the wealth band, the league band (0, 1, 2, 3..6), the board band and the run kind
///   (`CONFIDENCE_LATER`), then the closing (`board_confidence_closing`).
/// A national team (`exe::database_is_national_team`) instead gets the FA's verdict by the board confidence
/// (`NATIONAL_CONFIDENCE_TEXTS`, with the nation's adjective, form 9, 100 bytes), then the closing.
/// The three tables were generated from the original by emulating it over every input; the "UNKNOWN ... STATE" texts
/// of the original are unreachable (the bands, the wealth band and the run kind never leave their ranges).
/// Original: `MANAGER_MANAGER::build_board_confidence_text` (0x690d80, manager_manager.cpp, thiscall, ret 0x10).
/// Defect of the original: a national team without a nation gets an adjective buffer never written (the copy writes
/// an empty one).
pub(crate) unsafe extern "thiscall" fn build_board_confidence_text(this: usize, club: usize, days: i32, buf: usize, _size: u32) {
    let club = Club(club);
    write_u8(buf, 0);
    let league = get_league_confidence_band(this, club.addr());
    let board = get_board_confidence_band(this, club.addr()) as usize;
    let (mut kind, mut run) = (0u8, 0u8);
    get_current_run(this, club.addr(), &mut kind, &mut run);
    let mut first = [0u8; 0x5dc];
    let fp = first.as_mut_ptr() as usize;
    if exe::database_is_national_team(club) != 0 {
        covered!("sbB_1 national team");
        national_confidence_sentence(this, club, fp);
    } else {
        let wealth = (exe::finance_get_wealth_band(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club)) as i8 + 2) as usize;
        let g = club.gender_name() as i32;
        // ponytail: the run's length goes to every template; only those with a %d read it
        let say = |dst: usize, tpl: u32| story_lang!(dst, tpl, g, g, club.addr() + club::NAME, run as i8 as i32);
        let k = kind as usize - 1;
        if days < 30 {
            covered!("sbB_1 first days");
            say(buf, CONFIDENCE_FIRST_DAYS[wealth]);
            return;
        }
        if days < 60 {
            covered!("sbB_1 early days");
            say(buf, CONFIDENCE_EARLY_DAYS[wealth][board][k]);
            return;
        }
        covered!("sbB_1 later");
        say(fp, CONFIDENCE_LATER[wealth][league.min(3) as usize][board][k]);
    }
    board_confidence_closing(this, club, fp, buf);
}

/// The FA's verdict on a national team manager into `first` (`NATIONAL_CONFIDENCE_TEXTS`, by the board confidence),
/// with the nation's adjective (`exe::nation_get_nationality_adjective`, form 9; only with a nation).
unsafe fn national_confidence_sentence(this: usize, club: Club, first: usize) {
    let mut adjective = [0u8; 100];
    let adj = adjective.as_mut_ptr() as usize;
    if club.nation().addr() != 0 {
        exe::nation_get_nationality_adjective(club.nation(), adj, 100, 9);
    }
    let confidence = BoardRecord(club_board_row(this, club)).confidence();
    let tpl = NATIONAL_CONFIDENCE_TEXTS.iter().find(|t| confidence > t.0).map_or(0x9f1e24, |t| t.1);
    story_lang!(first, tpl, 0x64, 0x64, adj);
}

/// The end of the board confidence text from 60 days in charge: `buf` = `first`, or `first` and a second paragraph.
///
/// With the manager's last ultimatum (`board_record::LAST_ULTIMATUM_DATE`) less than 120 days ago and the board
/// confidence under 6500, the sentence is replaced by the arrogance one: above 4275 "somewhat unsatisfied", above 3500
/// "(somewhat) disappointed", each with the fans' support clause when the favourites mood
/// (`board_record::FAVOURITES_MOOD`) is above 1500; at 3500 or less the sentence stays. Otherwise a second paragraph
/// by the favourites mood: under -750 the resignation calls (confidence under 3750: "if things don't improve"; under
/// 5500: "please ignore"; else none), under -500 the waning support, above 1500 the rapport with the fans.
unsafe fn board_confidence_closing(this: usize, club: Club, first: usize, buf: usize) {
    let row = BoardRecord(club_board_row(this, club));
    let mut second = [0u8; 0x3e8];
    let sp = second.as_mut_ptr() as usize;
    let date = row.last_ultimatum_date_addr();
    let confidence = row.confidence();
    if exe::date_days_since(G_CURRENT_DATE, read_u32(date), read_u32(date + 4)) < 120 && confidence < 6500 {
        let fans = row.favourites_mood() > 1500;
        let tpl = if confidence > 4275 {
            Some(if fans { 0x9f1cfc } else { 0x9f1c54 }) // "... somewhat unsatisfied ... recent arrogance towards the board."
        } else if confidence > 3500 {
            Some(if fans { 0x9f1b28 } else { 0x9f1a7c }) // "... (somewhat) disappointed ... recent arrogance ..."
        } else {
            None
        };
        if let Some(tpl) = tpl {
            covered!("sbB_1 arrogance");
            let g = club.gender_name() as i32;
            story_lang!(first, tpl, g, g, club.addr() + club::NAME);
        }
    } else {
        let mood = row.favourites_mood();
        let tpl = if mood < -750 {
            if confidence < 3750 {
                Some(0x9f1a00) // "If things don't improve then we may have to start listening to those supporters ..."
            } else if confidence < 5500 {
                Some(0x9f1988) // "Please ignore those supporters who have been calling for your resignation. ..."
            } else {
                None
            }
        } else if mood < -500 {
            Some(0x9f192c) // "We are sad to see that our fans' enthusiastic support for the team appears to be waning."
        } else if mood > 1500 {
            Some(0x9f18e8) // "We are pleased to see that you have built a rapport with the fans."
        } else {
            None
        };
        if let Some(tpl) = tpl {
            covered!("sbB_1 fans paragraph");
            story_lang!(sp, tpl);
        }
    }
    if second[0] != 0 {
        story_crt!(buf, 0x991948, first, sp); // "%s\n\n%s"
    } else {
        story_crt!(buf, 0x991944, first); // "%s"
    }
}

/// Comparer for `build_board_confidence_text`: side by side over the buffer (the size the caller gives, at most 0x1388).
pub(crate) unsafe extern "thiscall" fn compare_build_board_confidence_text(this: usize, club: usize, days: i32, buf: usize, size: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32)>(%%I0%%);
    let n = (size as usize).clamp(1, 0x1388);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(buf, n)], || o(this, club, days, buf, size), || build_board_confidence_text(this, club, days, buf, size));
    SCOREBOARD.record(equal, || format!("build_board_confidence_text(days {}) {:?} x {:?}", days,
        String::from_utf8_lossy(&mo.0[0].1[..0x100.min(n)]), String::from_utf8_lossy(&mr.0[0].1[..0x100.min(n)])));
}
