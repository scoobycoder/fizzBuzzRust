#[test]
fn it_should_fizz_for_3() {

	assert_eq!("fizz", fizz_buzzer(3))
}

#[test]
fn it_should_buzz_for_5() {
	assert_eq!("buzz", fizz_buzzer(5))
}

#[test]
fn it_should_fizzbuzz_for_15() {
	assert_eq!("fizzbuzz", fizz_buzzer(15))
}

#[test]
fn it_should_return_number_for_all_other_numbers() {
	assert_eq!("1", fizz_buzzer(1))
}

fn fizz_buzzer(number: i32) -> String {
	if number % 15 == 0 {
		"fizzbuzz".to_string()		
	} else if number % 3 == 0 {
		"fizz".to_string()	
	} else if number  % 5 == 0 {
		"buzz".to_string()
	} else {
		number.to_string()
	}

}

