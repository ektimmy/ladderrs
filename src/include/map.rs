use ratatui::{
    Frame,
    prelude::{
        Constraint,
        Direction,
        Layout,
        Rect,
    },
};
use std::rc::Rc;

fn map_outline(f: &Frame) -> Rc<[Rect]> {
    
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(f.area());
    return layout
}

fn map_outline_with_error(f: &Frame) -> Rc<[Rect]> {
    
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(f.area());
    
    let _ = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(layout[1]);

    return layout
}
