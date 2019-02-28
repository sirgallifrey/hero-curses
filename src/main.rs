extern crate cursive;

mod characters;
mod play_states;

use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use crate::characters::{Player, PlayerClass, Character};

fn main() {
  let mut siv = Cursive::default();

  // We can quit by pressing `q`
  siv.add_global_callback('q', show_quit_dialog);

  // let player = Player { 
  //   strenght: 10,
  //   constitution: 12,
  //   dexterety: 15,
  //   inteligence: 12,
  //   wisdom: 13,
  //   charisma: 12,
  //   class: PlayerClass::Mage,
  //   level: 1
  //   };

  // // Add a simple view
  // siv.add_layer(TextView::new(
  //   format!("Player hit points {}", player.get_hit_points())
  // ));

  play_states::start_menu(&mut siv);

  // Run the event loop
  siv.run();
}

fn show_quit_dialog(siv: &mut cursive::Cursive) {
  // Creates a dialog with a single "Quit" button
  siv.add_layer(
    // Most views can be configured in a chainable way
    Dialog::around(TextView::new("Do you really want to quit the game?"))
      .title("Quiting")
      .button("Cancel", |s| { s.pop_layer(); })
      .button("Quit", |s| s.quit()),
  );
}
