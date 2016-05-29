extern crate glium;


pub fn get<'a>() -> glium::DrawParameters<'a>{
    glium::DrawParameters{
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    }
}
