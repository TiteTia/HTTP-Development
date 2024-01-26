use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    crabs: Vec<Crab>,
    clan_system: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        Beach {
            crabs: Vec::new(),
            clan_system: ClanSystem::new(),
        }
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crabs.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crabs.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        self.crabs.iter().max_by_key(|crab| crab.speed())
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        self.crabs.iter().filter(|crab| crab.name() == name).collect()
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        let crab_i = self.crabs.get(i).expect("Index out of bounds");
        let crab_j = self.crabs.get(j).expect("Index out of bounds");
        let diet = Diet::random_diet();
        let color = Color::cross(crab_i.color(), crab_j.color());
        let new_crab = Crab::new(name, 1, color, diet);

        self.add_crab(new_crab);
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.clan_system
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        self.clan_system.add_member_to_clan(clan_id, crab_name);
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let clan1_speed = self.calculate_average_speed(id1)?;
        let clan2_speed = self.calculate_average_speed(id2)?;

        match clan1_speed.cmp(&clan2_speed) {
            Ordering::Greater => Ok(Some(id1.to_string())),
            Ordering::Less => Ok(Some(id2.to_string())),
            Ordering::Equal => Ok(None),
        
        }
    }
    
    fn calculate_average_speed(&self, clan_id: &str) -> Result<u32, String> {
        let clan_system = self.get_clan_system(); // Assuming you have a method to get the clan system

        let clan = clan_system.clans.iter().find(|c| c.id == clan_id);

        if let Some(clan) = clan {
            let total_speed: u32 = clan
                .members
                .iter()
                .filter_map(|member_name| {
                    self.get_crab_speed_by_name(member_name).ok()
                })
                .sum();

            let member_count = clan.members.len() as u32;
            if member_count > 0 {
                Ok(total_speed / member_count)
            } else {
                Err("Clan has no members".to_string())
            }
        } else {
            Err("Clan not found".to_string())
        }
    }

    fn get_crab_speed_by_name(&self, crab_name: &str) -> Result<u32, String> {
        for crab in self.crabs() {
            if crab.name() == crab_name {
                return Ok(crab.speed());
            }
        }
        Err(format!("Crab with name '{}' not found", crab_name))
    }
}
