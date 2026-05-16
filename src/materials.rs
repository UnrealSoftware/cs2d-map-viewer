use macroquad::prelude::*;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams,
};

#[derive(Debug)]
pub struct Materials {
    pub grayscale_to_alpha: Material,
}

impl Materials {
    pub async fn load() -> Self {
        let grayscale_alpha_material = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("shaders/grayscale_to_alpha.vert"),
                fragment: include_str!("shaders/grayscale_to_alpha.frag"),
            },
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    alpha_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
            .expect("failed to create shadow material");

        Self {
            grayscale_to_alpha: grayscale_alpha_material
        }
    }
}