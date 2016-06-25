pub static VERTEX_SHADER_SRC: &'static str = r#"
    #version 140
    in vec3 vertex;
    in vec3 normal; 
    in vec3 attr;
    in vec3 colour;
    in float scale;
    out vec3 v_position;
    out vec3 v_normal;
    out vec3 v_colour;
    uniform mat4 perspective;
    uniform mat4 view; 
    void main() {

        mat4 new_model;
        new_model[0] = vec4(scale,  0.0,  0.0,  0.0);
        new_model[1] = vec4(0.0,  scale,  0.0,  0.0);
        new_model[2] = vec4(0.0,  0.0,  scale,  0.0);
        new_model[3] = vec4(attr,1.0f);
        mat4 modelview = view * new_model;

        gl_Position = perspective * modelview * vec4(vertex, 1.0);
        v_normal = transpose(inverse(mat3(new_model))) * normal;
        v_colour = colour;
    }
"#;

pub static FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 140

    in vec3 v_normal;
    in vec3 v_colour;
    out vec4 color;

    void main() {
        vec3 u_light = vec3(0.7,0.6,0.3);
        float brightness = dot(normalize(v_normal), normalize(u_light));
        vec3 dark_color = vec3(v_colour[0] / 2.0, v_colour[1] / 2.0, v_colour[2] / 2.0);
        vec3 regular_color = v_colour;
        color = vec4(mix(dark_color, regular_color, brightness), 1.0);
    }
"#;
