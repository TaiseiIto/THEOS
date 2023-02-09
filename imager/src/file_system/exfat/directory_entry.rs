mod entry_type;
mod file_attributes;
mod general_flags;
mod raw_allocation_bitmap;
mod raw_file;
mod raw_file_name;
mod raw_stream_extension;
mod raw_upcase_table;
mod raw_volume_guid;
mod raw_volume_label;
mod type_code;

use {
    std::{
        char,
        collections::VecDeque,
        convert::{
            From,
            Into,
        },
        ffi,
        mem,
        path::PathBuf,
        str,
    },
    super::{
        allocation_bitmap,
        cluster,
        super::super::time,
        upcase_table,
    },
};

const DIRECTORY_ENTRY_SIZE: usize = 0x20;
const FILE_NAME_BLOCK_LENGTH: usize = 0xf;

#[derive(Clone, Debug)]
pub enum DirectoryEntry {
    AllocationBitmap {
        bitmap_identifier: bool,
        first_cluster: u32,
        data_length: usize,
    },
    File {
        file_attributes: file_attributes::FileAttributes,
        create_time: time::Time,
        modified_time: time::Time,
        accessed_time: time::Time,
        stream_extension: Box<Self>,
    },
    FileName {
        general_flags: general_flags::GeneralFlags,
        file_name: [u16; FILE_NAME_BLOCK_LENGTH],
        next_file_name: Option<Box<Self>>,
    },
    StreamExtension {
        general_flags: general_flags::GeneralFlags,
        name_length: u8,
        name_hash: u16,
        first_cluster: u32,
        data_length: usize,
        file_name: Box<Self>,
    },
    UpcaseTable {
        table_checksum: u32,
        first_cluster: u32,
        data_length: usize,
        upcase_table: upcase_table::UpcaseTable,
    },
    VolumeGuid {
        general_flags: general_flags::GeneralFlags,
        volume_guid: u128,
    },
    VolumeLabel {
        volume_label: String,
    },
}

impl DirectoryEntry {
    pub fn allocation_bitmap(bitmap_identifier: usize, first_cluster: u32, data_length: usize) -> Self {
        let bitmap_identifier: bool = match bitmap_identifier % 2 {
            0 => false,
            1 => true,
            _ => panic!("Can't construct an allocation bitmap directory entry."),
        };
        Self::AllocationBitmap {
            bitmap_identifier,
            first_cluster,
            data_length,
        }
    }

    pub fn allocation_bitmaps(clusters: &mut cluster::Clusters, root_directory_entries: &Vec<Self>, num_of_fats: usize) -> Vec<Self> {
        let num_of_clusters: usize = clusters.len();
        let cluster_size: usize = clusters.cluster_size();
        let bits_per_cluster: usize = 8 * cluster_size;
        let allocation_bitmap_directory_entry: usize = 1;
        let num_of_root_directory_entries: usize = root_directory_entries.len() + allocation_bitmap_directory_entry;
        let size_of_root_directory_entries: usize = DIRECTORY_ENTRY_SIZE * num_of_root_directory_entries;
        let num_of_clusters_of_root_directory: usize = (size_of_root_directory_entries + cluster_size - 1) / cluster_size;
        let num_of_clusters: usize = num_of_clusters + num_of_clusters_of_root_directory;
        let mut num_of_allocation_bitmap_clusters: usize = 0;
        loop {
            let num_of_clusters: usize = num_of_clusters + num_of_fats * num_of_allocation_bitmap_clusters;
            let necessary_allocation_bitmap_clusters: usize = (num_of_clusters + bits_per_cluster - 1) / bits_per_cluster;
            if necessary_allocation_bitmap_clusters <= num_of_allocation_bitmap_clusters {
                break;
            }
            num_of_allocation_bitmap_clusters += 1;
        }
        let allocation_bitmaps: Vec<allocation_bitmap::AllocationBitmap> = (0..num_of_fats)
            .map(|_| allocation_bitmap::AllocationBitmap::new(clusters))
            .collect();
        let allocation_bitmaps: Vec<Vec<u8>> = allocation_bitmaps
            .into_iter()
            .map(|ref allocation_bitmap| allocation_bitmap.into())
            .collect();
        let allocation_bitmap_length: usize = allocation_bitmaps
            .iter()
            .map(|allocation_bitmap| allocation_bitmap.len())
            .max()
            .expect("Can't get allocation_bitmap_length.");
        let allocation_bitmaps: Vec<u32> = allocation_bitmaps
            .into_iter()
            .map(|allocation_bitmap| clusters.append(&allocation_bitmap, 0xff))
            .collect();
        allocation_bitmaps
            .into_iter()
            .enumerate()
            .map(|(i, cluster_number)| Self::allocation_bitmap(i, cluster_number, allocation_bitmap_length))
            .collect()
    }

