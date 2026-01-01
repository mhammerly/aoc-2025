use std::io::{BufRead, Lines};
use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum WorksheetError {
    #[error("no operands found")]
    NoOperands,

    #[error("malformed operator: `{0}`")]
    BadOperator(String),

    #[error(transparent)]
    BadOperand(#[from] ParseIntError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// The way each operand should be read.
pub enum NumberFormat {
    /// The way we're used to; left-to-right, top-to-bottom. Left/right alignment doesn't matter.
    ///
    /// ```text
    /// 12
    /// 345
    /// +
    ///
    /// 12 + 345
    /// ```
    LeftRightTopBottom,

    /// The "cephalopod math" way; top-to-bottom, right-to-left. Alignment matters.
    ///
    /// ```text
    /// 12
    /// 345
    /// +
    ///
    /// 5 + 24 + 13
    /// ```
    TopBottomRightLeft,
}

impl NumberFormat {}

#[derive(Default)]
pub struct Worksheet {
    /// Unparsed rows of the input file which contain problem operands.
    raw_operands: Vec<String>,

    /// Each problem's operator and the index within its input row that the operator appears in.
    /// The index marks the start of a new problem.
    operators: Vec<(usize, char)>,
}

impl Worksheet {
    /// Load the input file to create a `Worksheet`. Most input lines are buffered until the final
    /// line containing operators which is lightly parsed.
    pub fn new<B: BufRead>(lines: Lines<B>) -> Result<Worksheet, WorksheetError> {
        let mut worksheet = Worksheet::default();
        for line in lines {
            tracing::debug!(?line);
            let line = line?;
            match line.chars().nth(0) {
                Some('+') | Some('*') => {
                    worksheet.operators = line
                        .chars()
                        .enumerate()
                        .filter(|(_i, c)| c != &' ')
                        .collect();
                }
                _ => {
                    worksheet.raw_operands.push(line);
                }
            }
        }

        Ok(worksheet)
    }

    /// Parses `self.raw_operands` according to `format`.
    ///
    /// Returns a `Vec<Vec<u64>>`. Each `Vec<u64>` is the list of operands for a problem.
    fn parse_operands(&self, format: NumberFormat) -> Result<Vec<Vec<u64>>, WorksheetError> {
        // Use the position of the operators to split each row of operands into individual operands
        // with their left/right alignment preserved. Also transposes operands so that they are
        // grouped by row rather than by column.
        //
        // Each operator's index (except the first) is treated as the end of an operand segment. We
        // skip the first because its index is 0 so it can't be the end of anything, and we have to
        // repeat this manually one last time after the fold because there is no final operator
        // to give us the end of the last segment.
        let (last_index, mut segments): (usize, Vec<Vec<&str>>) =
            self.operators.iter().skip(1).map(|(i, _)| i).fold(
                (0, vec![]),
                |(start, mut segments), next| {
                    segments.push(
                        self.raw_operands
                            .iter()
                            .map(|row| &row[start..*next - 1])
                            .collect(),
                    );
                    (*next, segments)
                },
            );
        // Grab the last segment manually.
        segments.push(
            self.raw_operands
                .iter()
                .map(|row| &row[last_index..])
                .collect(),
        );

        // Parse the segmented operands according to `format`.
        //
        // `segments` is a `Vec<Vec<&str>>. Each `Vec<&str>` is the list of pre-parsed operands
        // for a problem.
        let parsed_operands = match format {
            // Simply strip whitespace and parse each operand as u64.
            NumberFormat::LeftRightTopBottom => segments
                .iter()
                .map(|operands| {
                    operands
                        .iter()
                        .map(|operand| operand.trim().parse::<u64>())
                        .collect::<Result<Vec<u64>, ParseIntError>>()
                })
                .collect::<Result<Vec<Vec<u64>>, ParseIntError>>()?,
            // Each operand should be a &str of the same length. Get the last character of each,
            // strip whitespace, and then parse as u64. Then get the second-to-last character of
            // each, strip whitespace, and then parse as u64. So on.
            NumberFormat::TopBottomRightLeft => segments
                .iter()
                .map(|operands| {
                    (0..operands[0].len())
                        .rev()
                        .map(|i| {
                            operands
                                .iter()
                                .filter_map(|operand| operand.chars().nth(i).filter(|c| c != &' '))
                                .collect::<String>()
                                .parse::<u64>()
                        })
                        .collect::<Result<Vec<u64>, ParseIntError>>()
                })
                .collect::<Result<Vec<Vec<u64>>, ParseIntError>>()?,
        };

        Ok(parsed_operands)
    }

    pub fn solve(&self, format: NumberFormat) -> Result<u64, WorksheetError> {
        let parsed_operands = self.parse_operands(format)?;

        // `self.operators` and `parsed_operands` should be the same length. The first problem's
        // operator is the first element in `self.operators` and its operands are the first
        // `Vec<u64>` in `parsed_operands`. Zip them, and apply the operator to the operands.
        let mut total = 0;
        for (operands, (_, operator)) in parsed_operands.iter().zip(self.operators.iter()) {
            tracing::debug!(?operands, ?operator);
            let result: u64 = match operator {
                '+' => operands.iter().sum(),
                '*' => operands.iter().product(),
                c => return Err(WorksheetError::BadOperator(c.to_string())),
            };
            total += result;
        }

        Ok(total)
    }
}
