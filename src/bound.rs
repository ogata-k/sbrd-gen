use serde::Serialize;

// TODO fieldのpubを外す
#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct ValueBound<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<T>,
    #[serde(skip_serializing_if = "use_include_end")]
    pub include_end: bool,
}

fn use_include_end(b: &bool) -> bool {
    *b
}

impl<T: Serialize> ValueBound<T> {
    pub fn get_start(&self) -> &Option<T> {
        &self.start
    }

    pub fn get_end(&self) -> &Option<T> {
        &self.end
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
