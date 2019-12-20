pub struct Layers {
    data: Vec<u32>,
    width: usize,
    height: usize,
    num_layers: usize,
}

impl Layers {
    pub fn new(data_str: &str, width: usize, height: usize) -> Layers {
        let data = data_str.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let num_layers = data_str.len() / (width * height);
        Layers {
            data: data,
            width: width,
            height: height,
            num_layers: num_layers,
        }
    }

    pub fn get_layer(&self, index: usize) -> Vec<u32> {
        let layer_size = self.width * self.height;
        let layer_start = index * layer_size;
        let later_end = layer_start + layer_size;
        let slice = &self.data[layer_start..later_end];
        slice.to_vec()
    }

    pub fn get_num_matching(&self, index: usize, digit: u32) -> u32 {
        let layer = self.get_layer(index);
        let mut num_zeros = 0;
        for i in &layer {
            if *i == digit {
                num_zeros += 1;
            }
        }
        num_zeros
    }

    pub fn combine_layers(&self) -> Vec<u32> {
        let layer_size = self.width * self.height;
        let mut image = Vec::new();
        println!("layer_size: {}", layer_size);
        image.reserve(layer_size);
        for image_index in 0..layer_size {
            for layer_index in 0..self.num_layers {
                let layer = self.get_layer(layer_index);
                if layer[image_index] != 2 {
                    image.push(layer[image_index]);
                    break;
                }
            }
        }
        image
    }
}

pub fn run_part1(input8: &str) {
    let layers = Layers::new(input8, 25, 6);
    let mut min_num_zeros = 10000;
    for index in 0..(layers.num_layers) {
        let num_zeros = layers.get_num_matching(index, 0);
        let num_ones = layers.get_num_matching(index, 1);
        let num_twos = layers.get_num_matching(index, 2);
        if num_zeros < min_num_zeros {
            min_num_zeros = num_zeros;
            println!("zeros: {}  prod: {}", num_zeros, num_ones * num_twos);
        }
    }
}

pub fn run_part2(input8: &str) {
    let layers = Layers::new(input8, 25, 6);
    let image = layers.combine_layers();

    let image_str = format!("{:?}", image);

    println!("{:?}", image);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_layer_0() {
        let layers = Layers::new("123456789012", 3, 2);
        assert_eq!(layers.get_layer(0), [1, 2, 3, 4, 5, 6].to_vec());
    }

    #[test]
    fn get_layer_1() {
        let layers = Layers::new("123456789012", 3, 2);
        assert_eq!(layers.get_layer(1), [7, 8, 9, 0, 1, 2].to_vec());
    }

    #[test]
    fn get_num_zeros_0() {
        let layers = Layers::new("123456789012", 3, 2);
        assert_eq!(layers.get_num_matching(0, 0), 0);
    }

    #[test]
    fn get_num_zeros_1() {
        let layers = Layers::new("123456789012", 3, 2);
        assert_eq!(layers.get_num_matching(1, 0), 1);
    }
}
