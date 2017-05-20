#version 330                                                             
                                                                         
uniform mat4 matrix;

in vec3 position;
in vec2 tex_coord;

smooth out vec2 frag_texcoord;

void main() {
    frag_texcoord = tex_coord;
    gl_Position = matrix * vec4(position, 1.0);
}