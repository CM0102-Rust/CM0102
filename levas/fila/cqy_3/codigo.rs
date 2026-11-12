// ---------- batch %%LEVA%%: the names of the rounds, groups and stages, and the competition, stage and round label (comp_util.cpp) ----------

/// The full names of the round, group and stage ids (form 0 of `comp_util_get_stage_name`): id and the langlib format
/// string the original prints, sorted by id (the text in the comment).
const STAGE_NAMES_FULL: [(i16, usize); 322] = [
    (1, 0x9a7c3c), // Replay
    (2, 0x9a7c30), // Replay 2
    (3, 0x9a7c24), // First Leg
    (4, 0x9a7c18), // Second Leg
    (5, 0x9a7c10), // Decider
    (10, 0x9a7c04), // First Round
    (11, 0x9a7bf0), // First Round Replay
    (12, 0x9a7bd4), // First Round Second Replay
    (13, 0x9a7bbc), // First Round First Leg
    (14, 0x9a7ba4), // First Round Second Leg
    (20, 0x9a7b94), // Second Round
    (21, 0x9a7b80), // Second Round Replay
    (22, 0x9a7b64), // Second Round Second Replay
    (23, 0x9a7b4c), // Second Round First Leg
    (24, 0x9a7b34), // Second Round Second Leg
    (25, 0x9a7b1c), // Second Round Decider
    (30, 0x9a7b10), // Third Round
    (31, 0x9a7afc), // Third Round Replay
    (32, 0x9a7ae0), // Third Round Second Replay
    (33, 0x9a7ac8), // Third Round First Leg
    (34, 0x9a7ab0), // Third Round Second Leg
    (40, 0x9a7aa0), // Fourth Round
    (41, 0x9a7a8c), // Fourth Round Replay
    (42, 0x9a7a70), // Fourth Round Second Replay
    (43, 0x9a7a58), // Fourth Round First Leg
    (44, 0x9a7a40), // Fourth Round Second Leg
    (50, 0x9a7a34), // Fifth Round
    (51, 0x9a7a20), // Fifth Round Replay
    (52, 0x9a7a04), // Fifth Round Second Replay
    (53, 0x9a79ec), // Fifth Round First Leg
    (54, 0x9a79d4), // Fifth Round Second Leg
    (60, 0x9a79c8), // Sixth Round
    (61, 0x9a79b4), // Sixth Round Replay
    (62, 0x9a7998), // Sixth Round Second Replay
    (63, 0x9a7980), // Sixth Round First Leg
    (64, 0x9a7968), // Sixth Round Second Leg
    (70, 0x9a7958), // Seventh Round
    (71, 0x9a7940), // Seventh Round Replay
    (72, 0x9a7924), // Seventh Round Second Replay
    (73, 0x9a790c), // Seventh Round First Leg
    (74, 0x9a78f0), // Seventh Round Second Leg
    (80, 0x9a78e0), // Eighth Round
    (81, 0x9a78cc), // Eighth Round Replay
    (82, 0x9a78b0), // Eighth Round Second Replay
    (83, 0x9a7898), // Eighth Round First Leg
    (84, 0x9a7880), // Eighth Round Second Leg
    (90, 0x9a7874), // Ninth Round
    (91, 0x9a7860), // Ninth Round Replay
    (92, 0x9a7844), // Ninth Round Second Replay
    (93, 0x9a782c), // Ninth Round First Leg
    (94, 0x9a7814), // Ninth Round Second Leg
    (100, 0x9a7808), // Tenth Round
    (101, 0x9a77f4), // Tenth Round Replay
    (102, 0x9a77d8), // Tenth Round Second Replay
    (103, 0x9a77c0), // Tenth Round First Leg
    (104, 0x9a77a8), // Tenth Round Second Leg
    (110, 0x9a7798), // Eleventh Round
    (111, 0x9a7780), // Eleventh Round Replay
    (112, 0x9a7760), // Eleventh Round Second Replay
    (113, 0x9a7744), // Eleventh Round First Leg
    (114, 0x9a7728), // Eleventh Round Second Leg
    (120, 0x9a7718), // Quarter Final
    (121, 0x9a7700), // Quarter Final Replay
    (122, 0x9a76e4), // Quarter Final Second Replay
    (123, 0x9a76cc), // Quarter Final First Leg
    (124, 0x9a76b0), // Quarter Final Second Leg
    (125, 0x9a7698), // Quarter Final Decider
    (130, 0x9a768c), // Semi Final
    (131, 0x9a7678), // Semi Final Replay
    (132, 0x9a765c), // Semi Final Second Replay
    (133, 0x9a7644), // Semi Final First Leg
    (134, 0x9a762c), // Semi Final Second Leg
    (135, 0x9a7618), // Semi Final Decider
    (140, 0x9a7604), // Third Place Playoff
    (141, 0x9a75e8), // Third Place Playoff Replay
    (142, 0x9a75c4), // Third Place Playoff Second Replay
    (143, 0x9a75a4), // Third Place Playoff First Leg
    (144, 0x9a7584), // Third Place Playoff Second Leg
    (150, 0x9a757c), // Final
    (151, 0x9a756c), // Final Replay
    (152, 0x9a7558), // Final Second Replay
    (153, 0x9a7548), // Final First Leg
    (154, 0x9a7534), // Final Second Leg
    (155, 0x9a7524), // Final Decider
    (160, 0x9a751c), // Playoff
    (161, 0x9a750c), // Playoff Replay
    (162, 0x9a74f4), // Playoff Second Replay
    (163, 0x9a74e0), // Playoff First Leg
    (164, 0x9a74cc), // Playoff Second Leg
    (170, 0x9a74bc), // Playoff Final
    (171, 0x9a74a4), // Playoff Final Replay
    (172, 0x9a7488), // Playoff Final Second Replay
    (173, 0x9a7470), // Playoff Final First Leg
    (174, 0x9a7454), // Playoff Final Second Leg
    (180, 0x9a7440), // Playoff Semi Final
    (181, 0x9a7424), // Playoff Semi Final Replay
    (182, 0x9a7400), // Playoff Semi Final Second Replay
    (183, 0x9a73e0), // Playoff Semi Final First Leg
    (184, 0x9a73c0), // Playoff Semi Final Second Leg
    (190, 0x9a73b8), // Playout
    (191, 0x9a73a8), // Playout Replay
    (192, 0x9a7390), // Playout Second Replay
    (193, 0x9a737c), // Playout First Leg
    (194, 0x9a7368), // Playout Second Leg
    (200, 0x9a7350), // First Preliminary Round
    (201, 0x9a7330), // First Preliminary Round Replay
    (202, 0x9a7308), // First Preliminary Round Second Replay
    (203, 0x9a72e4), // First Preliminary Round First Leg
    (204, 0x9a72c0), // First Preliminary Round Second Leg
    (210, 0x9a72a4), // Second Preliminary Round
    (211, 0x9a7284), // Second Preliminary Round Replay
    (212, 0x9a725c), // Second Preliminary Round Second Replay
    (213, 0x9a7238), // Second Preliminary Round First Leg
    (214, 0x9a7214), // Second Preliminary Round Second Leg
    (220, 0x9a7200), // Qualifying Round
    (221, 0x9a71e8), // Qualifying Round Replay
    (222, 0x9a71c8), // Qualifying Round Second Replay
    (223, 0x9a71ac), // Qualifying Round First Leg
    (224, 0x9a7190), // Qualifying Round Second Leg
    (230, 0x9a717c), // Preliminary Round
    (231, 0x9a7160), // Preliminary Round Replay
    (232, 0x9a7140), // Preliminary Round Second Replay
    (233, 0x9a7124), // Preliminary Round First Leg
    (234, 0x9a7104), // Preliminary Round Second Leg
    (240, 0x9a70f4), // Promotion Final
    (243, 0x9a70d8), // Promotion Final First Leg
    (244, 0x9a70bc), // Promotion Final Second Leg
    (250, 0x9a70a4), // First Qualifying Phase
    (253, 0x9a7080), // First Qualifying Phase First Leg
    (254, 0x9a705c), // First Qualifying Phase Second Leg
    (260, 0x9a7044), // Second Qualifying Phase
    (263, 0x9a7020), // Second Qualifying Phase First Leg
    (264, 0x9a6ffc), // Second Qualifying Phase Second Leg
    (270, 0x9a6fe4), // Third Qualifying Phase
    (273, 0x9a6fc0), // Third Qualifying Phase First Leg
    (274, 0x9a6f9c), // Third Qualifying Phase Second Leg
    (280, 0x9a6f90), // Playoff 1
    (283, 0x9a6f7c), // Playoff 1 First Leg
    (284, 0x9a6f64), // Playoff 1 Second Leg
    (290, 0x9a6f58), // Playoff 2
    (293, 0x9a6f44), // Playoff 2 First Leg
    (294, 0x9a6f2c), // Playoff 2 Second Leg
    (300, 0x9a6f1c), // North Playoff
    (310, 0x9a7604), // Third Place Playoff
    (311, 0x9a75e8), // Third Place Playoff Replay
    (312, 0x9a75c4), // Third Place Playoff Second Replay
    (313, 0x9a75a4), // Third Place Playoff First Leg
    (314, 0x9a7584), // Third Place Playoff Second Leg
    (320, 0x9a6f08), // Elimination Final
    (323, 0x9a6eec), // Elimination Final First Leg
    (324, 0x9a6ecc), // Elimination Final Second Leg
    (330, 0x9a6eb8), // Minor Semi Final
    (333, 0x9a6e9c), // Minor Semi Final First Leg
    (334, 0x9a6e80), // Minor Semi Final Second Leg
    (340, 0x9a6e6c), // Major Semi Final
    (343, 0x9a6e50), // Major Semi Final First Leg
    (344, 0x9a6e34), // Major Semi Final Second Leg
    (350, 0x9a6e20), // Preliminary Final
    (353, 0x9a6e04), // Preliminary Final First Leg
    (354, 0x9a6de4), // Preliminary Final Second Leg
    (360, 0x9a6dd8), // Grand Final
    (363, 0x9a6dc0), // Grand Final First Leg
    (364, 0x9a6da8), // Grand Final Second Leg
    (370, 0x9a6d94), // Interzone Playoff
    (373, 0x9a6d78), // Interzone Playoff First Leg
    (374, 0x9a6d58), // Interzone Playoff Second Leg
    (1000, 0x9a6d50), // Group 1
    (1001, 0x9a6d48), // Group 2
    (1002, 0x9a6d40), // Group 3
    (1003, 0x9a6d38), // Group 4
    (1004, 0x9a6d30), // Group 5
    (1005, 0x9a6d28), // Group 6
    (1006, 0x9a6d20), // Group 7
    (1007, 0x9a6d18), // Group 8
    (1008, 0x9a6d10), // Group 9
    (1009, 0x9a6d04), // Group 10
    (1010, 0x9a6cf8), // Group 11
    (1011, 0x9a6cec), // Group 12
    (1012, 0x9a6ce0), // Group 13
    (1013, 0x9a6cd4), // Group 14
    (1014, 0x9a6cc8), // Group 15
    (1015, 0x9a6cbc), // Group 16
    (1016, 0x9a6cb0), // Group 17
    (1017, 0x9a6ca4), // Group 18
    (1018, 0x9a6c98), // Group 19
    (1019, 0x9a6c8c), // Group 20
    (1020, 0x9a6c78), // Second Placed Teams
    (1021, 0x9a6c64), // First Round Group 1
    (1022, 0x9a6c50), // First Round Group 2
    (1023, 0x9a6c3c), // First Round Group 3
    (1024, 0x9a6c28), // First Round Group 4
    (1025, 0x9a6c14), // First Round Group 5
    (1026, 0x9a6c00), // First Round Group 6
    (1027, 0x9a6bec), // First Round Group 7
    (1028, 0x9a6bd8), // First Round Group 8
    (1029, 0x9a6bc4), // First Round Group 9
    (1030, 0x9a6bac), // First Round Group 10
    (1031, 0x9a6b94), // Second Round Group 1
    (1032, 0x9a6b7c), // Second Round Group 2
    (1033, 0x9a6b64), // Second Round Group 3
    (1034, 0x9a6b4c), // Second Round Group 4
    (1035, 0x9a6b34), // Second Round Group 5
    (1036, 0x9a6b20), // Semi Final Group 1
    (1037, 0x9a6b0c), // Semi Final Group 2
    (1038, 0x9a6af8), // Semi Final Group 3
    (1039, 0x9a6ae8), // Carribean Zone
    (1040, 0x9a6ad0), // Central American Zone
    (1041, 0x9a6ab8), // Premier/First(South)
    (1042, 0x9a6aa0), // Premier/First(North)
    (1043, 0x9a6a8c), // Eastern Conference
    (1044, 0x9a6a78), // Western Conference
    (1045, 0x9a6a5c), // Eastern Conference Playoff
    (1046, 0x9a6a40), // Western Conference Playoff
    (1047, 0x9a6a38), // MLS Cup
    (1048, 0x9a6a28), // Playoff Group 1
    (1049, 0x9a6a18), // Playoff Group 2
    (1050, 0x9a6a08), // Autumn Season
    (1051, 0x9a69f8), // Spring Season
    (1052, 0x99b800), // Group Stage
    (1053, 0x9a69e4), // Northern Section
    (1054, 0x9a69d0), // Southern Section
    (1055, 0x9a69c8), // Group A
    (1056, 0x9a69c0), // Group B
    (1057, 0x9a69b8), // Group C
    (1058, 0x9a69b0), // Group D
    (1059, 0x9a69a8), // Group E
    (1060, 0x9a69a0), // Group F
    (1061, 0x9a6998), // Group G
    (1062, 0x9a6990), // Group H
    (1063, 0x9a6988), // Group I
    (1064, 0x9a6980), // Group J
    (1065, 0x9a6978), // Group K
    (1066, 0x9a6970), // Group L
    (1067, 0x9a6968), // Group M
    (1068, 0x9a6960), // Group N
    (1069, 0x9a6958), // Group O
    (1070, 0x9a6950), // Group P
    (1071, 0x9a6938), // Oceania/Asia Playoff
    (1072, 0x9a6924), // First(South)/Second
    (1073, 0x9a6910), // First(North)/Second
    (1074, 0x9a6904), // First Stage
    (1075, 0x9a68f4), // Second Stage
    (1076, 0x9a68e4), // Opening Stage
    (1077, 0x9a68d4), // Closing Stage
    (1078, 0x9a68c4), // Interior Zone
    (1079, 0x9a68b0), // Metropolitan Zone
    (1080, 0x9a68a4), // Promotion
    (1081, 0x9a6894), // First Period
    (1082, 0x9a6884), // Second Period
    (1083, 0x9a6874), // Third Period
    (1084, 0x9a6864), // Fourth Period
    (1085, 0x9a6854), // Phase 1 Group A
    (1086, 0x9a6844), // Phase 1 Group B
    (1087, 0x9a6834), // Phase 1 Group C
    (1088, 0x9a6824), // Phase 1 Group D
    (1089, 0x9a6814), // Phase 1 Group E
    (1090, 0x9a6804), // Phase 1 Group F
    (1091, 0x9a67f4), // Phase 1 Group G
    (1092, 0x9a67e4), // Phase 1 Group H
    (1093, 0x9a67d4), // Phase 2 Group A
    (1094, 0x9a67c4), // Phase 2 Group B
    (1095, 0x9a67b4), // Phase 2 Group C
    (1096, 0x9a67a4), // Phase 2 Group D
    (1097, 0x9a6790), // Phase 1 Group Stage
    (1098, 0x9a677c), // Phase 2 Group Stage
    (1099, 0x9a6768), // Championship Group
    (1100, 0x9a6754), // Relegation Group
    (1101, 0x9a6740), // Promotion Group A
    (1102, 0x9a672c), // Promotion Group B
    (1103, 0x9a6718), // Promotion Group C
    (1104, 0x9a6704), // Promotion Group D
    (1105, 0x9a66ec), // Promotion Group Stage
    (1106, 0x9a66d8), // Relegation Playoff
    (1107, 0x9a66d0), // North
    (1108, 0x9a66c4), // Northeast
    (1109, 0x9a66b4), // West/Southwest
    (1110, 0x9a66ac), // South
    (1111, 0x9a6684), // Central<%s - COMMENT - Central section>
    (1112, 0x9a6670), // Promotion Playoff
    (1113, 0x9a6660), // First Promotion
    (1114, 0x9a664c), // Second Promotion
    (1115, 0x9a663c), // Third Promotion
    (1116, 0x9a6628), // Fourth Promotion
    (1117, 0x9a6610), // Group A First Period
    (1118, 0x9a65f8), // Group A Second Period
    (1119, 0x9a65e0), // Group A Third Period
    (1120, 0x9a65c8), // Group B First Period
    (1121, 0x9a65b0), // Group B Second Period
    (1122, 0x9a6598), // Group B Third Period
    (1123, 0x9a6d50), // Group 1
    (1124, 0x9a6d48), // Group 2
    (1125, 0x9a6d40), // Group 3
    (1126, 0x9a6d38), // Group 4
    (1127, 0x9a6d30), // Group 5
    (1128, 0x9a6d28), // Group 6
    (1129, 0x9a6d20), // Group 7
    (1130, 0x9a6d18), // Group 8
    (1131, 0x9a5358), // Championship
    (1132, 0x9a66d0), // North
    (1133, 0x9a6588), // East Svealand
    (1134, 0x9a6578), // West Svealand
    (1135, 0x9a6568), // East Gotaland
    (1136, 0x9a6558), // West Gotaland
    (1137, 0x9a6548), // South Gotaland
    (1138, 0x9a6530), // Second Round Group 6
    (1139, 0x9a6510), // Central American Zone Group 1
    (1140, 0x9a64f0), // Central American Zone Group 2
    (1141, 0x9a64d8), // Classification Group 1
    (1142, 0x9a64c0), // Classification Group 2
    (1143, 0x9a64a8), // Classification Group 3
    (1144, 0x9a6490), // Classification Group 4
    (1145, 0x9a6478), // Classification Group 5
    (1146, 0x9a6460), // Classification Groups
    (1147, 0x9a644c), // Central Conference
    (1148, 0x9a6430), // Northern Relegation Section
    (1149, 0x9a6414), // Southern Relegation Section
    (1150, 0x9a6400), // Third Round Group A
    (1151, 0x9a63ec), // Third Round Group B
    (1152, 0x9a4094), // East
    (1153, 0x9a408c), // West
    (1154, 0x9a63d8), // East Quarter Final
    (1155, 0x9a63c4), // West Quarter Final
];

