#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Calendar;

impl Calendar {
    pub const ISO_8601: Calendar = Calendar;
}