// PS1="\[\033[01;32m\]Creditizens_Rusting_life:)\[\033[00m\]\[\033[01;34m\]->\[\033[00m\] "
// from the `main` function the compiler compiles ;)
// for comments : // or /**/
// use keyword `mod` to reference the module imported
/*
mod ev_num;

mod fibo;
mod struct_and_methods;
mod enum_types;
use crate::enum_types::MangaKissaIssue;
*/

mod display_example;

fn main() {
  println!(
    "{:?}",
    display_example::display_is_implemented_to_rectangle()
  );
}

  /*

  // BUUUuuuuut!
  println!("error if you omit this ';'!");
  // last one so implicitly runs and no need `;`
  println!("Shibuya, MangaKissa!");
  // `::` like `.` in Python `<module.func>`
  println!("{}", ev_num::even_number(13));
  // use `:?` when print is not implemented and can;t be formatted
  println!("{:?}", fibo::range_fibo(15));
  println!("{:?}", struct_and_methods::reserve_manga_kissa());
  // use of enum variants
  println!("{:?}", enum_types::issues_at_shibuya_action(
    &MangaKissaIssue::OneIssue("Naruto without any sound!".to_string())
    )
  );
  println!("{:?}", enum_types::issues_at_shibuya_action(
    &MangaKissaIssue::ListOfNumbers([109, 75012, 007].to_vec())
    )
  );
  println!("{:?}", enum_types::issues_at_shibuya_action(
    &MangaKissaIssue::IssueRanking(109)
    )
  );

  */
