use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;

//FIXIT: get the version from Cargo.toml somehow
pub fn about(siv: &mut Cursive, cb: fn(siv: &mut Cursive)) {
  let text = "Hero Curses \n
Created by Adilson Schmitt Junior <@sirgallifrey> \n
Version: 0.0.0";
  siv.add_layer(
    Dialog::around(TextView::new(text)).button("Back", move |s| {
      s.pop_layer();
      cb(s);
    }),
  );
}
