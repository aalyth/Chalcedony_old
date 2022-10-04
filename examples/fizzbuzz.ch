# comment

fn fizzbuzz(num: u16) -> none:
        if num % 3 == 0 && num % 5 == 0:
                print("FizzBuzz")
        elif !(num % 3):
                print("Fizz")
        elif !(num % 5):
                print("Buzz")
        else:
                print("{num}")
        end
        print("\n")
end

fn main() -> none:
        auto num = 1
        while num <= 15:
                fizzbuzz(num)
                num += 1
        end
end
