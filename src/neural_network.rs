use macroquad::rand::gen_range;

#[derive(Clone, Debug)]
struct Layer {
    weights: Vec<f32>,
    biases: Vec<f32>,
    inputs: usize,
    outputs: usize,
}

impl Layer {
    fn new(inputs: usize, outputs: usize) -> Self {
        let mut weights = vec![0.0; outputs * inputs];
        let mut biases = vec![0.0; outputs];

        for i in 0..outputs {
            biases[i] = gen_range(-1.0, 1.0);
            for j in 0..inputs {
                weights[i * inputs + j] = gen_range(-1.0, 1.0);
            }
        }

        Self {
            weights,
            biases,
            inputs,
            outputs,
        }
    }

    fn feed_forward(&self, inputs: &[f32]) -> Vec<f32> {
        let mut outputs = vec![0.0; self.outputs];

        for i in 0..self.outputs {
            let mut sum = self.biases[i];
            for j in 0..self.inputs {
                sum += inputs[j] * self.weights[i * self.inputs + j];
            }
            outputs[i] = sum.max(0.0);
        }

        outputs
    }
}

#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(layer_sizes: Vec<usize>) -> Self {
        let mut layers = Vec::new();

        for i in 0..layer_sizes.len() - 1 {
            layers.push(Layer::new(layer_sizes[i], layer_sizes[i + 1]));
        }

        Self { layers }
    }

    pub fn feed_forward(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        for layer in &self.layers {
            inputs = layer.feed_forward(&inputs);
        }
        inputs
    }

    pub fn mutate(&mut self, rate: f32) {
        for layer in &mut self.layers {
            for bias in &mut layer.biases {
                if gen_range(0.0, 1.0) < rate {
                    *bias += gen_range(-0.1, 0.1);
                }
            }
            for weight in &mut layer.weights {
                if gen_range(0.0, 1.0) < rate {
                    *weight += gen_range(-0.1, 0.1);
                }
            }
        }
    }
}
