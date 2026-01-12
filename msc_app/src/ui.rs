use ratatui::{
    Frame, buffer::Buffer, layout::{Alignment, Constraint, Direction, Layout, Position, Rect}, style::{Style, Stylize}, text::{Line}, widgets::{Block, Padding, Paragraph, Scrollbar, Widget}
};

use super::app::*;


impl App{
    pub(super) fn draw(&self, frame : &mut Frame){
        frame.render_widget(self, frame.area());
        let max_size_window = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Max(110),
                Constraint::Min(0)
            ])
            .split(frame.area());
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(14),
                Constraint::Length(8)
            ])
            .split(max_size_window[0]);
        let botton_inner_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(40),
                    Constraint::Percentage(60)
                ])
                .split(main_layout[1]);
        let top_inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ])
            .split(main_layout[0]);
        match self.input_mode {
            InputMode::Normal => {},
             #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                botton_inner_layout[0].x + self.char_index as u16 + 1, 
                botton_inner_layout[0].y + 1
            )),
        }
        // Renderização da scrollbar lateral
        frame.render_stateful_widget(
        Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
            top_inner_layout[1],
            &mut self.scroll_state.clone());
        
    }
}


// Implementando o método principal para ordenar a renderização da interface
// Trait aplicado a uma referência ao nosso app para não alterar seus campos
impl Widget for &App{
    fn render(self, area: Rect, buf: &mut Buffer){
        // Títulos das subseções de tela
        let title_info = Line::from(" Info ".bold());
        let title_timeline = Line::from(" TimeLine ".bold());
        let title_playlist = Line::from(" Playlist ".bold());
        let title_controls = Line::from(" Controls ".bold());
        let title_command_input = Line::from(" Command Input ".bold());

        // Setando o estilo de cada bloco de informação
        let info_block = Block::bordered()
            .title(title_info.left_aligned())
            .border_type(ratatui::widgets::BorderType::Double);
        let timeline_block = Block::bordered()
            .title(title_timeline.left_aligned())
            .border_type(ratatui::widgets::BorderType::Double);
        let playlist_block = Block::bordered()
            .title(title_playlist.left_aligned())
            .border_type(ratatui::widgets::BorderType::Double);
        let controls_block = Block::bordered()
            .title(title_controls.left_aligned())
            .border_type(ratatui::widgets::BorderType::Double);
        let command_input_block = Block::bordered()
            .title(title_command_input.left_aligned())
            .border_type(ratatui::widgets::BorderType::Double);

        // Layout de divisão da tela
        // Divisão principal da tela
        let max_size_window = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Max(111),
                Constraint::Min(0)
            ])
            .split(area);
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(14),
                Constraint::Length(8)
            ])
            .split(max_size_window[0]);
        // Divisião principal da parte superior
        let top_inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ])
            .split(main_layout[0]);
        // Divisão principal da parte inferior
        let botton_inner_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(40),
                    Constraint::Percentage(60)
                ])
                .split(main_layout[1]);
        // Divisões internas dos layouts botton_inner_layout e top_inner_layout
        // Divisão interna da parte superior : top_inner_layout
        let div_left_side_top_inner_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Percentage(65),
                        Constraint::Percentage(35)
                    ])
                    .split(top_inner_layout[0]);

        // Texto informando os comandos
        Paragraph::new("[TAB] Focus  [Space] Play/Pause  [Ret] Load  [n/p] Skip  [Arrows] Scroll/Vol  [q] Quit".bold())
                        .block(controls_block.padding(Padding::new(0, 0, botton_inner_layout[1].height/3, 0)))
                        .alignment(Alignment::Center)
                        .render(botton_inner_layout[1], buf);
        // Local para renderizar as listas de músicas 
        // Correspondendo a sessão de playlist
        let mut playlist_vec_clone = self.play_list_vec.clone();
        // Transforma no tipo line e adiciona o > caso seja a música atual
        let formated_playlist_vec : Vec<_> = playlist_vec_clone
        .iter_mut()
        .enumerate()
        .map(|(index, information)|{
            let index_str = if index != self.current_file_msc_index {String::from("  ")} else {String::from("> ")};
            information.insert_str(0, &index_str);
            information.clone().bold().into()
        })
        .collect();

        Paragraph::new(formated_playlist_vec)
                        .style(match self.scroll_state_window {
                            ScrollState::Activated => Style::default().fg(ratatui::style::Color::LightBlue),
                            ScrollState::Deactivated => Style::default(),
                        })
                        .block(playlist_block.padding(Padding::new(1, 0, top_inner_layout[1].height/16, 0)))
                        .alignment(Alignment::Left)
                        .scroll((self.vertical_scroll as u16, 0))
                        .render(top_inner_layout[1], buf);
                        
        
        // Local correspondente ao input da playlist
        Paragraph::new(self.user_playlist_input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(ratatui::style::Color::LightBlue),
            })
            .block(command_input_block)
            .render(botton_inner_layout[0], buf);

        let mut information_text = vec![
            String::from("S o n g :  "),
            String::from("A r t i s t :  "),
            String::from("A l b u m :  "),
        ];
        // Adiciona as informações da música atual
        if !self.current_mcs_infor.is_empty() {
            for (index, information) in self.current_mcs_infor.clone().iter().enumerate(){
                if index < 3 {
                    information_text[index].push_str(information);
                }
            }
        }
        let lines_vec : Vec<_> = information_text
        .iter_mut()
        .map(|text|text.clone().bold().into())
        .collect();
        Paragraph::new(lines_vec)
            .block(info_block.clone().padding(Padding::new(5, 0, div_left_side_top_inner_layout[0].height / 6, 0)))
            .render(div_left_side_top_inner_layout[0], buf);
        // Espaço de exibição de estadp
        let mut state_text = match self.msc_state {
            MscState::Paused => String::from("[ STATUS : PAUSED ]"), 
            MscState::Playing => String::from("[ STATUS : PLAYING ]"),
        };
        let valor_formatado = format!("{:.2}", self.msc_sound_value * 100.0);
        state_text.push_str(&format!("    [ VOL : {valor_formatado}% ]")[..]);
        Paragraph::new(state_text.bold())
            .block(info_block.padding(Padding::new(4, 0, (div_left_side_top_inner_layout[0].height as f64 / 1.8 ) as u16 , 0)))
            .render(div_left_side_top_inner_layout[0], buf);

        // Desenhando o tempo total da música
        let total_seconds_current_msc = (self.msc_time / 1_000) % 60;
        let total_min_current_msc = self.msc_time / 60_000 ;
        let mut total_seconds_current_msc_as_st = total_seconds_current_msc.to_string();
        let mut total_min_current_msc_as_st = total_min_current_msc.to_string();
        if total_min_current_msc < 10  {
            total_min_current_msc_as_st.insert(0, '0');
        }
        if total_seconds_current_msc < 10 {
            total_seconds_current_msc_as_st.insert(0, '0');
        }
        let current_total_time_as_st = format!(" {}:{}", total_min_current_msc_as_st, total_seconds_current_msc_as_st);
        // Desenhando o tempo decorrido da música
        let seconds_passed = (self.current_msc_time / 1_000) % 60;
        let min_passed = self.current_msc_time / 60_000;
        let mut seconds_passed_as_st = seconds_passed.to_string();
        let mut min_passed_as_st = min_passed.to_string();
        if seconds_passed < 10 {
            seconds_passed_as_st.insert(0, '0');
        }
        if min_passed < 10 {
            min_passed_as_st.insert(0, '0');
        }
        let passed_time = format!("{}:{} ", min_passed_as_st, seconds_passed_as_st);
        let mut timeline = String::new();
        timeline.push_str(&passed_time);
        for i in 0 .. 40{
            if (((self.current_msc_time as f64 / self.msc_time as f64) * 100.0) / 2.5 ) as i32  == i || i == 40{
                timeline.push('o');
            }else{
                timeline.push('-');
            }
        }
        // Area de timeline, tempo de música
        timeline.push_str(&current_total_time_as_st);
        Paragraph::new(timeline.bold())
            .block(timeline_block.padding(Padding::new(1, 0, div_left_side_top_inner_layout[1].height/3, 0)))
            .centered()
            .render(div_left_side_top_inner_layout[1], buf);
    }
}




