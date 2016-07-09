use vertex;
use entity::Entity;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use time::PreciseTime;

#[derive(Debug)]
pub struct EntityRange{
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct EntityPackage{
    pub total: usize,
    pub each: HashMap<usize,EntityRange>,
    pub attrs: Vec<vertex::Attr>,
}

pub fn pack(entities:  &mut Vec<Vec<Rc<RefCell<Entity>>>>) -> EntityPackage{

    let mut start = PreciseTime::now();
    let mut finish = PreciseTime::now();

    let mut package_total = 0;
    for ent in 0..entities.len() {
        package_total += entities[ent].len();
    }

    let mut package = EntityPackage{
        total: package_total,
        each: HashMap::new(),
        attrs: Vec::with_capacity(package_total)
    };
    
    let mut start_index = 0;
    let mut end_index = 0;
    for (key,item) in entities.iter().enumerate(){
        let ents_len = item.len();
        end_index += ents_len;

        let ent_range = EntityRange{
            start: start_index, 
            end: end_index
        };
        for ent in item{
            let e = ent.borrow();
            package.attrs.push(vertex::Attr { attr: [e.pos[0], e.pos[1], e.pos[2]],scale:0.85,colour:e.colour });
        }
        package.each.insert(key,ent_range);
        start_index = end_index;
    }
    finish = PreciseTime::now();
    let took = start.to(finish);
    //println!("entity_model_packer::pack took:{}ms.", took.num_milliseconds());
    // println!("EntityPackage:{:?}",package.each);
    package
}