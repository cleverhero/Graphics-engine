#version 330

uniform sampler2D tex;

in smooth vec3 frag_position;
in smooth vec3 frag_normal;
in smooth vec2 frag_texcoord;

out vec4 pos_texture;
out vec4 norm_texture;
out vec4 text_texture;
out vec4 FragColor;

void main() {
    pos_texture = vec4(frag_position, 1.0);
    norm_texture = vec4(frag_normal, 1.0);
    text_texture = texture(tex, frag_texcoord);
}