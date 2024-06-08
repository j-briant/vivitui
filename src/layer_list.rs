use gdal::vector::LayerAccess;
use gdal::Dataset;
use ratatui::prelude::*;
use ratatui::style::palette::tailwind;
use ratatui::{
    symbols::border,
    widgets::{block::*, *},
};

const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;

#[derive(Debug, Default)]
pub struct LayerList {
    pub items: Vec<String>,
    pub state: ListState,
}

impl LayerList {
    pub fn new(dataset: &Dataset) -> Self {
        let layers: Vec<String> = dataset.layers().map(|l| l.name()).collect();
        LayerList {
            items: layers,
            state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // Select the previous item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl StatefulWidget for LayerList {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        //let title = Title::from(" Layer list ".bold().yellow());
        let instructions = Title::from(Line::from(vec!["<Top>".into(), "<Down>".into()]));
        let block = Block::default()
            .title(" Layer list ".bold().yellow())
            .title_alignment(Alignment::Center)
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::PLAIN);

        StatefulWidget::render(
            List::new(self.items)
                .block(block)
                .highlight_symbol(">> ")
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::REVERSED)
                        .fg(SELECTED_STYLE_FG),
                )
                .direction(ListDirection::TopToBottom),
            area,
            buf,
            state,
        );
    }
}
