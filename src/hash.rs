use std::hash::{Hash, Hasher};

use kuchiki::{NodeDataRef, ElementData, NodeRef, Node};
use servo_css_parser::style::properties::{PropertyDeclaration, PropertyDeclarationId};

pub struct HashableNodeRef {
    pub node: NodeRef,
}

impl HashableNodeRef {
    pub fn new(node: &NodeDataRef<ElementData>) -> Self {
        Self { node: node.as_node().clone() }
    }
}

impl Hash for HashableNodeRef {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        let a: *const Node = &*self.node.0;
        a.hash(state)
    }
}

impl Eq for HashableNodeRef {}
impl PartialEq<HashableNodeRef> for HashableNodeRef {
    fn eq(&self, other: &HashableNodeRef) -> bool {
        let a: *const Node = &*self.node.0;
        let b: *const Node = &*other.node.0;
        a == b
    }
}

pub struct HashablePropertyDeclaration {
    pub property_declaration: PropertyDeclaration,
}

impl HashablePropertyDeclaration {
    pub fn new(property_declaration: PropertyDeclaration) -> Self {
        Self { property_declaration: property_declaration }
    }
}

impl Hash for HashablePropertyDeclaration {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        match self.property_declaration.id() {
            PropertyDeclarationId::Longhand(id) => {
                id.hash(state);
            },
            PropertyDeclarationId::Custom(name) => {
                name.hash(state);
            }
        }
    }
}

impl Eq for HashablePropertyDeclaration {}
impl PartialEq<HashablePropertyDeclaration> for HashablePropertyDeclaration {
    fn eq(&self, other: &HashablePropertyDeclaration) -> bool {
        self.property_declaration.id() == other.property_declaration.id()
    }
}
