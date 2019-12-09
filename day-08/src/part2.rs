use std::fs::read_to_string;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;

type Row = Vec<u32>;
type Layer = Vec<Row>;
type ImageLayers = Vec<Layer>;

fn create_layers(input: String) -> ImageLayers
{
    let mut layers = ImageLayers::new();

    let mut layer = Layer::new();
    let mut row = Row::new();

    for (idx, c) in input.chars().enumerate()
    {
        row.push(c.to_digit(10).unwrap() as u32);

        if idx == 0
        {
            continue;
        }
        else if (idx + 1) % (IMAGE_WIDTH * IMAGE_HEIGHT) == 0
        {
            layer.push(row.clone());
            row.clear();
            layers.push(layer.clone());
            layer.clear();
        }
        else if (idx + 1) % IMAGE_WIDTH == 0
        {
            layer.push(row.clone());
            row.clear();
        }
    }

    return layers;
}

fn decode_image(encoded_image: ImageLayers) -> Layer
{
    let mut decoded_image = Layer::new();

    for y in 0..IMAGE_HEIGHT
    {
        let mut row = Row::new();

        for x in 0..IMAGE_WIDTH
        {
            let mut color = 0;

            for layer in &encoded_image
            {
                if layer[y][x] != 2
                {
                    color = layer[y][x];
                    break;
                }
            }

            row.push(color);
        }

        decoded_image.push(row);
    }

    return decoded_image;
}

pub fn part2()
{
    let mut input = read_to_string("input/part1.txt").unwrap();
    input = input.trim_end().to_string();

    let layers = create_layers(input);

    let decoded_image = decode_image(layers);

    for y in 0..IMAGE_HEIGHT
    {
        for x in 0..IMAGE_WIDTH
        {
            print!("{}", decoded_image[y][x]);
        }

        println!("");
    }
}
