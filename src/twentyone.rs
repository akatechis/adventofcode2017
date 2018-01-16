use self::Pixel::*;
type Image = Vec<Vec<Pixel>>;
type Rule = (Image, Image);

#[derive(Clone, Debug, PartialEq)]
enum Pixel {
  On, Off
}

fn create_image() -> Image {
  vec![
    parse_pixel_line(".#."),
    parse_pixel_line("..#"),
    parse_pixel_line("###")
  ]
}

fn count_on_pixels(image: &Image) -> usize {
  let mut sum = 0;
  for line in image {
    for pixel in line {
      if *pixel == On {
        sum += 1;
      }
    }
  }

  sum
}

fn sub_image(image: &Image, row: usize, col: usize, size: usize) -> Image {
  let mut subimage = vec![];

  for r in row..row+size {
    let mut copied_row = vec![];
    for c in col..col+size {
      copied_row.push(image[r][c].clone());
    }
    subimage.push(copied_row);
  }

  subimage
}

fn split_image(image: &Image) -> Vec<Vec<Image>> {
  let img_size = image.len();
  let split_size = if img_size % 2 == 0 { 2 } else { 3 };
  let last_row = (img_size / split_size) + 1;

  let mut images: Vec<Vec<Image>> = vec![];
  let mut row_ptr = 0;

  while row_ptr <= last_row {
    let mut row_images = vec![];
    let mut col_ptr = 0;
    while col_ptr < img_size {
      let sub_image = sub_image(image, row_ptr, col_ptr, split_size);
      row_images.push(sub_image);

      col_ptr += split_size;
    }

    images.push(row_images);
    row_ptr += split_size
  }

  images
}

fn parse_pixel_line(line: &str) -> Vec<Pixel> {
  line.chars().map(|ch| match ch {
    '#' => On, '.' => Off, _ => panic!()
  }).collect()
}

fn parse_rule_fragment(rule_str: &str) -> Image {
  let mut fragment = vec![];
  for line in rule_str.split('/') {
    fragment.push(parse_pixel_line(line));
  }
  fragment
}

fn rotate(src_img: &Image) -> Image {
  let mut img: Image = vec![];
  let size = src_img.len();

  for col in 0..size {
    let mut row_pixels = vec![];
    for row in (0..size).rev() {
      row_pixels.push(src_img[row][col].clone());
    }
    img.push(row_pixels);
  }

  img
}

fn vertical_flip(src_img: &Image) -> Image {
  let mut img: Image = vec![];
  for row_pixels in src_img.iter().rev() {
    img.push(row_pixels.clone());
  }
  img
}

fn horizontal_flip(src_img: &Image) -> Image {
  let mut img: Image = vec![];
  for row_pixels in src_img.iter() {
    img.push(row_pixels.iter().rev().map(|p|p.clone()).collect());
  }
  img
}

fn images_equal(left: &Image, right: &Image) -> bool {
  left.len() == right.len() && left.iter().zip(right.iter())
  .all(|(l_ln, r_ln)| l_ln.iter().zip(r_ln.iter())
    .all(|(l_px, r_px)| l_px == r_px))
}

fn image_matches_rule(img: &Image, rule: &Rule) -> bool {
  if images_equal(img, &rule.0) {
    return true;
  }
  else if images_equal(&rotate(img), &rule.0) {
    return true;
  }
  else if images_equal(&horizontal_flip(img), &rule.0) {
    return true;
  }
  else if images_equal(&vertical_flip(&img), &rule.0) {
    return true;
  }
  else if images_equal(&horizontal_flip(&rotate(&img)), &rule.0) {
    return true;
  }
  else if images_equal(&horizontal_flip(&vertical_flip(&img)), &rule.0) {
    return true;
  }
  else if images_equal(&vertical_flip(&rotate(&img)), &rule.0) {
    return true;
  }
  else if images_equal(&rotate(&horizontal_flip(&vertical_flip(&img))), &rule.0) {
    return true;
  }
  return false;
}

