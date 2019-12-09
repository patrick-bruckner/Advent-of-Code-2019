use std::fs::read_to_string;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;

type Row = Vec<u32>;
type Layer = Vec<Row>;
type ImageLayers = Vec<Layer>;

fn get_count(layer: &Layer, to_count: u32) -> u32
{
    let mut count = 0;

    for pixel in layer.iter().flatten()
    {
        if *pixel == to_count
        {
            count += 1;
        }
    }

    return count
}

pub fn part1()
{
    let mut input = read_to_string("input/part1.txt").unwrap();
    input = input.trim_end().to_string();

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

    let mut layer_with_least_zeros = 0;
    let mut min_zero_count = std::u32::MAX;

    for (idx, layer) in layers.iter().enumerate()
    {
        let count = get_count(layer, 0);
        if count < min_zero_count
        {
            min_zero_count = count;
            layer_with_least_zeros = idx;
        }
    }

    let number_of_ones = get_count(&layers[layer_with_least_zeros], 1);
    let number_of_twos = get_count(&layers[layer_with_least_zeros], 2);

    println!("Part1 answer: {}", number_of_ones * number_of_twos);
}
