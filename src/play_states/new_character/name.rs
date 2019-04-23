use cursive::traits::*;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::Cursive;
use std::rc::Rc;

type Callback = fn(s: &mut Cursive, name: String);

pub fn enter_name_view(siv: &mut Cursive, cb: Callback) {
  siv.add_layer(
    Dialog::new()
      .title("New Game")
      .padding((1, 1, 1, 0))
      .content(
        LinearLayout::vertical()
          .child(TextView::new("Enter the name of your hero"))
          .child(DummyView.fixed_width(1))
          .child(
            EditView::new()
              .on_submit(move |s, name| validate_name(s, name.to_owned(), cb))
              .with_id("enter_hero_name")
              .fixed_width(20),
          ),
      )
      .button("Ok", move |s| {
        let name: Rc<String> = s
          .call_on_id("enter_hero_name", |view: &mut EditView| view.get_content())
          .unwrap();
        let name_string = match Rc::try_unwrap(name) {
          Ok(n) => n,
          Err(_) => String::from(""),
        };
        validate_name(s, name_string, cb);
      }),
  );
}

fn validate_name(siv: &mut Cursive, name: String, cb: Callback) {
  if name.is_empty() {
    siv.add_layer(Dialog::info("Please enter a name!"));
  } else {
    let content = format!(
      "Hello {}!\n
Nice to meet you, we are in much need of heroes around here!",
      name
    );
    siv.pop_layer();
    siv.add_layer(
      Dialog::around(TextView::new(content))
        .padding((1, 1, 1, 0))
        .button("Next", move |s| {
          s.pop_layer();
          cb(s, name.to_owned());
        }),
    );
  }
}
