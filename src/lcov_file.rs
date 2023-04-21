use std::collections::HashMap;

use anyhow::Result;

use crate::parsers::*;

pub struct Lcov {
    name: std::path::PathBuf,
    files: Vec<LcovFile>,
}

pub struct LcovSummary {
    total_lines: usize,
    total_lines_hit: usize,
    total_functions: usize,
    total_functions_hit: usize,
}

impl LcovSummary {
    fn lines_percentage(&self) -> f64 {
        self.total_lines_hit as f64 / self.total_lines as f64 * 100.
    }

    fn functions_percentage(&self) -> f64 {
        self.total_functions_hit as f64 / self.total_functions as f64 * 100.
    }
}

impl Lcov {
    /// Parse an LCOV file.
    pub fn parse(name: std::path::PathBuf) -> Result<Self> {
        let source = std::fs::read_to_string(&name)?;
        let mut files = vec![];
        for line in source.lines() {
            if line.starts_with("SF:") {
                let (_, source) = source_file_path(line).unwrap();
                files.push(LcovFile::new(&source));
                continue;
            }

            if line.starts_with("FN:") {
                let (_, (_, name)) = function_name(line).unwrap();
                if let Some(file) = files.last_mut() {
                    file.function_hits.insert(name.to_string(), 0);
                }
                continue;
            }

            if line.starts_with("FNDA:") {
                let (_, (hits, name)) = function_hit_count(line).unwrap();
                if let Some(file) = files.last_mut() {
                    *file.function_hits.get_mut(name).unwrap() = hits;
                }
                continue;
            }

            if line.starts_with("FNF:") {
                let (_, found) = functions_found(line).unwrap();
                if let Some(file) = files.last_mut() {
                    file.functions_found = found;
                }
                continue;
            }

            if line.starts_with("FNH:") {
                let (_, hit) = functions_hit(line).unwrap();
                if let Some(file) = files.last_mut() {
                    file.functions_hit = hit;
                }
                continue;
            }

            if line.starts_with("LF:") {
                let (_, found) = lines_found(line).unwrap();
                if let Some(file) = files.last_mut() {
                    file.lines_found = found;
                }
                continue;
            }

            if line.starts_with("LH:") {
                let (_, hit) = lines_hit(line).unwrap();
                if let Some(file) = files.last_mut() {
                    file.lines_hit = hit;
                }
                continue;
            }

            if line.starts_with("BF:") {
                let (_, found) = branches_found(line).unwrap();
                if let Some(file) = files.last_mut() {
                    file.branches_found = found;
                }
                continue;
            }

            if line.starts_with("BH:") {
                let (_, hit) = branches_hit(line).unwrap();
                if let Some(file) = files.last_mut() {
                    file.branches_hit = hit;
                }
                continue;
            }
        }

        Ok(Self { name, files })
    }

    /// Return a reference to the parsed files.
    pub fn files(&self) -> &[LcovFile] {
        &self.files
    }

    /// Return a mutable reference to the parsed files.
    pub fn files_mut(&mut self) -> &mut [LcovFile] {
        &mut self.files
    }

    pub fn diffstd(&self, other: &Self) {
        todo!();
    }

    /// Print a summary of the diff of two files to stdout.
    pub fn diffsummarystd(&self, other: &Self) {
        use prettytable::{format::consts::FORMAT_CLEAN, format::Alignment, Cell, Row, Table};

        let summary = self.summary();
        let summary_other = other.summary();

        let lines_diff = summary_other.lines_percentage() - summary.lines_percentage();
        let functions_diff = summary_other.functions_percentage() - summary.functions_percentage();

        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);
        table.set_titles(Self::title_row());
        table.add_row(Self::sub_title_row());

        table.add_row(Row::new(vec![
            Cell::new_align(&self.name.to_string_lossy(), Alignment::RIGHT),
            Cell::new("│"),
            Cell::new_align(&summary.total_lines_hit.to_string(), Alignment::RIGHT),
            Cell::new_align(&summary.total_lines.to_string(), Alignment::RIGHT),
            Cell::new_align(
                &Self::color_percentage(summary.lines_percentage(), 70., 80.),
                Alignment::RIGHT,
            ),
            Cell::new("│"),
            Cell::new_align(&summary.total_functions_hit.to_string(), Alignment::RIGHT),
            Cell::new_align(&summary.total_functions.to_string(), Alignment::RIGHT),
            Cell::new_align(
                &Self::color_percentage(summary.functions_percentage(), 70., 80.),
                Alignment::RIGHT,
            ),
        ]));

