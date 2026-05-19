use macroquad::prelude::*;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams,
};

#[derive(Debug)]
pub struct Materials {
    pub grayscale_to_alpha: Material,
    pub lum_to_alpha: Material,
    pub lum_to_alpha_white: Material,
    pub mask_black : Material,
    pub mask_magenta: Material,
    pub premultiplied_cutoff: Material,
    pub premultiplied_alpha: Material,
    pub light_blend: Material,
    pub shade_blend: Material,
}

impl Materials {
    pub async fn load() -> Self {
        let grayscale_to_alpha = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("shaders/default.vert"),
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

        let lum_to_alpha = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("shaders/default.vert"),
                fragment: include_str!("shaders/lum_to_alpha.frag"),
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

        let lum_to_alpha_white = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("shaders/default.vert"),
                fragment: include_str!("shaders/lum_to_alpha_white.frag"),
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

        let mask_black = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("shaders/default.vert"),
                fragment: include_str!("shaders/mask_black.frag"),
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

        let mask_magenta = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("shaders/default.vert"),
                fragment: include_str!("shaders/mask_magenta.frag"),
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

        let premultiplied_cutoff = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("shaders/default.vert"),
                fragment: include_str!("shaders/premultiplied_cutoff.frag"),
            },
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        ).unwrap();

        let premultiplied_alpha = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("shaders/default.vert"),
                fragment: include_str!("shaders/premultiplied_alpha.frag"),
            },
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
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
                vertex: include_str!("shaders/default.vert"),
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
                vertex: include_str!("shaders/default.vert"),
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
            lum_to_alpha,
            lum_to_alpha_white,
            mask_black,
            mask_magenta,
            premultiplied_cutoff,
            premultiplied_alpha,
            light_blend,
            shade_blend,
        }
    }

    /// Use the default material
    #[inline(always)]
    pub fn use_default(&self) {
        gl_use_material(&self.premultiplied_alpha)
    }
}