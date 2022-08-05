use std::mem;
use std::mem::MaybeUninit;
use std::alloc::{self, Layout};

pub struct MinecraftColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl MinecraftColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }
}

//Required to make color list shorter
const fn c(r: u8, g: u8, b: u8) -> MinecraftColor {
    MinecraftColor::new(r, g, b)
}

const MINECRAFT_COLOR_ARRAY: [MinecraftColor; 248] = [
    c(0, 0, 0),
    c(0, 0, 0),
    c(0, 0, 0),
    c(0, 0, 0),
    c(89, 125, 39),
    c(109, 153, 48),
    c(127, 178, 56),
    c(67, 94, 29),
    c(174, 164, 115),
    c(213, 201, 140),
    c(247, 233, 163),
    c(130, 123, 86),
    c(140, 140, 140),
    c(171, 171, 171),
    c(199, 199, 199),
    c(105, 105, 105),
    c(180, 0, 0),
    c(220, 0, 0),
    c(255, 0, 0),
    c(135, 0, 0),
    c(112, 112, 180),
    c(138, 138, 220),
    c(160, 160, 255),
    c(84, 84, 135),
    c(117, 117, 117),
    c(144, 144, 144),
    c(167, 167, 167),
    c(88, 88, 88),
    c(0, 87, 0),
    c(0, 106, 0),
    c(0, 124, 0),
    c(0, 65, 0),
    c(180, 180, 180),
    c(220, 220, 220),
    c(255, 255, 255),
    c(135, 135, 135),
    c(115, 118, 129),
    c(141, 144, 158),
    c(164, 168, 184),
    c(86, 88, 97),
    c(106, 76, 54),
    c(130, 94, 66),
    c(151, 109, 77),
    c(79, 57, 40),
    c(79, 79, 79),
    c(96, 96, 96),
    c(112, 112, 112),
    c(59, 59, 59),
    c(45, 45, 180),
    c(55, 55, 220),
    c(64, 64, 255),
    c(33, 33, 135),
    c(100, 84, 50),
    c(123, 102, 62),
    c(143, 119, 72),
    c(75, 63, 38),
    c(180, 177, 172),
    c(220, 217, 211),
    c(255, 252, 245),
    c(135, 133, 129),
    c(152, 89, 36),
    c(186, 109, 44),
    c(216, 127, 51),
    c(114, 67, 27),
    c(125, 53, 152),
    c(153, 65, 186),
    c(178, 76, 216),
    c(94, 40, 114),
    c(72, 108, 152),
    c(88, 132, 186),
    c(102, 153, 216),
    c(54, 81, 114),
    c(161, 161, 36),
    c(197, 197, 44),
    c(229, 229, 51),
    c(121, 121, 27),
    c(89, 144, 17),
    c(109, 176, 21),
    c(127, 204, 25),
    c(67, 108, 13),
    c(170, 89, 116),
    c(208, 109, 142),
    c(242, 127, 165),
    c(128, 67, 87),
    c(53, 53, 53),
    c(65, 65, 65),
    c(76, 76, 76),
    c(40, 40, 40),
    c(108, 108, 108),
    c(132, 132, 132),
    c(153, 153, 153),
    c(81, 81, 81),
    c(53, 89, 108),
    c(65, 109, 132),
    c(76, 127, 153),
    c(40, 67, 81),
    c(89, 44, 125),
    c(109, 54, 153),
    c(127, 63, 178),
    c(67, 33, 94),
    c(36, 53, 125),
    c(44, 65, 153),
    c(51, 76, 178),
    c(27, 40, 94),
    c(72, 53, 36),
    c(88, 65, 44),
    c(102, 76, 51),
    c(54, 40, 27),
    c(72, 89, 36),
    c(88, 109, 44),
    c(102, 127, 51),
    c(54, 67, 27),
    c(108, 36, 36),
    c(132, 44, 44),
    c(153, 51, 51),
    c(81, 27, 27),
    c(17, 17, 17),
    c(21, 21, 21),
    c(25, 25, 25),
    c(13, 13, 13),
    c(176, 168, 54),
    c(215, 205, 66),
    c(250, 238, 77),
    c(132, 126, 40),
    c(64, 154, 150),
    c(79, 188, 183),
    c(92, 219, 213),
    c(48, 115, 112),
    c(52, 90, 180),
    c(63, 110, 220),
    c(74, 128, 255),
    c(39, 67, 135),
    c(0, 153, 40),
    c(0, 187, 50),
    c(0, 217, 58),
    c(0, 114, 30),
    c(91, 60, 34),
    c(111, 74, 42),
    c(129, 86, 49),
    c(68, 45, 25),
    c(79, 1, 0),
    c(96, 1, 0),
    c(112, 2, 0),
    c(59, 1, 0),
    c(147, 124, 113),
    c(180, 152, 138),
    c(209, 177, 161),
    c(110, 93, 85),
    c(112, 57, 25),
    c(137, 70, 31),
    c(159, 82, 36),
    c(84, 43, 19),
    c(105, 61, 76),
    c(128, 75, 93),
    c(149, 87, 108),
    c(78, 46, 57),
    c(79, 76, 97),
    c(96, 93, 119),
    c(112, 108, 138),
    c(59, 57, 73),
    c(131, 93, 25),
    c(160, 114, 31),
    c(186, 133, 36),
    c(98, 70, 19),
    c(72, 82, 37),
    c(88, 100, 45),
    c(103, 117, 53),
    c(54, 61, 28),
    c(112, 54, 55),
    c(138, 66, 67),
    c(160, 77, 78),
    c(84, 40, 41),
    c(40, 28, 24),
    c(49, 35, 30),
    c(57, 41, 35),
    c(30, 21, 18),
    c(95, 75, 69),
    c(116, 92, 84),
    c(135, 107, 98),
    c(71, 56, 51),
    c(61, 64, 64),
    c(75, 79, 79),
    c(87, 92, 92),
    c(46, 48, 48),
    c(86, 51, 62),
    c(105, 62, 75),
    c(122, 73, 88),
    c(64, 38, 46),
    c(53, 43, 64),
    c(65, 53, 79),
    c(76, 62, 92),
    c(40, 32, 48),
    c(53, 35, 24),
    c(65, 43, 30),
    c(76, 50, 35),
    c(40, 26, 18),
    c(53, 57, 29),
    c(65, 70, 36),
    c(76, 82, 42),
    c(40, 43, 22),
    c(100, 42, 32),
    c(122, 51, 39),
    c(142, 60, 46),
    c(75, 31, 24),
    c(26, 15, 11),
    c(31, 18, 13),
    c(37, 22, 16),
    c(19, 11, 8),
    c(133, 33, 34),
    c(163, 41, 42),
    c(189, 48, 49),
    c(100, 25, 25),
    c(104, 44, 68),
    c(127, 54, 83),
    c(148, 63, 97),
    c(78, 33, 51),
    c(64, 17, 20),
    c(79, 21, 25),
    c(92, 25, 29),
    c(48, 13, 15),
    c(15, 88, 94),
    c(18, 108, 115),
    c(22, 126, 134),
    c(11, 66, 70),
    c(40, 100, 98),
    c(50, 122, 120),
    c(58, 142, 140),
    c(30, 75, 74),
    c(60, 31, 43),
    c(74, 37, 53),
    c(86, 44, 62),
    c(45, 23, 32),
    c(14, 127, 93),
    c(17, 155, 114),
    c(20, 180, 133),
    c(10, 95, 70),
    c(70, 70, 70),
    c(86, 86, 86),
    c(100, 100, 100),
    c(52, 52, 52),
    c(152, 123, 103),
    c(186, 150, 126),
    c(216, 175, 147),
    c(114, 92, 77),
    c(89, 117, 105),
    c(109, 144, 129),
    c(127, 167, 150),
    c(67, 88, 79)
];

