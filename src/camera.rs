use vecmath;

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

    pub fn get_view_matrix(&self, direction: [f32; 3]) ->  [[f32; 4]; 4]{
        view_matrix_new(self.position, direction , self.up)
    }

    pub fn new() -> Camera{
        Camera{
            position: [0.0,0.0,0f32],
            up: [0.0, 1.0, 0.0f32]
        }
    }
}

pub fn view_matrix_new(camera_pos: [f32;3], camera_target: [f32;3], camera_up: [f32;3]) -> [[f32;4];4]{
    let zaxis = vecmath::vec3_normalized_sub(camera_target, camera_pos);
    let xaxis = vecmath::vec3_normalized(vecmath::vec3_cross(camera_up, zaxis));
    let yaxis = vecmath::vec3_cross(zaxis, xaxis);

    [
        [xaxis[0], yaxis[0], zaxis[0],0.0],
        [xaxis[1], yaxis[1], zaxis[1], 0.0],
        [xaxis[2], yaxis[2], zaxis[2], 0.0],
        [-vecmath::vec3_dot(xaxis, camera_pos), -vecmath::vec3_dot(yaxis, camera_pos), -vecmath::vec3_dot(zaxis, camera_pos), 1.0]
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
