use ratatui::prelude::*;
use ratatui::style::palette::tailwind;
use ratatui::widgets::{block::*, *};

use crate::data::LayerInfo;

const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;

#[derive(Debug, Default, Clone)]
pub struct LayerList {
    pub items: Vec<LayerInfo>,
    pub state: ListState,
}

impl LayerList {
    pub fn new(layer_info: Vec<LayerInfo>) -> Self {
        LayerList {
            items: layer_info,
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
        let block = Block::default().title(" Layer list ".bold().yellow());

        StatefulWidget::render(
            List::new(
                self.items
                    .into_iter()
                    .map(|li| li.name)
                    .collect::<Vec<String>>(),
            )
            .block(block)
            .highlight_symbol(">> ")
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(SELECTED_STYLE_FG),
            ),
            area,
            buf,
            state,
        );
    }
}