const fn color_distance(c1: &MinecraftColor, c2: &MinecraftColor) -> f64 {
    let ra = (c1.red + c2.red) as f64 / 2.0;

    let rd = c1.red as f64 - c2.red as f64;
    let gd = c1.green as f64 - c2.green as f64;
    let bd = c1.blue as f64 - c2.blue as f64;

    let weight_r = 2.0 + ra / 256.0;
    let weight_g = 4.0;
    let weight_b = 2.0 + (255.0 - ra) / 256.0;

    weight_r * rd * rd + weight_g * gd * gd + weight_b * bd * bd
}

const fn get_mc_index(color: MinecraftColor) -> i8{
    let mut index: i16 = 0;
    let mut best: f64 = -1.0;

    let mut i: usize = 3;

    //Magic value: 248 = mc color size
    while i < 248 {
        let c = &MINECRAFT_COLOR_ARRAY[i];
        let d = color_distance(&color, c);

        if d < best || best == -1.0
        {
            best = d;
            index = i as i16;
        }

        i+=1;
    }

    if index < 128 {
        return index as i8;
    } else{
        return (-129 + (index - 127)) as i8;
    }

}

const fn compute_conversion_table() -> [i8; 16777216]{
    let mut table: [i8; 16777216] = [0; 16777216];

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    while r < 256 {
        while g < 256 {
            while b < 256 {
                table[(g * 256 * 256) + (b * 256 ) + r] = get_mc_index(
                    MinecraftColor::new(
                        r as u8,
                        g as u8,
                        b as u8
                    ));
                b += 1;
            }
            g += 1;
        }
        r += 1;
    }

    return table
}

