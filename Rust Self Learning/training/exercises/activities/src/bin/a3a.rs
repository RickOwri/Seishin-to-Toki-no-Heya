// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

const X:bool = true;
const Y:bool = false;

fn hello_goodbye(a: bool) {
    if a == true {
        println!("hello");   
    }
    if a == false {
        println!("goodbye");    
    }
}

fn main() {
    hello_goodbye(X);
    hello_goodbye(Y);
}
