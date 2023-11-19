#[cfg(not(feature = "std"))]
use core::cell::RefCell;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, rc::Rc, collections::BTreeMap};

#[cfg(feature = "std")]
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};


#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
struct NodeElement<T>
where
    T: Eq + Ord + Clone,
{
    pub value: T,
    pub predecessor: Vec<Rc<RefCell<NodeElement<T>>>>,
    pub successor: Vec<Rc<RefCell<NodeElement<T>>>>,
    pub collected: bool,
}

fn add_elements_in_array<T>(head: Rc<RefCell<NodeElement<T>>>, array: &mut Vec<T>) -> ()
where
    T: Eq + Ord + Clone,
{
    let mut are_all_predecessors_collected = true;
    head.borrow().predecessor.iter().for_each(|element| {
        if !element.borrow().collected {
            are_all_predecessors_collected = false;
        }
    });
    if !are_all_predecessors_collected {
        return;
    }
    array.push(head.borrow().value.clone());
    head.borrow_mut().collected = true;

    head.borrow().successor.iter().for_each(|element| {
        if !element.borrow().collected {
            add_elements_in_array(element.clone(), array);
        }
    });
}

pub trait MergeAndMaintainRelativeOrder<T>
where
    T: Eq + Ord + Clone,
{
    fn merge_and_maintain_relative_order(&self) -> Vec<T>;
}

impl<T> MergeAndMaintainRelativeOrder<T> for Vec<Vec<T>>
where
    T: Eq + Ord + Clone,
{
    fn merge_and_maintain_relative_order(&self) -> Vec<T> {
        let mut elements = BTreeMap::new();
        // For every element in all rules create NodeElement that will
        // be used to keep track of immediate predecessors and successors
        self.iter().flatten().for_each(|value| {
            elements.insert(
                value,
                Rc::new(RefCell::new(NodeElement {
                    value: value.clone(),
                    predecessor: Vec::new(),
                    successor: Vec::new(),
                    // Used when we form final array of results to indicate
                    // that this node has already be collected in final array
                    collected: false,
                })),
            );
        });

        for lst in self.iter() {
            for (a, b) in lst.iter().zip(lst.iter().skip(1)) {
                let node = elements[a].clone();
                let next_node = elements[b].clone();

                if !node.borrow().successor.contains(&next_node) {
                    node.borrow_mut().successor.push(next_node.clone());
                }
                if !next_node.borrow().predecessor.contains(&node) {
                    next_node.borrow_mut().predecessor.push(node.clone());
                }
            }
        }

        let mut results = Vec::new();

        elements
            .into_values()
            .into_iter()
            .filter(|element| element.borrow().predecessor.len() == 0)
            .for_each(|head| {
                add_elements_in_array(head, &mut results);
            });
        results
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arrays = vec![
            vec!["a", "d", "e"],
            vec!["a", "b", "c", "e"],
            vec!["a", "c", "e"],
            vec!["c", "d", "e"],
        ];

        let result = arrays.merge_and_maintain_relative_order();

        assert_eq!(result, vec!["a", "b", "c", "d", "e"])
    }
}