const CONVERSION_TABLE: [i8; 16777216] = compute_conversion_table();

pub fn get_cached_index(color: MinecraftColor) -> i8 {
    println!("cached I");
    CONVERSION_TABLE[(color.green as usize * 256 * 256) + (color.blue as usize * 256) + color.red as usize]
}

//    int y, i;
//
//     // Write pixel data
//     for (y = 0; y < height; y++) {
//         unsigned char *pFrameData = pFrame->data[0] + y * pFrame->linesize[0];
//
//         //i = x
//         for (i = 0; i < width; i++) {
//             struct RgbColor rgbColor = col_getColor(pFrameData[(i * 3)], pFrameData[(i * 3) + 1],
//                                                     pFrameData[(i * 3) + 2]);
//             //buffer[(y*width)+i] = col_get_mc_index(col_getColor(pFrameData[(i * 3)], pFrameData[(i * 3) + 1], pFrameData[(i * 3) + 2]));
//             buffer[(y * width) + i] = col_get_cached_index(&rgbColor);
//         }
//     }
pub fn transform_frame_to_mc(data: &[u8], width: u32, height: u32) -> Vec<i8>{
    //height as usize * width as usize
    let mut buffer = Vec::<i8>::with_capacity((width * height) as usize);

    println!("yes 2!");

    println!("{}, {}", width, height);

    //len(data) = (width * 3) * height

    for y in 0..height  {
        for x in 0..width  {
            println!("AA! {}, {}", x, y);

            let a = MinecraftColor::new(
                data[((y * width * 3) + (x * 3)) as usize],
                data[((y * width * 3) + (x * 3) + 1) as usize],
                data[((y * width * 3) + (x * 3) + 1) as usize]
            );

            println!("BB");

            let d = get_cached_index(a);

            println!("DD");

            buffer.push(d);
        }
    }

    println!("yes 3!");

    return buffer;
    
    //for y in height:
    //for x in width:
    // r = pFrameData[(y * width * 3) + (i * 3)]
    // g = pFrameData[(y * width * 3) + (i * 3) + 1]
    // b = pFrameData[(y * width * 3) + (i * 3) + 2]

}