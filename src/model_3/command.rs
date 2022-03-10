use super::game_record::*;
use super::model::*;

use util::format;

const MAX_SUGGESTIONS: usize = 20;

#[derive(Debug)]
pub struct CommandSet {
    pub partial_name: String,
    pub number: Option<usize>,
    pub include_empty_targets: bool,
    pub force_number_events: bool,
    pub add_to_special_commands: bool,
    pub number_targets: bool,
    pub targets: Vec<CommandTarget>,
}

#[derive(Debug)]
pub struct CommandTarget {
    pub model_list: ModelList,
    pub target_type: TargetType,
    pub name: String,
    pub status: String,
    pub command_number: Option<usize>,
    pub events: Vec<CommandEvent>,
}

#[derive(Debug)]
pub struct CommandEvent {
    pub typ: GameEventType,
    pub number: Option<usize>,
    pub command_number: Option<usize>,
}

#[derive(Debug)]
pub enum ModelList {
    None,
    Character,
    Location,
    Quest,
    Item,
}

#[derive(Debug)]
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
    pub fn new(partial_name: &str, number: Option<usize>) -> Self {
        Self {
            partial_name: partial_name.to_lowercase(),
            number: number,
            include_empty_targets: true,
            force_number_events: false,
            add_to_special_commands: false,
            number_targets: false,
            targets: vec![],
        }
    }

    fn clone_empty(&self, partial_name: &str) -> Self {
        let mut command_set = Self::new(partial_name, self.number);
        command_set.add_to_special_commands = self.add_to_special_commands;
        command_set.force_number_events = self.force_number_events;
        command_set.include_empty_targets = self.include_empty_targets;
        command_set
    }

    pub fn new_gen(model: &Model, partial_name: &str, number: Option<usize>) -> Self {
        let mut command_set = Self::new(partial_name, number);
        command_set.generate(model);
        command_set
    }

    pub fn generate(&mut self, model: &Model) {
        self.number_targets = false;
        self.targets.clear();
        let has_number = self.number.is_some();
        match self.partial_name.as_str() {
            "k" => self.gen_no_target(model, GameEventType::KorokSeed),
            "c" => self.gen_no_target(model, GameEventType::OpenChest),
            "bl" => self.gen_no_target(model, GameEventType::BloodMoon),
            "we" => self.gen_no_target(model, GameEventType::SetWeaponSlots),
            "sh" => self.gen_no_target(model, GameEventType::SetBowSlots),
            "bo" => self.gen_no_target(model, GameEventType::SetShieldSlots),
            "he" => self.gen_no_target(model, GameEventType::SetHearts),
            "st" => self.gen_no_target(model, GameEventType::SetStamina),
            "die" => self.gen_no_target(model, GameEventType::LinkDeath),
            _ => {},
        };
        if self.add_to_special_commands || self.targets.is_empty() {
            if !has_number {
                self.gen_characters(model);
                self.gen_locations(model);
                self.gen_quests(model);
            }
        }
        let command_count = self.command_count();
        if command_count <= MAX_SUGGESTIONS || self.force_number_events {
            self.number_targets = false;
            let mut command_number = 1;
            for command_event in self.targets.iter_mut().map(|target| target.events.iter_mut()).flatten() {
                command_event.command_number = Some(command_number);
                command_number += 1;
            }
        } else {
            self.number_targets = true;
            let mut command_number = 1;
            for command_target in self.targets.iter_mut() {
                command_target.command_number = Some(command_number);
                command_number += 1;
            }
        }
    }

    fn gen_no_target(&mut self, model: &Model, event_type: GameEventType) {
        let mut target = CommandTarget::new(ModelList::None, TargetType::None, "", "");
        let current_count = Self::get_current_count_no_target(model, &event_type);
        let number_given = self.number.unwrap_or(1);
        let number = current_count + number_given;
        match event_type {
            GameEventType::BloodMoon | GameEventType::LinkDeath | GameEventType::KorokSeed | GameEventType::OpenChest | GameEventType::SetBowSlots
                | GameEventType::SetShieldSlots | GameEventType::SetWeaponSlots
                 => {
                target.events.push(CommandEvent::new(event_type, Some(number)));
            },
            GameEventType::SetHearts | GameEventType::SetStamina => {
                target.events.push(CommandEvent::new(event_type.clone(), Some(number)));
                target.events.push(CommandEvent::new(event_type.clone(), Some(number_given)));
            },
            _ => panic!("Unexpected GameEventType variant: {:?}", event_type)
        }
        self.targets.push(target);
    }

    fn gen_characters(&mut self, model: &Model) {
        let partial_name = self.partial_name.clone();
        for (_key, character) in model
            .characters
            .iter()
            .filter(|(key, _character)| key.contains(&partial_name)) {

            let status = character.status_description(model);
            let mut target = CommandTarget::new(ModelList::Character, TargetType::Character, &character.name, &status);
            if !character.is_mentioned() {
                target.events.push(CommandEvent::new(GameEventType::MentionCharacter, None));
            }
            if !character.is_met() && character.alive {
                target.events.push(CommandEvent::new(GameEventType::MeetCharacter, None));
            }
            if !character.is_met_in_flashback() {
                target.events.push(CommandEvent::new(GameEventType::MeetCharacterFlashback, None));
            }
            if self.include_empty_targets || !target.events.is_empty() {
                self.targets.push(target);
            }
        }
    }

    fn gen_locations(&mut self, model: &Model) {
        let partial_name = self.partial_name.clone();
        for (_key, location) in model
            .locations
            .iter()
            .filter(|(key, location)|
                key.contains(&partial_name)
                    || location.challenge.as_ref().map_or(false, |challenge| challenge.to_lowercase().contains(&partial_name))
            ) {

            let status = location.status_description(model);
            let mut target = CommandTarget::new(ModelList::Location, TargetType::Location, &location.name, &status);
            if !location.is_discovered() {
                target.events.push(CommandEvent::new(GameEventType::DiscoverLocation, None));
            }
            if location.has_dog_treasure() && !location.is_dog_treasure_found() {
                target.events.push(CommandEvent::new(GameEventType::FindDogTreasure, None));
            }
            match location.typ {
                LocationType::Shrine => {
                    if !location.is_started() {
                        target.events.push(CommandEvent::new(GameEventType::StartShrine, None));
                    }
                    if !location.is_completed() {
                        target.events.push(CommandEvent::new(GameEventType::CompleteShrine, None));
                    }
                },
                LocationType::TechLab => {
                    if !location.is_flame_lit() {
                        target.events.push(CommandEvent::new(GameEventType::LightFlame, None));
                    }
                }
                _ => {}
            }
            if self.include_empty_targets || !target.events.is_empty() {
                self.targets.push(target);
            }
        }
    }
    
    fn gen_quests(&mut self, model: &Model) {
        let partial_name = self.partial_name.clone();
        for (_key, quest) in model
            .quests
            .iter()
            .filter(|(key, _quest)| key.contains(&partial_name)) {

            let status = quest.status_description(model);
            let mut target = CommandTarget::new(ModelList::Quest, TargetType::Quest, &quest.name, &status);
            if !quest.is_started() {
                target.events.push(CommandEvent::new(GameEventType::StartQuest, None));
            }
            if !quest.is_completed() {
                target.events.push(CommandEvent::new(GameEventType::CompleteQuest, None));
            }
            if self.include_empty_targets || !target.events.is_empty() {
                self.targets.push(target);
            }
        }
    }

    fn get_current_count_no_target(model: &Model, event_type: &GameEventType) -> usize {
        match event_type {
            GameEventType::BloodMoon => model.blood_moons,
            GameEventType::KorokSeed => model.korok_seeds,
            GameEventType::LinkDeath => model.deaths,
            GameEventType::OpenChest => model.chests,
            GameEventType::SetBowSlots => model.bow_slots,
            GameEventType::SetHearts => model.hearts,
            GameEventType::SetShieldSlots => model.shield_slots,
            GameEventType::SetStamina => model.stamina,
            GameEventType::SetWeaponSlots => model.weapon_slots,
            _ => panic!("Unexpected GameEventType variant: {:?}", event_type)
        }
    }

    pub fn command_count(&self) -> usize {
        self.targets.iter().map(|target| target.events.iter()).flatten().count()
    }

    pub fn print_numbered(&self, model: &Model) {
        println!();
        for target in self.targets.iter() {
            let command_number = target.command_number.map_or("".to_string(), |x| format!("{:>2}: ", x));
            let name_with_shrine_challenge = match target.target_type {
                TargetType::Location => model.get_location(&target.name).name_with_shrine_challenge(),
                _ => target.name.to_string(),
            };
            format::println_indent_space(0, &format!("{}{} \"{}\": {}", command_number, target.target_type.variant_to_string(), name_with_shrine_challenge, target.status));
            if !self.number_targets {
                for event in target.events.iter() {
                    let command_number = event.command_number.map_or("".to_string(), |x| format!("{:>2}: ", x));
                    let number = event.number.map_or("".to_string(), |x| format!(": {}", x));
                    format::println_indent_space(1, &format!("{}{}{}", command_number, event.typ.variant_to_string(), number));
                }
            }
        }
    }

    pub fn regen_with_chosen_target(&self, model: &Model, command_number: usize) -> Self {
        assert!(self.number_targets);
        let name = self.targets
            .iter()
            .find(|x| x.command_number.unwrap() == command_number)
            .map(|x| x.name.clone())
            .unwrap();
        let mut command_set = self.clone_empty(&name);
        command_set.force_number_events = true;
        command_set.generate(model);
        assert!(!command_set.number_targets);
        command_set
    }

    pub fn apply_command(&self, model: &mut Model, game_record: &mut GameRecord, time: usize, command_number: usize) {
        assert!(!self.number_targets);
        for target in self.targets.iter() {
            for event in target.events.iter() {
                if event.command_number.unwrap() == command_number {
                    let game_event = GameEvent::new(time, event.typ.clone(), &target.name, event.number);
                    game_record.add_event(model, game_event);
                }
            }
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
            command_number: None,
            events: vec![]
        }
    }
}

