use crate::utils::lerp;
use bevy::prelude::{Component, Entity, Reflect};
use rand::Rng;

#[derive(Component, Reflect, Debug)]
pub struct NeuralNetwork {
    pub levels: Vec<NetworkLevel>,
    pub input_rays: Vec<Entity>,
}

impl NeuralNetwork {
    pub fn new(neuron_count_per_level: &Vec<u8>, input_rays: Vec<Entity>) -> Self {
        let mut network = Self {
            levels: Vec::new(),
            input_rays,
        };
        (0..neuron_count_per_level.len() - 1).for_each(|i| {
            network.levels.push(NetworkLevel::new(
                neuron_count_per_level[i],
                neuron_count_per_level[i + 1],
            ));
        });
        network
    }

    pub fn with_levels(
        levels: Vec<NetworkLevel>,
        input_rays: Vec<Entity>,
        mutate_factor: f32,
    ) -> Self {
        let mut network = Self { levels, input_rays };
        network.mutate(mutate_factor);
        network
    }

    pub fn feed_forward<'a>(&'a mut self, inputs: &'a Vec<f32>) -> &Vec<f32> {
        let mut outputs = inputs;
        self.levels.iter_mut().for_each(|level| {
            outputs = level.feed_forward(outputs);
        });
        // The last level outputs become our controls
        outputs
    }

    fn mutate(&mut self, amount: f32) {
        self.levels.iter_mut().for_each(|level| {
            let _ = level
                .biases
                .iter_mut()
                .map(|bias| {
                    let random: f32 = rand::thread_rng().gen_range(-1.0..1.);
                    *bias = lerp::<f32, f32>(*bias, random, amount);
                    *bias
                })
                .collect::<Vec<f32>>();

            level.weights.iter_mut().for_each(|weight_vec| {
                *weight_vec = weight_vec
                    .iter_mut()
                    .map(|weight| {
                        let random: f32 = rand::thread_rng().gen_range(-1.0..1.);
                        *weight = lerp::<f32, f32>(*weight, random, amount);
                        *weight
                    })
                    .collect::<Vec<f32>>();
            });
        });
    }
}

#[derive(Reflect, Debug, Clone)]
pub struct NetworkLevel {
    pub inputs: Vec<f32>,
    pub weights: Vec<Vec<f32>>,
    pub outputs: Vec<f32>,
    pub biases: Vec<f32>,
}

impl NetworkLevel {
    pub fn new(input_count: u8, output_count: u8) -> Self {
        let mut level = NetworkLevel {
            inputs: vec![0.; input_count.into()],
            weights: vec![vec![]; input_count.into()],
            outputs: vec![0.; output_count.into()],
            biases: vec![0.; output_count.into()],
        };
        (0..level.weights.len()).for_each(|i| level.weights[i] = vec![0.; output_count.into()]);

        level.randomize();
        level
    }

    fn randomize(&mut self) {
        (0..self.inputs.len()).for_each(|i| {
            (0..self.outputs.len()).for_each(|o| {
                self.weights[i][o] = rand::thread_rng().gen_range(-1.0..1.);
            });
        });

        (0..self.biases.len()).for_each(|i| {
            self.biases[i] = rand::thread_rng().gen_range(-1.0..1.);
        });
    }

    fn feed_forward(&mut self, inputs: &[f32]) -> &Vec<f32> {
        (0..self.inputs.len()).for_each(|i| {
            self.inputs[i] = inputs[i];
        });

        (0..self.outputs.len()).for_each(|oi| {
            let mut sum = 0.;
            (0..self.inputs.len()).for_each(|i| sum += self.inputs[i] * self.weights[i][oi]);
            if sum > self.biases[oi] {
                self.outputs[oi] = 1.;
            } else {
                self.outputs[oi] = 0.;
            }
        });

        &self.outputs
    }
}
