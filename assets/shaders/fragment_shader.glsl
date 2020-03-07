#version 150

in vec3 v_normal;
in vec3 v_position;

out vec4 color;

uniform vec3 u_light;
uniform vec3 u_color;

const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
    vec3 ambient_color = vec3(u_color.x / 2.0, u_color.y / 2.0, u_color.z / 2.0);

    float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);
    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(u_light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
    color = vec4(ambient_color + diffuse * u_color + specular * specular_color, 1.0);
}