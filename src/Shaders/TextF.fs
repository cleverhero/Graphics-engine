#version 330                                                                     
                                                                                 
uniform sampler2D tex;
uniform vec3 color;

in vec2 frag_texcoord;
out vec4 frag_output;

void main() {
	float a = texture(tex, frag_texcoord).r;
    frag_output = vec4(1.0, 1.0, 1.0, a);
    
}