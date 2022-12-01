use anyhow::Context;
use itertools::Itertools;

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


    let top_3 = elf_inventories
        .iter()
        .map(|inventory| inventory.iter().sum::<i32>())
        // Part 2 wants the top 3 amount of calories among the elf inventories
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .collect_vec();

    // Part 1 wants the max calories among the elf inventories
    let elf_inventory_max_calories = top_3.get(0).copied().context("No elf with largest inventory")?;
    let elf_inventory_top3_calories = top_3.iter().sum::<i32>();

    println!("Top 3 elf inventories carrying the largest amount of calories: {:?}", top_3);
    println!("PART 1: The inventory carrying the largest amounf of calories: {}", elf_inventory_max_calories);
    println!("PART 2: Sum of the top 3 inventories: {}", elf_inventory_top3_calories);

    Ok(())
}
