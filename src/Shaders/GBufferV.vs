#version 330

uniform mat4 matrix;
uniform mat4 model_matrix;

in vec3 position;
in vec3 normal;
in vec2 tex_coord;

out smooth vec3 frag_position;
out smooth vec3 frag_normal;
out smooth vec2 frag_texcoord;

void main() {
    frag_position = (model_matrix * vec4(position, 1.0)).xyz;
    frag_normal = (model_matrix * vec4(normal, 0.0)).xyz;
    frag_texcoord = tex_coord;
    gl_Position = matrix * vec4(position, 1.0);
}