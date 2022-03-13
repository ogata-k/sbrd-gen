use crate::builder::{GeneratorBuilder, Nullable};
use crate::error::{BuildError, GenerateError};
use crate::generator::{GeneratorBase, Randomizer};
use crate::value::{DataValue, DataValueMap, SbrdReal};
use crate::GeneratorType;
use rand::distributions::Distribution;
use rand_distr::Normal;

/// The generator with generate [`DataValue::Real`] from normal distribution
///
/// [`DataValue::Real`]: ../../value/enum.DataValue.html#variant.Real
#[derive(Debug, Clone, Copy)]
pub struct NormalGenerator {
    nullable: Nullable,
    distribution: Normal<SbrdReal>,
}

impl<R: Randomizer + ?Sized> GeneratorBase<R> for NormalGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
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
            return Err(BuildError::InvalidType(generator_type));
        }

        let (mean, std_dev): (SbrdReal, SbrdReal) = match parameters {
            None => Err(BuildError::NotExistValueOf("parameters".to_string())),
            Some(parameters) => {
                let _mean = parameters
                    .get(Self::MEAN)
                    .map(|v| {
                        v.to_parse_string().parse::<SbrdReal>().map_err(|e| {
                            BuildError::FailParseValue(
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
                            BuildError::FailParseValue(
                                v.to_parse_string(),
                                "Real".to_string(),
                                e.to_string(),
                            )
                        })
                    })
                    .unwrap_or_else(|| Ok(1.0))?;
                if _std_dev < 0.0 {
                    return Err(BuildError::InvalidValue(format!(
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
                BuildError::FailBuildDistribution("Normal".to_string(), e.to_string())
            })?,
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        _context: &DataValueMap<&str>,
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
