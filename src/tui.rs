use std::time::Duration;

use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::{config::Endpoint, probe};

const PROGRESS: [char; 4] = ['|', '/', '-', '\\'];

pub struct State {
    tick: usize,
    endpoints: Vec<(String, Option<probe::Result>)>,
}

impl State {
    pub fn new(endpoints: &[Endpoint]) -> Self {
        Self {
            endpoints: endpoints
                .iter()
                .map(|e| (format!("{}:", e.name), None))
                .collect(),
            tick: 0,
        }
    }

    pub fn set_result(&mut self, index: usize, result: probe::Result) {
        self.endpoints[index].1 = Some(result);
    }

    pub fn tick(&mut self) {
        self.tick = (self.tick + 1) % PROGRESS.len()
    }

    pub fn view(&self, frame: &mut Frame) {
        let frame_area = frame.size();
        for (i, (name, result)) in self.endpoints.iter().enumerate() {
            let mut area = frame_area;
            area.y += i as u16;
            frame.render_widget(Paragraph::new(name.as_str()), area);
            area.x += name.len() as u16 + 1;
            self.view_result(frame, result.as_ref(), area)
        }
    }

    fn view_result(&self, frame: &mut Frame, result: Option<&probe::Result>, area: Rect) {
        match result {
            Some(Ok(duration)) => {
                frame.render_widget(Paragraph::new(format!("OK ({duration:?})")), area);
            }
            Some(Err(err)) => {
                frame.render_widget(Paragraph::new(format!("ERROR: {err:?}")), area);
            }
            None => {
                frame
                    .buffer_mut()
                    .get_mut(area.x, area.y)
                    .set_char(PROGRESS[self.tick]);
            }
        }
    }

    pub fn result(self) -> probe::Result {
        let mut max_duration = Duration::ZERO;
        for result in self.endpoints.into_iter().filter_map(|(_, r)| r) {
            max_duration = max_duration.max(result?);
        }
        Ok(max_duration)
    }
}
