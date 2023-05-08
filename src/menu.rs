// Defines the in-sim menu structure.


extern crate xplm;

use xplm::menu::{ActionItem, Menu, MenuClickHandler};

use xplm::{ debugln };

use std::rc;
use crate::windows;

pub fn create_menu( win_mgr: rc::Rc<windows::WindowChrome> ) -> Menu{

    let action_handler = ActionHandler1{ _win_mgr: win_mgr };

    let plugins_submenu = Menu::new("OpenGL Test").unwrap();
        plugins_submenu.add_child(ActionItem::new("Show Window", action_handler).unwrap());
        plugins_submenu.add_to_plugins_menu();

    plugins_submenu

}


struct ActionHandler1{
    _win_mgr: rc::Rc<windows::WindowChrome>,
}

impl MenuClickHandler for ActionHandler1 {
    fn item_clicked(&mut self, _item: &ActionItem) {
        debugln!("Show Window...");
        self._win_mgr.get_window_ref().set_visible(true);
    }
}


