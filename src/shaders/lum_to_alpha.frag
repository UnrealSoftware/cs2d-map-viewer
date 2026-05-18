#version 100
varying lowp vec2 uv;
varying lowp vec4 color;

uniform sampler2D Texture;

void main() {
    lowp vec4 tex = texture2D(Texture, uv);
    gl_FragColor = vec4(tex.r * color.r,  tex.g * color.g, tex.b * color.b, (tex.r * 0.21 + tex.g * 0.72 + tex.b * 0.07) * color.a);
}