use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;

use super::about;

pub fn start_menu(siv: &mut cursive::Cursive) {
  let mut select = SelectView::new()
        .h_align(HAlign::Center)
        .autojump();

  let options = vec!(
    String::from("New game"),
    String::from("Continue saved game"),
    String::from("About"),
    String::from("Quit")
  );

  select.add_all_str(options);

  select.set_on_submit(show_next_window);

  siv.add_layer(
      Dialog::around(select.scrollable().fixed_size((20, 10)))
          .title("Hero Curses"),
  );
}

fn show_next_window(siv: &mut Cursive, option: &str) {
    siv.pop_layer();
    
    //FIXIT: i don't want to be matchig strings, but I don't know better for now
    match option {
      "About" =>  about(siv, start_menu),
      "Quit" => siv.quit(),
      _ => start_menu(siv)
    }
}
