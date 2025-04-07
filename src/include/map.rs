use crate::include::popups::Popup;

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

pub struct Map {
    //      inputs, height
    inputs: Vec<(String, u16)>,
    //      outputs, height   
    outputs: Vec<(String, u16)>,
    //     x, y matricies detailing each line instruction and appropriate dimensions to place each
    //     component.
    workspace: Vec<(Vec<(String, u16)>, Vec<(String, u16)>)>,
    //      sets the line, x cell, y cell to be highlighted or revised  
    last_known_index: (usize, usize, usize), 
    //      sets the 0,0 position to start drawing the workspace  
    last_known_origin: (usize, usize, usize),   
    pub error: Option<String>,
}

impl Map {
    pub fn new() -> Self {
        let mut map = Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            workspace: Vec::new(),
            last_known_index: (0,0,0),
            last_known_origin: (0,0,0),
            error: None,  
        };
        map.add_line();
        return map;  
    }
    fn map_outline(&mut self, f: &mut Frame) { 
        let layout = &Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]).split(f.area());  
        self.render_inputs_area(f, layout[0]);
        self.render_workspace_area(f, layout[1]);
        self.render_outputs_area(f, layout[2]);
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
        self.render_inputs_area(f, layout[0]);
        self.render_workspace_area(f, layout[1]);
        self.render_outputs_area(f, layout[2]); 
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
    fn render_inputs_area(&self, f: &mut Frame, area: Rect) {
        let inputbar = Block::bordered();
        f.render_widget(inputbar, area);  
        let (mut y_counter, mut starting_counter) = (4,0);  
        let (origin, _, _) = self.last_known_origin.clone();  
        for (i, h) in self.inputs.iter() {
            if starting_counter >= origin { 
                let popup_area = Rect { 
                    x: 0,
                    y: y_counter.clone(),
                    // 80 percent of 20 percent or 16 percent 
                    width: area.width,
                    height: h.clone(),
                };
                y_counter += h.clone()+4;
                let popup = Popup::default()
                    .content(i.clone());
                f.render_widget(popup, popup_area);  
            }; 
            starting_counter += 1;
        };
    }
    fn render_outputs_area(&self, f: &mut Frame, area: Rect) {
        let outputbar = Block::bordered();
        f.render_widget(outputbar, area);
        let (mut y_counter, mut starting_counter) = (4,0); 
        let (origin, _, _) = self.last_known_origin.clone(); 
        for (o, h) in self.outputs.iter() {
            if starting_counter >= origin { 
                let popup_area = Rect { 
                    x: area.width * 4,
                    y: y_counter.clone(),
                    width: area.width, 
                    height: h.clone(),
                };
                y_counter += h.clone()+4;   
                let popup = Popup::default()
                    .content(o.clone());
                f.render_widget(popup, popup_area); 
            }; 
            starting_counter += 1; 
        };
    } 
    fn render_workspace_area(&self, f: &mut Frame, area: Rect) {
        let workspace  = Block::bordered();
        f.render_widget(workspace, area);
    }    
    fn add_line(&mut self) {
        //let newinputs = self.inputs.len().to_string();  
        self.inputs.push(("inputs".to_string(), 4));

        //let newoutputs = self.outputs.len().to_string();
        self.outputs.push(("outputs".to_string(), 4));

        let mut vec1 = Vec::new();
        vec1.push(("vec1 drawspace".to_string(), 4));
        let mut vec2 = Vec::new();
        vec2.push(("vec2 drawspace".to_string(), 4));
        self.workspace.push((vec1, vec2)); 
    }
}
