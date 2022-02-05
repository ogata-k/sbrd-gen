use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct ValueBound<T> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "min")]
    start: Option<T>,
    #[serde(
        skip_serializing_if = "not_include_end",
        rename = "include_max",
        default = "default_not_include"
    )]
    include_end: bool,
    #[serde(skip_serializing_if = "Option::is_none", rename = "max")]
    end: Option<T>,
}

fn not_include_end(b: &bool) -> bool {
    b == &false
}

fn default_not_include() -> bool {
    false
}

impl<T> Default for ValueBound<T> {
    fn default() -> Self {
        ValueBound::<T>::new(None, None)
    }
}

impl<T> ValueBound<T> {
    pub fn new(start: Option<T>, end: Option<(bool, T)>) -> Self {
        let (_include_end, _end): (bool, Option<T>) = match end {
            None => (false, None),
            Some((_include_end, _end)) => (_include_end, Some(_end)),
        };

        Self {
            start: start,
            end: _end,
            include_end: _include_end,
        }
    }

    pub fn get_start(&self) -> &Option<T> {
        &self.start
    }

    pub fn get_end(&self) -> &Option<T> {
        &self.end
    }

    pub fn is_include_end(&self) -> bool {
        self.include_end
    }

    pub fn convert_into<U>(self) -> ValueBound<U>
    where
        T: Into<U>,
    {
        self.convert_with(|v| v.into())
    }

    pub fn convert_with<F, U>(self, mut convert: F) -> ValueBound<U>
    where
        F: FnMut(T) -> U,
    {
        let Self {
            start,
            include_end,
            end,
        } = self;

        ValueBound {
            start: start.map(|s| convert(s)),
            include_end,
            end: end.map(|e| convert(e)),
        }
    }
}

impl<T: PartialOrd> From<std::ops::Range<T>> for ValueBound<T> {
    fn from(range: std::ops::Range<T>) -> Self {
        let std::ops::Range { start, end } = range;
        ValueBound {
            start: Some(start),
            include_end: false,
            end: Some(end),
        }
    }
}

impl<T: PartialOrd> From<std::ops::RangeFrom<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeFrom<T>) -> Self {
        let std::ops::RangeFrom { start } = range;
        ValueBound {
            start: Some(start),
            include_end: false,
            end: None,
        }
    }
}

impl<T: PartialOrd> From<std::ops::RangeInclusive<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeInclusive<T>) -> Self {
        let (start, end) = range.into_inner();
        ValueBound {
            start: Some(start),
            include_end: true,
            end: Some(end),
        }
    }
}

impl<T: PartialOrd> From<std::ops::RangeTo<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeTo<T>) -> Self {
        let std::ops::RangeTo { end } = range;
        ValueBound {
            start: None,
            include_end: false,
            end: Some(end),
        }
    }
}

impl<T: PartialOrd> From<std::ops::RangeToInclusive<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeToInclusive<T>) -> Self {
        let std::ops::RangeToInclusive { end } = range;
        ValueBound {
            start: None,
            include_end: true,
            end: Some(end),
        }
    }
}
