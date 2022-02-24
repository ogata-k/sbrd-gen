use crate::builder::{ChildGeneratorBuilder, Weight};
use crate::error::{BuildError, GenerateError};
use crate::file::open_sbrd_file;
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap};
use either::Either;
use rand::seq::SliceRandom;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub(crate) type WeightedSelectable<R> = (Weight, Either<String, Box<dyn Generator<R>>>);
pub(crate) trait RandomSelectableGenerator<R: 'static + Randomizer + ?Sized> {
    fn build_selectable(
        children: Option<Vec<ChildGeneratorBuilder>>,
        chars: Option<String>,
        values: Option<Vec<DataValue>>,
        filepath: Option<PathBuf>,
    ) -> Result<Vec<WeightedSelectable<R>>, BuildError> {
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

    fn get_selectable(&self) -> &[WeightedSelectable<R>];

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