    pub fn file(path: &PathBuf, first_cluster: u32, data_length: usize, upcase_table: &upcase_table::UpcaseTable) -> Self {
        let file_attributes = file_attributes::FileAttributes::new(path);
        let create_time: time::Time = time::Time::last_changed_time(path);
        let modified_time: time::Time = time::Time::last_modified_time(path);
        let accessed_time: time::Time = time::Time::last_accessed_time(path);
        let file_name: &ffi::OsStr = path.file_name().expect(&format!("Can't extract base name from {}", path.display()));
        let file_name: &str = file_name.to_str().expect("Can't convert OsStr to String.");
        let file_name: String = file_name.to_string();
        let stream_extension: Box<Self> = Box::new(Self::stream_extension(file_name, first_cluster, data_length, upcase_table));
        Self::File {
            file_attributes,
            create_time,
            modified_time,
            accessed_time,
            stream_extension,
        }
    }

    pub fn get_file_name(&self) -> Option<String> {
        match self {
            Self::File {
                file_attributes: _,
                create_time: _,
                modified_time: _,
                accessed_time: _,
                stream_extension,
            } => stream_extension.get_file_name(),
            Self::StreamExtension {
                general_flags: _,
                name_length,
                name_hash: _,
                first_cluster: _,
                data_length: _,
                file_name,
            } => {
                let file_name: Vec<u16> = file_name.get_file_name_words();
                let file_name: String = char::decode_utf16(file_name.iter().cloned())
                    .filter_map(|c| c.ok())
                    .collect();
                let file_name: String = file_name
                    .chars()
                    .take(*name_length as usize)
                    .collect();
                Some(file_name)
            },
            _ => None,
        }
    }

