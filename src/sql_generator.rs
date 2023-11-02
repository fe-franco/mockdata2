use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

// Define the SqlInsertable trait
pub(crate) trait SqlInsertable {
    fn to_insert_sql(&self, table_name: &str) -> String;
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
                fn to_insert_sql(&self, table_name: &str) -> String {
                    let fields = vec![$(stringify!($field_name)),*];
                    let values = vec![$(format!("{:?}", self.$field_name)),*];
                    // remove double quotes from values and replace with single quotes if it does not start with TO_DATE
                    let values = values.iter().map(|v| {
                        if v.starts_with("\"TO_DATE") {
                            v.replace("\"", "")
                        } else {
                            v.replace("\"", "'")
                        }
                    }).collect::<Vec<String>>();

                    format!(
                        "INSERT INTO {} ({}) VALUES ({});",
                        table_name,
                        fields.join(", "),
                        values.join(", ")
                    )
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

    fn get_struct_name(&self) -> String {
        let full_name = std::any::type_name::<T>();
        full_name
            .split("::")
            .last()
            .unwrap_or(full_name)
            .to_string()
    }

    pub(crate) fn write_to_file(&self) -> std::io::Result<()>
    where
        T: SqlInsertable,
    {
        let table_name = self.get_struct_name();
        let dir = "data";
        let filename = format!("{}/{}.sql", dir, table_name);
        fs::create_dir_all(dir)?;
        let file = File::create(&filename)?;
        let mut writer = BufWriter::new(file);

        for item in &self.data {
            writeln!(writer, "{}", item.to_insert_sql(&table_name))?;
        }

        Ok(())
    }
}
