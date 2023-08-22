use crate::model::{Attributes, Class, Race, SystemData};

pub fn mock_db() -> SystemData {
    let mut races = vec![];
    let mut classes = vec![];
    for i in 0..10 {
        races.push(Race {
            name: format!("Race #{}", i),
            attr_bonus: Attributes::new(i),
        });
        classes.push(Class {
            name: format!("Class #{}", i),
        });
    }

    SystemData {
        races: Some(races),
        classes: Some(classes),
    }
}
