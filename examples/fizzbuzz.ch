fn fizzbuzz(num: u16) -> none:
	if num % 3 == 0:
		print("fizz")	
	end
	if num % 5 == 0:
		print("buzz")
	end
	if num % 3 != 0 && num % 5 != 0:
		print("{num}")
	end
	print("\n")
end

fn main() -> none:
	auto current = 0	
	while current < 100:
		fizzbuzz(current)
	end
end
