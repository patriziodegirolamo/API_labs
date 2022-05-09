use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug)]

pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let totminutes;
        if hours<0 { totminutes = ((24 + hours%24)%24  )*60 +minutes }
        else{ totminutes = hours*60+minutes;}


        let new_tot_minutes;
        if totminutes <0 { new_tot_minutes= ((60*24) + totminutes%(60*24))% (60*24)}
        else {new_tot_minutes =  (totminutes)% (60*24);}


        let new_hours = new_tot_minutes / 60;


        let new_minutes = new_tot_minutes - (new_hours *60);


        Self{hours:new_hours, minutes:new_minutes}

    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut totminutes = self.hours*60+self.minutes;

        totminutes = (totminutes+minutes)% (60*24);

        let new_tot_minutes;
        if totminutes <0 { new_tot_minutes= ((60*24) + totminutes%(60*24))% (60*24)}
        else {new_tot_minutes =  (totminutes)% (60*24);}

        let new_hours = new_tot_minutes / 60;

        let new_minutes = new_tot_minutes - (new_hours *60);

        Self{hours:new_hours, minutes:new_minutes}
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        if self.hours<10 && self.minutes>9 {
            return write!(f, "0{}:{}", self.hours, self.minutes)
        }
        else if self.hours > 9 && self.minutes <10 {
            return write!(f, "{}:0{}", self.hours, self.minutes) }
        else if self.hours < 10 && self.minutes <10{
            return write!(f, "0{}:0{}", self.hours, self.minutes) }
        return write!(f, "{}:{}", self.hours, self.minutes) }

         }

impl PartialEq<Self> for Clock {
    fn eq(&self, other: &Self) -> bool {
        return self.hours==other.hours && self.minutes==other.minutes
    }
}

impl Add<Self> for Clock{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let totmin = rhs.hours*60 + rhs.minutes;
        return self.add_minutes(totmin);
    }
}
impl Add<i32> for Clock{
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        return self.add_minutes(rhs);
    }
}

impl Sub<Self> for Clock{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let totmin = rhs.hours*60 - rhs.minutes;
        return self.add_minutes(totmin);
    }

}
impl Sub<i32> for Clock{
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        return self.add_minutes(rhs);
    }

}
