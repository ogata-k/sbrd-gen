use crate::generators::error::{CompileError, GenerateError};
use crate::generators::{Generator, RandomSelectableGenerator, Randomizer, WeightedSelectable};
use crate::{
    DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable, SbrdInt, ValueBound,
};

pub struct DuplicatePermutationGenerator<R: 'static + Randomizer + ?Sized> {
    nullable: Nullable,
    count_range: ValueBound<SbrdInt>,
    separator: String,
    selectable_values: Vec<WeightedSelectable<R>>,
}

impl<R: Randomizer + ?Sized> RandomSelectableGenerator<R> for DuplicatePermutationGenerator<R> {
    fn get_selectable(&self) -> &[WeightedSelectable<R>] {
        &self.selectable_values
    }
}

impl<R: Randomizer + ?Sized> Generator<R> for DuplicatePermutationGenerator<R> {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            range,
            children,
            chars,
            values,
            filepath,
            separator,
            ..
        } = builder;

        if generator_type != GeneratorType::DuplicatePermutation {
            return Err(CompileError::InvalidType(generator_type));
        }

        let count_range = match range {
            None => Self::default_range(),
            Some(r) => r
                .try_convert_with(|s| {
                    s.to_parse_string().parse::<SbrdInt>().map_err(|e| {
                        CompileError::FailParseValue(
                            s.to_parse_string(),
                            "Int".to_string(),
                            e.to_string(),
                        )
                    })
                })?
                .without_no_bound_from_other((0..).into()),
        };
        if let Some(s) = count_range.get_start() {
            if s < &0 {
                return Err(CompileError::InvalidValue(count_range.to_string()));
            }
        }
        if count_range.is_empty() {
            return Err(CompileError::RangeEmpty(count_range.convert_into()));
        }

        let _separator = separator.unwrap_or_else(|| "".to_string());

        let selectable_values = Self::build_selectable(children, chars, values, filepath)?;

        Ok(Self {
            nullable,
            count_range,
            separator: _separator,
            selectable_values,
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        value_map: &DataValueMap,
    ) -> Result<DataValue, GenerateError> {
        let mut result: String = String::new();
        let mut is_first = true;
        let count = rng.gen_range(self.count_range);
        for _ in 0..count {
            let value_string = self.choose(rng, value_map)?.to_permutation_string();
            if value_string.is_empty() {
                continue;
            }

            if is_first {
                result += &value_string;
                is_first = false;
            } else {
                result.push_str(&self.separator);
                result += &value_string;
            }
        }

        Ok(result.into())
    }
}

impl<R: Randomizer + ?Sized> DuplicatePermutationGenerator<R> {
    fn default_range() -> ValueBound<SbrdInt> {
        ValueBound::new(Some(1), Some((true, 15)))
    }
}
