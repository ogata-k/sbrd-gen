use crate::builder::{ChildGeneratorBuilder, GeneratorBuilder, Weight};
use crate::error::{BuildError, GenerateError};
use crate::eval::Evaluator;
use crate::file::open_sbrd_file;
use crate::value::{DataValue, DataValueMap};
use either::Either;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Core of random data generator
pub trait Randomizer: 'static + Rng {}
impl<R: 'static + Rng> Randomizer for R {}

/// Base trait for a generator
pub trait GeneratorBase<R: Randomizer + ?Sized> {
    /// Create generator from builder
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
    where
        Self: Sized;

    /// Can generate null flag
    fn is_nullable(&self) -> bool;

    /// Cannot generate null flag
    fn is_required(&self) -> bool {
        !self.is_nullable()
    }

    /// Generate dummy data considering nullable
    fn generate(
        &self,
        rng: &mut R,
        context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        if self.is_required() {
            self.generate_without_null(rng, context)
        } else {
            if rng.gen_bool(0.1) {
                return Ok(DataValue::Null);
            }

            self.generate_without_null(rng, context)
        }
    }

    /// Generate dummy data not considering nullable
    fn generate_without_null(
        &self,
        rng: &mut R,
        context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError>;
}

/// Child generator with condition
pub type CasedChild<R> = (Option<String>, Box<dyn GeneratorBase<R>>);
/// Base trait for a generator use child generators with condition.
///
/// If a child generator's condition is [`Option::Some`], then evaluate it's condition.
/// If a child generator's condition is [`Option::None`], then default condition. Default condition always must exist.
///
/// [`Option::Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
/// [`Option::None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
pub trait CasedChildGeneratorBase<R: Randomizer + ?Sized> {
    /// Build selectable child generator list
    fn build_selectable(
        children: Option<Vec<ChildGeneratorBuilder>>,
    ) -> Result<Vec<CasedChild<R>>, BuildError> {
        match children {
            None => Err(BuildError::NotExistValueOf("children".to_string())),
            Some(children) => {
                let mut _children: Vec<(Option<String>, Box<dyn GeneratorBase<R>>)> = Vec::new();
                let mut has_default_case = false;
                for child_builder in children.into_iter() {
                    let ChildGeneratorBuilder {
                        condition, builder, ..
                    } = child_builder;
                    has_default_case = has_default_case || condition.is_none();
                    _children.push((condition, builder.build()?));
                }

                if _children.is_empty() {
                    return Err(BuildError::EmptySelectableChildren);
                }

                if !has_default_case {
                    return Err(BuildError::NotExistDefaultCase);
                }

                Ok(_children)
            }
        }
    }

    /// Get selectable child generators
    fn get_children(&self) -> &[CasedChild<R>];

    /// Generate dummy data considering nullable from picked out child generator
    fn generate_from_children(
        &self,
        rng: &mut R,
        context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        for (condition, generator) in self.get_children().iter() {
            return match condition {
                None => generator.generate(rng, context),
                Some(_condition) => {
                    let evaluator = Evaluator::new(_condition, context);
                    let is_satisfy = evaluator.eval_bool().map_err(|e| {
                        GenerateError::FailEval(
                            e,
                            _condition.clone(),
                            context
                                .iter()
                                .map(|(k, v)| (k.to_string(), v.clone()))
                                .collect::<DataValueMap<String>>(),
                        )
                    })?;
                    if !is_satisfy {
                        continue;
                    }

                    generator.generate(rng, context)
                }
            };
        }

        Err(GenerateError::FailGenerate(
            "No match case condition".to_string(),
        ))
    }
}

/// Child generator with weight
pub type WeightedChild<R> = (Weight, Box<dyn GeneratorBase<R>>);
/// Base trait for a generator use child generator with weight
pub trait WeightedChildGeneratorBase<R: Randomizer + ?Sized> {
    /// Build selectable child generator with weight
    fn build_selectable(
        children: Option<Vec<ChildGeneratorBuilder>>,
    ) -> Result<Vec<WeightedChild<R>>, BuildError> {
        match children {
            None => Err(BuildError::NotExistValueOf("children".to_string())),
            Some(children) => {
                let mut select_values = Vec::new();
                for child_builder in children.into_iter() {
                    let ChildGeneratorBuilder {
                        weight, builder, ..
                    } = child_builder;
                    select_values.push((weight.unwrap_or(1), builder.build()?));
                }

                if select_values.is_empty() {
                    return Err(BuildError::EmptySelectableChildren);
                }

                if select_values.iter().fold(0, |acc, item| acc + item.0) == 0 {
                    return Err(BuildError::AllWeightsZero);
                }

                Ok(select_values)
            }
        }
    }

