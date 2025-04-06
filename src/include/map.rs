use crossterm::event;
use crossterm::event::Event;
use ratatui::{
    DefaultTerminal,
    Frame,
    prelude::{
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    text::Text,
    widgets::{
        Block,
        Paragraph,
    },
};
use std::collections::HashMap;

pub struct Map {
    inputs: Vec<String>,
    outputs: Vec<String>,
    workspace: HashMap<String, Vec<String>>,
    pub error: Option<String>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            workspace: HashMap::new(),
            error: None,  
        }
    }
    fn map_outline(&mut self, f: &mut Frame) { 
        let layout = &Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]).split(f.area());  
        self.render_inputs(f, layout[0]);
        self.render_workspace(f, layout[1]);
        self.render_outputs(f, layout[2]);
    }
    fn map_outline_with_error(&mut self, f: &mut Frame, errmsg: String) {
        let err = &Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ]).split(f.area());
    
        let layout = &Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]).split(err[1]);
        self.render_errmsg(f, err[0], errmsg);
        self.render_inputs(f, layout[0]);
        self.render_workspace(f, layout[1]);
        self.render_outputs(f, layout[2]); 
    }
    pub fn generate_map(mut self, mut terminal: DefaultTerminal) -> Result<Map, std::io::Error> {
        loop {
            match self.error.clone() {
                None => {
                    terminal.draw(|f| self.map_outline(f))?;
                }, 
                Some(errmsg) => {
                    terminal.draw(|f| self.map_outline_with_error(f, errmsg))?;
                },
            };    
            if matches!(event::read()?, Event::Key(_)) {
                break;
            };
        };
        return Ok(self)
    }
    fn render_errmsg(&self, f: &mut Frame, area: Rect, errmsg: String) {
        let msgbar = Paragraph::new(Text::from(errmsg))
            .block(Block::bordered());
        f.render_widget(msgbar, area);
    }
    fn render_inputs(&self, f: &mut Frame, area: Rect) {
        let inputbar = Block::bordered();
        f.render_widget(inputbar, area);
    }
    fn render_outputs(&self, f: &mut Frame, area: Rect) {
        let outputbar = Block::bordered();
        f.render_widget(outputbar, area);
    }
    fn render_workspace(&self, f: &mut Frame, area: Rect) {
        let workspace  = Block::bordered();
        f.render_widget(workspace, area);
    }    
}
