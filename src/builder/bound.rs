use rand::distributions::uniform::{SampleRange, SampleUniform, UniformSampler};
use rand::distributions::{Distribution, Standard};
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct ValueBound<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<T>,
    #[serde(
        skip_serializing_if = "skip_serialize_include_end",
        default = "default_include_end"
    )]
    include_end: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<T>,
}

fn skip_serialize_include_end(b: &bool) -> bool {
    b == &default_include_end()
}

fn default_include_end() -> bool {
    true
}

impl<T> Default for ValueBound<T> {
    fn default() -> Self {
        ValueBound::<T>::new_full()
    }
}

impl<T> ValueBound<T> {
    pub fn new_full() -> Self {
        Self::new(None, None)
    }

    pub fn new(start: Option<T>, end: Option<(bool, T)>) -> Self {
        let (_include_end, _end): (bool, Option<T>) = match end {
            None => (false, None),
            Some((_include_end, _end)) => (_include_end, Some(_end)),
        };

        Self {
            start,
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
            start: start.map(|s| {
                #[allow(clippy::redundant_closure)]
                convert(s)
            }),
            include_end,
            end: end.map(|e| {
                #[allow(clippy::redundant_closure)]
                convert(e)
            }),
        }
    }

    pub fn try_convert_with<F, U, E>(self, mut convert: F) -> Result<ValueBound<U>, E>
    where
        F: FnMut(T) -> Result<U, E>,
    {
        let Self {
            start,
            include_end,
            end,
        } = self;

        let _start = match start {
            None => None,
            Some(s) => Some(convert(s)?),
        };

        let _end = match end {
            None => None,
            Some(e) => Some(convert(e)?),
        };

        Ok(ValueBound {
            start: _start,
            include_end,
            end: _end,
        })
    }

    pub fn without_no_bound_from_other(self, other: ValueBound<T>) -> ValueBound<T> {
        let Self {
            start,
            include_end,
            end,
        } = self;
        let Self {
            start: other_start,
            include_end: other_include_end,
            end: other_end,
        } = other;

        ValueBound {
            start: start.or(other_start),
            include_end: if end.is_some() {
                include_end
            } else {
                other_include_end
            },
            end: end.or(other_end),
        }
    }
}

impl<T: std::cmp::PartialOrd> ValueBound<T> {
    pub fn contains(&self, v: &T) -> bool {
        let Self {
            start,
            include_end,
            end,
        } = &self;

        match (start, end) {
            (Some(s), Some(e)) => s <= v && ((*include_end && v <= e) || (!*include_end && v < e)),
            (Some(s), None) => s <= v,
            (None, Some(e)) => (*include_end && v <= e) || (!*include_end && v < e),
            (None, None) => true,
        }
    }

    pub fn is_empty(&self) -> bool {
        let Self {
            start,
            include_end,
            end,
        } = &self;

        match (start, end) {
            (Some(s), Some(e)) => !((*include_end && s <= e) || (!*include_end && s < e)),
            // 厳密には違う時があるが判定ができないのであきらめる
            (_, _) => false,
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for ValueBound<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            start,
            include_end,
            end,
        } = &self;

        match (start, end, include_end) {
            (Some(s), Some(e), true) => write!(f, "{}..={}", s, e),
            (Some(s), Some(e), false) => write!(f, "{}..{}", s, e),
            (Some(s), None, _) => write!(f, "{}..", s),
            (None, Some(e), true) => write!(f, "..={}", e),
            (None, Some(e), false) => write!(f, "..{}", e),
            (None, None, _) => write!(f, "..",),
        }
    }
}

impl<T: PartialOrd> From<std::ops::RangeFull> for ValueBound<T> {
    fn from(_range: std::ops::RangeFull) -> Self {
        ValueBound::new(None, None)
    }
}

impl<T: PartialOrd> From<std::ops::Range<T>> for ValueBound<T> {
    fn from(range: std::ops::Range<T>) -> Self {
        let std::ops::Range { start, end } = range;
        ValueBound::new(Some(start), Some((false, end)))
    }
}

impl<T: PartialOrd> From<std::ops::RangeFrom<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeFrom<T>) -> Self {
        let std::ops::RangeFrom { start } = range;
        ValueBound::new(Some(start), None)
    }
}

impl<T: PartialOrd> From<std::ops::RangeInclusive<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeInclusive<T>) -> Self {
        let (start, end) = range.into_inner();
        ValueBound::new(Some(start), Some((true, end)))
    }
}

impl<T: PartialOrd> From<std::ops::RangeTo<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeTo<T>) -> Self {
        let std::ops::RangeTo { end } = range;
        ValueBound::new(None, Some((false, end)))
    }
}

impl<T: PartialOrd> From<std::ops::RangeToInclusive<T>> for ValueBound<T> {
    fn from(range: std::ops::RangeToInclusive<T>) -> Self {
        let std::ops::RangeToInclusive { end } = range;
        ValueBound::new(None, Some((true, end)))
    }
}

impl<T: SampleUniform + PartialOrd> SampleRange<T> for ValueBound<T>
where
    Standard: Distribution<T>,
{
    #[inline]
    fn sample_single<R: RngCore + ?Sized>(self, rng: &mut R) -> T {
        if self.start.is_none() || self.end.is_none() {
            loop {
                let s = rng.gen::<T>();
                if self.contains(&s) {
                    return s;
                }
            }
        }

        match (self.start, self.end) {
            (Some(s), Some(e)) => {
                if self.include_end {
                    T::Sampler::sample_single_inclusive(s, e, rng)
                } else {
                    T::Sampler::sample_single(s, e, rng)
                }
            }
            (_, _) => unreachable!(),
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
