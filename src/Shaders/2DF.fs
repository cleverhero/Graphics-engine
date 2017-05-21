#version 330                                                                     
                                                                                 
uniform sampler2D texture;
uniform vec3 color;

smooth in vec2 frag_texcoord;
out vec4 frag_output;

void main() {
    frag_output = vec4(color, 1.0);
}