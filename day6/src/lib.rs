use std::io::{BufRead, Lines};

#[derive(thiserror::Error, Debug)]
pub enum WorksheetError {
    #[error("no operands found")]
    NoOperands,

    #[error("malformed operator: `{0}`")]
    BadOperator(String),

    #[error(transparent)]
    BadOperand(#[from] std::num::ParseIntError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Default)]
pub struct Worksheet {
    operands: Vec<Vec<u64>>,
    operators: Vec<char>,
}

impl Worksheet {
    pub fn new() -> Worksheet {
        Worksheet::default()
    }

    pub fn solve<B: BufRead>(&mut self, lines: Lines<B>) -> Result<u64, WorksheetError> {
        for line in lines {
            tracing::debug!(?line);
            let line = line?;
            let mut values = line.split_whitespace().peekable();

            match values.peek() {
                // If the first value in this line is a + or *, this row is our operator row.
                Some(&"+") | Some(&"*") => {
                    self.operators = values
                        .map(|s| {
                            s.chars()
                                .nth(0)
                                .ok_or(WorksheetError::BadOperator(s.into()))
                        })
                        .collect::<Result<Vec<_>, WorksheetError>>()?
                }
                // Otherwise, it's another operand row. Parse as ints and throw in `self.operands`.
                _ => self.operands.push(
                    values
                        .map(str::parse::<u64>)
                        .collect::<Result<Vec<_>, _>>()?,
                ),
            }
        }

        // Get the first row of operands and use it as our base for applying further operations.
        // Alternatively, we could map each item in `self.operators` to 0 (for +) or 1 (for *) and
        // use that list of identity elements as our base.
        let mut results = self
            .operands
            .first()
            .ok_or(WorksheetError::NoOperands)?
            .clone();

        // Since the first set of operands has already been incorporated into our results, skip it
        // and iterate over the rest.
        for operand_row in self.operands.iter().skip(1) {
            // Zip each operand with the correct operator.
            for (i, (operand, operator)) in
                operand_row.iter().zip(self.operators.iter()).enumerate()
            {
                match operator {
                    '+' => results[i] += operand,
                    '*' => results[i] *= operand,
                    other => {
                        return Err(WorksheetError::BadOperator(other.to_string()));
                    }
                }
            }
        }

        Ok(results.iter().sum())
    }
}
