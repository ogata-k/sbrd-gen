use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

// TODO fieldのpubを外す
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct ValueBound<T> {
    pub start: Option<T>,
    pub end: Option<T>,
    pub include_end: bool,
}

impl<T: Serialize> Serialize for ValueBound<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut field_count = 0;
        if self.start.is_some() {
            field_count += 1;
        }
        if self.end.is_some() {
            field_count += 2;
        }

        let mut state = serializer.serialize_struct("ValueBounds", field_count)?;

        if let Some(v) = &self.start {
            state.serialize_field("start", v)?;
        }
        if let Some(v) = &self.end {
            state.serialize_field("end", v)?;
            state.serialize_field("include_end", &self.include_end)?;
        }

        state.end()
    }
}

impl<T: Serialize> ValueBound<T> {
    pub fn get_start(&self) -> Option<&T> {
        self.start.as_ref()
    }

    pub fn get_end(&self) -> Option<&T> {
        self.end.as_ref()
    }

    pub fn is_include_end(&self) -> bool {
        self.include_end
    }
}

impl<T: PartialOrd> From<std::ops::Range<T>> for ValueBound<T> {
    fn from(range: std::ops::Range<T>) -> Self {
        let std::ops::Range { start, end } = range;
        ValueBound {
            start: Some(start),
            end: Some(end),
            include_end: false,
        }
    }
}

impl<T: PartialOrd> From<std::ops::RangeFrom<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeFrom<T>) -> Self {
        let std::ops::RangeFrom { start } = range;
        ValueBound {
            start: Some(start),
            end: None,
            include_end: false,
        }
    }
}

impl<T: PartialOrd> From<std::ops::RangeInclusive<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeInclusive<T>) -> Self {
        let (start, end) = range.into_inner();
        ValueBound {
            start: Some(start),
            end: Some(end),
            include_end: true,
        }
    }
}

impl<T: PartialOrd> From<std::ops::RangeTo<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeTo<T>) -> Self {
        let std::ops::RangeTo { end } = range;
        ValueBound {
            start: None,
            end: Some(end),
            include_end: false,
        }
    }
}

impl<T: PartialOrd> From<std::ops::RangeToInclusive<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeToInclusive<T>) -> Self {
        let std::ops::RangeToInclusive { end } = range;
        ValueBound {
            start: None,
            end: Some(end),
            include_end: true,
        }
    }
}