    /// Get selectable list
    fn get_selectable(&self) -> &[WeightedChild<R>];

    /// Generate value from picked out child generator
    fn generate_from_children(
        &self,
        rng: &mut R,
        context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        let item = self
            .get_selectable()
            .choose_weighted(rng, |item| item.0)
            .map_err(|err| GenerateError::FailGenerate(err.to_string()))?;

        item.1.generate(rng, context)
    }
}

/// Base trait for a generator from input values
pub trait ValueGeneratorBase<R: Randomizer + ?Sized, T> {
    /// Function of parser the input value
    fn parse(input: &str) -> Result<T, BuildError>;

    /// Build selectable value
    fn build_selectable(
        chars: Option<String>,
        values: Option<Vec<DataValue>>,
        filepath: Option<PathBuf>,
    ) -> Result<Vec<T>, BuildError> {
        let mut selectable_values: Vec<T> = Vec::new();
        if let Some(chars) = chars {
            for c in chars.chars() {
                selectable_values.push(Self::parse(&c.to_string())?);
            }
        }

        if let Some(values) = values {
            for value in values.into_iter() {
                selectable_values.push(Self::parse(&value.to_parse_string())?);
            }
        }

        if let Some(filepath) = filepath {
            let file = open_sbrd_file(filepath.as_path())
                .map_err(|e| BuildError::FileError(e, filepath.clone()))?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.map_err(|e| BuildError::FileError(e, filepath.clone()))?;
                selectable_values.push(Self::parse(&line)?);
            }
        }

        if selectable_values.is_empty() {
            return Err(BuildError::EmptySelectableChildren);
        }

        Ok(selectable_values)
    }
}

/// Value as String or Child for a generator.
///
/// Usually, this structure is used by a generator which generate value as string,
/// because input value's type is unknown and a type of the generated value by child generator is also unknown.
pub type ValueOrChild<R> = Either<String, Box<dyn GeneratorBase<R>>>;
/// Base trait for a generator use picked out value from input values or generated value picked out child generator
pub trait ValueChildGeneratorBase<R: Randomizer + ?Sized> {
    /// Build selectable value and child generator
    fn build_selectable(
        children: Option<Vec<ChildGeneratorBuilder>>,
        chars: Option<String>,
        values: Option<Vec<DataValue>>,
        filepath: Option<PathBuf>,
    ) -> Result<Vec<ValueOrChild<R>>, BuildError> {
        let mut select_values = Vec::new();
        if let Some(children) = children {
            for child_builder in children.into_iter() {
                let ChildGeneratorBuilder { builder, .. } = child_builder;
                select_values.push(Either::Right(builder.build()?));
            }
        }

        if let Some(chars) = chars {
            select_values.extend(chars.chars().map(|c| Either::Left(c.to_string())));
        }

        if let Some(values) = values {
            select_values.extend(values.into_iter().map(|v| Either::Left(v.to_string())));
        }

        if let Some(filepath) = filepath {
            let file = open_sbrd_file(filepath.as_path())
                .map_err(|e| BuildError::FileError(e, filepath.clone()))?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.map_err(|e| BuildError::FileError(e, filepath.clone()))?;
                select_values.push(Either::Left(line));
            }
        }

        if select_values.is_empty() {
            return Err(BuildError::EmptySelectable);
        }

        Ok(select_values)
    }

    /// Get selectable list
    fn get_selectable(&self) -> &[ValueOrChild<R>];

    /// Pick out value from input values or generated value picked out child generator
    fn choose(
        &self,
        rng: &mut R,
        context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        self.get_selectable()
            .choose_weighted(rng, |_| 1)
            .map_err(|err| GenerateError::FailGenerate(err.to_string()))
            .and_then(|either| match either {
                Either::Left(item) => Ok(item.clone().into()),
                Either::Right(item) => item.generate(rng, context),
            })
    }
}
