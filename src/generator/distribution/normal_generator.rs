use crate::builder::{GeneratorBuilder, Nullable};
use crate::generator::error::{CompileError, GenerateError};
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap, SbrdReal};
use crate::GeneratorType;
use rand::distributions::Distribution;
use rand_distr::Normal;

#[derive(Debug, Clone, Copy)]
pub struct NormalGenerator {
    nullable: Nullable,
    distribution: Normal<SbrdReal>,
}

impl<R: Randomizer + ?Sized> Generator<R> for NormalGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            parameters,
            ..
        } = builder;

        if generator_type != GeneratorType::DistNormal {
            return Err(CompileError::InvalidType(generator_type));
        }

        let (mean, std_dev): (SbrdReal, SbrdReal) = match parameters {
            None => Err(CompileError::NotExistValueOf("parameters".to_string())),
            Some(parameters) => {
                let _mean = parameters
                    .get(Self::MEAN)
                    .map(|v| {
                        v.to_parse_string().parse::<SbrdReal>().map_err(|e| {
                            CompileError::FailParseValue(
                                v.to_parse_string(),
                                "Real".to_string(),
                                e.to_string(),
                            )
                        })
                    })
                    .unwrap_or_else(|| Ok(0.0))?;

                let _std_dev = parameters
                    .get(Self::STD_DEV)
                    .map(|v| {
                        v.to_parse_string().parse::<SbrdReal>().map_err(|e| {
                            CompileError::FailParseValue(
                                v.to_parse_string(),
                                "Real".to_string(),
                                e.to_string(),
                            )
                        })
                    })
                    .unwrap_or_else(|| Ok(1.0))?;
                if _std_dev < 0.0 {
                    return Err(CompileError::InvalidValue(format!(
                        "std_dev {} is less than 0.0",
                        _std_dev
                    )));
                }

                Ok((_mean, _std_dev))
            }
        }?;

        Ok(Self {
            nullable,
            distribution: Normal::new(mean, std_dev).map_err(|e| {
                CompileError::FailBuildDistribution("Normal".to_string(), e.to_string())
            })?,
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        _value_map: &DataValueMap,
    ) -> Result<DataValue, GenerateError> {
        Ok(DataValue::Real(self.distribution.sample(rng)))
    }
}

impl NormalGenerator {
    /// mean
    pub const MEAN: &'static str = "mean";
    /// standard deviation
    pub const STD_DEV: &'static str = "std_dev";
}
