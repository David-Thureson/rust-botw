use super::game_record::*;
use super::model::*;

const MAX_SUGGESTIONS: usize = 20;

pub struct CommandSet {
    pub number_targets: bool,
    pub targets: Vec<CommandTarget>
}

pub struct CommandTarget {
    pub model_list: ModelList,
    pub target_type: TargetType,
    pub name: String,
    pub status: String,
    pub events: Vec<GameEvent>,
}

pub enum ModelList {
    None,
    Character,
    Location,
    Quest,
    Item,
}

pub enum TargetType {
    None,
    Armor,
    Bow,
    Character,
    Creature,
    Hearts,
    Location,
    Material,
    Monster,
    Quest,
    Shield,
    Special,
    Stamina,
}

impl CommandSet {
    fn new() -> Self {
        Self {
            number_targets: false,
            targets: vec![],
        }
    }

    pub fn generate(model: &Model, partial_name: &str, number: Option<usize>) -> CommandSet {
        let mut command_set = CommandSet::new();
        let has_number = number.is_some();
        let partial_name = partial_name.to_lowercase();
        match partial_name.as_str() {
            "k" => Self::gen_no_target(model, &mut command_set, GameEventType::KorokSeed, number),
            "c" => Self::gen_no_target(model, &mut command_set, GameEventType::OpenChest, number),
            "we" => Self::gen_no_target(model, &mut command_set, GameEventType::SetWeaponSlots, number),
            "sh" => Self::gen_no_target(model, &mut command_set, GameEventType::SetBowSlots, number),
            "bo" => Self::gen_no_target(model, &mut command_set, GameEventType::SetShieldSlots, number),
            "he" => Self::gen_no_target(model, &mut command_set, GameEventType::SetHearts, number),
            "st" => Self::gen_no_target(model, &mut command_set, GameEventType::SetStamina, number),
            "die" => Self::gen_no_target(model, &mut command_set, GameEventType::LinkDeath, number),
            _ => {
                if !has_number {
                    Self::gen_characters(model, &mut command_set, &partial_name);
                    Self::gen_locations(model, &mut command_set, &partial_name);
                    Self::gen_quests(model, &mut command_set, &partial_name);
                }
                let command_count = command_set.targets.iter().map(|target| target.events.iter()).flatten().count();
                if command_count > MAX_SUGGESTIONS {
                    command_set.number_targets = true;
                }
            },
        }
        command_set
    }

    fn gen_no_target(model: &Model, command_set: &mut CommandSet, event_type: GameEventType, number: Option<usize>) {
        let mut target = CommandTarget::new(ModelList::None, TargetType::None, "", "");
        let current_count = Self::get_current_count_no_target(model, &event_type);
        let number_given = number.unwrap_or(1);
        let number = current_count + number_given;
        match event_type {
            GameEventType::LinkDeath | GameEventType::KorokSeed | GameEventType::OpenChest | GameEventType::SetWeaponSlots
                | GameEventType::SetBowSlots | GameEventType::SetShieldSlots => {
                target.events.push(GameEvent::new(0, event_type.clone(), "", Some(number)));
            },
            GameEventType::SetHearts | GameEventType::SetStamina => {
                target.events.push(GameEvent::new(0, event_type.clone(), "", Some(number)));
                target.events.push(GameEvent::new(0, event_type.clone(), "", Some(number_given)));
            },
            _ => panic!(format!("Unexpected GameEventType variant: {:?}", event_type))
        }
        command_set.targets.push(target);
    }

    fn gen_characters(model: &Model, command_set: &mut CommandSet, partial_name: &str) {
        for (_key, character) in model
            .characters
            .iter()
            .filter(|(key, _character)| key.contains(&partial_name)) {

            let mut target = CommandTarget::new(ModelList::Character, TargetType::Character, &character.name, "");
            if !character.is_mentioned() {
                target.events.push(GameEvent::new(0, GameEventType::MentionCharacter, &character.name, None));
            }
            if !character.is_met() && character.alive {
                target.events.push(GameEvent::new(0, GameEventType::MeetCharacter, &character.name, None));
            }
            if !character.is_met_in_flashback() {
                target.events.push(GameEvent::new(0, GameEventType::MeetCharacterFlashback, &character.name, None));
            }
            command_set.targets.push(target);
        }
    }

    fn gen_locations(model: &Model, command_set: &mut CommandSet, partial_name: &str) {
        for (_key, location) in model
            .locations
            .iter()
            .filter(|(key, _location)| key.contains(&partial_name)) {

            let mut target = CommandTarget::new(ModelList::Location, TargetType::Location, &location.name, "");
            if !location.is_discovered() {
                target.events.push(GameEvent::new(0, GameEventType::DiscoverLocation, &location.name, None));
            }
            if !location.has_dog_treasure() && !location.is_dog_treasure_found() {
                target.events.push(GameEvent::new(0, GameEventType::FindDogTreasure, &location.name, None));
            }
            match location.typ {
                LocationType::Shrine => {
                    if !location.is_started() {
                        target.events.push(GameEvent::new(0, GameEventType::StartShrine, &location.name, None));
                    }
                    if !location.is_completed() {
                        target.events.push(GameEvent::new(0, GameEventType::CompleteShrine, &location.name, None));
                    }
                },
                LocationType::TechLab => {
                    if !location.is_flame_lit() {
                        target.events.push(GameEvent::new(0, GameEventType::LightFlame, &location.name, None));
                    }
                }
                _ => {}
            }
            command_set.targets.push(target);
        }
    }
    
    fn gen_quests(model: &Model, command_set: &mut CommandSet, partial_name: &str) {
        for (_key, quest) in model
            .quests
            .iter()
            .filter(|(key, _quest)| key.contains(&partial_name)) {
    
            let mut target = CommandTarget::new(ModelList::Quest, TargetType::Quest, &quest.name, "");
            if !quest.is_started() {
                target.events.push(GameEvent::new(0, GameEventType::StartQuest, &quest.name, None));
            }
            if !quest.is_completed() {
                target.events.push(GameEvent::new(0, GameEventType::CompleteQuest, &quest.name, None));
            }
            command_set.targets.push(target);
        }
    }

    fn get_current_count_no_target(model: &Model, event_type: &GameEventType) -> usize {
        match event_type {
            GameEventType::LinkDeath => model.deaths,
            GameEventType::KorokSeed => model.korok_seeds,
            GameEventType::OpenChest => model.chests,
            GameEventType::SetWeaponSlots => model.weapon_slots,
            GameEventType::SetBowSlots => model.bow_slots,
            GameEventType::SetShieldSlots => model.shield_slots,
            GameEventType::SetHearts => model.hearts,
            GameEventType::SetStamina => model.stamina,
            _ => panic!(format!("Unexpected GameEventType variant: {:?}", event_type))
        }
    }
}

impl CommandTarget {
    pub fn new(model_list: ModelList, target_type: TargetType, name: &str, status: &str) -> Self {
        Self {
            model_list,
            target_type,
            name: name.to_string(),
            status: status.to_string(),
            events: vec![]
        }
    }
}

/*
AddToCompendium,
CharacterDeath,
CompleteQuest,
CompleteShrine,
DiscoverLocation,
FindDogTreasure,
IdentifyItem,
LightFlame,
MeetCharacter,
MeetCharacterFlashback,
MentionCharacter,
SetArmorLevel,
SetHearts,
SetItemCount,
SetStamina,
StartQuest,
StartShrine,
*/