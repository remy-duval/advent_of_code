use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use commons::eyre::Result;

pub const TITLE: &str = "Day 21: Allergen Assessment";

pub fn run(raw: String) -> Result<()> {
    let recipes = parse(&raw);
    let containers = find_containers(&recipes);
    println!(
        "The number of occurrences of safe ingredients is {}",
        first_part(&recipes, &containers)
    );
    println!(
        "The canonical dangerous ingredient list is:\n{}",
        second_part(containers)
    );

    Ok(())
}

/// One of the listed recipe
struct Recipe<'a> {
    /// The recipe ingredients
    ingredients: Vec<&'a str>,
    /// The allergens that are guaranteed to be found in the ingredients
    allergens: Vec<&'a str>,
}

fn parse(s: &str) -> Vec<Recipe<'_>> {
    s.lines()
        .map(|s| {
            let elements = s.split_once("(contains");
            let ingredients =
                elements.map_or_else(Vec::new, |(list, _)| list.split_whitespace().collect());
            let allergens = elements
                .and_then(|(_, allergens)| allergens.strip_suffix(')'))
                .map_or_else(Vec::new, |list| list.split(',').map(str::trim).collect());

            Recipe {
                ingredients,
                allergens,
            }
        })
        .collect()
}

fn first_part(recipes: &[Recipe<'_>], containers: &HashMap<&str, &str>) -> usize {
    recipes
        .iter()
        .flat_map(|recipe| recipe.ingredients.iter())
        .filter(|&&ing| !containers.contains_key(ing))
        .count()
}

fn second_part(containers: HashMap<&str, &str>) -> String {
    let mut containers = containers.into_iter().collect_vec();
    containers.sort_unstable_by_key(|(_, allergen)| *allergen);
    containers.into_iter().map(|(food, _)| food).join(",")
}

/// Build the food that contain each allergen
fn find_containers<'a>(recipes: &[Recipe<'a>]) -> HashMap<&'a str, &'a str> {
    let mut allergens: HashMap<&str, HashSet<&str>> = HashMap::with_capacity(recipes.len());
    // Fill the mappings between allergens and the food that can contain them
    recipes.iter().for_each(|recipe| {
        let ingredients: HashSet<_> = recipe.ingredients.clone().into_iter().collect();
        recipe.allergens.iter().for_each(|&allergen| {
            allergens
                .entry(allergen)
                .and_modify(|current| {
                    *current = current.intersection(&ingredients).copied().collect();
                })
                .or_insert_with(|| ingredients.clone());
        });
    });

    let mut mappings = HashMap::with_capacity(allergens.len());
    // Since each food can contain at most one allergen
    // And each allergen is contained by exactly one food
    // Attribute each allergen and remove the attributed food from the other potentials
    loop {
        let guaranteed = allergens.iter().find_map(|(key, value)| {
            if value.len() == 1 {
                Some((*key, *value.iter().next().unwrap()))
            } else {
                None
            }
        });

        if let Some((allergen, container)) = guaranteed {
            mappings.insert(container, allergen);
            allergens.retain(|&key, foods| {
                foods.retain(|food| *food != container);
                key != allergen
            })
        } else {
            break;
        }
    }

    mappings
}

#[cfg(test)]
mod tests;
