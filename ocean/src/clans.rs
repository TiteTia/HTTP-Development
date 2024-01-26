#[derive(Debug)]
pub struct Clan {
    pub id: String,
    pub members: Vec<String>,
}

#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    pub clans: Vec<Clan>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem {
            clans: Vec::new(),
        }
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        if let Some(clan) = self.clans.iter().find(|c| c.id == clan_id) {
            clan.members.clone()
        } else {
            Vec::new()
        }
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        if let Some(clan) = self.clans.iter().find(|c| c.id == clan_id) {
            clan.members.len()
        } else {
            0
        }
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        self.clans
            .iter()
            .max_by_key(|clan| clan.members.len())
            .map(|clan| clan.id.clone())
    }

    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        if let Some(clan) = self.clans.iter_mut().find(|c| c.id == clan_id) {
            clan.members.push(crab_name.to_string());
        } else {
            let new_clan = Clan {
                id: clan_id.to_string(),
                members: vec![crab_name.to_string()],
            };
            self.clans.push(new_clan);
        }
    }
}