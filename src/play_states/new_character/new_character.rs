use cursive::Cursive;
use cursive::views::{Dialog, TextView};
use super::name::enter_name_view;

pub fn new_character(siv: &mut Cursive) {
  enter_name_view(siv, |siv, name| {
    siv.add_layer(
      Dialog::around(TextView::new(name))
        .padding((1, 1, 1, 0))
        .button("Next", move |s| {
          s.pop_layer();
          
        })
    );
  });
}

