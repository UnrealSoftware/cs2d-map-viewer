#version 100
varying lowp vec2 uv;
varying lowp vec4 color;

uniform sampler2D Texture;

void main() {
    lowp vec4 tex = texture2D(Texture, uv);

    if (length(tex.rgb - vec3(0.0, 0.0, 0.0)) < 0.05) {
        discard;
    }

    gl_FragColor = tex;
}