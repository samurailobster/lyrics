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
        1 => println!("On the first day of Christmas {} {}", WHO, gifts[0]),
        2 => println!("On the second day of Christmas {} {} {}", WHO, gifts[1], gifts[0]),
        3 => println!("On the third day of Christmas {} Three French hens Two turtle doves And a partridge in a pear tree", WHO),
        4 => println!("On the fourth day of Christmas {} Four calling birds Three French hens Two turtle doves And a partridge in a pear tree", WHO),
        5 => println!("On the fifth day of Christmas {} Five gold rings Four calling birds Three French hens Two turtle doves And a partridge in a pear tree", WHO),
        6 => println!("On the sixth day of Christmas {} Six geese a laying Five gold rings Four calling birds Three French hens Two turtle doves And a partridge in a pear tree", WHO),
        7 => println!("On the seventh day of Christmas {} Seven swans a swimming Six geese a laying Five gold rings Four calling birds Three French hens Two turtle doves And a partridge in a pear tree", WHO),
        8 => println!("On the eighth day of Christmas {} Eight maids a milking Seven swans a swimming Six geese a laying Five gold rings Four calling birds Three French hens Two turtle doves And a partridge in a pear tree", WHO),
        9 => println!("On the ninth day of Christmas {} Nine ladies dancing Eight maids a milking Seven swans a swimming Six geese a laying Five gold rings, badam-pam-pam Four calling birds Three French hens Two turtle doves And a partridge in a pear tree", WHO),
        10 => println!("On the tenth day of Christmas {} Ten lords a leaping Nine ladies dancing Eight maids a milking Seven swans a swimming Six geese a laying Five gold rings, badam-pam-pam Four calling birds Three French hens Two turtle doves And a partridge in a pear tree", WHO),
        11 => println!("On the eleventh day of Christmas  {} Eleven pipers piping Ten lords a leaping Nine ladies dancing Eight maids a milking Seven swans a swimming Six geese a laying Five gold rings, badam-pam-pam Four calling birds Three French hens Two turtle doves And a partridge in a pear tree", WHO),
        12 => println!("On the twelfth day of Christmas {} Twelve drummers drumming Eleven pipers piping Ten lords a leaping Nine ladies dancing Eight maids a milking Seven swans a swimming Six geese a laying Five gold rings, badam-pam-pam Four calling birds Three French hens Two turtle doves And a partridge in a pear tree", WHO),
        _ => return
    }
}
}