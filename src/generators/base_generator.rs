use either::Either;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::generators::error::{CompileError, GenerateError};
use crate::{ChildGeneratorBuilder, DataValue, DataValueMap, GeneratorBuilder, Weight};

pub trait Randomizer: Rng {}
impl<R: Rng> Randomizer for R {}

pub trait Generator<R: Randomizer + ?Sized> {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized;

    fn is_nullable(&self) -> bool;

    fn is_required(&self) -> bool {
        !self.is_nullable()
    }

    fn generate(&self, rng: &mut R, value_map: &DataValueMap) -> Result<DataValue, GenerateError> {
        if self.is_required() {
            self.generate_without_null(rng, value_map)
        } else {
            if rng.gen_bool(0.1) {
                return Ok(DataValue::Null);
            }

            self.generate_without_null(rng, value_map)
        }
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        value_map: &DataValueMap,
    ) -> Result<DataValue, GenerateError>;
}

pub(crate) type WeightedSelectable<R> = (Weight, Either<String, Box<dyn Generator<R>>>);
pub(crate) trait RandomSelectableGenerator<R: 'static + Randomizer + ?Sized> {
    fn build_selectable(
        children: Option<Vec<ChildGeneratorBuilder>>,
        chars: Option<String>,
        values: Option<Vec<DataValue>>,
        filepath: Option<PathBuf>,
    ) -> Result<Vec<WeightedSelectable<R>>, CompileError> {
        // children xor (chars, values, file)
        if !((children.is_some() && (chars.is_none() || values.is_none() || filepath.is_none()))
            || (children.is_none() && (chars.is_some() || values.is_some() || filepath.is_some())))
        {
            return Err(CompileError::InvalidValue(
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
            let file = File::open(filepath).map_err(CompileError::FileError)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.map_err(CompileError::FileError)?;
                select_values.push((1, Either::Left(line)));
            }
        }

        if select_values.is_empty() {
            return Err(CompileError::EmptyRandomize);
        }

        if select_values.iter().fold(0, |acc, item| acc + item.0) == 0 {
            return Err(CompileError::AllWeightsZero);
        }

        Ok(select_values)
    }

    fn get_selectable(&self) -> &[WeightedSelectable<R>];

    fn choose(&self, rng: &mut R, value_map: &DataValueMap) -> Result<DataValue, GenerateError> {
        self.get_selectable()
            .choose_weighted(rng, |item| item.0)
            .map_err(|err| GenerateError::FailGenerate(err.to_string()))
            .and_then(|(_, either)| match either {
                Either::Left(item) => Ok(item.clone().into()),
                Either::Right(item) => item.generate(rng, value_map),
            })
    }
}
