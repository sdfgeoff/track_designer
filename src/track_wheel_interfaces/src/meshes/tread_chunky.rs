use meshtools::mesh::{Face, Mesh, VertWeight, Vertex};

pub fn get_mesh() -> Mesh {
    Mesh {
        vertices: vec![
            Vertex::new(2.1200030175805296e-08, 0.4850001633167267, 0.0),
            Vertex::new(0.40000009536743164, 0.3550000786781311, 0.0),
            Vertex::new(6.993824186452002e-09, 0.1600000411272049, 0.0),
            Vertex::new(0.40000009536743164, 0.029999984428286552, 0.0),
            Vertex::new(0.2000000774860382, 0.4850001633167267, 0.0),
            Vertex::new(0.20000004768371582, 0.1600000262260437, 0.0),
            Vertex::new(-6.556714060579338e-10, -0.015000012703239918, 0.0),
            Vertex::new(-0.40000009536743164, -0.14500002562999725, 0.0),
            Vertex::new(-1.48618761741659e-08, -0.34000009298324585, 0.0),
            Vertex::new(-0.40000009536743164, -0.47000014781951904, 0.0),
            Vertex::new(-0.20000004768371582, -0.015000004321336746, 0.0),
            Vertex::new(-0.2000000774860382, -0.34000009298324585, 0.0),
            Vertex::new(0.5000001192092896, 0.3550000786781311, 0.0),
            Vertex::new(0.5000001192092896, 0.029999980702996254, 0.0),
            Vertex::new(-0.5000001192092896, -0.14500001072883606, 0.0),
            Vertex::new(-0.5000001192092896, -0.47000014781951904, 0.0),
            Vertex::new(-0.13500000536441803, 0.4850001633167267, 0.0),
            Vertex::new(-0.13500002026557922, 0.1600000411272049, 0.0),
            Vertex::new(-0.20000001788139343, 0.4850001633167267, 0.0),
            Vertex::new(-0.20000003278255463, 0.1600000560283661, 0.0),
            Vertex::new(-0.40000006556510925, 0.3550001382827759, 0.0),
            Vertex::new(-0.40000009536743164, 0.030000019818544388, 0.0),
            Vertex::new(-0.5000001192092896, 0.3550001382827759, 0.0),
            Vertex::new(-0.5000001192092896, 0.030000023543834686, 0.0),
            Vertex::new(0.13500003516674042, -0.01500001922249794, 0.0),
            Vertex::new(0.13500002026557922, -0.34000009298324585, 0.0),
            Vertex::new(0.20000004768371582, -0.015000022016465664, 0.0),
            Vertex::new(0.20000003278255463, -0.34000009298324585, 0.0),
            Vertex::new(0.40000009536743164, -0.14500005543231964, 0.0),
            Vertex::new(0.40000006556510925, -0.47000014781951904, 0.0),
            Vertex::new(0.5000001192092896, -0.14500007033348083, 0.0),
            Vertex::new(0.5000001192092896, -0.47000014781951904, 0.0),
            Vertex::new(0.5000001192092896, -0.47000014781951904, 0.065000019967556),
            Vertex::new(0.5000001192092896, -0.14500007033348083, 0.065000019967556),
            Vertex::new(0.40000006556510925, -0.47000014781951904, 0.065000019967556),
            Vertex::new(0.40000009536743164, -0.14500005543231964, 0.065000019967556),
            Vertex::new(0.20000003278255463, -0.34000009298324585, 0.065000019967556),
            Vertex::new(
                0.20000004768371582,
                -0.015000022016465664,
                0.065000019967556,
            ),
            Vertex::new(0.13500002026557922, -0.34000009298324585, 0.065000019967556),
            Vertex::new(0.13500003516674042, -0.01500001922249794, 0.065000019967556),
            Vertex::new(-0.5000001192092896, 0.030000023543834686, 0.065000019967556),
            Vertex::new(-0.5000001192092896, 0.3550001382827759, 0.065000019967556),
            Vertex::new(
                -0.40000009536743164,
                0.030000019818544388,
                0.065000019967556,
            ),
            Vertex::new(-0.40000006556510925, 0.3550001382827759, 0.065000019967556),
            Vertex::new(-0.20000003278255463, 0.1600000560283661, 0.065000019967556),
            Vertex::new(-0.20000001788139343, 0.4850001633167267, 0.065000019967556),
            Vertex::new(-0.13500002026557922, 0.1600000411272049, 0.065000019967556),
            Vertex::new(-0.13500000536441803, 0.4850001633167267, 0.065000019967556),
            Vertex::new(-0.5000001192092896, -0.47000014781951904, 0.065000019967556),
            Vertex::new(-0.5000001192092896, -0.14500001072883606, 0.065000019967556),
            Vertex::new(0.5000001192092896, 0.029999980702996254, 0.065000019967556),
            Vertex::new(0.5000001192092896, 0.3550000786781311, 0.065000019967556),
            Vertex::new(-0.2000000774860382, -0.34000009298324585, 0.065000019967556),
            Vertex::new(
                -0.20000004768371582,
                -0.015000004321336746,
                0.065000019967556,
            ),
            Vertex::new(
                -0.40000009536743164,
                -0.47000014781951904,
                0.065000019967556,
            ),
            Vertex::new(
                -1.48618761741659e-08,
                -0.34000009298324585,
                0.065000019967556,
            ),
            Vertex::new(
                -0.40000009536743164,
                -0.14500002562999725,
                0.065000019967556,
            ),
            Vertex::new(
                -6.556714060579338e-10,
                -0.015000012703239918,
                0.065000019967556,
            ),
            Vertex::new(0.20000004768371582, 0.1600000262260437, 0.065000019967556),
            Vertex::new(0.2000000774860382, 0.4850001633167267, 0.065000019967556),
            Vertex::new(0.40000009536743164, 0.029999984428286552, 0.065000019967556),
            Vertex::new(6.993824186452002e-09, 0.1600000411272049, 0.065000019967556),
            Vertex::new(0.40000009536743164, 0.3550000786781311, 0.065000019967556),
            Vertex::new(
                2.1200030175805296e-08,
                0.4850001633167267,
                0.065000019967556,
            ),
            Vertex::new(
                0.43500009179115295,
                -0.40500012040138245,
                0.1950000524520874,
            ),
            Vertex::new(
                0.43500009179115295,
                -0.21000006794929504,
                0.1950000524520874,
            ),
            Vertex::new(
                0.40000006556510925,
                -0.40500012040138245,
                0.1950000524520874,
            ),
            Vertex::new(
                0.40000009536743164,
                -0.21000006794929504,
                0.1950000524520874,
            ),
            Vertex::new(
                0.20000003278255463,
                -0.27500006556510925,
                0.1950000524520874,
            ),
            Vertex::new(
                0.20000004768371582,
                -0.08000002056360245,
                0.1950000524520874,
            ),
            Vertex::new(
                0.14500002562999725,
                -0.27500006556510925,
                0.1950000524520874,
            ),
            Vertex::new(
                0.14500004053115845,
                -0.08000002056360245,
                0.1950000524520874,
            ),
            Vertex::new(
                -0.43500009179115295,
                0.09500003606081009,
                0.1950000524520874,
            ),
            Vertex::new(-0.43500009179115295, 0.2900001108646393, 0.1950000524520874),
            Vertex::new(
                -0.40000009536743164,
                0.09500003606081009,
                0.1950000524520874,
            ),
            Vertex::new(-0.40000006556510925, 0.2900001108646393, 0.1950000524520874),
            Vertex::new(-0.20000003278255463, 0.2250000536441803, 0.1950000524520874),
            Vertex::new(-0.20000001788139343, 0.4200000762939453, 0.1950000524520874),
            Vertex::new(-0.14500002562999725, 0.2250000536441803, 0.1950000524520874),
            Vertex::new(-0.14500001072883606, 0.4200000762939453, 0.1950000524520874),
            Vertex::new(
                -0.43500009179115295,
                -0.40500012040138245,
                0.1950000524520874,
            ),
            Vertex::new(
                -0.43500009179115295,
                -0.21000002324581146,
                0.1950000524520874,
            ),
            Vertex::new(0.43500009179115295, 0.0949999988079071, 0.1950000524520874),
            Vertex::new(0.43500009179115295, 0.2900000512599945, 0.1950000524520874),
            Vertex::new(
                -0.2000000774860382,
                -0.27500006556510925,
                0.1950000524520874,
            ),
            Vertex::new(
                -0.20000004768371582,
                -0.08000000566244125,
                0.1950000524520874,
            ),
            Vertex::new(
                -0.40000009536743164,
                -0.40500012040138245,
                0.1950000524520874,
            ),
            Vertex::new(
                -0.015000016428530216,
                -0.27500006556510925,
                0.1950000524520874,
            ),
            Vertex::new(
                -0.40000009536743164,
                -0.21000002324581146,
                0.1950000524520874,
            ),
            Vertex::new(
                -0.01500000711530447,
                -0.08000001311302185,
                0.1950000524520874,
            ),
            Vertex::new(0.20000004768371582, 0.2250000536441803, 0.1950000524520874),
            Vertex::new(0.2000000774860382, 0.4200000762939453, 0.1950000524520874),
            Vertex::new(0.40000009536743164, 0.0949999988079071, 0.1950000524520874),
            Vertex::new(0.015000014565885067, 0.2250000536441803, 0.1950000524520874),
            Vertex::new(0.40000009536743164, 0.2900000512599945, 0.1950000524520874),
            Vertex::new(0.015000022016465664, 0.4200000762939453, 0.1950000524520874),
            Vertex::new(0.050000011920928955, 0.5000001192092896, 0.0),
            Vertex::new(0.40000009536743164, 0.5000001192092896, 0.0),
            Vertex::new(0.20000004768371582, 0.5000001192092896, 0.0),
            Vertex::new(0.5000001192092896, 0.5000001192092896, 0.0),
            Vertex::new(-0.050000011920928955, 0.5000001192092896, 0.0),
            Vertex::new(-0.20000004768371582, 0.5000001192092896, 0.0),
            Vertex::new(-0.40000009536743164, 0.5000001192092896, 0.0),
            Vertex::new(-0.5000001192092896, 0.5000001192092896, 0.0),
            Vertex::new(-0.050000011920928955, -0.5000001192092896, 0.0),
            Vertex::new(-0.40000009536743164, -0.5000001192092896, 0.0),
            Vertex::new(-0.20000004768371582, -0.5000001192092896, 0.0),
            Vertex::new(-0.5000001192092896, -0.5000001788139343, 0.0),
            Vertex::new(0.050000011920928955, -0.5000001192092896, 0.0),
            Vertex::new(0.20000004768371582, -0.5000001192092896, 0.0),
            Vertex::new(0.40000006556510925, -0.5000001788139343, 0.0),
            Vertex::new(0.5000001192092896, -0.5000001788139343, 0.0),
        ],
        faces: vec![
            Face(67, 33, 35),
            Face(39, 25, 38),
            Face(91, 62, 59),
            Face(57, 87, 55),
            Face(31, 33, 32),
            Face(93, 63, 61),
            Face(46, 76, 44),
            Face(69, 35, 37),
            Face(58, 2, 5),
            Face(50, 3, 13),
            Face(6, 55, 8),
            Face(7, 53, 10),
            Face(48, 81, 49),
            Face(16, 46, 17),
            Face(40, 22, 23),
            Face(21, 40, 23),
            Face(50, 12, 51),
            Face(16, 101, 18),
            Face(63, 4, 0),
            Face(4, 96, 0),
            Face(12, 97, 1),
            Face(6, 25, 24),
            Face(36, 25, 27),
            Face(35, 30, 28),
            Face(17, 24, 2),
            Face(16, 2, 0),
            Face(14, 56, 7),
            Face(19, 42, 21),
            Face(11, 54, 9),
            Face(22, 43, 20),
            Face(28, 13, 3),
            Face(59, 1, 4),
            Face(5, 28, 3),
            Face(18, 47, 16),
            Face(24, 5, 2),
            Face(60, 5, 3),
            Face(32, 29, 31),
            Face(21, 14, 7),
            Face(15, 49, 14),
            Face(10, 21, 7),
            Face(39, 26, 24),
            Face(8, 52, 11),
            Face(9, 48, 15),
            Face(6, 19, 10),
            Face(17, 44, 19),
            Face(20, 45, 18),
            Face(34, 27, 29),
            Face(37, 28, 26),
            Face(10, 57, 6),
            Face(63, 2, 61),
            Face(62, 12, 1),
            Face(47, 78, 46),
            Face(64, 34, 32),
            Face(40, 73, 41),
            Face(95, 59, 63),
            Face(53, 89, 57),
            Face(71, 37, 39),
            Face(90, 61, 58),
            Face(54, 80, 48),
            Face(92, 58, 60),
            Face(68, 38, 36),
            Face(82, 51, 83),
            Face(49, 88, 56),
            Face(41, 75, 43),
            Face(70, 39, 38),
            Face(56, 85, 53),
            Face(82, 60, 50),
            Face(44, 74, 42),
            Face(75, 72, 74),
            Face(65, 66, 64),
            Face(77, 74, 76),
            Face(86, 85, 88),
            Face(81, 86, 88),
            Face(79, 76, 78),
            Face(84, 89, 85),
            Face(90, 95, 93),
            Face(69, 66, 67),
            Face(94, 82, 83),
            Face(90, 94, 91),
            Face(69, 70, 68),
            Face(42, 72, 40),
            Face(66, 36, 34),
            Face(52, 86, 54),
            Face(94, 51, 62),
            Face(43, 77, 45),
            Face(55, 84, 52),
            Face(32, 65, 64),
            Face(45, 79, 47),
            Face(20, 103, 22),
            Face(1, 98, 4),
            Face(0, 100, 16),
            Face(18, 102, 20),
            Face(11, 104, 8),
            Face(25, 109, 27),
            Face(29, 111, 31),
            Face(15, 105, 9),
            Face(8, 108, 25),
            Face(27, 110, 29),
            Face(9, 106, 11),
            Face(67, 65, 33),
            Face(39, 24, 25),
            Face(91, 94, 62),
            Face(57, 89, 87),
            Face(31, 30, 33),
            Face(93, 95, 63),
            Face(46, 78, 76),
            Face(69, 67, 35),
            Face(58, 61, 2),
            Face(50, 60, 3),
            Face(6, 57, 55),
            Face(7, 56, 53),
            Face(48, 80, 81),
            Face(16, 47, 46),
            Face(40, 41, 22),
            Face(21, 42, 40),
            Face(50, 13, 12),
            Face(16, 100, 101),
            Face(63, 59, 4),
            Face(4, 98, 96),
            Face(12, 99, 97),
            Face(6, 8, 25),
            Face(36, 38, 25),
            Face(35, 33, 30),
            Face(17, 6, 24),
            Face(16, 17, 2),
            Face(14, 49, 56),
            Face(19, 44, 42),
            Face(11, 52, 54),
            Face(22, 41, 43),
            Face(28, 30, 13),
            Face(59, 62, 1),
            Face(5, 26, 28),
            Face(18, 45, 47),
            Face(24, 26, 5),
            Face(60, 58, 5),
            Face(32, 34, 29),
            Face(21, 23, 14),
            Face(15, 48, 49),
            Face(10, 19, 21),
            Face(39, 37, 26),
            Face(8, 55, 52),
            Face(9, 54, 48),
            Face(6, 17, 19),
            Face(17, 46, 44),
            Face(20, 43, 45),
            Face(34, 36, 27),
            Face(37, 35, 28),
            Face(10, 53, 57),
            Face(63, 0, 2),
            Face(62, 51, 12),
            Face(47, 79, 78),
            Face(64, 66, 34),
            Face(40, 72, 73),
            Face(95, 91, 59),
            Face(53, 85, 89),
            Face(71, 69, 37),
            Face(90, 93, 61),
            Face(54, 86, 80),
            Face(92, 90, 58),
            Face(68, 70, 38),
            Face(82, 50, 51),
            Face(49, 81, 88),
            Face(41, 73, 75),
            Face(70, 71, 39),
            Face(56, 88, 85),
            Face(82, 92, 60),
            Face(44, 76, 74),
            Face(75, 73, 72),
            Face(65, 67, 66),
            Face(77, 75, 74),
            Face(86, 84, 85),
            Face(81, 80, 86),
            Face(79, 77, 76),
            Face(84, 87, 89),
            Face(90, 91, 95),
            Face(69, 68, 66),
            Face(94, 92, 82),
            Face(90, 92, 94),
            Face(69, 71, 70),
            Face(42, 74, 72),
            Face(66, 68, 36),
            Face(52, 84, 86),
            Face(94, 83, 51),
            Face(43, 75, 77),
            Face(55, 87, 84),
            Face(32, 33, 65),
            Face(45, 77, 79),
            Face(20, 102, 103),
            Face(1, 97, 98),
            Face(0, 96, 100),
            Face(18, 101, 102),
            Face(11, 106, 104),
            Face(25, 108, 109),
            Face(29, 110, 111),
            Face(15, 107, 105),
            Face(8, 104, 108),
            Face(27, 109, 110),
            Face(9, 105, 106),
        ],
        vertex_groups: vec![
            (
                "edge_right".to_string(),
                vec![
                    VertWeight::new(12, 1.0),
                    VertWeight::new(13, 1.0),
                    VertWeight::new(30, 1.0),
                    VertWeight::new(31, 1.0),
                    VertWeight::new(99, 1.0),
                    VertWeight::new(111, 1.0),
                ],
            ),
            (
                "edge_left".to_string(),
                vec![
                    VertWeight::new(14, 1.0),
                    VertWeight::new(15, 1.0),
                    VertWeight::new(22, 1.0),
                    VertWeight::new(23, 1.0),
                    VertWeight::new(103, 1.0),
                    VertWeight::new(107, 1.0),
                ],
            ),
        ]
        .into_iter()
        .collect(),
    }
}