/// The short names of the round, group and stage ids (form 1 of `comp_util_get_stage_name`): id and the langlib format
/// string the original prints, sorted by id (the text in the comment).
const STAGE_NAMES_SHORT: [(i16, usize); 322] = [
    (1, 0x9a5314), // Rep<%s - COMMENT - replay>
    (2, 0x9a5314), // Rep<%s - COMMENT - replay>
    (3, 0x9a530c), // Leg 1
    (4, 0x9a5304), // Leg 2
    (5, 0x9a7c10), // Decider
    (10, 0x9a52fc), // 1st Rnd
    (11, 0x9a52ec), // 1st Rnd Replay
    (12, 0x9a52dc), // 1st Rnd Rep 2
    (13, 0x9a52cc), // 1st Rnd Leg 1
    (14, 0x9a52bc), // 1st Rnd Leg 2
    (20, 0x9a52b4), // 2nd Rnd
    (21, 0x9a52a4), // 2nd Rnd Replay
    (22, 0x9a5290), // 2nd Rnd Replay 2
    (23, 0x9a5280), // 2nd Rnd Leg 1
    (24, 0x9a5270), // 2nd Rnd Leg 2
    (25, 0x9a5260), // 2nd Rnd Decider
    (30, 0x9a5258), // 3rd Rnd
    (31, 0x9a5248), // 3rd Rnd Replay
    (32, 0x9a5234), // 3rd Rnd Replay 2
    (33, 0x9a5224), // 3rd Rnd Leg 1
    (34, 0x9a5214), // 3rd Rnd Leg 2
    (40, 0x9a520c), // 4th Rnd
    (41, 0x9a51fc), // 4th Rnd Replay
    (42, 0x9a51e8), // 4th Rnd Replay 2
    (43, 0x9a51d8), // 4th Rnd Leg 1
    (44, 0x9a51c8), // 4th Rnd Leg 2
    (50, 0x9a51a8), // 5th Rnd
    (51, 0x9a5198), // 5th Rnd Replay
    (52, 0x9a5184), // 5th Rnd Replay 2
    (53, 0x9a5174), // 5th Rnd Leg 1
    (54, 0x9a5164), // 5th Rnd Leg 2
    (60, 0x9a515c), // 6th Rnd
    (61, 0x9a514c), // 6th Rnd Replay
    (62, 0x9a5138), // 6th Rnd Replay 2
    (63, 0x9a512c), // 6th Rnd L1
    (64, 0x9a5120), // 6th Rnd L2
    (70, 0x9a5114), // 7th Round
    (71, 0x9a5104), // 7th Rnd Replay
    (72, 0x9a50f0), // 7th Rnd Replay 2
    (73, 0x9a50e0), // 7th Rnd Leg 1
    (74, 0x9a50d0), // 7th Rnd Leg 2
    (80, 0x9a50c8), // 8th Rnd
    (81, 0x9a50b8), // 8th Rnd Replay
    (82, 0x9a50a4), // 8th Rnd Replay 2
    (83, 0x9a5094), // 8th Rnd Leg 1
    (84, 0x9a5084), // 8th Rnd Leg 2
    (90, 0x9a507c), // 9th Rnd
    (91, 0x9a506c), // 9th Rnd Replay
    (92, 0x9a5058), // 9th Rnd Replay 2
    (93, 0x9a5048), // 9th Rnd Leg 1
    (94, 0x9a5038), // 9th Rnd Leg 2
    (100, 0x9a502c), // 10th Rnd
    (101, 0x9a501c), // 10th Rnd Replay
    (102, 0x9a5008), // 10th Rnd Replay 2
    (103, 0x9a4ff8), // 10th Rnd Leg 1
    (104, 0x9a4fe8), // 10th Rnd Leg 2
    (110, 0x9a4fdc), // 11th Rnd
    (111, 0x9a4fcc), // 11th Rnd Replay
    (112, 0x9a4fb8), // 11th Rnd Replay 2
    (113, 0x9a4fa8), // 11th Rnd Leg 1
    (114, 0x9a4f98), // 11th Rnd Leg 2
    (120, 0x9a4f8c), // Qtr Final
    (121, 0x9a4f78), // Qtr Final Replay
    (122, 0x9a4f64), // Qtr Final Replay 2
    (123, 0x9a4f54), // Qtr Final Leg 1
    (124, 0x9a4f44), // Qtr Final Leg 2
    (125, 0x9a4f30), // Qtr Final Decider
    (130, 0x9a768c), // Semi Final
    (131, 0x9a7678), // Semi Final Replay
    (132, 0x9a4f1c), // Semi Final Replay 2
    (133, 0x9a4f08), // Semi Final Leg 1
    (134, 0x9a4ef4), // Semi Final Leg 2
    (135, 0x9a7618), // Semi Final Decider
    (140, 0x9a4ee0), // 3rd Place Playoff
    (141, 0x9a4ecc), // 3rd Place Replay
    (142, 0x9a4eb8), // 3rd Place Replay 2
    (143, 0x9a4ea4), // 3rd Place Pl. Leg 1
    (144, 0x9a4e90), // 3rd Place Pl. Leg 2
    (150, 0x9a757c), // Final
    (151, 0x9a756c), // Final Replay
    (152, 0x9a4e80), // Final Replay 1
    (153, 0x9a4e74), // Final Leg 1
    (154, 0x9a4e68), // Final Leg 2
    (155, 0x9a7524), // Final Decider
    (160, 0x9a751c), // Playoff
    (161, 0x9a750c), // Playoff Replay
    (162, 0x9a4e54), // Playoff Replay 2
    (163, 0x9a4e44), // Playoff Leg 1
    (164, 0x9a4e34), // Playoff Leg 2
    (170, 0x9a4e28), // Pl. Final
    (171, 0x9a4e14), // Pl. Final Replay
    (172, 0x9a4e00), // Pl. Final Replay 2
    (173, 0x9a4df0), // Pl. Final Leg 1
    (174, 0x9a4de0), // Pl. Final Leg 2
    (180, 0x9a4dd0), // Pl. Semi Final
    (181, 0x9a4dbc), // Pl. Semi Final Rep
    (182, 0x9a4da4), // Pl. Semi Final Rep 2
    (183, 0x9a4d8c), // Pl. Semi Final Leg 1
    (184, 0x9a4d74), // Pl. Semi Final Leg 2
    (190, 0x9a73b8), // Playout
    (191, 0x9a73a8), // Playout Replay
    (192, 0x9a4d60), // Playout Replay 2
    (193, 0x9a4d50), // Playout Leg 1
    (194, 0x9a4d40), // Playout Leg 2
    (200, 0x9a4d30), // 1st Pre. Rnd
    (201, 0x9a4d1c), // 1st Pre. Rnd Replay
    (202, 0x9a4d04), // 1st Pre. Rnd Replay 2
    (203, 0x9a4cf0), // 1st Pre. Rnd Leg 1
    (204, 0x9a4cdc), // 1st Pre. Rnd Leg 2
    (210, 0x9a4ccc), // 2nd Pre. Rnd
    (211, 0x9a4cb8), // 2nd Pre. Rnd Replay
    (212, 0x9a4ca0), // 2nd Pre. Rnd Replay 2
    (213, 0x9a4c8c), // 2nd Pre. Rnd Leg 1
    (214, 0x9a4c78), // 2nd Pre. Rnd Leg 2
    (220, 0x9a4c6c), // Qual.Rnd
    (221, 0x9a4c5c), // Qual.Rnd Replay
    (222, 0x9a4c48), // Qual.Rnd Replay 2
    (223, 0x9a4c38), // Qual. Rnd Leg 1
    (224, 0x9a4c28), // Qual. Rnd Leg 2
    (230, 0x9a4c1c), // Prelim Rnd
    (231, 0x9a4c08), // Prelim Rnd Replay
    (232, 0x9a4bf4), // Prelim Rnd Replay 2
    (233, 0x9a4be0), // Prelim Rnd Leg 1
    (234, 0x9a4bcc), // Prelim Rnd Leg 2
    (240, 0x9a4bc0), // Pro. Final
    (243, 0x9a4bac), // Pro. Final Leg 1
    (244, 0x9a4b98), // Pro. Final Leg 2
    (250, 0x9a4b90), // Qual.1
    (253, 0x9a4b80), // Qual.1 Leg 1
    (254, 0x9a4b70), // Qual.1 Leg 2
    (260, 0x9a4b68), // Qual.2
    (263, 0x9a4b58), // Qual.2 Leg 1
    (264, 0x9a4b48), // Qual.2 Leg 2
    (270, 0x9a4b40), // Qual.3
    (273, 0x9a4b30), // Qual.3 Leg 1
    (274, 0x9a4b20), // Qual.3 Leg 2
    (280, 0x9a4af4), // Pl. 1<%s - COMMENT - Playoff 1 short name>
    (283, 0x9a4ac0), // Pl. 1 Leg 1<%s - COMMENT - Playoff 1 short name>
    (284, 0x9a4a8c), // Pl. 1 Leg 2<%s - COMMENT - Playoff 1 short name>
    (290, 0x9a4a60), // Pl. 2<%s - COMMENT - Playoff 2 short name>
    (293, 0x9a4a2c), // Pl. 2 Leg 1<%s - COMMENT - Playoff 2 short name>
    (294, 0x9a49f8), // Pl. 2 Leg 2<%s - COMMENT - Playoff 2 short name>
    (300, 0x9a49d0), // North Pl.<%s - COMMENT - North Playoff>
    (310, 0x9a4ee0), // 3rd Place Playoff
    (311, 0x9a4ecc), // 3rd Place Replay
    (312, 0x9a4eb8), // 3rd Place Replay 2
    (313, 0x9a4ea4), // 3rd Place Pl. Leg 1
    (314, 0x9a4e90), // 3rd Place Pl. Leg 2
    (320, 0x9a49c4), // Elim. Final
    (323, 0x9a49b0), // Elim. Final Leg 1
    (324, 0x9a499c), // Elim. Final Leg 2
    (330, 0x9a6eb8), // Minor Semi Final
    (333, 0x9a4984), // Minor Semi Final Leg 1
    (334, 0x9a496c), // Minor Semi Final Leg 2
    (340, 0x9a6e6c), // Major Semi Final
    (343, 0x9a4954), // Major Semi Final Leg 1
    (344, 0x9a493c), // Major Semi Final Leg 2
    (350, 0x9a492c), // Prelim. Final
    (353, 0x9a4918), // Prelim. Final Leg 1
    (354, 0x9a4904), // Prelim. Final Leg 2
    (360, 0x9a6dd8), // Grand Final
    (363, 0x9a48f0), // Grand Final Leg 1
    (364, 0x9a48dc), // Grand Final Leg 2
    (370, 0x9a48cc), // Interzone Pl.
    (373, 0x9a48b8), // Interzone Pl. Leg 1
    (374, 0x9a48a4), // Interzone Pl. Leg 2
    (1000, 0x9a489c), // Grp 1
    (1001, 0x9a4894), // Grp 2
    (1002, 0x9a488c), // Grp 3
    (1003, 0x9a4884), // Grp 4
    (1004, 0x9a487c), // Grp 5
    (1005, 0x9a4874), // Grp 6
    (1006, 0x9a486c), // Grp 7
    (1007, 0x9a4864), // Grp 8
    (1008, 0x9a485c), // Grp 9
    (1009, 0x9a4854), // Grp 10
    (1010, 0x9a484c), // Grp 11
    (1011, 0x9a4844), // Grp 12
    (1012, 0x9a483c), // Grp 13
    (1013, 0x9a4834), // Grp 14
    (1014, 0x9a482c), // Grp 15
    (1015, 0x9a4824), // Grp 16
    (1016, 0x9a481c), // Grp 17
    (1017, 0x9a4814), // Grp 18
    (1018, 0x9a480c), // Grp 19
    (1019, 0x9a4804), // Grp 20
    (1020, 0x9a6c78), // Second Placed Teams
    (1021, 0x9a47f4), // 1st Rnd Grp 1
    (1022, 0x9a47e4), // 1st Rnd Grp 2
    (1023, 0x9a47d4), // 1st Rnd Grp 3
    (1024, 0x9a47c4), // 1st Rnd Grp 4
    (1025, 0x9a47b4), // 1st Rnd Grp 5
    (1026, 0x9a47a4), // 1st Rnd Grp 6
    (1027, 0x9a4794), // 1st Rnd Grp 7
    (1028, 0x9a4784), // 1st Rnd Grp 8
    (1029, 0x9a4774), // 1st Rnd Grp 9
    (1030, 0x9a4764), // 1st Rnd Grp 10
    (1031, 0x9a4754), // 2nd Rnd Grp 1
    (1032, 0x9a4744), // 2nd Rnd Grp 2
    (1033, 0x9a4734), // 2nd Rnd Grp 3
    (1034, 0x9a4724), // 2nd Rnd Grp 4
    (1035, 0x9a4714), // 2nd Rnd Grp 5
    (1036, 0x9a4700), // Semi Final Grp 1
    (1037, 0x9a46ec), // Semi Final Grp 2
    (1038, 0x9a46d8), // Semi Final Grp 3
    (1039, 0x9a46cc), // Carr. Zone
    (1040, 0x9a46bc), // Cent.Amer.Zone
    (1041, 0x9a751c), // Playoff
    (1042, 0x9a751c), // Playoff
    (1043, 0x9a46b0), // East Conf.
    (1044, 0x9a46a4), // West Conf.
    (1045, 0x9a4690), // East Conf. Playoff
    (1046, 0x9a467c), // West Conf. Playoff
    (1047, 0x9a6a38), // MLS Cup
    (1048, 0x9a466c), // Playoff Grp 1
    (1049, 0x9a465c), // Playoff Grp 2
    (1050, 0x9a4654), // Autumn
    (1051, 0x9a464c), // Spring
    (1052, 0x99b800), // Group Stage
    (1053, 0x9a66d0), // North
    (1054, 0x9a66ac), // South
    (1055, 0x9a4644), // Grp A
    (1056, 0x9a463c), // Grp B
    (1057, 0x9a4634), // Grp C
    (1058, 0x9a462c), // Grp D
    (1059, 0x9a4624), // Grp E
    (1060, 0x9a461c), // Grp F
    (1061, 0x9a4614), // Grp G
    (1062, 0x9a460c), // Grp H
    (1063, 0x9a4604), // Grp I
    (1064, 0x9a45fc), // Grp J
    (1065, 0x9a45f4), // Grp K
    (1066, 0x9a45ec), // Grp L
    (1067, 0x9a45e4), // Grp M
    (1068, 0x9a45dc), // Grp N
    (1069, 0x9a45d4), // Grp O
    (1070, 0x9a45cc), // Grp P
    (1071, 0x9a751c), // Playoff
    (1072, 0x9a6924), // First(South)/Second
    (1073, 0x9a6910), // First(North)/Second
    (1074, 0x9a45c0), // 1st Stage
    (1075, 0x9a45b4), // 2nd Stage
    (1076, 0x9a45ac), // Open
    (1077, 0x9a45a4), // Close
    (1078, 0x9a459c), // Int.
    (1079, 0x9a4594), // Metro.
    (1080, 0x9a458c), // Pro.
    (1081, 0x9a4580), // Period 1
    (1082, 0x9a4574), // Period 2
    (1083, 0x9a4568), // Period 3
    (1084, 0x9a455c), // Period 4
    (1085, 0x9a4550), // Ph.1 Grp A
    (1086, 0x9a4544), // Ph.1 Grp B
    (1087, 0x9a4538), // Ph.1 Grp C
    (1088, 0x9a452c), // Ph.1 Grp D
    (1089, 0x9a4520), // Ph.1 Grp E
    (1090, 0x9a4514), // Ph.1 Grp F
    (1091, 0x9a4508), // Ph.1 Grp G
    (1092, 0x9a44fc), // Ph.1 Grp H
    (1093, 0x9a44f0), // Ph.2 Grp A
    (1094, 0x9a44e4), // Ph.2 Grp B
    (1095, 0x9a44d8), // Ph.2 Grp C
    (1096, 0x9a44cc), // Ph.2 Grp D
    (1097, 0x9a44c4), // Phase 1
    (1098, 0x9a44bc), // Phase 2
    (1099, 0x9a44b0), // Champ. Grp
    (1100, 0x9a44a4), // Releg. Grp
    (1101, 0x9a4498), // Pro.Grp A
    (1102, 0x9a448c), // Pro.Grp B
    (1103, 0x9a4480), // Pro.Grp C
    (1104, 0x9a4474), // Pro.Grp D
    (1105, 0x9a4460), // Pro. Group Stage
    (1106, 0x9a4434), // Rel. Pl<%s - COMMENT - Relegation playoff>
    (1107, 0x9a421c), // N<%s - COMMENT - North>
    (1108, 0x9a4414), // NE<%s - COMMENT - Northeast>
    (1109, 0x9a43f0), // W/SW<%s - COMMENT - West/Southwest>
    (1110, 0x9a43d8), // S<%s - COMMENT - South>
    (1111, 0x9a43a4), // Central<%s - COMMENT - Central section(short name)>
    (1112, 0x9a4394), // Pro. Playoff
    (1113, 0x9a4360), // Pro, 1<%s - COMMENT - First Promotion short name>
    (1114, 0x9a432c), // Pro, 2<%s - COMMENT - First Promotion short name>
    (1115, 0x9a42f8), // Pro, 3<%s - COMMENT - First Promotion short name>
    (1116, 0x9a42c4), // Pro, 4<%s - COMMENT - First Promotion short name>
    (1117, 0x9a42b4), // Grp A Period 1
    (1118, 0x9a42a4), // Grp A Period 2
    (1119, 0x9a4294), // Grp A Period 3
    (1120, 0x9a4284), // Grp B Period 1
    (1121, 0x9a4274), // Grp B Period 2
    (1122, 0x9a4264), // Grp B Period 3
    (1123, 0x9a489c), // Grp 1
    (1124, 0x9a4894), // Grp 2
    (1125, 0x9a488c), // Grp 3
    (1126, 0x9a4884), // Grp 4
    (1127, 0x9a487c), // Grp 5
    (1128, 0x9a4874), // Grp 6
    (1129, 0x9a486c), // Grp 7
    (1130, 0x9a4864), // Grp 8
    (1131, 0x9a4234), // Champ.<%s - COMMENT - Championship short name>
    (1132, 0x9a421c), // N<%s - COMMENT - North>
    (1133, 0x9a41f8), // E.S<%s - COMMENT - East Svealand>
    (1134, 0x9a41d4), // W.S<%s - COMMENT - West Svealand>
    (1135, 0x9a41b0), // E.G<%s - COMMENT - East Gotaland>
    (1136, 0x9a418c), // W.G<%s - COMMENT - West Gotaland>
    (1137, 0x9a4168), // S.G<%s - COMMENT - South Gotaland>
    (1138, 0x9a4158), // 2nd Rnd Grp 6
    (1139, 0x9a4140), // Cent.Amer.Zone Grp 1
    (1140, 0x9a4128), // Cent.Amer.Zone Grp 2
    (1141, 0x9a411c), // Class.Grp 1
    (1142, 0x9a4110), // Class.Grp 2
    (1143, 0x9a4104), // Class.Grp 3
    (1144, 0x9a40f8), // Class.Grp 4
    (1145, 0x9a40ec), // Class.Grp 5
    (1146, 0x9a40e0), // Class.Grps
    (1147, 0x9a40d4), // Cent. Conf.
    (1148, 0x9a40c8), // North Rel.
    (1149, 0x9a40bc), // South Rel.
    (1150, 0x9a40ac), // 3rd Rnd Grp A
    (1151, 0x9a409c), // 3rd Rnd Grp B
    (1152, 0x9a4094), // East
    (1153, 0x9a408c), // West
    (1154, 0x9a407c), // E. Qtr Final
    (1155, 0x9a406c), // W. Qtr Final
];

