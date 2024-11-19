fn main() {
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas my true love sent to me:", days[i]);
        for j in (0..=i).rev() {
            if i != 0 && j == 0 {
                print!("and ");
            }
            println!("{}", gifts[j]);
        }
        println!(); // Print a blank line between verses
    }
}