fn merge_images(icons: Vec<Vec<Image>>) -> Image {
  let split_size = icons[0][0].len();
  let image = vec![];
  let mut icon_row_ptr = 0;

  while icon_row_ptr < icons.len() {
    let row = vec![];
    let mut mapped_icons = icons[icon_row_ptr/split_size];
    let mut col_ptr = 0;

    while col_ptr < mapped_icons.len() * split_size {
      row.push(mapped_icons[col_ptr/split_size]);

      col_ptr += 1;
    }

    image.push(row);
    icon_row_ptr += 1;
  }

  image
}

fn transform_image(src: &mut Image, ruleset: &Vec<Rule>, iterations: usize) {
  let mut src_image = src.clone();
  for _ in 0..iterations {
    let mut icons = split_image(&src_image);

    for row in 0..icons.len() {
      for col in 0..icons[row].len() {
        let &(_, ref new_icon) = ruleset.iter().find(|rule|
          image_matches_rule(&icons[row][col], rule)).unwrap();
        icons[row][col] = new_icon.clone();
      }
    }

    src_image = merge_images(icons);
  }
}

fn main_1() {
  let ruleset = include_str!("../input/twentyone").lines().map(|rule| {
    let mut rule_parts = rule.split(" => ");
    let rule_in = parse_rule_fragment(rule_parts.next().unwrap());
    let rule_out = parse_rule_fragment(rule_parts.next().unwrap());
    (rule_in, rule_out)
  })
  .collect();

  let mut src_image = create_image();
  transform_image(&mut src_image, &ruleset, 5);
}

