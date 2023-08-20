#[derive(Default, Clone)]
pub struct Attributes {
    str: i8,
    dex: i8,
    con: i8,
    int: i8,
    wis: i8,
}

pub struct PlayerCharacter<'a> {
    name: Option<String>,
    race: Option<&'a Race>,
    attributes: Attributes,
    max_hp: i16,
    hp: i16,
    xp: u32,
    level: u8,
    class: Option<&'a Class>,
}

impl<'a> Default for PlayerCharacter<'a> {
    fn default() -> PlayerCharacter<'a> {
        PlayerCharacter::builder().build()
    }
}

impl<'a> PlayerCharacter<'a> {
    pub fn builder() -> PlayerCharacterBuilder<'a> {
        PlayerCharacterBuilder::default()
    }
}

pub struct Race {
    name: String,
    attr_bonus: Attributes,
}

impl Default for Race {
    fn default() -> Race {
        Race {
            name: "Human".to_string(),
            attr_bonus: Attributes::default(),
        }
    }
}

#[derive(Default)]
pub struct Class {
    name: String,
}

pub struct PlayerCharacterBuilder<'a> {
    name: Option<String>,
    race: Option<&'a Race>,
    attributes: Attributes,
    max_hp: i16,
    hp: i16,
    xp: u32,
    level: u8,
    class: Option<&'a Class>,
}

impl<'a> Default for PlayerCharacterBuilder<'a> {
    fn default() -> PlayerCharacterBuilder<'a> {
        PlayerCharacterBuilder {
            name: None,
            race: None,
            attributes: Attributes::default(),
            max_hp: 0,
            hp: 0,
            xp: 0,
            level: 0,
            class: None,
        }
    }
}

impl<'a> PlayerCharacterBuilder<'a> {
    pub fn new() -> PlayerCharacterBuilder<'a> {
        PlayerCharacterBuilder::default()
    }

    pub fn name(mut self, name: &str) -> PlayerCharacterBuilder<'a> {
        self.name = Some(name.to_string());
        self
    }

    pub fn attr(mut self, attributes: Attributes) -> PlayerCharacterBuilder<'a> {
        self.attributes = attributes;
        self
    }

    pub fn hp(mut self, hp: i16) -> PlayerCharacterBuilder<'a> {
        self.max_hp = hp;
        self.hp = hp;
        self
    }

    pub fn xp(mut self, xp: u32) -> PlayerCharacterBuilder<'a> {
        self.xp = xp;
        self
    }

    pub fn race(mut self, race: &'a Race) -> PlayerCharacterBuilder<'a> {
        self.race = Some(race);
        self
    }

    pub fn class(mut self, class: &'a Class) -> PlayerCharacterBuilder<'a> {
        self.class = Some(class);
        self
    }

    pub fn build(self) -> PlayerCharacter<'a> {
        PlayerCharacter {
            name: self.name,
            race: self.race,
            attributes: self.attributes,
            max_hp: self.max_hp,
            hp: self.hp,
            xp: self.xp,
            level: self.level,
            class: self.class,
        }
    }
}
