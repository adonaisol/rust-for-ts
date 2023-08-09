fn main1() {
    // map over a list containing 1, 2, 3 and increase each value by 1 
    let x = vec![1, 2, 3];
    let y:Vec<_> = x.iter().map(|x: &i32| { return x + 1; }).collect();
    println!("{:?}", y);

    // read file called lines and print out each line individually
    let file = std::fs::read_to_string("lines").unwrap();
    file
        .lines()
        .enumerate() // .step_by(2)
        .filter(|(i, _)| i % 2 == 0)
        .for_each(|line| println!("{}", line.1));
        // .for_each(|(_, line)| println!("{}", line));

        let file = std::fs::read_to_string("lines").unwrap();
    file
        .lines()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|line| println!("{}", line.1));

    print_color(Color::Blue);
    println!("{}", Color::Yellow.is_green());
    println!("{}", Color::Green.is_green());
    println!("{}", Color::Blue.is_green_parts());
    println!("{}", Color::Red.is_green_parts());

    let mut items: Vec<Item> = vec![];
    append(&mut items);


    println!("{}", exercise([1,2,3].to_vec(), 5));
}

enum Color {
    Green,
    Blue,
    Red,
    Yellow
}

fn print_color(color: Color) {
    //println!("{}", color)
    match color {
        Color::Blue => println!("blue"),
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Yellow => todo!(),
    }
}

impl Color {
    fn is_green(&self) -> bool {
        if matches!(&self, Color::Green) {
            return true
        }

        return false
    }

    fn is_green_parts(&self) -> bool {
        if matches!(&self, Color::Blue | Color::Yellow) {
            return true
        }

        return false
    }
}


struct Custom {
    age: usize,
    name: String
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello Fem".into()));
}

enum Input {
    None,
    Some(usize)
}

fn multiplier(input: Input) -> Input {
    match input {
        Input::Some(input) => Input::Some(input * 5),
        Input::None => Input::None,
    }
}

fn multiplier2(input: Option<usize>) -> Option<usize> {
    return input.map(|x| x * 5); // 
}

fn multiplier3(input: Option<usize>) -> Option<usize> {
    let numbers = input?; // ? operator
    return Some(numbers * 5);
}

fn exercise2(nums: Vec<usize>, index: usize) -> usize {
    let val = nums.get(index);

    match val {
        None => index * 5,
        Some(val) => val * 5,
    }
}

fn exercise(nums: Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index) * 5
}

fn main() {
    let arg = std::env::args().nth(2).expect("No file exists");

    let file = std::fs::read_to_string(&arg);
    file.expect("Unable to read the file to string")
        .lines()
        .for_each(|line| { 
            let num = line.parse::<f64>();
            
            match num.is_ok() {
                true => println!("{}", num.expect("")),
                false => println!("{}", "Line not a number")
            }
             
        });

    print!("{:?}", &arg)
}