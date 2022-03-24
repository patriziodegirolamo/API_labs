/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let trimmed;

    trimmed = code.replace(" ", "");

    if trimmed.chars().count() <= 1 {
        return false;
    }

    let mut sum = 0;
    for (i,car) in trimmed.chars().rev().enumerate(){
        match car {
            '0' ..= '9' => {
                let mut car_dig = car.to_digit(10).unwrap();
                if (i+1)%2 == 0 {
                    car_dig = car_dig *2;
                    if car_dig > 9{
                        car_dig = car_dig-9;
                    }
                }
                sum = sum + car_dig;
            }
            _ => {return false} //no digit
        }
    }

    return sum%10 == 0;
}