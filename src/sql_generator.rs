use indicatif::ProgressBar;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

// Define the SqlInsertable trait
pub(crate) trait SqlInsertable {
    fn to_insert_fields(&self) -> Vec<String>;
    fn to_insert_values(&self) -> Vec<String>;
}

#[macro_export]
macro_rules! define_and_impl_sql_insertable {
    ($($struct_name:ident { $($vis:vis $field_name:ident : $field_type:ty),* }),*) => {
        $(
            #[derive(Debug, Clone)]
            #[allow(non_camel_case_types, non_snake_case)]
            pub struct $struct_name {
                $($vis $field_name : $field_type),*
            }

            impl crate::sql_generator::SqlInsertable for $struct_name {
                fn to_insert_fields(&self) -> Vec<String> {
                    vec![$(stringify!($field_name).to_string()),*]
                }

                fn to_insert_values(&self) -> Vec<String> {
                    vec![$({
                        let value = format!("{:?}", self.$field_name);
                        if value.starts_with("\"TO_DATE") {
                            value.replace("\"", "")
                        } else {
                            value.replace("'", "").replace("\"", "'")
                        }
                    }),*]
                }
            }
        )*
    };
}

pub(crate) struct SqlGenerator<T> {
    data: Vec<T>,
}

impl<T> SqlGenerator<T> {
    pub(crate) fn new(data: Vec<T>) -> Self {
        SqlGenerator { data }
    }

    fn get_struct_name(&self) -> &str {
        let full_name = std::any::type_name::<T>();
        full_name.split("::").last().unwrap_or(full_name)
    }

    pub(crate) fn write_to_file(&self, pb: &ProgressBar) -> std::io::Result<()>
    where
        T: SqlInsertable,
    {
        const CHUNK_SIZE: usize = 200;

        let table_name = self.get_struct_name();
        let dir = "data";
        let filename = format!("{}/{}.sql", dir, table_name);
        fs::create_dir_all(dir)?;
        let file = File::create(&filename)?;
        let mut writer = BufWriter::new(file);

        if self.data.is_empty() {
            return Ok(());
        }

        for chunk in self.data.chunks(CHUNK_SIZE) {
            // Write the initial part of the INSERT statement
            let first_item = &chunk[0];
            let fields: Vec<_> = first_item.to_insert_fields();
            writeln!(
                writer,
                "INSERT INTO {} ({}) VALUES",
                table_name,
                fields.join(", ")
            )?;

            // Write the values for each item in the chunk
            for (index, item) in chunk.iter().enumerate() {
                let values: Vec<_> = item.to_insert_values();
                write!(writer, "({})", values.join(", "))?;

                if index < chunk.len() - 1 {
                    writeln!(writer, ",")?;
                } else {
                    writeln!(writer, ";")?;
                }

                pb.inc(1);
            }
        }

        Ok(())
    }
}
