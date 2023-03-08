
use std::result;


fn try_sth(first_num:i32, second_num:i32) -> Result<i32, String>{
    return Ok(10);
}
fn main() {


    let content = match try_sth(1, 2){

        Ok(content) => { content },
        Err(er) => {panic!("Wrong input!")}
    };


    print!("{}", content);
    print!("{}", content);
    print!("{}", content);
    print!("{}", content);
                
}
    


