const VERSES : u8 = 13;
const WHO : &str = "My true love gave to me";


fn main() {
    
    for each_lyric in 1..VERSES{
        
    call_verse(each_lyric);

}

fn call_verse(each_lyric : u8) {

    let gifts = ["A partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds", "Five gold rings", "Six geese a laying",
                 "Seven swans a swimming", "Eight maids a milking", "Nine ladies dancing", "Ten lords a leaping", "Eleven pipers piping", "Twelve drummers drumming"];

   match each_lyric {
       // There's gotta be a better way.
        1 => println!("On the first day of Christmas {} {}", WHO, gifts[0]),
        2 => println!("On the second day of Christmas {} {} {}", WHO, gifts[1], gifts[0]),
        3 => println!("On the third day of Christmas {} {} {} {}" , WHO, gifts[2], gifts[1], gifts[0]),
        4 => println!("On the fourth day of Christmas {} {} {} {} {}" , WHO, gifts[3], gifts[2], gifts[1], gifts[0]),
        5 => println!("On the fifth day of Christmas {} {} {} {} {} {}" , WHO, gifts[4], gifts[3], gifts[2], gifts[1], gifts[0]),
        6 => println!("On the sixth day of Christmas {} {} {} {} {} {} {}" , WHO, gifts[5], gifts[4], gifts[3], gifts[2], gifts[1], gifts[0]),
        7 => println!("On the seventh day of Christmas {} {} {} {} {} {} {} {}" , WHO, gifts[6], gifts[5], gifts[4], gifts[3], gifts[2], gifts[1], gifts[0]),
        8 => println!("On the eighth day of Christmas {} {} {} {} {} {} {} {} {}" , WHO, gifts[7], gifts[6], gifts[5], gifts[4], gifts[3], gifts[2], gifts[1], gifts[0]),
        9 => println!("On the ninth day of Christmas {} {} {} {} {} {} {} {} {} {}" , WHO, gifts[8],gifts[7], gifts[6], gifts[5], gifts[4], gifts[3], gifts[2], gifts[1], gifts[0]),
        10 => println!("On the tenth day of Christmas {} {} {} {} {} {} {} {} {} {} {}" , WHO, gifts[9], gifts[8],gifts[7], gifts[6], gifts[5], gifts[4], gifts[3], gifts[2], gifts[1], gifts[0]),
        11 => println!("On the eleventh day of Christmas  {} {} {} {} {} {} {} {} {} {} {} {}" , WHO, gifts[10], gifts[9], gifts[8],gifts[7], gifts[6], gifts[5], gifts[4], gifts[3], gifts[2], gifts[1], gifts[0]),
        12 => println!("On the twelfth day of Christmas {} {} {} {} {} {} {} {} {} {} {} {} {}" , WHO, gifts[11], gifts[10], gifts[9], gifts[8],gifts[7], gifts[6], gifts[5], gifts[4], gifts[3], gifts[2], gifts[1], gifts[0]),
        _ => return
    }
}
}