pub fn main() {
  main_1();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn images_equal_works() {
    let img1 = vec![
      parse_pixel_line(".#."),
      parse_pixel_line("..#"),
      parse_pixel_line("###")
    ];
    let img2 = vec![
      parse_pixel_line(".#."),
      parse_pixel_line("..#"),
      parse_pixel_line("###")
    ];
    let img3 = vec![
      parse_pixel_line(".#."),
      parse_pixel_line("#.#"),
      parse_pixel_line("###")
    ];

    assert_eq!(true, images_equal(&img1, &img2));
    assert_eq!(false, images_equal(&img1, &img3));
  }

  #[test]
  fn rotate_works() {
    let img1 = vec![
      parse_pixel_line(".#."),
      parse_pixel_line("..#"),
      parse_pixel_line("###")
    ];
    let img3 = vec![
      parse_pixel_line("#.."),
      parse_pixel_line("#.#"),
      parse_pixel_line("##.")
    ];
    assert_eq!(img3, rotate(&img1));
  }

  #[test]
  fn vertical_flip_works() {
    let img1 = vec![
      parse_pixel_line(".#."),
      parse_pixel_line("..#"),
      parse_pixel_line("###")
    ];
    let img4 = vec![
      parse_pixel_line("###"),
      parse_pixel_line("..#"),
      parse_pixel_line(".#.")
    ];
    assert_eq!(img4, vertical_flip(&img1));
  }

  #[test]
  fn horizontal_flip_works() {
    let img1 = vec![
      parse_pixel_line(".#."),
      parse_pixel_line("..#"),
      parse_pixel_line("###")
    ];
    let img4 = vec![
      parse_pixel_line(".#."),
      parse_pixel_line("#.."),
      parse_pixel_line("###")
    ];
    assert_eq!(img4, horizontal_flip(&img1));
  }

  #[test]
  fn image_matches_rule_works() {
    let img1 = vec![
      parse_pixel_line(".#."),
      parse_pixel_line("..#"),
      parse_pixel_line("###")
    ];
    let img2 = vec![
      parse_pixel_line(".#."),
      parse_pixel_line("#.."),
      parse_pixel_line("###")
    ];
    let img3 = vec![
      parse_pixel_line("#.."),
      parse_pixel_line("#.#"),
      parse_pixel_line("##.")
    ];
    let img4 = vec![
      parse_pixel_line("###"),
      parse_pixel_line("..#"),
      parse_pixel_line(".#.")
    ];

    let rule = (parse_rule_fragment(".#./..#/###"), parse_rule_fragment("../../.."));

    assert_eq!(true, image_matches_rule(&img1, &rule));
    assert_eq!(true, image_matches_rule(&img2, &rule));
    assert_eq!(true, image_matches_rule(&img3, &rule));
    assert_eq!(true, image_matches_rule(&img4, &rule));
  }

  #[test]
  fn sub_image_works() {
    let image = vec![
      parse_pixel_line("#..#"),
      parse_pixel_line("...."),
      parse_pixel_line("...."),
      parse_pixel_line("#..#")
    ];

    let one_one = vec![
      parse_pixel_line("#."),
      parse_pixel_line("..")
    ];
    let one_two = vec![
      parse_pixel_line(".#"),
      parse_pixel_line("..")
    ];
    let two_one = vec![
      parse_pixel_line(".."),
      parse_pixel_line("#.")
    ];
    let two_two = vec![
      parse_pixel_line(".."),
      parse_pixel_line(".#")
    ];

    assert_eq!(one_one, sub_image(&image, 0, 0, 2));
    assert_eq!(one_two, sub_image(&image, 0, 2, 2));
    assert_eq!(two_one, sub_image(&image, 2, 0, 2));
    assert_eq!(two_two, sub_image(&image, 2, 2, 2));
  }

  #[test]
  fn split_image_works() {
    let image = vec![
      parse_pixel_line("#..#"),
      parse_pixel_line("...."),
      parse_pixel_line("...."),
      parse_pixel_line("#..#")
    ];
    let expected = vec![
      vec![
        vec![
          parse_pixel_line("#."),
          parse_pixel_line("..")
        ],
        vec![
          parse_pixel_line(".#"),
          parse_pixel_line("..")
        ]
      ],
      vec![
        vec![
          parse_pixel_line(".."),
          parse_pixel_line("#.")
        ],
        vec![
          parse_pixel_line(".."),
          parse_pixel_line(".#")
        ]
      ]
    ];
    assert_eq!(expected, split_image(&image));
  }

  #[test]
  fn merge_images_works() {
    let icons = vec![
      vec![
        vec![
          parse_pixel_line(".#"),
          parse_pixel_line(".#")
        ],
        vec![
          parse_pixel_line("#."),
          parse_pixel_line("#.")
        ]
      ],
      vec![
        vec![
          parse_pixel_line(".#"),
          parse_pixel_line("##")
        ],
        vec![
          parse_pixel_line("#."),
          parse_pixel_line("##")
        ]
      ]
    ];
    let expected = vec![
      parse_pixel_line(".##."),
      parse_pixel_line(".##."),
      parse_pixel_line(".##."),
      parse_pixel_line("####")
    ];

    let image = merge_images(icons);
    assert_eq!(expected, image);
  }

  #[test]
  fn parse_rule_fragment_works() {
    assert_eq!(vec![vec![Off, Off], vec![Off, Off]], parse_rule_fragment("../.."));
    assert_eq!(vec![
      vec![On, On, On],
      vec![Off, On, On],
      vec![On, Off, Off]
    ], parse_rule_fragment("###/.##/#.."));
  }

  #[test]
  fn count_on_pixels_works() {
    let image = vec![
      vec![On, On, Off],
      vec![Off, Off, Off],
      vec![On, On, On]
    ];
    assert_eq!(5, count_on_pixels(&image));
  }

  #[test]
  fn transform_image_works() {
    let ruleset = vec![
      (parse_rule_fragment("../.#"), parse_rule_fragment("##./#../...")),
      (parse_rule_fragment(".#./..#/###"), parse_rule_fragment("#..#/..../..../#..#"))
    ];
    let mut img = create_image();

    transform_image(&mut img, &ruleset, 2);
    assert_eq!(12, count_on_pixels(&img));
  }
}
