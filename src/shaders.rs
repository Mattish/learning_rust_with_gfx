pub static VERTEX_SHADER_SRC: &'static str = r#"
    #version 330

    in vec2 position;
    smooth out vec2 my_attr;

    uniform mat4 matrix;

    void main() {
        my_attr = position;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

pub static FRAGMENT_SHADER_SRC: &'static str = r#"
#version 330

out vec4 color;
smooth in vec2 my_attr;

void main() {
    color = vec4(my_attr, 0.0, 1.0);
}
"#;
