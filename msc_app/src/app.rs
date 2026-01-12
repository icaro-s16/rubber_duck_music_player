use std::{ fs::File, io::{self, Seek, SeekFrom}};
use msc_player::{audio, library};
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}};
use ratatui::widgets::ScrollbarState;
use ratatui::DefaultTerminal;
use chrono;



#[derive(Debug,Clone, Copy)]
pub(crate) enum InputMode {
    Normal,
    Editing
}

#[derive(Debug)]
pub(crate) enum MscState{
    Playing,
    Paused
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ScrollState{
    Activated,
    Deactivated
}


pub struct App{
    // Input correspondente a pasta da playlist
    pub(super) user_playlist_input : String,
    // Posição do cursor na área de input
    pub(super) char_index : usize,
    // Modo de input
    pub(super) input_mode : InputMode,
    // PlayList Atual do usuário
    pub(super) play_list_vec : Vec<String>,
    // Vetor de arquivos das músicas
    pub(super) file_msc_vec : Vec<File>,
    // index da musica atual de acordo com o vetor
    pub(super) current_file_msc_index : usize,
    pub(super) current_msc : Option<File>,
    pub(super) current_mcs_infor : Vec<String>,
    // Estado atual da música
    pub(super) msc_state : MscState,
    pub(super) msc_sound_value : f32,
    // Estados relacionados ao scroll da lista de música
    pub(super) scroll_state_window : ScrollState,
    pub(super) scroll_state : ScrollbarState,
    pub(super) vertical_scroll : usize,
    // Variavel para armazena o sink da musica
    pub(super) msc_audio_device : audio::Audio,
    // Indicador se a musica foi carregada
    pub(super) msc_is_loaded : bool,
    // Tempo atual da música
    pub(super) current_msc_time :  u128,
    pub(super) msc_time : u128,
    // Indicador de saida do programa
    pub(super) exit : bool
}

impl Default for App{
    fn default() -> Self {
        Self{
            user_playlist_input : String::new(),
            char_index : 0,
            input_mode : InputMode::Normal,
            play_list_vec : Vec::new(),
            file_msc_vec : Vec::new(),
            current_mcs_infor : Vec::new(),
            current_file_msc_index : 0,
            current_msc : None,
            msc_state : MscState::Paused,
            msc_sound_value : 1.0,
            scroll_state_window : ScrollState::Deactivated,
            scroll_state : ScrollbarState::new(0),
            vertical_scroll : 1,
            msc_audio_device : audio::Audio::build(),
            msc_is_loaded : false,
            current_msc_time : 0,
            msc_time: 0,
            exit : false,
        }
    }
}

impl App{
    pub fn run(&mut self, terminal : &mut DefaultTerminal) -> io::Result<()>{
        let mut current_system_time = chrono::Local::now().timestamp_millis();
        while !self.exit{
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_eventes()?;
            self.update_msc_time(&mut current_system_time);
        }
        Ok(())
    }
    // Lidar com input unitário do usuário
    pub(super) fn handle_eventes(&mut self) -> io::Result<()>{
        if event::poll(core::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                },
                _ => {}
            }
        }
        Ok(())
    }
    // Lidando com os eventos do teclado
    pub(super) fn handle_key_event(&mut self, key_event : KeyEvent){
        match (self.input_mode.clone(), self.scroll_state_window.clone()){
            (InputMode::Normal, ScrollState::Deactivated)=> match key_event.code{
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char(' ') => {
                    self.toggle_play_pause();
                },
                KeyCode::Up => {
                    self.change_volume('+');
                },
                KeyCode::Down => {
                    self.change_volume('-');
                }
                KeyCode::Char('p') => {
                    self.load_new_msc('+');
                },
                KeyCode::Char('n') => {
                    self.load_new_msc('-');
                }
                KeyCode::Tab => self.input_mode = InputMode::Editing,
                _ => {}
            },
            (InputMode::Editing, ScrollState::Deactivated) if key_event.kind == KeyEventKind::Press => match key_event.code{
                KeyCode::Enter => self.submit_message(),
                KeyCode::Char(to_insert) => self.enter_char(to_insert),
                KeyCode::Backspace => self.delete_char(),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),
                KeyCode::Tab => { self.input_mode = InputMode::Normal; self.scroll_state_window = ScrollState::Activated},
                _ => {}
            }
            (InputMode::Normal, ScrollState::Activated) => match key_event.code{
                KeyCode::Up => {
                    self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
                    self.scroll_state = self.scroll_state.position(self.vertical_scroll);
                },
                KeyCode::Down => {
                    self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                    self.scroll_state = self.scroll_state.position(self.vertical_scroll);
                },
                KeyCode::Tab => self.scroll_state_window = ScrollState::Deactivated,
                _ => {}
            }
            _ => {}
        }
    }
    pub(super)  fn exit(&mut self){
        self.exit = true;
    }

    // Funções para receber uma escrita do usuário

    fn move_cursor_left(&mut self){
        let cursor_moved_left = self.char_index.saturating_sub(1);
        self.char_index = self.clamp_cursor(cursor_moved_left);
    }
    fn move_cursor_right(&mut self){
        let cursor_moved_right = self.char_index.saturating_add(1);
        self.char_index = self.clamp_cursor(cursor_moved_right);
    }
    // Adiciona o char de acordo com o byte da última posição da palavra
    fn enter_char(&mut self, new_char : char){
        let index = self.byte_index();
        self.user_playlist_input.insert(index, new_char);
        self.move_cursor_right();
    }
    // Retorna o valor em bytes do char em que o cursor está atualmente
    fn byte_index(&self) -> usize{
        self.user_playlist_input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.char_index)
            .unwrap_or(self.user_playlist_input.len())
    }
    fn delete_char(&mut self){
        let is_not_cursor_leftmost = self.char_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.char_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.user_playlist_input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.user_playlist_input.chars().skip(current_index);
            self.user_playlist_input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    fn clamp_cursor(&self, new_cursor_pos : usize) -> usize{
        new_cursor_pos.clamp(0, self.user_playlist_input.chars().count())
    }
    fn submit_message(&mut self){
        self.msc_audio_device.sink.stop();
        self.current_msc = None;
        self.msc_time = 0;
        self.current_msc_time = 0;
        // Carrega um vetor com os nomes das músicas no presentes no arquivo
        self.play_list_vec = library::playlist_msc_names(&self.user_playlist_input[..]);
        // Carrega um vetor de tipos File, correspondente às músicas no arquivo
        self.file_msc_vec = library::msc_files_list(&self.user_playlist_input[..]).unwrap_or(Vec::new());
        // Reseta o valor do index da música atual
        self.current_file_msc_index = 0;
        self.restart_the_current_msc();
        // Ajusta o scroll de acordo com o tamnho da playlist
        self.scroll_state = self.scroll_state.content_length(self.play_list_vec.len());
    }
    // Função para mudar de música na playlist
    // pode tanto avançar quanto voltar uma música
    // Ajusta as variáveis da struct principal
    fn load_new_msc(&mut self, operation : char){
        
        if (operation == '+' && self.current_file_msc_index != self.play_list_vec.len() - 1) || 
        (operation == '-' && self.current_file_msc_index != 0)
        {
            self.msc_audio_device.sink.pause();
            self.msc_audio_device.sink.clear();
            self.msc_state = MscState::Paused;
            self.current_file_msc_index = if operation == '-' {self.current_file_msc_index - 1} else {self.current_file_msc_index + 1};
            self.restart_the_current_msc();
        }
    }
    fn change_volume(&mut self, operation : char){
        match operation{
            '-' => {
                self.msc_sound_value = if self.msc_sound_value >= 0.001 {self.msc_sound_value - 0.01} else {self.msc_sound_value};
                self.msc_audio_device.sink.set_volume(self.msc_sound_value);
            },
            '+' => {
                self.msc_sound_value = if self.msc_sound_value < 1.0 {self.msc_sound_value + 0.01} else {self.msc_sound_value};
                self.msc_audio_device.sink.set_volume(self.msc_sound_value);
            }
            _ => {}
        }
    }
    fn toggle_play_pause(&mut self){
        match self.msc_state {
            MscState::Paused => {
                // Inicia a música no dispositivo sink
                if !self.msc_is_loaded && self.current_msc.is_some(){
                    if let Ok(clonned_file) = self.current_msc.as_ref().unwrap().try_clone(){
                        self.msc_audio_device.start_msc(clonned_file).expect("erro");
                    }
                    self.msc_is_loaded = true;
                }
                self.msc_audio_device.sink.play();
                self.msc_state = MscState::Playing;
            },
            MscState::Playing => {
                self.msc_audio_device.sink.pause();
                self.msc_state = MscState::Paused;
            }
        }
    }

    fn restart_the_current_msc(&mut self){
        if let Some(file) = self.file_msc_vec.get(self.current_file_msc_index){
            if let (Ok(file_1), Ok(mut file_2)) = ((*file).try_clone(), (*file).try_clone()){
                self.current_mcs_infor = library::get_msc_information(file_1);
                if file_2.seek(SeekFrom::Start(0)).is_ok(){
                    self.msc_time = library::get_msc_duration(file_2);
                    self.current_msc_time = 0;
                }
            }
            if let Ok(ref mut clonned_file) = (*file).try_clone(){
                if clonned_file.seek(SeekFrom::Start(0)).is_ok(){
                    self.current_msc = if let Ok(file) = clonned_file.try_clone() {Some(file)} else {None};
                    if let Ok(clonned_file_msc) = clonned_file.try_clone(){
                        _ = self.msc_audio_device.start_msc(clonned_file_msc);
                        self.msc_audio_device.sink.set_volume(self.msc_sound_value);
                        self.msc_audio_device.sink.pause();
                        self.msc_is_loaded = true;
                    }
                }
            }
        }else{
            self.current_msc = None;   
            self.current_mcs_infor = Vec::new();
        }
    }

    fn update_msc_time(&mut self, current_system_time : &mut i64) {
        match self.msc_state {
            MscState::Playing if self.msc_time > self.current_msc_time=> {
                let after_loop_system_time = chrono::Local::now().timestamp_millis();
                self.current_msc_time += (after_loop_system_time - *current_system_time) as u128;
                *current_system_time = after_loop_system_time;
            },
            MscState::Paused => {
                *current_system_time = chrono::Local::now().timestamp_millis();
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn duratio_test(){
        let mut d = App::default();
        d.user_playlist_input = "/home/icaro_s/hybridtheory2000/2000 - Hybrid Theory (Special Edition)".to_string();
        d.submit_message();
        println!("{}", d.msc_time);
        println!("{}", d.current_msc_time);
    }

}