/// The plural names of the round, group and stage ids (form 3 of `comp_util_get_stage_name`): id and the langlib format
/// string the original prints, sorted by id (the text in the comment).
const STAGE_NAMES_PLURAL: [(i16, usize); 321] = [
    (1, 0x9a7c3c), // Replay
    (2, 0x9a7c30), // Replay 2
    (3, 0x9a63b8), // First Legs
    (4, 0x9a63ac), // Second Legs
    (5, 0x9a7c10), // Decider
    (10, 0x9a6384), // First Round<%s - Plural, arg not used>
    (11, 0x9a6370), // First Round Replays
    (12, 0x9a6354), // First Round Second Replays
    (13, 0x9a633c), // First Round First Legs
    (14, 0x9a6324), // First Round Second Legs
    (20, 0x9a62fc), // Second Round<%s - Plural, arg not used>
    (21, 0x9a62e4), // Second Round Replays
    (22, 0x9a62c8), // Second Round Second Replays
    (23, 0x9a62b0), // Second Round First Legs
    (24, 0x9a6294), // Second Round Second Legs
    (25, 0x9a627c), // Second Round Deciders
    (30, 0x9a6254), // Third Round<%s - Plural, arg not used>
    (31, 0x9a6240), // Third Round Replays
    (32, 0x9a6224), // Third Round Second Replays
    (33, 0x9a620c), // Third Round First Legs
    (34, 0x9a61f4), // Third Round Second Legs
    (40, 0x9a61cc), // Fourth Round<%s - Plural, arg not used>
    (41, 0x9a61b4), // Fourth Round Replays
    (42, 0x9a6198), // Fourth Round Second Replays
    (43, 0x9a6180), // Fourth Round First Legs
    (44, 0x9a6164), // Fourth Round Second Legs
    (50, 0x9a613c), // Fifth Round<%s - Plural, arg not used>
    (51, 0x9a6128), // Fifth Round Replays
    (52, 0x9a610c), // Fifth Round Second Replays
    (53, 0x9a60f4), // Fifth Round First Legs
    (54, 0x9a60dc), // Fifth Round Second Legs
    (60, 0x9a60b4), // Sixth Round<%s - Plural, arg not used>
    (61, 0x9a60a0), // Sixth Round Replays
    (62, 0x9a6084), // Sixth Round Second Replays
    (63, 0x9a606c), // Sixth Round First Legs
    (64, 0x9a6054), // Sixth Round Second Legs
    (70, 0x9a6028), // Seventh Round<%s - Plural, arg not used>
    (71, 0x9a6010), // Seventh Round Replays
    (72, 0x9a5ff0), // Seventh Round Second Replays
    (73, 0x9a5fd4), // Seventh Round First Legs
    (74, 0x9a5fb8), // Seventh Round Second Legs
    (80, 0x9a5f90), // Eighth Round<%s - Plural, arg not used>
    (81, 0x9a5f78), // Eighth Round Replays
    (82, 0x9a5f5c), // Eighth Round Second Replays
    (83, 0x9a5f44), // Eighth Round First Legs
    (84, 0x9a5f28), // Eighth Round Second Legs
    (90, 0x9a5f00), // Ninth Round<%s - Plural, arg not used>
    (91, 0x9a5eec), // Ninth Round Replays
    (92, 0x9a5ed0), // Ninth Round Second Replays
    (93, 0x9a5eb8), // Ninth Round First Legs
    (94, 0x9a5ea0), // Ninth Round Second Legs
    (100, 0x9a5e78), // Tenth Round<%s - Plural, arg not used>
    (101, 0x9a5e64), // Tenth Round Replays
    (102, 0x9a5e48), // Tenth Round Second Replays
    (103, 0x9a5e30), // Tenth Round First Legs
    (104, 0x9a5e18), // Tenth Round Second Legs
    (110, 0x9a5dec), // Eleventh Round<%s - Plural, arg not used>
    (111, 0x9a5dd4), // Eleventh Round Replays
    (112, 0x9a5db4), // Eleventh Round Second Replays
    (113, 0x9a5d98), // Eleventh Round First Legs
    (114, 0x9a5d7c), // Eleventh Round Second Legs
    (120, 0x9a5d6c), // Quarter Finals
    (121, 0x9a5d54), // Quarter Final Replays
    (122, 0x9a5d34), // Quarter Final Second Replays
    (123, 0x9a5d18), // Quarter Final First Legs
    (124, 0x9a5cfc), // Quarter Final Second Legs
    (125, 0x9a5ce4), // Quarter Final Deciders
    (130, 0x9a5cd8), // Semi Finals
    (131, 0x9a5cc4), // Semi Final Replays
    (132, 0x9a5ca8), // Semi Final Second Replays
    (133, 0x9a5c90), // Semi Final First Legs
    (134, 0x9a5c78), // Semi Final Second Legs
    (135, 0x9a5c64), // Semi Final Deciders
    (140, 0x9a5c4c), // Third Place Playoffs
    (141, 0x9a5c30), // Third Place Playoff Replays
    (142, 0x9a5c0c), // Third Place Playoff Second Replays
    (143, 0x9a5bec), // Third Place Playoff First Legs
    (144, 0x9a5bcc), // Third Place Playoff Second Legs
    (150, 0x9a5bc4), // Finals
    (151, 0x9a5bb4), // Final Replays
    (152, 0x9a5b9c), // Final Second Replays
    (153, 0x9a5b88), // Final First Legs
    (154, 0x9a5b74), // Final Second Legs
    (155, 0x9a5b64), // Final Deciders
    (160, 0x9a5b58), // Playoffs
    (161, 0x9a5b48), // Playoff Replays
    (162, 0x9a5b30), // Playoff Second Replays
    (163, 0x9a5b1c), // Playoff First Legs
    (164, 0x9a5b08), // Playoff Second Legs
    (170, 0x9a5af8), // Playoff Finals
    (171, 0x9a5ae0), // Playoff Final Replays
    (172, 0x9a5ac0), // Playoff Final Second Replays
    (173, 0x9a5aa4), // Playoff Final First Legs
    (174, 0x9a5a88), // Playoff Final Second Legs
    (180, 0x9a5a74), // Playoff Semi Finals
    (181, 0x9a5a58), // Playoff Semi Final Replays
    (182, 0x9a5a34), // Playoff Semi Final Second Replays
    (183, 0x9a5a14), // Playoff Semi Final First Legs
    (184, 0x9a59f4), // Playoff Semi Final Second Legs
    (190, 0x9a59e8), // Playouts
    (191, 0x9a59d8), // Playout Replays
    (192, 0x9a59c0), // Playout Second Replays
    (193, 0x9a59ac), // Playout First Legs
    (194, 0x9a5998), // Playout Second Legs
    (200, 0x9a5964), // First Preliminary Round<%s - Plural, arg not used>
    (201, 0x9a5944), // First Preliminary Round Replays
    (202, 0x9a591c), // First Preliminary Round Second Replays
    (203, 0x9a58f8), // First Preliminary Round First Legs
    (204, 0x9a58d4), // First Preliminary Round Second Legs
    (210, 0x9a58a0), // Second Preliminary Round<%s - Plural, arg not used>
    (211, 0x9a587c), // Second Preliminary Round Replays
    (212, 0x9a5854), // Second Preliminary Round Second Replays
    (213, 0x9a5830), // Second Preliminary Round First Legs
    (214, 0x9a5808), // Second Preliminary Round Second Legs
    (220, 0x9a57dc), // Qualifying Round<%s - Plural, arg not used>
    (221, 0x9a57c0), // Qualifying Round Replays
    (222, 0x9a57a0), // Qualifying Round Second Replays
    (223, 0x9a5784), // Qualifying Round First Legs
    (224, 0x9a5764), // Qualifying Round Second Legs
    (230, 0x9a5734), // Preliminary Round<%s - Plural, arg not used>
    (231, 0x9a5718), // Preliminary Round Replays
    (232, 0x9a56f4), // Preliminary Round Second Replays
    (233, 0x9a56d4), // Preliminary Round First Legs
    (234, 0x9a56b4), // Preliminary Round Second Legs
    (240, 0x9a70f4), // Promotion Final
    (243, 0x9a70d8), // Promotion Final First Leg
    (244, 0x9a70bc), // Promotion Final Second Leg
    (250, 0x9a70a4), // First Qualifying Phase
    (253, 0x9a5690), // First Qualifying Phase First Legs
    (254, 0x9a566c), // First Qualifying Phase Second Legs
    (260, 0x9a7044), // Second Qualifying Phase
    (263, 0x9a5648), // Second Qualifying Phase First Legs
    (264, 0x9a5624), // Second Qualifying Phase Second Legs
    (270, 0x9a6fe4), // Third Qualifying Phase
    (273, 0x9a5600), // Third Qualifying Phase First Legs
    (274, 0x9a55dc), // Third Qualifying Phase Second Legs
    (280, 0x9a6f90), // Playoff 1
    (283, 0x9a55c4), // Playoff 1 First Legs
    (284, 0x9a55ac), // Playoff 1 Second Legs
    (290, 0x9a6f58), // Playoff 2
    (293, 0x9a5594), // Playoff 2 First Legs
    (294, 0x9a557c), // Playoff 2 Second Legs
    (300, 0x9a6f1c), // North Playoff
    (310, 0x9a5c4c), // Third Place Playoffs
    (311, 0x9a5c30), // Third Place Playoff Replays
    (312, 0x9a5c0c), // Third Place Playoff Second Replays
    (313, 0x9a5bec), // Third Place Playoff First Legs
    (314, 0x9a5bcc), // Third Place Playoff Second Legs
    (320, 0x9a5568), // Elimination Finals
    (323, 0x9a5548), // Elimination Final First Legs
    (324, 0x9a5528), // Elimination Final Second Legs
    (330, 0x9a5514), // Minor Semi Finals
    (333, 0x9a54f8), // Minor Semi Final First Legs
    (334, 0x9a54d8), // Minor Semi Final Second Legs
    (340, 0x9a54c4), // Major Semi Finals
    (343, 0x9a54a8), // Major Semi Final First Legs
    (344, 0x9a5488), // Major Semi Final Second Legs
    (350, 0x9a5474), // Preliminary Finals
    (353, 0x9a5454), // Preliminary Final First Legs
    (354, 0x9a5434), // Preliminary Final Second Legs
    (360, 0x9a5424), // Grand Finals
    (363, 0x9a540c), // Grand Final First Legs
    (364, 0x9a53f4), // Grand Final Second Legs
    (370, 0x9a53e0), // Interzone Playoffs
    (373, 0x9a53c0), // Interzone Playoffs First Leg
    (374, 0x9a53a0), // Interzone Playoffs Second Leg
    (1000, 0x9a6d50), // Group 1
    (1001, 0x9a6d48), // Group 2
    (1002, 0x9a6d40), // Group 3
    (1003, 0x9a6d38), // Group 4
    (1004, 0x9a6d30), // Group 5
    (1005, 0x9a6d28), // Group 6
    (1006, 0x9a6d20), // Group 7
    (1007, 0x9a6d18), // Group 8
    (1008, 0x9a6d10), // Group 9
    (1009, 0x9a6d04), // Group 10
    (1010, 0x9a6cf8), // Group 11
    (1011, 0x9a6cec), // Group 12
    (1012, 0x9a6ce0), // Group 13
    (1013, 0x9a6cd4), // Group 14
    (1014, 0x9a6cc8), // Group 15
    (1015, 0x9a6cbc), // Group 16
    (1016, 0x9a6cb0), // Group 17
    (1017, 0x9a6ca4), // Group 18
    (1018, 0x9a6c98), // Group 19
    (1019, 0x9a6c8c), // Group 20
    (1020, 0x9a6c78), // Second Placed Teams
    (1021, 0x9a6c64), // First Round Group 1
    (1022, 0x9a6c50), // First Round Group 2
    (1023, 0x9a6c3c), // First Round Group 3
    (1024, 0x9a6c28), // First Round Group 4
    (1025, 0x9a6c14), // First Round Group 5
    (1026, 0x9a6c00), // First Round Group 6
    (1027, 0x9a6bec), // First Round Group 7
    (1028, 0x9a6bd8), // First Round Group 8
    (1029, 0x9a6bc4), // First Round Group 9
    (1030, 0x9a6bac), // First Round Group 10
    (1031, 0x9a6b94), // Second Round Group 1
    (1032, 0x9a6b7c), // Second Round Group 2
    (1033, 0x9a6b64), // Second Round Group 3
    (1034, 0x9a6b4c), // Second Round Group 4
    (1035, 0x9a6b34), // Second Round Group 5
    (1036, 0x9a6b20), // Semi Final Group 1
    (1037, 0x9a6b0c), // Semi Final Group 2
    (1038, 0x9a6af8), // Semi Final Group 3
    (1039, 0x9a6ae8), // Carribean Zone
    (1040, 0x9a6ad0), // Central American Zone
    (1041, 0x9a6ab8), // Premier/First(South)
    (1042, 0x9a6aa0), // Premier/First(North)
    (1043, 0x9a6a8c), // Eastern Conference
    (1044, 0x9a6a78), // Western Conference
    (1045, 0x9a5384), // Eastern Conference Playoffs
    (1046, 0x9a5368), // Western Conference Playoffs
    (1047, 0x9a6a38), // MLS Cup
    (1048, 0x9a6a28), // Playoff Group 1
    (1049, 0x9a6a18), // Playoff Group 2
    (1050, 0x9a6a08), // Autumn Season
    (1051, 0x9a69f8), // Spring Season
    (1053, 0x9a69e4), // Northern Section
    (1054, 0x9a69d0), // Southern Section
    (1055, 0x9a69c8), // Group A
    (1056, 0x9a69c0), // Group B
    (1057, 0x9a69b8), // Group C
    (1058, 0x9a69b0), // Group D
    (1059, 0x9a69a8), // Group E
    (1060, 0x9a69a0), // Group F
    (1061, 0x9a6998), // Group G
    (1062, 0x9a6990), // Group H
    (1063, 0x9a6988), // Group I
    (1064, 0x9a6980), // Group J
    (1065, 0x9a6978), // Group K
    (1066, 0x9a6970), // Group L
    (1067, 0x9a6968), // Group M
    (1068, 0x9a6960), // Group N
    (1069, 0x9a6958), // Group O
    (1070, 0x9a6950), // Group P
    (1071, 0x9a6938), // Oceania/Asia Playoff
    (1072, 0x9a6924), // First(South)/Second
    (1073, 0x9a6910), // First(North)/Second
    (1074, 0x9a6904), // First Stage
    (1075, 0x9a68f4), // Second Stage
    (1076, 0x9a68e4), // Opening Stage
    (1077, 0x9a68d4), // Closing Stage
    (1078, 0x9a68c4), // Interior Zone
    (1079, 0x9a68b0), // Metropolitan Zone
    (1080, 0x9a68a4), // Promotion
    (1081, 0x9a6894), // First Period
    (1082, 0x9a6884), // Second Period
    (1083, 0x9a6874), // Third Period
    (1084, 0x9a6864), // Fourth Period
    (1085, 0x9a6854), // Phase 1 Group A
    (1086, 0x9a6844), // Phase 1 Group B
    (1087, 0x9a6834), // Phase 1 Group C
    (1088, 0x9a6824), // Phase 1 Group D
    (1089, 0x9a6814), // Phase 1 Group E
    (1090, 0x9a6804), // Phase 1 Group F
    (1091, 0x9a67f4), // Phase 1 Group G
    (1092, 0x9a67e4), // Phase 1 Group H
    (1093, 0x9a67d4), // Phase 2 Group A
    (1094, 0x9a67c4), // Phase 2 Group B
    (1095, 0x9a67b4), // Phase 2 Group C
    (1096, 0x9a67a4), // Phase 2 Group D
    (1097, 0x9a6790), // Phase 1 Group Stage
    (1098, 0x9a677c), // Phase 2 Group Stage
    (1099, 0x9a6768), // Championship Group
    (1100, 0x9a6754), // Relegation Group
    (1101, 0x9a6740), // Promotion Group A
    (1102, 0x9a672c), // Promotion Group B
    (1103, 0x9a6718), // Promotion Group C
    (1104, 0x9a6704), // Promotion Group D
    (1105, 0x9a66ec), // Promotion Group Stage
    (1106, 0x9a66d8), // Relegation Playoff
    (1107, 0x9a66d0), // North
    (1108, 0x9a66c4), // Northeast
    (1109, 0x9a66b4), // West/Southwest
    (1110, 0x9a66ac), // South
    (1111, 0x9a6684), // Central<%s - COMMENT - Central section>
    (1112, 0x9a6670), // Promotion Playoff
    (1113, 0x9a6660), // First Promotion
    (1114, 0x9a664c), // Second Promotion
    (1115, 0x9a663c), // Third Promotion
    (1116, 0x9a6628), // Fourth Promotion
    (1117, 0x9a6610), // Group A First Period
    (1118, 0x9a65f8), // Group A Second Period
    (1119, 0x9a65e0), // Group A Third Period
    (1120, 0x9a65c8), // Group B First Period
    (1121, 0x9a65b0), // Group B Second Period
    (1122, 0x9a6598), // Group B Third Period
    (1123, 0x9a6d50), // Group 1
    (1124, 0x9a6d48), // Group 2
    (1125, 0x9a6d40), // Group 3
    (1126, 0x9a6d38), // Group 4
    (1127, 0x9a6d30), // Group 5
    (1128, 0x9a6d28), // Group 6
    (1129, 0x9a6d20), // Group 7
    (1130, 0x9a6d18), // Group 8
    (1131, 0x9a5358), // Championship
    (1132, 0x9a66d0), // North
    (1133, 0x9a6588), // East Svealand
    (1134, 0x9a6578), // West Svealand
    (1135, 0x9a6568), // East Gotaland
    (1136, 0x9a6558), // West Gotaland
    (1137, 0x9a6548), // South Gotaland
    (1138, 0x9a6530), // Second Round Group 6
    (1139, 0x9a6510), // Central American Zone Group 1
    (1140, 0x9a64f0), // Central American Zone Group 2
    (1141, 0x9a64d8), // Classification Group 1
    (1142, 0x9a64c0), // Classification Group 2
    (1143, 0x9a64a8), // Classification Group 3
    (1144, 0x9a6490), // Classification Group 4
    (1145, 0x9a6478), // Classification Group 5
    (1146, 0x9a6460), // Classification Groups
    (1147, 0x9a644c), // Central Conference
    (1148, 0x9a6430), // Northern Relegation Section
    (1149, 0x9a6414), // Southern Relegation Section
    (1150, 0x9a6400), // Third Round Group A
    (1151, 0x9a63ec), // Third Round Group B
    (1152, 0x9a4094), // East
    (1153, 0x9a408c), // West
    (1154, 0x9a5344), // East Quarter Finals
    (1155, 0x9a5330), // West Quarter Finals
];

