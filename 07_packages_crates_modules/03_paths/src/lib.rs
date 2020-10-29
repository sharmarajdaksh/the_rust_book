mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

mod back_of_house {
  // We can also use pub to designate structs and enums as public, but there
  // are a few extra details. If we use pub before a struct definition, we make
  // the struct public, but the struct’s fields will still be private. We can
  // make each field public or not on a case-by-case basis.
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  // In contrast, if we make an enum public, all of its variants are then
  // public. We only need the pub before the enum keyword
  pub enum Appetizer {
    Soup,
    Salad,
  }

  impl Breakfast {
    // Note that because back_of_house::Breakfast has a private field, the
    // struct needs to provide a public associated function that constructs an
    // instance of Breakfast (we’ve named it summer here). If Breakfast didn’t
    // have such a function, we couldn’t create an instance of Breakfast in
    // eat_at_restaurant because we couldn’t set the value of the private
    // seasonal_fruit field in eat_at_restaurant.

    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }

  fn fix_incorrect_order() {
    cook_order();

    // Super can be used to make paths relative to parent
    super::serve_order();
  }

  fn cook_order() {}
}

// All items (functions, methods, structs, enums, modules, and constants) are
// private by default. Items in a parent module can’t use the private items
// inside child modules, but items in child modules can use the items in their
//ancestor modules.

pub fn eat_at_restaurant() {
  let mut meal = back_of_house::Breakfast::summer("Rye");
  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;

  // Absolute path
  // crate:: can be used since add_to_waitlist is defined in the same
  // crate as eat_at_resta
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path
  front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}
