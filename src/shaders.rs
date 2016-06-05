pub static VERTEX_SHADER_SRC: &'static str = r#"
    #version 140
    in vec3 vertex;
    in vec3 attr;
    out vec3 v_position;
    uniform mat4 perspective;
    uniform mat4 view;
    void main() {
        mat4 new_model;
        new_model[0] = vec4(0.25,0.0,  0.0,  0.0);
        new_model[1] = vec4(0.0,  0.25,0.0,  0.0);
        new_model[2] = vec4(0.0,  0.0,  0.25,0.0);
        new_model[3] = vec4(attr,1.0f);
        mat4 modelview = view * new_model;

        gl_Position = perspective * modelview * vec4(vertex, 1.0);
        v_position = gl_Position.xyz / gl_Position.w;
    }
"#;

pub static FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 140

    in vec3 v_position;

    out vec4 color;

    uniform vec3 u_light;

    void main() {
        color = vec4(1.0,0.0,0.0,1.0);
    }
"#;
