mod include;

use crate::include::map::Map;

fn main() {
    let ok_result = Map::new(); 
    let ok_terminal = ratatui::init();
    let ok = None;
    ok_result.generate_map(ok_terminal, ok);
    ratatui::restore();    
   
    let bad_result = Map::new();
    let bad = Some("this is a test of an error.");
    let bad_terminal = ratatui::init();
    bad_result.generate_map(bad_terminal, bad);
    ratatui::restore();
}