/// The comp_util.cpp source path of the error boxes.
const COMP_UTIL_SOURCE: usize = 0x99ec20;

/// Writes the localised name of a round, group or stage id into `buf` and returns `buf`.
///
/// `comp_util_get_stage_name(buf, size, id, form)` (cdecl; the id is the word of the third argument, the form the byte
/// of the fourth). First `buf` gets a copy of the string at 0xad9c64 (the game's empty text). With `size` < 30: the
/// error box of line 0x110f (`show_error_at`, comp_util.cpp) and nothing else. Form 0 (full name), 1 (short name) or 3
/// (plural name): id 0 prints the string at 0xad9c64 (sprintf `exe::crt_sprintf`, the string as the format); an id of
/// the form's table (`STAGE_NAMES_FULL`, `STAGE_NAMES_SHORT`, `STAGE_NAMES_PLURAL`: 322, 322 and 321 ids from 1 to
/// 1155, extracted by following the original's switches for every id and form) prints its text (langlib_sprintf
/// `FN_LANGLIB_SPRINTF`, the text as the format, no arguments: the "<%s - ...>" parts are translator comments the
/// langlib drops); any other id writes "Cannot find index %d" with the id into the global text buffer 0xde1f64 and
/// leaves `buf` as it is. Any other form: the error box of line 0x15e2.
/// Original: `comp_util_get_stage_name` (0x4b4850, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_get_stage_name(buf: usize, size: i32, id: u32, form: u32) -> usize {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    copy_c_string(buf, 0xad9c64);
    if size < 0x1e {
        show_error_at(COMP_UTIL_SOURCE, 0x110f);
        return buf;
    }
    let table: &[(i16, usize)] = match form as u8 {
        0 => &STAGE_NAMES_FULL, // cqy3: the full names
        3 => &STAGE_NAMES_PLURAL,
        1 => &STAGE_NAMES_SHORT,
        _ => {
            show_error_at(COMP_UTIL_SOURCE, 0x15e2);
            return buf;
        }
    };
    let id = id as u16 as i16;
    if id == 0 {
        sprintf(buf, 0xad9c64);
        return buf;
    }
    match table.binary_search_by_key(&id, |e| e.0) {
        Ok(k) => {
            covered!("cqy_3 stage name");
            lang_sprintf(buf, table[k].1);
        }
        Err(_) => {
            covered!("cqy_3 stage name not found");
            sprintf(0xde1f64, 0x9a51b0, id as i32); // the global text buffer, "Cannot find index %d"
        }
    }
    buf
}

