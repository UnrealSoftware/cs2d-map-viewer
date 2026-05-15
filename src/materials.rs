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

const VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
    color = color0 / 255.0;
}
"#;

const FRAGMENT: &str = r#"#version 100
varying lowp vec2 uv;
varying lowp vec4 color;

uniform sampler2D Texture;

void main() {
    lowp vec4 tex = texture2D(Texture, uv);
    gl_FragColor = vec4(color.r, color.g, color.b, tex.r * color.a);
}
"#;