impl CommandEvent {
    pub fn new(typ: GameEventType, number: Option<usize>) -> Self {
        Self {
            typ,
            number,
            command_number: None
        }
    }
}

impl TargetType {
    pub fn variant_to_string(&self) -> &str {
        match self {
            TargetType::None => "None",
            TargetType::Armor => "Armor",
            TargetType::Bow => "Bow",
            TargetType::Character => "Character",
            TargetType::Creature => "Creature",
            TargetType::Hearts => "Hearts",
            TargetType::Location => "Location",
            TargetType::Material => "Material",
            TargetType::Monster => "Monster",
            TargetType::Quest => "Quest",
            TargetType::Shield => "Shield",
            TargetType::Special => "Special",
            TargetType::Stamina => "Stamina",
        }
    }
}

pub fn try_suggest_commands() {
    let model = Model::new();
    //bg!(&model.get_location("Mezza Lo Shrine"));

    // CommandSet::generate(&model, "MEZ", None).print_numbered();
    // CommandSet::generate(&model, "Tarrey", None).print_numbered();
    // CommandSet::generate(&model, "bridge", None).print_numbered();
    // CommandSet::generate(&model, "blessing", None).print_numbered(&model);
    CommandSet::new_gen(&model, "test", None).print_numbered(&model);
}