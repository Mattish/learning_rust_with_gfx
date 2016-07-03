use vertex;
use entity::Entity;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct EntityRange{
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct EntityPackage{
    pub total: usize,
    pub each: HashMap<String,EntityRange>,
    pub attrs: Vec<vertex::Attr>,
}

pub fn pack(entities:  &mut[Rc<RefCell<Entity>>]) -> EntityPackage{

    let mut package = EntityPackage{
        total: 0,
        each: HashMap::new(),
        attrs: Vec::new()
    };

    let mut ents_map = HashMap::new();

    let mut running_total = 0;
    for i in 0..entities.len() {
            let mut e = entities[i].borrow();

            package.total = package.total + 1;
            let mut models = ents_map.entry(e.model_name.clone()).or_insert(Vec::new());
            models.push(entities[i].clone());
    }

    let mut start_index = 0;
    let mut end_index = 0;
    for key in ents_map.keys(){
        let ents = ents_map.get(key).unwrap();
        let ents_len = ents.len();
        end_index = end_index + ents_len;

        let ent_range = EntityRange{
            start: start_index, 
            end: end_index
        };
        for i in 0..ents_len{
            let e = ents[i].borrow();
            package.attrs.push(vertex::Attr { attr: [e.pos[0], e.pos[1], e.pos[2]],scale:0.85,colour:[1.0,0.0,0.0] });
        }
        package.each.insert(key.clone(),ent_range);
        start_index = end_index;
    }
    
    package
}