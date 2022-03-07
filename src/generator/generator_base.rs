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
pub trait Generator<R: Randomizer + ?Sized> {
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
        value_map: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        if self.is_required() {
            self.generate_without_null(rng, value_map)
        } else {
            if rng.gen_bool(0.1) {
                return Ok(DataValue::Null);
            }

            self.generate_without_null(rng, value_map)
        }
    }

    /// Generate dummy data not considering nullable
    fn generate_without_null(
        &self,
        rng: &mut R,
        value_map: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError>;
}

/// Child generator with condition
pub type CasedChild<R> = (Option<String>, Box<dyn Generator<R>>);
/// Base trait for a generator use child generators with condition.
///
/// If a child generator's condition is [`Option::Some`], then evaluate it's condition.
/// If a child generator's condition is [`Option::None`], then default condition. Default condition always must exist.
///
/// [`Option::Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
/// [`Option::None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
pub trait CasedChildGenerator<R: Randomizer + ?Sized> {
    /// Build selectable child generator list
    fn build_selectable(
        children: Option<Vec<ChildGeneratorBuilder>>,
    ) -> Result<Vec<CasedChild<R>>, BuildError> {
        match children {
            None => Err(BuildError::NotExistValueOf("children".to_string())),
            Some(children) => {
                let mut _children: Vec<(Option<String>, Box<dyn Generator<R>>)> = Vec::new();
                let mut has_default_case = false;
                for child_builder in children.into_iter() {
                    let ChildGeneratorBuilder {
                        condition, builder, ..
                    } = child_builder;
                    has_default_case = has_default_case || condition.is_none();
                    _children.push((condition, builder.build()?));
                }

                if _children.is_empty() {
                    return Err(BuildError::EmptyChildren);
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
        value_map: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        for (condition, generator) in self.get_children().iter() {
            return match condition {
                None => generator.generate(rng, value_map),
                Some(_condition) => {
                    let evaluator = Evaluator::new(_condition, value_map);
                    let is_satisfy = evaluator.eval_bool().map_err(|e| {
                        GenerateError::FailEval(
                            e,
                            _condition.clone(),
                            value_map
                                .iter()
                                .map(|(k, v)| (k.to_string(), v.clone()))
                                .collect::<DataValueMap<String>>(),
                        )
                    })?;
                    if !is_satisfy {
                        continue;
                    }

                    generator.generate(rng, value_map)
                }
            };
        }

        Err(GenerateError::FailGenerate(
            "No match case condition".to_string(),
        ))
    }
}

/// Base trait for a generator from input values with many options
pub trait MultiOptionsValueGenerator<R: Randomizer + ?Sized, T> {
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
            return Err(BuildError::EmptySelectValues);
        }

        Ok(selectable_values)
    }
}
/// Base trait for a generator from input values with only one option
pub trait SingleOptionValueGenerator<R: Randomizer + ?Sized, T> {
    /// Function of parser the input value
    fn parse(input: &str) -> Result<T, BuildError>;

    /// Build selectable value
    fn build_selectable(
        chars: Option<String>,
        values: Option<Vec<DataValue>>,
        filepath: Option<PathBuf>,
    ) -> Result<Vec<T>, BuildError> {
        let mut some_count = 0;
        if chars.is_some() {
            some_count += 1;
        }
        if values.is_some() {
            some_count += 1;
        }
        if filepath.is_some() {
            some_count += 1;
        }
        if some_count == 0 || some_count > 1 {
            return Err(BuildError::OnlyOneOptionSpecifiedNot(vec![
                "chars".to_string(),
                "values".to_string(),
                "filepath".to_string(),
            ]));
        }

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
            return Err(BuildError::EmptySelectValues);
        }

        Ok(selectable_values)
    }
}

