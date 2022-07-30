# comment

fn asdf(w: i8) => none:
	for i to w:
		print(i)		
	end
end

fn compare(n: i8, n2: f32) => none:
	if n2 > 0:
		print("positive")
	else:
		print("negative")
	end
	if n == 69:
		print("nice")
	elif n != 69:
		print("not nice")
	end
end

fn main() => i32:
    auto n1 = 15 # this converts it to type u8
    i8 n2 = 5 # another way of variable declaration
    return 0
end
