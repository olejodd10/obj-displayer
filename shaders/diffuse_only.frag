#version 140

in vec3 v_position;
in vec2 v_tex_coords;
in vec3 v_normal;

out vec4 color;

uniform vec3 u_light;
uniform vec3 specular_color;
uniform sampler2D ambient_tex;
uniform sampler2D diffuse_tex;

void main() {
    vec3 ambient_color = texture(ambient_tex, v_tex_coords).rgb;
    vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;

    float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(u_light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

    color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}