
fn main() -> anyhow::Result<()>{
    let file_name = match std::env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("Warn: No file passed as program argument. Will use the test file");
            String::from("files/day1/test.txt")
        }
    };

    let file_content = std::fs::read_to_string(file_name)?;

    let elf_inventories = file_content
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .split("\n")
                .map(|food_calories| i32::from_str_radix(food_calories, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&elf_inventories);

    Ok(())
}

fn part1(elf_inventories: &[Vec<i32>]){
    let summed_calories = elf_inventories
        .iter()
        .map(|elf_inventory| elf_inventory.iter().sum::<i32>())
        .collect::<Vec<_>>();

    let max_calories = summed_calories
        .iter()
        .max()
        .expect("No max calories found. Is the vector empty ?");

    println!("The elf with the most calories is carrying {} cal.", max_calories);
}
