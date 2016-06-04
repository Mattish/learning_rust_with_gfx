pub static VERTEX_SHADER_SRC: &'static str = r#"
    #version 140
    in vec3 vertex;
    out vec3 v_position;
    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;
    void main() {
        mat4 modelview = view * model;
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
