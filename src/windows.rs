
use xplm::{ self, debugln };
use xplm::geometry::*;
use xplm::draw::{self, GraphicsState, Color};

use core::mem::size_of;
use std::ffi::c_void;



// macOS OpenGL bindings magic. *****************************************
#[cfg(target_os="macos")]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}



struct WindowDelegate{
    //custom window instance that can be delegated
    vbo_h: u32,
    //vao_h: u32, //Haven't got VAO's to work on Apple OpenGL yet.
    
}

impl WindowDelegate{

    fn new() -> Self{
        let mut new_window = WindowDelegate{
            vbo_h: 0,        
        };
        new_window.prepare_opengl_geometry();

        new_window
    }


    fn prepare_opengl_geometry( &mut self ){
        
        debugln!("Creating delegate window with vbo...");
        let mut vbo: u32 = 0;
    
        debugln!("Generate VBO buffer..");
        unsafe{
            gl::GenBuffers(1, &mut vbo);
        }
        debugln!("Gen buffers returned: {}", vbo);
    
        if vbo != 0{
            self.vbo_h = vbo;
        }else{
            panic!("Failed to create VBO buffer handle.");
        }
        

        type Vertex = [f32; 3];
    
        //Create our vertex data in prep for unsafe block.
        let vtx_size: usize = size_of::<[Vertex; 3]>();
        //debugln!("VERTICES struct size: {} bytes", vtx_size);
    
        const VERTICES: [Vertex; 3] =
        [
            [0.0, 0.0, 0.0], //left bot
            [25.0, 50.0, 0.0], //up middle
            [50.0, 0.0, 0.0],  //right bot
        ];
    
        unsafe{
            //bind our vbo id
            gl::BindBuffer(gl::ARRAY_BUFFER, 
                vbo);
            
            //push data
            debugln!("Uploading vtx data to VBO..");
            gl::BufferData(gl::ARRAY_BUFFER, 
                vtx_size as isize,
                VERTICES.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );
    
            //bind to default vbo 0
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    
    }

}


impl Drop for WindowDelegate{
    fn drop(&mut self){
        unsafe{
            gl::DeleteBuffers(1, &self.vbo_h);
        }
    }
}


impl xplm::window::WindowDelegate for WindowDelegate{

    fn draw(&mut self, _window: &xplm::window::Window) {

        let g = _window.geometry();

        let margin = 10;
        
        draw::draw_translucent_dark_box(
            Rect::from_left_bottom_right_top( 
                g.left() + margin,
                g.bottom() + margin,
                g.left() + g.width() - margin,
                g.bottom() + g.height() - margin,        
            )            
        );

        // Draw String bottom left corner, would be nice if it tracked with the window? 
        let color = Color::from_rgb( 1.0, 0.2, 0.2 );
        let top_edge = g.bottom() + g.height();

        draw::draw_string( &color, g.left() + margin, top_edge - 40, "OpenGL Hello World" ).unwrap();

        let gfx_state = GraphicsState{
            alpha_blending: false,
            lighting: false,
            fog: false,
            alpha_testing: false,
            depth_testing: false,
            depth_writing: true,
            textures: 0,
        };
        draw::set_state(&gfx_state);


        unsafe{
            gl::PushMatrix();
                
                gl::Translatef( g.left() as f32, 
                                g.bottom() as f32, 
                                0.0 );


                // Draw a yellow triangle in immediate mode.
                gl::Color3f( 1.0, 1.0, 0.5 );
                //gl::FrontFace( gl::CCW );

                    //gl::Begin( gl::LINE_LOOP );
                    gl::Begin( gl::TRIANGLES );
                        gl::Vertex3f( 0.0, 0.0, 0.0 ); //left bot
                        gl::Vertex3f( 25.0, 50.0, 0.0 ); //up middle
                        gl::Vertex3f( 50.0, 0.0, 0.0 ); //right bot                        
                    gl::End();


                // Draw a blue triangle using the VBO data.
                gl::Translatef( 100.0, 0.0, 0.0 );
                gl::Color3f( 0.5, 1.0, 1.0 );
                //gl::FrontFace( gl::CW );
                
                    gl::BindBuffer( gl::ARRAY_BUFFER, self.vbo_h );
                        gl::VertexPointer( 3, gl::FLOAT, 0, std::ptr::null() );
                        gl::DrawArrays(gl::TRIANGLES, 0, 3);
                    gl::BindBuffer( gl::ARRAY_BUFFER, 0 );
                    
            gl::PopMatrix();
        }

    }

}




pub struct WindowChrome{
    xp_win_h: xplm::window::WindowRef,
}

impl WindowChrome{

    pub fn new() -> Self{

        // Create Window wrapper.
        let left = 100;
        let bot = 100;
        let top = bot + 480;
        let right = left + 640;
        
        let left_top = Point::from_xy(left, top);
        let bot_right = Point::from_xy(bot, right);    
    
        let r = Rect::from_corners(left_top, bot_right);
    
        let options = xplm::window::WindowOptions{
            visible: true,
            decoration: xplm::window::WindowDecoration::RoundRectangle,
            layer: xplm::window::WindowLayer::FloatingWindows,
            title: Some(String::from("OpenGL Test")),
        };
    
        // Crete delegate struct instance.
        let window_delegate = WindowDelegate::new();
    
        let raw_window = xplm::window::Window::new(r, window_delegate, options);
        //raw_window.delegate().unwrap().set_draw_window_bounds(r);
    
        //debugln!("Creating win manager...");
        let new_manager = WindowChrome{
            xp_win_h: raw_window,
        };
    
        new_manager

    }


    pub fn get_window_ref( &self ) -> &xplm::window::WindowRef{
        //self._clients.get(0).unwrap().clone()
        &self.xp_win_h
    }
}