    pub fn read(bytes: &Vec<u8>, clusters: &cluster::Clusters) -> Vec<Self> {
        let directory_entries: Vec<[u8; DIRECTORY_ENTRY_SIZE]> = bytes
            .chunks(DIRECTORY_ENTRY_SIZE)
            .map(|directory_entry| directory_entry.try_into().expect("Can't read directory entry."))
            .collect();
        let directory_entries: Vec<[u8; DIRECTORY_ENTRY_SIZE]> = directory_entries
            .into_iter()
            .filter_map(|directory_entry| {
                let type_code: u8 = directory_entry[0];
                let in_use: bool = type_code & 0x80 != 0;
                match in_use {
                    true => Some(directory_entry),
                    false => None,
                }
            })
            .collect();
        let mut directory_entries: VecDeque<[u8; DIRECTORY_ENTRY_SIZE]> = VecDeque::from(directory_entries);
        let directory_entry: Option<[u8; DIRECTORY_ENTRY_SIZE]> = directory_entries.remove(0);
        let directory_entries: Vec<u8> = directory_entries
            .into_iter()
            .map(|directory_entry| directory_entry.into_iter())
            .flatten()
            .collect();
        let directory_entries: Vec<Self> = match directory_entries.len() {
            0 => vec![],
            _ => Self::read(&directory_entries, clusters),
        };
        let mut directory_entries: VecDeque<Self> = VecDeque::from(directory_entries);
        let directory_entry: Option<Self> = match directory_entry {
            Some(directory_entry) => {
                let type_code: u8 = directory_entry[0];
                let type_code = type_code::TypeCode::from(type_code);
                match type_code {
                    type_code::TypeCode::File => {
                        let file = raw_file::RawFile::from(&directory_entry);
                        let file_attributes: [u8; 2] = directory_entry[4..6]
                            .try_into()
                            .expect("Can't read a file directory entry.");
                        let file_attributes: u16 = unsafe {
                            mem::transmute::<[u8; 2], u16>(file_attributes)
                        };
                        let file_attributes = file_attributes::FileAttributes::from(file_attributes);
                        let create_time = time::Time::from_fat_timestamp(file.create_timestamp(), file.create_10ms_increment(), file.create_utc_offset());
                        let modified_time = time::Time::from_fat_timestamp(file.last_modified_timestamp(), file.last_modified_10ms_increment(), file.last_modified_utc_offset());
                        let accessed_time = time::Time::from_fat_timestamp(file.last_accessed_timestamp(), 0, file.last_accessed_utc_offset());
                        let stream_extension: Self = directory_entries.remove(0).expect("Can't read a file directory entry.");
                        let stream_extension: Box<Self> = Box::new(stream_extension);
                        Some(Self::File {
                            file_attributes,
                            create_time,
                            modified_time,
                            accessed_time,
                            stream_extension,
                        })
                    },
                    type_code::TypeCode::StreamExtension => {
                        let stream_extension = raw_stream_extension::RawStreamExtension::from(&directory_entry);
                        let general_flags = general_flags::GeneralFlags::from(stream_extension.general_flags());
                        let name_length: u8 = stream_extension.name_length();
                        let name_hash: u16 = stream_extension.name_hash();
                        let first_cluster: u32 = stream_extension.first_cluster();
                        let data_length: usize = stream_extension.data_length() as usize;
                        let file_name: Self = directory_entries.remove(0).expect("Can't read stream extension directory entry.");
                        let file_name: Box<Self> = Box::new(file_name);
                        Some(Self::StreamExtension {
                            general_flags,
                            name_length,
                            name_hash,
                            first_cluster,
                            data_length,
                            file_name,
                        })
                    },
                    type_code::TypeCode::FileName => {
                        let file_name = raw_file_name::RawFileName::from(&directory_entry);
                        let general_flags = general_flags::GeneralFlags::from(file_name.general_flags());
                        let file_name: [u16; FILE_NAME_BLOCK_LENGTH] = file_name.file_name();
                        let next_file_name: Option<Box<Self>> = match directory_entries.remove(0) {
                            Some(directory_entry) => match directory_entry {
                                Self::FileName {
                                    general_flags,
                                    file_name,
                                    next_file_name,
                                } => Some(Box::new(Self::FileName {
                                    general_flags,
                                    file_name,
                                    next_file_name,
                                })),
                                _ => {
                                    directory_entries.push_front(directory_entry);
                                    None
                                },
                            },
                            None => None,
                        };
                        Some(Self::FileName {
                            general_flags,
                            file_name,
                            next_file_name,
                        })
                    },
                    type_code::TypeCode::UpcaseTable => {
                        let upcase_table = raw_upcase_table::RawUpcaseTable::from(&directory_entry);
                        let table_checksum: u32 = upcase_table.table_checksum();
                        let first_cluster: u32 = upcase_table.first_cluster();
                        let data_length: usize = upcase_table.data_length() as usize;
                        let upcase_table: upcase_table::UpcaseTable = clusters.upcase_table(first_cluster, data_length);
                        Some(Self::UpcaseTable {
                            table_checksum,
                            first_cluster,
                            data_length,
                            upcase_table,
                        })
                    },
                    type_code::TypeCode::VolumeLabel => {
                        let volume_label = raw_volume_label::RawVolumeLabel::from(&directory_entry);
                        let character_count: usize = volume_label.character_count() as usize;
                        let volume_label: [u16; raw_volume_label::VOLUME_LABEL_MAX_LENGTH] = volume_label.volume_label();
                        let volume_label: String = char::decode_utf16(volume_label[0..character_count].iter().cloned())
                            .filter_map(|c| c.ok())
                            .collect();
                        Some(Self::VolumeLabel {
                            volume_label,
                        })
                    },
                    type_code::TypeCode::VolumeGuid => {
                        let volume_guid = raw_volume_guid::RawVolumeGuid::from(&directory_entry);
                        let general_flags = general_flags::GeneralFlags::from(volume_guid.general_flags() as u8);
                        let volume_guid: u128 = volume_guid.volume_guid();
                        Some(Self::VolumeGuid {
                            general_flags,
                            volume_guid,
                        })
                    },
                    type_code::TypeCode::AllocationBitmap => {
                        let allocation_bitmap = raw_allocation_bitmap::RawAllocationBitmap::from(&directory_entry);
                        let bitmap_identifier: bool = allocation_bitmap.bitmap_flags() & 0x01 != 0;
                        let first_cluster: u32 = allocation_bitmap.first_cluster();
                        let data_length: usize = allocation_bitmap.data_length() as usize;
                        Some(Self::AllocationBitmap {
                            bitmap_identifier,
                            first_cluster,
                            data_length,
                        })
                    },
                }
            },
            None => None,
        };
        if let Some(directory_entry) = directory_entry {
            directory_entries.push_front(directory_entry);
        }
        let directory_entries: Vec<Self> = directory_entries
            .into_iter()
            .collect();
        directory_entries
    }

