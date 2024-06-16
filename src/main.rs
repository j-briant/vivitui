use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use gdal::Dataset;
use layer_list::LayerList;
use ratatui::prelude::*;
use std::io;
use std::path::PathBuf;
use vivitui::data::LayerInfo;
use vivitui::{data, extent::ExtentUi, layer_list, position_map::PositionMapUi, srs::SrsUi, tui};

#[derive(Debug)]
pub struct App {
    layer_list: LayerList,
    layer_info: Vec<LayerInfo>,
    exit: bool,
}

impl App {
    pub fn new(dataset: Dataset) -> Self {
        let layer_info = LayerInfo::from_dataset(&dataset);
        let layer_list = LayerList::new(&dataset);
        Self {
            layer_info,
            layer_list,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    // Layer list is a stateful widget -> &mut self
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let layer_list = self.layer_list.clone();
        layer_list.render(area, buf, &mut self.layer_list.state);
    }

    fn render_srs(&self, area: Rect, buf: &mut Buffer) {
        if let Some(li) = self
            .layer_info
            .get(self.layer_list.state.selected().unwrap_or(0))
        {
            SrsUi::new(li.srs.clone()).render(area, buf);
        }
    }

    fn render_extent(&self, area: Rect, buf: &mut Buffer) {
        if let Some(li) = self
            .layer_info
            .get(self.layer_list.state.selected().unwrap_or(0))
        {
            ExtentUi::new(li.extent.clone()).render(area, buf);
        }
    }

    fn render_position_map(&self, area: Rect, buf: &mut Buffer) {
        if let Some(li) = self
            .layer_info
            .get(self.layer_list.state.selected().unwrap_or(0))
        {
            PositionMapUi::new(li.position_map.clone()).render(area, buf);
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('j') | KeyCode::Down => self.layer_list.next(),
            KeyCode::Char('k') | KeyCode::Up => self.layer_list.previous(),
            _ => {}
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(60),
            ],
        )
        .split(area);

        let inner_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(40),
                Constraint::Max(20),
                Constraint::Percentage(40),
            ],
        )
        .split(main_layout[1]);

        self.render_list(main_layout[0], buf);
        self.render_srs(inner_layout[0], buf);
        self.render_extent(inner_layout[1], buf);
        self.render_position_map(main_layout[2], buf);
    }
}

fn main() -> io::Result<()> {
    let data = data::dataset(PathBuf::from("data/my_dataset.gpkg"));
    match data {
        Ok(d) => {
            let mut terminal = tui::init()?;
            let app_result = App::new(d).run(&mut terminal);
            tui::restore()?;
            app_result
        }
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "error while loading data",
        )),
    }
}
