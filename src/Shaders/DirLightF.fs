#version 330

uniform sampler2D pos_texture;
uniform sampler2D norm_texture;

uniform vec3 light_color;
uniform vec3 light_vector;

in vec2 frag_texcoord;
out vec4 frag_output;

void main() {
    vec4 normal = texture(norm_texture, frag_texcoord);
    vec3 normal_vector = normalize(normal.xyz);

    float diffuse = max(dot(normal_vector, light_vector), 0.0);
    frag_output = vec4(light_color * diffuse, 1.0);
}