/// Comparer for `comp_util_get_stage_name`: side by side, the original into the caller's buffer and the copy into its
/// own buffer of the same size (both write the same "Cannot find index" text into the global buffer); the two texts
/// must be equal. With `size` < 30 or a form other than 0, 1 and 3 (the error box), only the original.
pub(crate) unsafe extern "C" fn compare_comp_util_get_stage_name(buf: usize, size: i32, id: u32, form: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, u32, u32) -> usize>(%%I0%%);
    if size < 0x1e || !matches!(form as u8, 0 | 1 | 3) {
        return o(buf, size, id, form);
    }
    let mut mine = vec![0u8; (size as usize).max(0x400)];
    comp_util_get_stage_name(mine.as_mut_ptr() as usize, size, id, form);
    let r = o(buf, size, id, form);
    let a = c_string_bytes(buf);
    let b = c_string_bytes(mine.as_ptr() as usize);
    SCOREBOARD.record(a == b, || format!("comp_util_get_stage_name(id {}, form {}): {:?} x {:?}", id as u16 as i16, form as u8,
        String::from_utf8_lossy(&a), String::from_utf8_lossy(&b)));
    r
}

/// Lower-cases a C string in place, one character at a time (tolower `0x9465fc`, cdecl, the signed character), with a
/// signed byte index as in the original.
unsafe fn lower_case_in_place(p: usize) {
    let tolower: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x9465fcusize);
    let mut k: i8 = 0;
    let mut c = read_u8(p) as i8;
    while c != 0 {
        write_u8(p.wrapping_add(k as isize as usize), tolower(c as i32) as u8);
        k = k.wrapping_add(1);
        c = read_u8(p.wrapping_add(k as isize as usize)) as i8;
    }
}

