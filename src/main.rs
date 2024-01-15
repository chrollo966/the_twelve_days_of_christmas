fn main() {
    let ordinal = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let lyrics = ["And a partridge in a pear tree.", "Two turtle doves,", "Three French hens,", "Four calling hens,", "Five gold rings,", "Six geese a-laying,",  "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];

    for i in 1..=12 {
        println!("On the {} day of Christmas,", ordinal[i - 1]);
        println!("my true love sent to me,");
        for j in (1..=i).rev() {
            println!("{}", lyrics[j - 1]);
        }
        println!(" ");
    }
}
