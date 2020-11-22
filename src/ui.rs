use tui::{
    *,
    layout::*,
    backend::Backend,
    widgets::*,
};

pub fn draw<B: Backend>(f: &mut Frame<B>){
    let chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .margin(0)
                            .constraints(
                                [
                                    Constraint::Percentage(30),
                                    Constraint::Percentage(50),
                                ]
                                .as_ref(),
                            )
                            .split(f.size());
    let left_pane =
                        Block::default().title("Todos").borders(Borders::RIGHT).border_type(BorderType::Double);
    let right_pane =
                        Block::default().title("Description").borders(Borders::NONE);
    f.render_widget(left_pane, chunks[0]);
    f.render_widget(right_pane, chunks[1]);
}