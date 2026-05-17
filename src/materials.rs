use macroquad::prelude::*;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams,
};

#[derive(Debug)]
pub struct Materials {
    pub grayscale_to_alpha: Material,
    pub light_blend: Material,
    pub shade_blend: Material,
}

impl Materials {
    pub async fn load() -> Self {
        let grayscale_to_alpha = load_material(
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
        ).unwrap();

        let light_blend = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("shaders/blend.vert"),
                fragment: include_str!("shaders/blend.frag"),
            },
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::One,
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        ).unwrap();

        let shade_blend = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("shaders/blend.vert"),
                fragment: include_str!("shaders/blend.frag"),
            },
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::DestinationColor),
                        BlendFactor::Zero,
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        ).unwrap();

        Self {
            grayscale_to_alpha,
            light_blend,
            shade_blend,
        }
    }
}