/// Value or Child generator with weight
pub type WeightedValueChild<R> = (Weight, Either<String, Box<dyn Generator<R>>>);
/// Base trait for a generator use picked out value from input values or generated value picked out child generator with many options
pub trait MultiOptionsValueChildGenerator<R: Randomizer + ?Sized> {
    /// Build selectable value and child generator with weight list
    fn build_selectable(
        children: Option<Vec<ChildGeneratorBuilder>>,
        chars: Option<String>,
        values: Option<Vec<DataValue>>,
        filepath: Option<PathBuf>,
    ) -> Result<Vec<WeightedValueChild<R>>, BuildError> {
        // children xor (chars, values, file)
        if !((children.is_some() && (chars.is_none() || values.is_none() || filepath.is_none()))
            || (children.is_none() && (chars.is_some() || values.is_some() || filepath.is_some())))
        {
            return Err(BuildError::NotExistValueOf(
                "children xor (chars, values, file)".to_string(),
            ));
        }

        let mut select_values = Vec::new();
        if let Some(children) = children {
            for child_builder in children.into_iter() {
                let ChildGeneratorBuilder {
                    weight, builder, ..
                } = child_builder;
                select_values.push((weight.unwrap_or(1), Either::Right(builder.build()?)));
            }
        }

        if let Some(chars) = chars {
            select_values.extend(chars.chars().map(|c| (1, Either::Left(c.to_string()))));
        }

        if let Some(values) = values {
            select_values.extend(values.into_iter().map(|v| (1, Either::Left(v.to_string()))));
        }

        if let Some(filepath) = filepath {
            let file = open_sbrd_file(filepath.as_path())
                .map_err(|e| BuildError::FileError(e, filepath.clone()))?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.map_err(|e| BuildError::FileError(e, filepath.clone()))?;
                select_values.push((1, Either::Left(line)));
            }
        }

        if select_values.is_empty() {
            return Err(BuildError::EmptyRandomize);
        }

        if select_values.iter().fold(0, |acc, item| acc + item.0) == 0 {
            return Err(BuildError::AllWeightsZero);
        }

        Ok(select_values)
    }

    /// Get selectable list
    fn get_selectable(&self) -> &[WeightedValueChild<R>];

    /// Pick out value from input values or generated value picked out child generator
    fn choose(
        &self,
        rng: &mut R,
        value_map: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        self.get_selectable()
            .choose_weighted(rng, |item| item.0)
            .map_err(|err| GenerateError::FailGenerate(err.to_string()))
            .and_then(|(_, either)| match either {
                Either::Left(item) => Ok(item.clone().into()),
                Either::Right(item) => item.generate(rng, value_map),
            })
    }
}
/// Base trait for a generator use picked out value from input values or generated value picked out child generator with only one option
pub trait SingleOptionValueChildGenerator<R: Randomizer + ?Sized> {
    /// Build selectable value and child generator with weight list
    fn build_selectable(
        children: Option<Vec<ChildGeneratorBuilder>>,
        chars: Option<String>,
        values: Option<Vec<DataValue>>,
        filepath: Option<PathBuf>,
    ) -> Result<Vec<WeightedValueChild<R>>, BuildError> {
        let mut some_count = 0;
        if children.is_some() {
            some_count += 1;
        }
        if chars.is_some() {
            some_count += 1;
        }
        if values.is_some() {
            some_count += 1;
        }
        if filepath.is_some() {
            some_count += 1;
        }
        if some_count == 0 || some_count > 1 {
            return Err(BuildError::OnlyOneOptionSpecifiedNot(vec![
                "children".to_string(),
                "chars".to_string(),
                "values".to_string(),
                "filepath".to_string(),
            ]));
        }

        let mut select_values = Vec::new();
        if let Some(children) = children {
            for child_builder in children.into_iter() {
                let ChildGeneratorBuilder {
                    weight, builder, ..
                } = child_builder;
                select_values.push((weight.unwrap_or(1), Either::Right(builder.build()?)));
            }
        }

        if let Some(chars) = chars {
            select_values.extend(chars.chars().map(|c| (1, Either::Left(c.to_string()))));
        }

        if let Some(values) = values {
            select_values.extend(values.into_iter().map(|v| (1, Either::Left(v.to_string()))));
        }

        if let Some(filepath) = filepath {
            let file = open_sbrd_file(filepath.as_path())
                .map_err(|e| BuildError::FileError(e, filepath.clone()))?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.map_err(|e| BuildError::FileError(e, filepath.clone()))?;
                select_values.push((1, Either::Left(line)));
            }
        }

        if select_values.is_empty() {
            return Err(BuildError::EmptyRandomize);
        }

        if select_values.iter().fold(0, |acc, item| acc + item.0) == 0 {
            return Err(BuildError::AllWeightsZero);
        }

        Ok(select_values)
    }

    /// Get selectable list
    fn get_selectable(&self) -> &[WeightedValueChild<R>];

    /// Pick out value from input values or generated value picked out child generator
    fn choose(
        &self,
        rng: &mut R,
        value_map: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        self.get_selectable()
            .choose_weighted(rng, |item| item.0)
            .map_err(|err| GenerateError::FailGenerate(err.to_string()))
            .and_then(|(_, either)| match either {
                Either::Left(item) => Ok(item.clone().into()),
                Either::Right(item) => item.generate(rng, value_map),
            })
    }
}