/// Writes a label of competition, stage and round names into `buf` (`size` ≥ 0x67), using the long or the short
/// competition name by `form`; returns 1 (0 with the error box).
///
/// `comp_util_format_comp_stage_round_label(comp, stage, round, form, buf, size)` (cdecl; the stage and round ids are
/// the words of their arguments, the form the byte). With no `buf` or `size` < 0x67: the error box of line 0x25af and
/// 0. The German regional league (`KEY_CLUB_COMP_GERMAN_REGIONAL`) with stage 0x453 becomes the North division
/// (`KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_NORTH`, a 0x6b-byte record of `G_CLUB_COMP_TABLE`) and stage 0; then, with
/// stage 0x456, the South division and stage 0.
///
/// Form 1: the competition's long name (`club_comp::NAME`, +4) and its gender byte (+0x37); form 2: the short name
/// (+0x38) and its gender byte (+0x52); form 3: the same as form 2. The stage and the round names come from
/// `comp_util_get_stage_name` (100-byte buffers; form 0, or 1 in form 3) with their genders
/// (comp_util_get_stage_name_gender `exe::comp_util_get_stage_name_gender`; the round's with form 1 only in form 3). With
/// no competition the name is empty and its gender 0. Any other form leaves the three names unset (the original then
/// prints its stack leftovers; here they are empty) and the genders 0.
///
/// With a competition: in German (langlib_get_language `exe::langlib_get_language` = 8) and a stage or a round, the
/// competition name goes through the grammar modifier "s" (langlib_apply_grammar_modifier
/// `exe::langlib_apply_grammar_modifier`(name, 0xad9c64, "s", 100, 100), the result copied back) and the stage and round
/// names are lower-cased (`lower_case_in_place`). Then, by which ids are not 0: stage and round, the format 0x9a7cc0
/// (competition, stage, round, each with its gender twice); stage only, 0x9a7c94; round only, 0x9a7c68; neither, the
/// competition name copied. With no competition: stage and round, 0x9a7c44 (stage, round); stage only or round only,
/// that name copied; neither, `buf` left as it is. The formats are printed through langlib_sprintf
/// (`FN_LANGLIB_SPRINTF`). Returns 1.
/// Original: `comp_util_format_comp_stage_round_label` (0x4bfa30, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_format_comp_stage_round_label(comp: usize, stage: u32, round: u32, form: u32, buf: usize, size: i32) -> u32 {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if buf == 0 || size < 0x67 {
        show_error_at(COMP_UTIL_SOURCE, 0x25af);
        return 0;
    }
    let (mut comp, mut stage) = (comp, stage);
    if comp != 0 {
        let regional = read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL);
        let table = read_usize(G_CLUB_COMP_TABLE);
        if read_usize(comp) == regional && stage as u16 == 0x453 {
            comp = table + read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_NORTH) * 0x6b; // cqy3: the North division
            stage = 0;
        }
        if comp != 0 && read_usize(comp) == regional && stage as u16 == 0x456 {
            comp = table + read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_SOUTH) * 0x6b;
            stage = 0;
        }
    }
    let (mut name, mut stage_name, mut round_name) = ([0u8; 100], [0u8; 100], [0u8; 100]); // cleared: see the header
    let (pn, ps, pr) = (name.as_mut_ptr() as usize, stage_name.as_mut_ptr() as usize, round_name.as_mut_ptr() as usize);
    let (mut comp_gender, mut stage_gender, mut round_gender) = (0u8, 0u8, 0u8);
    let names = match form as u8 {
        1 => Some((4usize, 0x37usize, 0u32, 0u32)),
        2 => Some((0x38, 0x52, 0, 0)),
        3 => Some((0x38, 0x52, 1, 1)),
        _ => None,
    };
    if let Some((name_at, gender_at, stage_form, round_form)) = names {
        if comp != 0 {
            copy_c_string(pn, comp + name_at);
            comp_gender = read_u8(comp + gender_at);
        } // with no competition the name stays empty (the original writes its first byte 0)
        exe::comp_util_get_stage_name(ps as *mut u8, 100, stage, stage_form);
        stage_gender = exe::comp_util_get_stage_name_gender(stage, stage_form);
        exe::comp_util_get_stage_name(pr as *mut u8, 100, round, round_form);
        round_gender = exe::comp_util_get_stage_name_gender(round, round_form);
    }
    let (has_stage, has_round) = (stage as u16 != 0, round as u16 != 0);
    let (cg, sg, rg) = (comp_gender as i8 as i32, stage_gender as i8 as i32, round_gender as i8 as i32);
    if comp != 0 {
        if exe::langlib_get_language() == 8 && (has_stage || has_round) {
            covered!("cqy_3 label in german");
            let m = exe::langlib_apply_grammar_modifier(pn, 0xad9c64 as *const u8, 0x99c368 as *const u8, 100, 100);
            copy_c_string(pn, m);
            if has_stage {
                lower_case_in_place(ps);
            }
            if round as u16 != 0 {
                lower_case_in_place(pr);
            }
        }
        if has_stage && has_round {
            covered!("cqy_3 label competition stage round");
            lang_sprintf(buf, 0x9a7cc0, cg, cg, sg, sg, rg, rg, pn, ps, pr);
        } else if has_stage {
            lang_sprintf(buf, 0x9a7c94, cg, cg, sg, sg, pn, ps);
        } else if has_round {
            lang_sprintf(buf, 0x9a7c68, cg, cg, rg, rg, pn, pr);
        } else {
            copy_c_string(buf, pn);
        }
    } else if has_stage && has_round {
        covered!("cqy_3 label stage round");
        lang_sprintf(buf, 0x9a7c44, sg, sg, rg, rg, ps, pr);
    } else if has_stage {
        copy_c_string(buf, ps);
    } else if has_round {
        copy_c_string(buf, pr);
    }
    1
}

