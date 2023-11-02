use indicatif::ProgressBar;
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
                    let mut fields = String::new();
                    let mut values = String::new();

                    $(
                        fields.push_str(stringify!($field_name));
                        fields.push(',');

                        let value = format!("{:?}", self.$field_name);
                        if value.starts_with("\"TO_DATE") {
                            values.push_str(&value.replace("\"", ""));
                        } else {
                            values.push_str(&value.replace("'", "").replace("\"", "'"));
                        }
                        values.push(',');
                    )*

                    fields.pop(); // Remove trailing comma
                    values.pop(); // Remove trailing comma

                    format!(
                        "INTO {} ({}) VALUES ({})",
                        table_name,
                        fields,
                        values
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

    fn get_struct_name(&self) -> &str {
        let full_name = std::any::type_name::<T>();
        full_name.split("::").last().unwrap_or(full_name)
    }

    pub(crate) fn write_to_file(&self, pb: &ProgressBar) -> std::io::Result<()>
    where
        T: SqlInsertable,
    {
        let table_name = self.get_struct_name();
        let dir = "data";
        let filename = format!("{}/{}.sql", dir, table_name);
        fs::create_dir_all(dir)?;
        let file = File::create(&filename)?;
        let mut writer = BufWriter::new(file);

        for chunk in self.data.chunks(1000) {
            writer.write_all(b"INSERT ALL\n")?;
            for item in chunk {
                let sql = item.to_insert_sql(table_name);
                writer.write_all(sql.as_bytes())?;
                writer.write_all(b"\n")?;
                pb.inc(1);
            }
            writer.write_all(format!("SELECT * FROM dual;\n").as_bytes())?;
            writer.write_all(format!("commit;\n").as_bytes())?;
        }

        writer.write_all(format!("select count(*) from {};\n", table_name).as_bytes())?;

        Ok(())
    }
}