    pub fn upcase_table(upcase_table: &upcase_table::UpcaseTable, clusters: &mut cluster::Clusters) -> Self {
        let bytes: Vec<u8> = upcase_table.into();
        let data_length: usize = bytes.len();
        let first_cluster: u32 = clusters.append(&bytes, 0);
        let table_checksum: u32 = upcase_table.table_checksum();
        let upcase_table: upcase_table::UpcaseTable = upcase_table.clone();
        Self::UpcaseTable {
            table_checksum,
            first_cluster,
            data_length,
            upcase_table,
        }
    }

    pub fn volume_guid(volume_guid: u128) -> Self {
        let general_flags = general_flags::GeneralFlags::volume_guid();
        Self::VolumeGuid {
            general_flags,
            volume_guid,
        }
    }

    pub fn volume_label(volume_label: &str) -> Self {
        let volume_label: String = volume_label.to_string();
        Self::VolumeLabel {
            volume_label,
        }
    }

    fn directory_entry_set_length(&self) -> usize {
        match self {
            Self::AllocationBitmap {
                bitmap_identifier: _,
                first_cluster: _,
                data_length: _,
            } => 1,
            Self::File {
                file_attributes: _,
                create_time: _,
                modified_time: _,
                accessed_time: _,
                stream_extension,
            } => 1 + stream_extension.directory_entry_set_length(),
            Self::FileName {
                general_flags: _,
                file_name: _,
                next_file_name,
            } => 1 + match next_file_name {
                Some(next_file_name) => next_file_name.directory_entry_set_length(),
                None => 0,
            },
            Self::StreamExtension {
                general_flags: _,
                name_length: _,
                name_hash: _,
                first_cluster: _,
                data_length: _,
                file_name,
            } => 1 + file_name.directory_entry_set_length(),
            Self::UpcaseTable {
                table_checksum: _,
                first_cluster: _,
                data_length: _,
                upcase_table: _,
            } => 1,
            Self::VolumeGuid {
                general_flags: _,
                volume_guid: _,
            } => 1,
            Self::VolumeLabel {
                volume_label: _,
            } => 1,
        }
    }

    fn entry_type(&self) -> entry_type::EntryType {
        match self {
            Self::AllocationBitmap {
                bitmap_identifier: _,
                first_cluster: _,
                data_length: _,
            } => entry_type::EntryType::allocation_bitmap(),
            Self::File {
                file_attributes: _,
                create_time: _,
                modified_time: _,
                accessed_time: _,
                stream_extension: _,
            } => entry_type::EntryType::file(),
            Self::FileName {
                general_flags: _,
                file_name: _,
                next_file_name: _,
            } => entry_type::EntryType::file_name(),
            Self::StreamExtension {
                general_flags: _,
                name_length: _,
                name_hash: _,
                first_cluster: _,
                data_length: _,
                file_name: _,
            } => entry_type::EntryType::stream_extension(),
            Self::UpcaseTable {
                table_checksum: _,
                first_cluster: _,
                data_length: _,
                upcase_table: _,
            } => entry_type::EntryType::upcase_table(),
            Self::VolumeGuid {
                general_flags: _,
                volume_guid: _,
            } => entry_type::EntryType::volume_guid(),
            Self::VolumeLabel {
                volume_label: _,
            } => entry_type::EntryType::volume_label(),
        }
    }

    fn file_name(mut file_name: Vec<u16>) -> Self {
        let general_flags = general_flags::GeneralFlags::file_name();
        let remaining_file_name: Option<Vec<u16>> = if FILE_NAME_BLOCK_LENGTH < file_name.len() {
            Some(file_name.split_off(FILE_NAME_BLOCK_LENGTH))
        } else {
            None
        };
        file_name.resize(FILE_NAME_BLOCK_LENGTH, 0x00);
        let file_name: [u16; FILE_NAME_BLOCK_LENGTH] = file_name.try_into().expect("Can't convert Vec<u16> to [u16; FILE_NAME_BLOCK_LENGTH]");
        let next_file_name: Option<Box<Self>> = match remaining_file_name {
            Some(remaining_file_name) => Some(Box::new(Self::file_name(remaining_file_name))),
            None => None,
        };
        Self::FileName {
            general_flags,
            file_name,
            next_file_name,
        }
    }

