#version 100
varying lowp vec2 uv;
varying lowp vec4 color;

uniform sampler2D Texture;
uniform lowp vec2 uv_scale;
uniform lowp vec2 uv_offset;

void main() {
    lowp vec2 tiled_uv = fract(uv * uv_scale + uv_offset);
    gl_FragColor = texture2D(Texture, tiled_uv) * color;
}