pub const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec3 position;
    in vec2 tex_coords;
    in vec3 normal;

    out vec3 v_position;
    out vec2 v_tex_coords;
    out vec3 v_normal;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        mat4 modelview = view * model;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = perspective * modelview * vec4(position, 1.0);
        v_position = gl_Position.xyz / gl_Position.w;
        v_tex_coords = tex_coords;
    }
    "#;

pub const DEFAULT_FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    in vec3 v_position;
    in vec2 v_tex_coords;
    in vec3 v_normal;

    out vec4 color;

    uniform vec3 u_light;

    const vec3 ambient_color = vec3(0.2, 0.2, 0.2);
    const vec3 diffuse_color = vec3(0.6, 0.6, 0.6);
    const vec3 specular_color = vec3(1.0, 1.0, 1.0);

    void main() {
        float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

        color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
    }
"#;


pub const SIMPLE_FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    in vec3 v_position;
    in vec2 v_tex_coords;
    in vec3 v_normal;

    out vec4 color;

    uniform vec3 u_light;
    uniform vec3 ambient_color;
    uniform vec3 diffuse_color;
    uniform vec3 specular_color;

    void main() {
        float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

        color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
    }
"#;

pub const ONLY_DIFFUSE_FRAGMENT_SHADER_SRC: &str = r#"
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
"#;

pub const COMPLETE_FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    in vec3 v_position;
    in vec2 v_tex_coords;
    in vec3 v_normal;

    out vec4 color;

    uniform vec3 u_light;
    uniform sampler2D ambient_tex;
    uniform sampler2D diffuse_tex;
    uniform sampler2D specular_tex;

    void main() {
        vec3 ambient_color = texture(ambient_tex, v_tex_coords).rgb;
        vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;
        vec3 specular_color = texture(specular_tex, v_tex_coords).rgb;

        float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

        color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
    }
"#;