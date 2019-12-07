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

    fn get_path_to_you(&self) -> ObjectListBase
    {
        for o in &self.objects
        {
            if (*o).borrow().name == "COM".to_string()
            {
                let mut path = ObjectListBase::new();
                self.get_path_to_inner("YOU", Rc::clone(&o), &mut path);
                return path;
            }
        }

        panic!("Origin not found");
    }

    fn get_path_to_santa(&self) -> ObjectListBase
    {
        for o in &self.objects
        {
            if (*o).borrow().name == "COM".to_string()
            {
                let mut path = ObjectListBase::new();
                self.get_path_to_inner("SAN", Rc::clone(&o), &mut path);
                return path;
            }
        }

        panic!("Origin not found");
    }

    fn get_path_to_inner(&self, destination: &str, start_object: WrappedObject, path: &mut ObjectListBase) -> bool
    {
        if (*start_object).borrow().name == destination.as_ref()
        {
            return true;
        }
        else if (*start_object).borrow().orbiters.len() == 0
        {
            return false;
        }
        else
        {
            for o in &(*start_object).borrow().orbiters
            {
                if self.get_path_to_inner(destination, Rc::clone(&o), path)
                {
                    path.insert(0, Rc::clone(&o));
                    return true;
                }
            }

            return false;
        }
    }
}

pub fn part2()
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

    let path_to_you = objects.get_path_to_you();
    let path_to_santa = objects.get_path_to_santa();

    let mut match_count = 0;

    for (a, b) in path_to_you.iter().zip(path_to_santa.iter())
    {
        if (*a).borrow().name == (*b).borrow().name
        {
            match_count += 1;
        }
        else
        {
            break;
        }
    }

    println!("Required orbital transfers: {}", (path_to_you.len() - 1 - match_count) + (path_to_santa.len() - 1 - match_count));
}
