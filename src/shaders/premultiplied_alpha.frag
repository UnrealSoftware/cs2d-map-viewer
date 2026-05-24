#version 100
varying lowp vec2 uv;
varying lowp vec4 color;
uniform sampler2D Texture;

void main() {
    lowp vec4 tex_color = texture2D(Texture, uv);
    lowp vec4 final_color = color * tex_color;

    final_color.rgb *= final_color.a;
    gl_FragColor = final_color;
}