        table.add_row(Row::new(vec![
            Cell::new_align(&other.name.to_string_lossy(), Alignment::RIGHT),
            Cell::new("│"),
            Cell::new_align(&summary_other.total_lines_hit.to_string(), Alignment::RIGHT),
            Cell::new_align(&summary_other.total_lines.to_string(), Alignment::RIGHT),
            Cell::new_align(
                &Self::color_percentage(summary_other.lines_percentage(), 70., 80.),
                Alignment::RIGHT,
            ),
            Cell::new("│"),
            Cell::new_align(
                &summary_other.total_functions_hit.to_string(),
                Alignment::RIGHT,
            ),
            Cell::new_align(&summary_other.total_functions.to_string(), Alignment::RIGHT),
            Cell::new_align(
                &Self::color_percentage(summary_other.functions_percentage(), 70., 80.),
                Alignment::RIGHT,
            ),
        ]));

        table.add_row(Row::new(vec![
            Cell::new_align("diff", Alignment::RIGHT),
            Cell::new("│"),
            Cell::new_align(
                {
                    let diff =
                        summary_other.total_lines_hit as isize - summary.total_lines_hit as isize;
                    &match diff {
                        diff if diff > 0 => format!("+ {diff}"),
                        diff if diff < 0 => format!("- {}", diff.abs()),
                        _ => String::new(),
                    }
                },
                Alignment::RIGHT,
            ),
            Cell::new_align(
                {
                    let diff = summary_other.total_lines as isize - summary.total_lines as isize;
                    &match diff {
                        diff if diff > 0 => format!("+ {diff}"),
                        diff if diff < 0 => format!("- {}", diff.abs()),
                        _ => String::new(),
                    }
                },
                Alignment::RIGHT,
            ),
            Cell::new_align(&Self::color_percentage_diff(lines_diff), Alignment::RIGHT),
            Cell::new("│"),
            Cell::new_align(
                {
                    let diff = summary_other.total_functions_hit as isize
                        - summary.total_functions_hit as isize;
                    &match diff {
                        diff if diff > 0 => format!("+ {diff}"),
                        diff if diff < 0 => format!("- {}", diff.abs()),
                        _ => String::new(),
                    }
                },
                Alignment::RIGHT,
            ),
            Cell::new_align(
                {
                    let diff =
                        summary_other.total_functions as isize - summary.total_functions as isize;
                    &match diff {
                        diff if diff > 0 => format!("+ {diff}"),
                        diff if diff < 0 => format!("- {}", diff.abs()),
                        _ => String::new(),
                    }
                },
                Alignment::RIGHT,
            ),
            Cell::new_align(
                &Self::color_percentage_diff(functions_diff),
                Alignment::RIGHT,
            ),
        ]));

