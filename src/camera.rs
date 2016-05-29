pub struct Camera{
    position: [f32;3],
    up: [f32;3]
}

impl Camera{
    pub fn set_pos(&mut self, position: &[f32; 3]){
        self.position[0] = position[0];
        self.position[1] = position[1];
        self.position[2] = position[2];
    }

    pub fn get_pos(&self) -> & [f32;3]{
        &self.position
    }

    pub fn get_view_matrix(&self, direction: &[f32; 3]) ->  [[f32; 4]; 4]{
        view_matrix(&self.position, direction , &self.up)
    }

    pub fn new() -> Camera{
        Camera{
            position: [0.0,0.0,0f32],
            up: [0.0, 1.0, 0.0f32]
        }
    }
}

pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s[0], u[0], f[0], 0.0],
        [s[1], u[1], f[1], 0.0],
        [s[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

pub fn get_perspectivei(height: u32, width: u32) -> [[f32;4]; 4]{
    let f_height = height as f32;
    let f_width = width as f32;
    get_perspective(f_height, f_width)
}

pub fn get_perspective(height: f32, width: f32) -> [[f32; 4]; 4] {
    let aspect_ratio = height/ width;

    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}
