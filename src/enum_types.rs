// enums are here to provide us several options `variants` types for a single variable/fn outcome 
//  What is interesting is that we give meaningful names for those types
// we use the keyword `enum`
#[allow(dead_code)]
pub enum MangaKissaIssue {
  ListOfNumbers(Vec<u64>),
  OneIssue(String),
  IssueRanking(u64),
  Unknown,
}

// use a function checking on the custom types we have created using `match` pattern
fn issues_at_shibuya(issue: MangaKissaIssue) -> bool {
  match issue {
    // `_` is used to say that we don't use the variable it is just a placeholder in this case
    MangaKissaIssue::OneIssue(_sentence) => false,
    MangaKissaIssue::ListOfNumbers(_list_nums) => true,
    // here the `_` means that it matches all other possible variants types
    _ => false//{
      //eprintln!("Type Not Valid, Only: ListOfNumbers<Vec<u64>> is Valid. OneIssue<String> permitted...");
      //false
    //}
  }
}


// let make our public function
pub fn issues_at_shibuya_action(issue: MangaKissaIssue) -> bool {

  match issue {
    // we use `ref` to borrow (the reference) issue and be able to use it again without moving the ownership of it 
    MangaKissaIssue::OneIssue(ref _sentence) => issues_at_shibuya(issue),
    MangaKissaIssue::ListOfNumbers(ref _list_nums) => issues_at_shibuya(issue),
    _ => {
      eprintln!("Not Valid!");
      false
    }

}
  
  /*
  // another way to do pattern matching is using `if let` closer to Python way
  // but the `match` way can be seen as a Python `case` style of matching
  // we use `ref` so borrow `issue` with placeholder `sentence` variable
  // we are going to use it therefore this time no `_` to notify compiler that we don't use it after, here we will be using it
  if let MangaKissaIssue::OneIssue(ref sentence) = issue {
    // we use `to_string()` to get a copy of the value
    // so here we don't move ownership of `issue` so that we can reuse it
    issues_at_shibuya(MangaKissaIssue::OneIssue(sentence.to_string()))
  }
  else if let MangaKissaIssue::ListOfNumbers(ref list_nums) = issue {
    // we use `to_vec()` to make a copy of the value behind the reference borrred
    issues_at_shibuya(MangaKissaIssue::ListOfNumbers(list_nums.to_vec()))
  }
  else {
    panic!("Unexpected issue variant!")
  }
  */
}