        table.printstd();
    }

    /// Return the summary of a an LCOV file.
    pub fn summary(&self) -> LcovSummary {
        let mut total_lines = 0;
        let mut total_lines_hit = 0;

        let mut total_functions = 0;
        let mut total_functions_hit = 0;
        for file in &self.files {
            total_lines += file.lines_found;
            total_lines_hit += file.lines_hit;
            total_functions += file.functions_found;
            total_functions_hit += file.functions_hit;
        }
        LcovSummary {
            total_lines,
            total_lines_hit,
            total_functions,
            total_functions_hit,
        }
    }

    /// Print the summary of an LCOV file to stdout.
    pub fn summarystd(&self) {
        use prettytable::{format::consts::FORMAT_CLEAN, format::Alignment, Cell, Row, Table};

        let summary = self.summary();
        let total_lines_p = summary.lines_percentage();
        let total_functions_p = summary.functions_percentage();

        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);
        table.set_titles(Self::title_row());
        table.add_row(Self::sub_title_row());

        table.add_row(Row::new(vec![
            Cell::new_align(&self.name.to_string_lossy(), Alignment::RIGHT),
            Cell::new("│"),
            Cell::new_align(&summary.total_lines_hit.to_string(), Alignment::RIGHT),
            Cell::new_align(&summary.total_lines.to_string(), Alignment::RIGHT),
            Cell::new_align(
                &Self::color_percentage(total_lines_p, 70., 80.),
                Alignment::RIGHT,
            ),
            Cell::new("│"),
            Cell::new_align(&summary.total_functions_hit.to_string(), Alignment::RIGHT),
            Cell::new_align(&summary.total_functions.to_string(), Alignment::RIGHT),
            Cell::new_align(
                &Self::color_percentage(total_functions_p, 70., 80.),
                Alignment::RIGHT,
            ),
        ]));

        table.printstd();
    }

    /// Print the LCOV file to stdout.
    pub fn printstd(&self) {
        use prettytable::{format::consts::FORMAT_CLEAN, format::Alignment, Cell, Row, Table};

        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);

        table.set_titles(Self::title_row());
        table.add_row(Self::sub_title_row());

        let summary = self.summary();
        let total_lines_p = summary.lines_percentage();
        let total_functions_p = summary.functions_percentage();

        for file in &self.files {
            let lines_percentage = (file.lines_hit as f64 / file.lines_found as f64) * 100.;

            let functions_percentage =
                (file.functions_hit as f64 / file.functions_found as f64) * 100.;

            let file_name = if let Some(i) = file.name.find("/src") {
                file.name.split_at(i + 1).1
            } else {
                &file.name
            };

            table.add_row(Row::new(vec![
                Cell::new(file_name),
                Cell::new("│"),
                Cell::new_align(&file.lines_hit.to_string(), Alignment::RIGHT),
                Cell::new_align(&file.lines_found.to_string(), Alignment::RIGHT),
                Cell::new_align(
                    &Self::color_percentage(lines_percentage, 70., 80.),
                    Alignment::RIGHT,
                ),
                Cell::new("│"),
                Cell::new_align(&file.functions_hit.to_string(), Alignment::RIGHT),
                Cell::new_align(&file.functions_found.to_string(), Alignment::RIGHT),
                Cell::new_align(
                    &Self::color_percentage(functions_percentage, 70., 80.),
                    Alignment::RIGHT,
                ),
            ]));
        }

        table.add_row(Row::new(vec![
            Cell::new_align(&self.name.to_string_lossy(), Alignment::RIGHT),
            Cell::new("│"),
            Cell::new_align(&summary.total_lines_hit.to_string(), Alignment::RIGHT),
            Cell::new_align(&summary.total_lines.to_string(), Alignment::RIGHT),
            Cell::new_align(
                &Self::color_percentage(total_lines_p, 70., 80.),
                Alignment::RIGHT,
            ),
            Cell::new("│"),
            Cell::new_align(&summary.total_functions_hit.to_string(), Alignment::RIGHT),
            Cell::new_align(&summary.total_functions.to_string(), Alignment::RIGHT),
            Cell::new_align(
                &Self::color_percentage(total_functions_p, 70., 80.),
                Alignment::RIGHT,
            ),
        ]));

        table.printstd();
    }

    fn color_percentage_diff(value: f64) -> String {
        use colored::*;

        if value == 0. {
            format!("= {value:.2}%").yellow().to_string()
        } else if value > 0. {
            format!("+ {value:.2}%").green().to_string()
        } else {
            let value = value.abs();
            format!("- {value:.2}%").red().to_string()
        }
    }

    fn color_percentage(value: f64, low: f64, mid: f64) -> String {
        use colored::*;

        let p = format!("{value:.2}%");
        format!(
            "{}",
            if value < low {
                p.red()
            } else if value < mid {
                p.yellow()
            } else {
                p.green()
            }
        )
    }

    fn title_row() -> prettytable::Row {
        use prettytable::{format::Alignment, Cell, Row};

        Row::new(vec![
            Cell::new(""),
            Cell::new(""),
            {
                let mut line = Cell::new_align("Lines", Alignment::CENTER);
                line.set_hspan(3);
                line
            },
            Cell::new(""),
            {
                let mut fs = Cell::new_align("Functions", Alignment::CENTER);
                fs.set_hspan(3);
                fs
            },
        ])
    }
    fn sub_title_row() -> prettytable::Row {
        use prettytable::{Cell, Row};

        Row::new(vec![
            Cell::new(""),
            Cell::new("│"),
            Cell::new("Hit"),
            Cell::new("Total"),
            Cell::new("H/T"),
            Cell::new("│"),
            Cell::new("Hit"),
            Cell::new("Total"),
            Cell::new("H/T"),
        ])
    }
}

#[derive(Debug)]
pub struct LcovFile {
    name: String,
    function_hits: HashMap<String, usize>,
    functions_found: usize,
    functions_hit: usize,
    lines_found: usize,
    lines_hit: usize,
    branches_found: usize,
    branches_hit: usize,
}

impl LcovFile {
    pub fn new(source: &impl AsRef<str>) -> Self {
        let source = source.as_ref();
        Self {
            name: source.to_string(),
            function_hits: Default::default(),
            functions_found: 0,
            functions_hit: 0,
            lines_found: 0,
            lines_hit: 0,
            branches_found: 0,
            branches_hit: 0,
        }
    }
}