/// Comparer for `comp_util_format_comp_stage_round_label`: side by side, the original into the caller's buffer and the
/// copy into its own buffer of the same size that starts with the caller's bytes (the "neither" case leaves it as it
/// was; `copy_c_string` and `c_string_bytes` are the database and contract_manager helpers); the two texts must be equal, and the returns. With no buffer or `size` < 0x67 (the error box), only the
/// original.
pub(crate) unsafe extern "C" fn compare_comp_util_format_comp_stage_round_label(comp: usize, stage: u32, round: u32, form: u32, buf: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, usize, i32) -> u32>(%%I1%%);
    if buf == 0 || size < 0x67 {
        return o(comp, stage, round, form, buf, size);
    }
    let mut mine = vec![0u8; (size as usize).max(0x400)];
    core::ptr::copy_nonoverlapping(buf as *const u8, mine.as_mut_ptr(), size as usize);
    let b = comp_util_format_comp_stage_round_label(comp, stage, round, form, mine.as_mut_ptr() as usize, size);
    let a = o(comp, stage, round, form, buf, size);
    let (ta, tb) = (c_string_bytes(buf), c_string_bytes(mine.as_ptr() as usize));
    SCOREBOARD.record(a == b && ta == tb, || format!("comp_util_format_comp_stage_round_label({:#x}, {}, {}, form {}): {} {:?} x {} {:?}", comp,
        stage as u16 as i16, round as u16 as i16, form as u8, a, String::from_utf8_lossy(&ta), b, String::from_utf8_lossy(&tb)));
    a
}
