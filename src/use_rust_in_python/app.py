# now we are going to make a simple python script
# to call those functions

# we need to import our `rust_extension`
import my_rust_extension

def main_cake(cake_parts):
  total_cake_for_creditizens = my_rust_extension.double_cake_parts(cake_parts)
  print(f"Initial cake portion for human not in metaverse: {cake_parts}.\nCreditizens will get double portion of cake: {total_cake_for_creditizens}")


if __name__ == "__main__":
  # we expect 4
  main_cake(2)