    fn get_file_name_words(&self) -> Vec<u16> {
        match self {
            Self::FileName {
                general_flags: _,
                file_name,
                next_file_name,
            } => {
                let mut file_name: Vec<u16> = file_name.to_vec();
                if let Some(next_file_name) = next_file_name {
                    let mut next_file_name: Vec<u16> = next_file_name.get_file_name_words();
                    file_name.append(&mut next_file_name);
                }
                file_name
            },
            Self::StreamExtension {
                general_flags: _,
                name_length: _,
                name_hash: _,
                first_cluster: _,
                data_length: _,
                file_name,
            } => file_name.get_file_name_words(),
            _ => panic!("Can't get file name words"),
        }
    }

    fn stream_extension(file_name: String, first_cluster: u32, data_length: usize, upcase_table: &upcase_table::UpcaseTable) -> Self {
        let general_flags = general_flags::GeneralFlags::stream_extension();
        let file_name: Vec<u16> = file_name
            .encode_utf16()
            .collect();
        let name_length: u8 = file_name.len() as u8;
        let name_hash: u16 = file_name
            .iter()
            .map(|c| upcase_table.capitalize_char(*c))
            .map(|c| [c as u8, (c >> 8) as u8])
            .flatten()
            .fold(0, |name_hash, c| (name_hash << 15) + (name_hash >> 1) + (c as u16));
        let file_name: Box<Self> = Box::new(Self::file_name(file_name));
        Self::StreamExtension {
            general_flags,
            name_length,
            name_hash,
            first_cluster,
            data_length,
            file_name,
        }
    }

    fn raw(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        match self {
            Self::AllocationBitmap {
                bitmap_identifier: _,
                first_cluster: _,
                data_length: _,
            } => (&raw_allocation_bitmap::RawAllocationBitmap::from(self)).into(),
            Self::File {
                file_attributes: _,
                create_time: _,
                modified_time: _,
                accessed_time: _,
                stream_extension: _,
            } => (&raw_file::RawFile::from(self)).into(),
            Self::FileName {
                general_flags: _,
                file_name: _,
                next_file_name: _,
            } => (&raw_file_name::RawFileName::from(self)).into(),
            Self::StreamExtension {
                general_flags: _,
                name_length: _,
                name_hash: _,
                first_cluster: _,
                data_length: _,
                file_name: _,
            } => (&raw_stream_extension::RawStreamExtension::from(self)).into(),
            Self::UpcaseTable {
                table_checksum: _,
                first_cluster: _,
                data_length: _,
                upcase_table: _,
            } => (&raw_upcase_table::RawUpcaseTable::from(self)).into(),
            Self::VolumeGuid {
                general_flags: _,
                volume_guid: _,
            } => (&raw_volume_guid::RawVolumeGuid::from(self)).into(),
            Self::VolumeLabel {
                volume_label: _,
            } => (&raw_volume_label::RawVolumeLabel::from(self)).into(),
        }
    }
}

impl Into<Vec<u8>> for &DirectoryEntry {
    fn into(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.raw().to_vec();
        let mut tail_bytes: Vec<u8> = match self {
            DirectoryEntry::File {
                file_attributes: _,
                create_time: _,
                modified_time: _,
                accessed_time: _,
                stream_extension,
            } => stream_extension.as_ref().into(),
            DirectoryEntry::StreamExtension {
                general_flags: _,
                name_length: _,
                name_hash: _,
                first_cluster: _,
                data_length: _,
                file_name,
            } => file_name.as_ref().into(),
            DirectoryEntry::FileName {
                general_flags: _,
                file_name: _,
                next_file_name,
            } => match next_file_name {
                Some(next_file_name) => next_file_name.as_ref().into(),
                None => vec![],
            },
            DirectoryEntry::UpcaseTable {
                table_checksum: _,
                first_cluster: _,
                data_length: _,
                upcase_table: _,
            } => vec![],
            DirectoryEntry::VolumeLabel {
                volume_label: _,
            } => vec![],
            DirectoryEntry::VolumeGuid {
                general_flags: _,
                volume_guid: _,
            } => vec![],
            DirectoryEntry::AllocationBitmap {
                bitmap_identifier: _,
                first_cluster: _,
                data_length: _,
            } => vec![],
        };
        bytes.append(&mut tail_bytes);
        bytes
    }
}

