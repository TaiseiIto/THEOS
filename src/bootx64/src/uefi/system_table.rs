use super::table_header;

#[derive(Debug)]
#[repr(C)]
pub struct SystemTable {
    header: table_header::TableHeader,
}

