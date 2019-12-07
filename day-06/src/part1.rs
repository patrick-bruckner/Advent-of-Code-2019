use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::cell::RefCell;

type WrappedObject = Rc<RefCell<Object>>;
type ObjectListBase = Vec<WrappedObject>;

struct Object
{
    name: String,
    orbiters: ObjectListBase
}

impl Object
{
    fn new(name: &String) -> Self
    {
        Self
        {
            name: name.clone(),
            orbiters: Vec::new()
        }
    }
}

struct ObjectList
{
    objects: ObjectListBase
}

impl ObjectList
{
    fn new() -> Self
    {
        Self
        {
            objects: Vec::new()
        }
    }

    fn get_object(&self, name: &String) -> WrappedObject
    {
        for o in &self.objects
        {
            if let Ok(object_inner) = (*o).try_borrow()
            {
                if object_inner.name == *name
                {
                    return Rc::clone(o);
                }
            }
        }

        panic!("Object not found");
    }

    fn add_orbit(&mut self, root_name: &String, orbiter_name: &String)
    {
        if !self.contains(root_name)
        {
            self.objects.push(Rc::new(RefCell::new(Object::new(root_name))));
        }

        if !self.contains(orbiter_name)
        {
            self.objects.push(Rc::new(RefCell::new(Object::new(orbiter_name))));
        }

        (*self.get_object(root_name)).borrow_mut().orbiters.push(self.get_object(orbiter_name))
    }

    fn contains(&self, name: &String) -> bool
    {
        for o in &self.objects
        {
            if (*o).borrow().name == *name
            {
                return true;
            }
        }

        return false;
    }

    fn count_all_orbits(&self) -> usize
    {
        for o in &self.objects
        {
            if (*o).borrow().name == "COM".to_string()
            {
                return self.count_all_orbits_inner(Rc::clone(&o), 0);
            }
        }

        panic!("Origin not found");
    }

    fn count_all_orbits_inner(&self, start_object: WrappedObject, base: usize) -> usize
    {
        let mut count = 0;

        for o in &(*start_object).borrow().orbiters
        {
            count += self.count_all_orbits_inner(Rc::clone(&o), base+1);
        }

        return count + base;
    }
}

pub fn part1()
{
    let file = File::open("input/part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut objects = ObjectList::new();

    for line in reader.lines()
    {
        match line
        {
            Ok(line_safe) => {
                let parts: Vec<String> = line_safe.split(')').map(|s| s.to_string()).collect();
                objects.add_orbit(&parts[0], &parts[1]);
            },
            _ => panic!("Failed to read file")
        }
    }

    println!("Number of direct and indirect orbits: {}", objects.count_all_orbits());
}
