mod include;

use crate::include::map::Map;

fn main() {
    let map = Map::new(); 
    let ok_terminal = ratatui::init();
    let mut map2 = map.generate_map(ok_terminal).unwrap();
    ratatui::restore();    
   
    map2.error = Some(String::from("this is a test of an error."));
    let bad_terminal = ratatui::init();
    map2.generate_map(bad_terminal);
    ratatui::restore();
}
