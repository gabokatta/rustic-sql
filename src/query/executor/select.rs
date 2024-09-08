use crate::query::executor::Executor;
use crate::query::structs::expression::ExpressionNode;
use crate::query::structs::ordering::OrderKind;
use crate::query::structs::row::Row;
use crate::utils::errors::Errored;
use crate::utils::files::{extract_header, split_csv};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Executor {
    pub fn run_select(&mut self, table: File) -> Result<(), Errored> {
        let mut reader = BufReader::new(table);
        let header = extract_header(&mut reader)?;
        println!("{}", header.join(","));
        let mut matched_rows: Vec<Row> = vec![];
        for line in reader.lines() {
            let l = line?;
            let fields = split_csv(&l);
            let mut row = Row::new(&header);
            row.set_new_values(fields)?;
            if row.matches_condition(&self.query)? {
                matched_rows.push(row)
            }
        }
        self.sort_rows(&mut matched_rows);
        self.output_rows(&matched_rows);
        Ok(())
    }

    fn sort_rows(&mut self, matched_rows: &mut [Row]) {
        for order in &self.query.ordering {
            matched_rows.sort_by(|a, b| {
                let l = ExpressionNode::get_variable_value(&a.values, &order.field);
                let r = ExpressionNode::get_variable_value(&b.values, &order.field);
                if let (Ok(a), Ok(b)) = (l, r) {
                    return match order.kind {
                        OrderKind::Asc => a.compare(&b).unwrap_or(Ordering::Equal),
                        OrderKind::Desc => b.compare(&a).unwrap_or(Ordering::Equal),
                    };
                }
                Ordering::Equal
            })
        }
    }

    fn output_rows(&self, matched_rows: &[Row]) {
        for row in matched_rows {
            row.print_values()
        }
    }
}
