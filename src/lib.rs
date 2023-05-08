
use xplm;
use xplm::menu::{Menu};
use xplm::data::borrowed::{FindError};
use xplm::plugin::{Plugin, PluginInfo};
use xplm::{ debugln, xplane_plugin};

use std::rc;

use crate::windows::WindowChrome;


mod menu;
mod windows;


// ****** macOS requires this to make OpenGL work inside X-Plane *******
//#[cfg(target_os = "macos")]
//{
    #[link(name = "OpenGL", kind = "framework")] extern {}
    mod gl {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }
//}




struct OpenGLPlugin {    
    _plugins_submenu: Menu,
    _window: rc::Rc<crate::windows::WindowChrome>,
}

impl OpenGLPlugin{
    
}



/* 
pub fn sym_finder( name: &str ) -> *mut c_void{
    let name = CString::new(name).unwrap();

    let symbol = unsafe{ xplm_sys::XPLMFindSymbol(name.as_ptr()) }; 
    if symbol.is_null() {
        debugln!("  Not found: {}", name.to_string_lossy());
    }else{
        debugln!("  Found: {}", name.to_string_lossy());
    }

    symbol
}
 */




impl Plugin for OpenGLPlugin {
    type Error = FindError; //FIXME: compare to other examples
    
    fn info() -> PluginInfo {
        let pi = PluginInfo{
            name: String::from("OpenGL Test-rs"),
            signature: String::from("OpenGL-test-rs.x-plugins.com"),
            description: String::from("OpenGL Rust Test"),
            version: String::from("0.1.0"),
        };
        
        pi
    }



    fn start( _info: PluginInfo ) -> Result<Self, Self::Error> {
        
        //debugln!("Finding GL Symbols..");        
        //gl::load_with( |symbol_name| sym_finder( symbol_name ) as *const _ );

        debugln!("Creating window..");        
        let window = rc::Rc::new( WindowChrome::new( ) );

        debugln!("Creating menu..");
        let plugins_submenu = menu::create_menu( rc::Rc::clone(&window) );
        
        debugln!("Creating plugin vars..");
        let plugin = OpenGLPlugin {
            _plugins_submenu: plugins_submenu,
            _window: window,
        };


        debugln!("Plugin started.");
        Ok(plugin)
    }


    fn enable(&mut self) -> Result<(), Self::Error> {
        debugln!("Test GL enabled.");
        
        Ok(())
    }

    
}






xplane_plugin!(OpenGLPlugin);

