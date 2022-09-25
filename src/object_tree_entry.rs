use std::{hash::Hash, rc::{Weak, Rc}, collections::{HashSet, HashMap}, cell::RefCell, fmt::Display};

use libesedb::Table;
use termtree::Tree;

use crate::{column_info_mapping::ColumnInfoMapping, esedb_utils::iter_records};
use anyhow::{Result, bail};


pub (crate) struct ObjectTreeEntry {
    name: String,
    id: i32,
    parent: Option<Weak<ObjectTreeEntry>>,
    children: RefCell<HashSet<Rc<ObjectTreeEntry>>>
}

impl Eq for ObjectTreeEntry {
}

impl PartialEq for ObjectTreeEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.id == other.id
    }
}

impl Hash for ObjectTreeEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.id.hash(state);
    }
}

impl Display for ObjectTreeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (id={})", self.name, self.id)
    }
}

impl ObjectTreeEntry {

    pub(crate) fn from<'a>(data_table: &Table<'a>, mapping: &ColumnInfoMapping) -> Result<Rc<ObjectTreeEntry>> {
        Self::populate_object_tree(data_table, mapping)
    }

    pub (crate)fn to_tree(me:&Rc<ObjectTreeEntry>, max_depth: u8) -> Tree<Rc<ObjectTreeEntry>> {
        let tree = Tree::new(Rc::clone(&me));
        if max_depth > 0 {
            let leaves: Vec<Tree<Rc<ObjectTreeEntry>>> = me
                .children.borrow()
                .iter()
                .map(|c| Self::to_tree(c, max_depth - 1)).collect();
            tree.with_leaves(leaves)
        } else {
            tree
        }
    }

    pub(crate) fn parent(&self) -> Option<Rc<ObjectTreeEntry>> {
        self.parent.as_ref().and_then(|p| p.upgrade())
    }

    pub (crate) fn get_by_path(&self, mut path: Vec<&str>) -> Option<Rc<ObjectTreeEntry>> {
        if let Some(next_folder) = path.pop() {
            match self.children.borrow().iter().find(|c| c.name == next_folder) {
                None => None,
                Some(child) => {
                    if path.len() == 0 {
                        Some(Rc::clone(child))
                    } else {
                        Self::get_by_path(&self, path)
                    }
                }
            }
        } else {
            None
        }
    }

    fn populate_object_tree<'a>(data_table: &Table<'a>, mapping: &ColumnInfoMapping) -> Result<Rc<ObjectTreeEntry>> {

        log::info!("populating the object tree");

        let mut downlinks = HashMap::new();
        let mut uplinks = HashMap::new();
        let mut names = HashMap::new();

        for record in iter_records(data_table) {
            let id = match record.ds_record_id(mapping)? {
                Some(v) => v,
                None => bail!("missing record id")
            };

            let parent_id = match record.ds_parent_record_id(mapping)? {
                Some(v) => v,
                None => bail!("missing parent record id")
            };

            let name = match record.ds_object_name2(mapping)? {
                Some(v) => v,
                None => bail!("missing name")
            };

            names.insert(id, name);
            uplinks.insert(id, parent_id);
            downlinks.entry(parent_id).or_insert(HashSet::new()).insert(id);
        }

        log::debug!("found {} entries in the DIT", names.len());
        log::debug!("ordering those elements in a tree structure now");
    
        Self::create_tree_node(0, None, &downlinks, &uplinks, &mut names)
    }

    fn create_tree_node(object_id: i32, parent: Option<&Rc<ObjectTreeEntry>>, downlinks: &HashMap<i32, HashSet<i32>>, uplinks: &HashMap<i32, i32>, names: &mut HashMap<i32, String>) -> Result<Rc<ObjectTreeEntry>> {
        let name = if object_id == 0 {
            String::new()
        } else {
            names.remove(&object_id).expect(&format!("missing name for object with id '{object_id}'"))
        };

        //log::trace!("inserting new object '{}'", name);

        let my_object = Rc::new(ObjectTreeEntry {
            name,
            id: object_id,
            parent: parent.and_then(|p| Some(Rc::downgrade(p))),
            children: RefCell::new(HashSet::new()),
        });

        for (child_id, _) in uplinks.iter().filter(|(_, parent)| **parent == object_id) {
            let child = Self::create_tree_node(
                *child_id, Some(&my_object), downlinks, uplinks, names)?;
            my_object.children.borrow_mut().insert(child);
        }
        Ok(my_object)
    }
}