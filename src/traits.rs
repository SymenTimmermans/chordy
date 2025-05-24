/// A trait to formalize the torsor relationship
pub trait Torsor<Group> {
    /// The action: torsor + group_element → torsor
    fn act(&self, g: Group) -> Self;
    
    /// The difference: torsor - torsor → group_element
    fn difference(&self, other: &Self) -> Group;
}
