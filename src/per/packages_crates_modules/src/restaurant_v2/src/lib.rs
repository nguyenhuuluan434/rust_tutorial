//define package front_of_house
//this file is called  crate root file
mod front_of_house;
mod back_of_house;

use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn order(){
    back_of_house::chef::cook();
}
