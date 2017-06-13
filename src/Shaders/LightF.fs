#version 330

uniform sampler2D pos_texture;
uniform sampler2D norm_texture;

uniform vec3 light_pos;
uniform vec3 light_color;

uniform vec3 light_attenuation;

uniform vec3 light_vector;
uniform float light_range;
uniform float light_maxradius;

in vec2 frag_texcoord;
out vec4 frag_output;

void main() {
    vec4 position = texture(pos_texture, frag_texcoord);
    vec4 normal = texture(norm_texture, frag_texcoord);
    vec3 ray_vector = light_pos.xyz - position.xyz;
    float distance = abs(length(ray_vector));
    if (distance > light_maxradius) {
        frag_output = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }
    float len = abs(length(light_vector));
    float alpha = abs(acos(dot(light_vector, ray_vector)/(distance * len)));

    if (alpha > light_range) {
        frag_output = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    ray_vector = normalize(ray_vector);
    vec3 normal_vector = normalize(normal.xyz);
    float diffuse = max(dot(normal_vector, ray_vector), 0.0);
    float attenuation_factor;
    if (diffuse > 0.0) {
        attenuation_factor = 1.0 / (
                        light_attenuation.x +
                        (light_attenuation.y * distance) +
                        (light_attenuation.z * distance * distance));
        
        attenuation_factor = max(attenuation_factor * ((light_maxradius - distance)/light_maxradius), 0.0);
        diffuse *= attenuation_factor;
    }
    frag_output = vec4(light_color * diffuse, 1.0);
}