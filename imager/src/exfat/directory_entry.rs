use {
    std::{
        char,
        collections::VecDeque,
        ffi,
        mem,
        path::PathBuf,
        str,
    },
    super::{
        allocation_bitmap,
        cluster,
        super::{
            binary::Binary,
            time,
        },
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
        file_attributes: FileAttributes,
        create_time: time::Time,
        modified_time: time::Time,
        accessed_time: time::Time,
        stream_extension: Box<Self>,
    },
    FileName {
        general_flags: GeneralFlags,
        file_name: [u16; FILE_NAME_BLOCK_LENGTH],
        next_file_name: Option<Box<Self>>,
    },
    StreamExtension {
        general_flags: GeneralFlags,
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
        general_flags: GeneralFlags,
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
        let num_of_clusters: usize = num_of_clusters + num_of_fats * num_of_allocation_bitmap_clusters;
        let allocation_bitmaps: Vec<allocation_bitmap::AllocationBitmap> = (0..num_of_fats)
            .map(|_| allocation_bitmap::AllocationBitmap::new(clusters))
            .collect();
        let allocation_bitmaps: Vec<Vec<u8>> = allocation_bitmaps
            .into_iter()
            .map(|allocation_bitmap| allocation_bitmap.to_bytes())
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
        let file_attributes = FileAttributes::new(path);
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
                let type_code = TypeCode::read(type_code);
                match type_code {
                    TypeCode::File => {
                        let file = RawFile::read(&directory_entry);
                        let file_attributes: [u8; 2] = directory_entry[4..6]
                            .try_into()
                            .expect("Can't read a file directory entry.");
                        let file_attributes: u16 = unsafe {
                            mem::transmute::<[u8; 2], u16>(file_attributes)
                        };
                        let file_attributes = FileAttributes::read(file_attributes);
                        let create_time = time::Time::from_fat_timestamp(file.create_timestamp, file.create_10ms_increment, file.create_utc_offset);
                        let modified_time = time::Time::from_fat_timestamp(file.last_modified_timestamp, file.last_modified_10ms_increment, file.last_modified_utc_offset);
                        let accessed_time = time::Time::from_fat_timestamp(file.last_accessed_timestamp, 0, file.last_accessed_utc_offset);
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
                    TypeCode::StreamExtension => {
                        let stream_extension = RawStreamExtension::read(&directory_entry);
                        let general_flags = GeneralFlags::read(stream_extension.general_flags);
                        let name_length: u8 = stream_extension.name_length;
                        let name_hash: u16 = stream_extension.name_hash;
                        let first_cluster: u32 = stream_extension.first_cluster;
                        let data_length: usize = stream_extension.data_length as usize;
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
                    TypeCode::FileName => {
                        let file_name = RawFileName::read(&directory_entry);
                        let general_flags = GeneralFlags::read(file_name.general_flags);
                        let file_name: [u16; FILE_NAME_BLOCK_LENGTH] = file_name.file_name;
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
                    TypeCode::UpcaseTable => {
                        let upcase_table = RawUpcaseTable::read(&directory_entry);
                        let table_checksum: u32 = upcase_table.table_checksum;
                        let first_cluster: u32 = upcase_table.first_cluster;
                        let data_length: usize = upcase_table.data_length as usize;
                        let upcase_table: upcase_table::UpcaseTable = clusters.upcase_table(first_cluster, data_length);
                        Some(Self::UpcaseTable {
                            table_checksum,
                            first_cluster,
                            data_length,
                            upcase_table,
                        })
                    },
                    TypeCode::VolumeLabel => {
                        let volume_label = RawVolumeLabel::read(&directory_entry);
                        let character_count: usize = volume_label.character_count as usize;
                        let volume_label: [u16; VOLUME_LABEL_MAX_LENGTH] = volume_label.volume_label;
                        let volume_label: String = char::decode_utf16(volume_label[0..character_count].iter().cloned())
                            .filter_map(|c| c.ok())
                            .collect();
                        Some(Self::VolumeLabel {
                            volume_label,
                        })
                    },
                    TypeCode::VolumeGuid => {
                        let volume_guid = RawVolumeGuid::read(&directory_entry);
                        let general_flags = GeneralFlags::read(volume_guid.general_flags as u8);
                        let volume_guid: u128 = volume_guid.volume_guid;
                        Some(Self::VolumeGuid {
                            general_flags,
                            volume_guid,
                        })
                    },
                    TypeCode::AllocationBitmap => {
                        let allocation_bitmap = RawAllocationBitmap::read(&directory_entry);
                        let bitmap_identifier: bool = allocation_bitmap.bitmap_flags & 0x01 != 0;
                        let first_cluster: u32 = allocation_bitmap.first_cluster;
                        let data_length: usize = allocation_bitmap.data_length as usize;
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
        let bytes: Vec<u8> = upcase_table.to_bytes();
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
        let general_flags = GeneralFlags::volume_guid();
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

    fn entry_type(&self) -> EntryType {
        match self {
            Self::AllocationBitmap {
                bitmap_identifier: _,
                first_cluster: _,
                data_length: _,
            } => EntryType::allocation_bitmap(),
            Self::File {
                file_attributes: _,
                create_time: _,
                modified_time: _,
                accessed_time: _,
                stream_extension: _,
            } => EntryType::file(),
            Self::FileName {
                general_flags: _,
                file_name: _,
                next_file_name: _,
            } => EntryType::file_name(),
            Self::StreamExtension {
                general_flags: _,
                name_length: _,
                name_hash: _,
                first_cluster: _,
                data_length: _,
                file_name: _,
            } => EntryType::stream_extension(),
            Self::UpcaseTable {
                table_checksum: _,
                first_cluster: _,
                data_length: _,
                upcase_table: _,
            } => EntryType::upcase_table(),
            Self::VolumeGuid {
                general_flags: _,
                volume_guid: _,
            } => EntryType::volume_guid(),
            Self::VolumeLabel {
                volume_label: _,
            } => EntryType::volume_label(),
        }
    }

    fn file_name(mut file_name: Vec<u16>) -> Self {
        let general_flags = GeneralFlags::file_name();
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
        let general_flags = GeneralFlags::stream_extension();
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
            } => RawAllocationBitmap::new(self).raw(),
            Self::File {
                file_attributes: _,
                create_time: _,
                modified_time: _,
                accessed_time: _,
                stream_extension: _,
            } => RawFile::new(self).raw(),
            Self::FileName {
                general_flags: _,
                file_name: _,
                next_file_name: _,
            } => RawFileName::new(self).raw(),
            Self::StreamExtension {
                general_flags: _,
                name_length: _,
                name_hash: _,
                first_cluster: _,
                data_length: _,
                file_name: _,
            } => RawStreamExtension::new(self).raw(),
            Self::UpcaseTable {
                table_checksum: _,
                first_cluster: _,
                data_length: _,
                upcase_table: _,
            } => RawUpcaseTable::new(self).raw(),
            Self::VolumeGuid {
                general_flags: _,
                volume_guid: _,
            } => RawVolumeGuid::new(self).raw(),
            Self::VolumeLabel {
                volume_label: _,
            } => RawVolumeLabel::new(self).raw(),
        }
    }
}

impl Binary for DirectoryEntry {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.raw().to_vec();
        let mut tail_bytes: Vec<u8> = match self {
            Self::File {
                file_attributes: _,
                create_time: _,
                modified_time: _,
                accessed_time: _,
                stream_extension,
            } => stream_extension.to_bytes(),
            Self::StreamExtension {
                general_flags: _,
                name_length: _,
                name_hash: _,
                first_cluster: _,
                data_length: _,
                file_name,
            } => file_name.to_bytes(),
            Self::FileName {
                general_flags: _,
                file_name: _,
                next_file_name,
            } => match next_file_name {
                Some(next_file_name) => next_file_name.to_bytes(),
                None => vec![],
            },
            Self::UpcaseTable {
                table_checksum: _,
                first_cluster: _,
                data_length: _,
                upcase_table: _,
            } => vec![],
            Self::VolumeLabel {
                volume_label: _,
            } => vec![],
            Self::VolumeGuid {
                general_flags: _,
                volume_guid: _,
            } => vec![],
            Self::AllocationBitmap {
                bitmap_identifier: _,
                first_cluster: _,
                data_length: _,
            } => vec![],
        };
        bytes.append(&mut tail_bytes);
        bytes
    }
}

#[derive(Debug)]
struct EntryType {
    type_code: TypeCode,
    type_importance: bool,
    type_category: bool,
    in_use: bool,
}

impl EntryType {
    fn allocation_bitmap() -> Self {
        let type_code = TypeCode::AllocationBitmap;
        let type_importance = false;
        let type_category = false;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    fn file() -> Self {
        let type_code = TypeCode::File;
        let type_importance = false;
        let type_category = false;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    fn file_name() -> Self {
        let type_code = TypeCode::FileName;
        let type_importance = false;
        let type_category = true;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    fn stream_extension() -> Self {
        let type_code = TypeCode::StreamExtension;
        let type_importance = false;
        let type_category = true;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    fn to_byte(&self) -> u8 {
        let type_code: u8 = self.type_code.to_byte();
        let type_importance: u8 = if self.type_importance {
            1 << 5
        } else {
            0
        };
        let type_category: u8 = if self.type_category {
            1 << 6
        } else {
            0
        };
        let in_use: u8 = if self.in_use {
            1 << 7
        } else {
            0
        };
        type_code + type_importance + type_category + in_use
    }

    fn upcase_table() -> Self {
        let type_code = TypeCode::UpcaseTable;
        let type_importance = false;
        let type_category = false;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    fn volume_guid() -> Self {
        let type_code = TypeCode::VolumeGuid;
        let type_importance = true;
        let type_category = false;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    fn volume_label() -> Self {
        let type_code = TypeCode::VolumeLabel;
        let type_importance = false;
        let type_category = false;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FileAttributes {
    read_only: bool,
    hidden: bool,
    system: bool,
    directory: bool,
    archive: bool,
}

impl FileAttributes {
    pub fn is_dir(&self) -> bool {
        self.directory
    }

    fn new(path: &PathBuf) -> Self {
        let read_only = true;
        let hidden = false;
        let system = true;
        let directory = path.is_dir();
        let archive = false;
        Self {
            read_only,
            hidden,
            system,
            directory,
            archive,
        }
    }

    fn to_word(&self) -> u16 {
        let read_only: u16 = match self.read_only {
            true => 1,
            false => 0,
        };
        let hidden: u16 = match self.hidden {
            true => 1 << 1,
            false => 0,
        };
        let system: u16 = match self.system {
            true => 1 << 2,
            false => 0,
        };
        let directory: u16 = match self.directory {
            true => 1 << 4,
            false => 0,
        };
        let archive: u16 = match self.archive {
            true => 1 << 5,
            false => 0,
        };
        read_only + hidden + system + directory + archive
    }

    fn read(word: u16) -> Self {
        let read_only: bool = word & 0x0001 != 0;
        let hidden: bool = word & 0x0002 != 0;
        let system: bool = word & 0x0004 != 0;
        let directory: bool = word & 0x0010 != 0;
        let archive: bool = word & 0x0020 != 0;
        Self {
            read_only,
            hidden,
            system,
            directory,
            archive,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GeneralFlags {
    allocation_possible: bool,
    no_fat_chain: bool,
}

impl GeneralFlags {
    fn file_name() -> Self {
        let allocation_possible = false;
        let no_fat_chain = false;
        Self {
            allocation_possible,
            no_fat_chain,
        }
    }

    fn read(byte: u8) -> Self {
        let allocation_possible: bool = byte & 0x01 != 0;
        let no_fat_chain: bool = byte & 0x02 != 0;
        Self {
            allocation_possible,
            no_fat_chain,
        }
    }

    fn stream_extension() -> Self {
        let allocation_possible = true;
        let no_fat_chain = false;
        Self {
            allocation_possible,
            no_fat_chain,
        }
    }

    fn to_byte(&self) -> u8 {
        let allocation_possible = if self.allocation_possible {
            1
        } else {
            0
        };
        let no_fat_chain = if self.no_fat_chain {
            2
        } else {
            0
        };
        allocation_possible + no_fat_chain
    }

    fn volume_guid() -> Self {
        let allocation_possible = false;
        let no_fat_chain = true;
        Self {
            allocation_possible,
            no_fat_chain,
        }
    }
}

#[derive(Debug)]
enum TypeCode {
    File,
    StreamExtension,
    FileName,
    UpcaseTable,
    VolumeLabel,
    VolumeGuid,
    AllocationBitmap,
}

impl TypeCode {
    fn read(byte: u8) -> Self {
        let type_code: u8 = byte & 0x1f;
        let type_category: bool = byte & 0x40 != 0;
        match type_code {
            0x00 => if type_category {
                Self::StreamExtension
            } else {
                Self::VolumeGuid
            },
            0x01 => if type_category {
                Self::FileName
            } else {
                Self::AllocationBitmap
            },
            0x02 => Self::UpcaseTable,
            0x03 => Self::VolumeLabel,
            0x05 => Self::File,
            _ => panic!("Can't read type code."),
        }
    }

    fn to_byte(&self) -> u8 {
        match self {
            Self::File => 0x05,
            Self::StreamExtension => 0x00,
            Self::FileName => 0x01,
            Self::UpcaseTable => 0x02,
            Self::VolumeLabel => 0x03,
            Self::VolumeGuid => 0x00,
            Self::AllocationBitmap => 0x01,
        }
    }
}

trait Raw {
    fn new(directory_entry: &DirectoryEntry) -> Self;
    fn raw(&self) -> [u8; DIRECTORY_ENTRY_SIZE];
    fn read(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self;
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(packed)]
struct RawAllocationBitmap {
    entry_type: u8,
    bitmap_flags: u8,
    reserved: [u8; 0x12],
    first_cluster: u32,
    data_length: u64,
}

impl Raw for RawAllocationBitmap {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::AllocationBitmap {
                bitmap_identifier,
                first_cluster,
                data_length,
            } => {
                let bitmap_flags: u8 = match bitmap_identifier {
                    true => 0x01,
                    false => 0x00,
                };
                let reserved: [u8; 0x12] = [0; 0x12];
                let first_cluster: u32 = *first_cluster;
                let data_length: u64 = *data_length as u64;
                Self {
                    entry_type,
                    bitmap_flags,
                    reserved,
                    first_cluster,
                    data_length,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawAllocationBitmap."),
        }
    }

    fn raw(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }

    fn read(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(packed)]
struct RawFile {
    entry_type: u8,
    secondary_count: u8,
    set_checksum: u16,
    file_attributes: u16,
    reserved_1: u16,
    create_timestamp: u32,
    last_modified_timestamp: u32,
    last_accessed_timestamp: u32,
    create_10ms_increment: u8,
    last_modified_10ms_increment: u8,
    create_utc_offset: i8,
    last_modified_utc_offset: i8,
    last_accessed_utc_offset: i8,
    reserved_2: [u8; 7],
}

impl Raw for RawFile {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::File {
                file_attributes,
                create_time,
                modified_time,
                accessed_time,
                stream_extension,
            } => {
                let secondary_count: u8 = stream_extension.directory_entry_set_length() as u8;
                let set_checksum: u16 = 0;
                let file_attributes: u16 = file_attributes.to_word();
                let reserved_1: u16 = 0;
                let create_timestamp: u32 = create_time.fat_timestamp();
                let last_modified_timestamp: u32 = modified_time.fat_timestamp();
                let last_accessed_timestamp: u32 = accessed_time.fat_timestamp();
                let create_10ms_increment: u8 = create_time.get_10ms_increment();
                let last_modified_10ms_increment: u8 = modified_time.get_10ms_increment();
                let create_utc_offset: i8 = create_time.utc_offset();
                let last_modified_utc_offset: i8 = modified_time.utc_offset();
                let last_accessed_utc_offset: i8 = accessed_time.utc_offset();
                let reserved_2: [u8; 7] = [0; 7];
                let raw_file = Self {
                    entry_type,
                    secondary_count,
                    set_checksum,
                    file_attributes,
                    reserved_1,
                    create_timestamp,
                    last_modified_timestamp,
                    last_accessed_timestamp,
                    create_10ms_increment,
                    last_modified_10ms_increment,
                    create_utc_offset,
                    last_modified_utc_offset,
                    last_accessed_utc_offset,
                    reserved_2,
                };
                let mut bytes: Vec<u8> = raw_file.raw().to_vec();
                let mut tail_bytes: Vec<u8> = stream_extension.to_bytes();
                bytes.append(&mut tail_bytes);
                let set_checksum: u16 = bytes
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| *i != 2 && *i != 3)
                    .map(|(_, byte)| byte)
                    .fold(0u16, |checksum, byte| (checksum << 15) + (checksum >> 1) + byte as u16);
                Self {
                    entry_type,
                    secondary_count,
                    set_checksum,
                    file_attributes,
                    reserved_1,
                    create_timestamp,
                    last_modified_timestamp,
                    last_accessed_timestamp,
                    create_10ms_increment,
                    last_modified_10ms_increment,
                    create_utc_offset,
                    last_modified_utc_offset,
                    last_accessed_utc_offset,
                    reserved_2,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawFile."),
        }
    }

    fn raw(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }

    fn read(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(packed)]
struct RawFileName {
    entry_type: u8,
    general_flags: u8,
    file_name: [u16; FILE_NAME_BLOCK_LENGTH],
}

impl Raw for RawFileName {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::FileName {
                general_flags,
                file_name,
                next_file_name: _,
            } => {
                let general_flags: u8 = general_flags.to_byte();
                let file_name: [u16; FILE_NAME_BLOCK_LENGTH] = *file_name;
                Self {
                    entry_type,
                    general_flags,
                    file_name,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawFileName."),
        }
    }

    fn raw(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }

    fn read(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(packed)]
struct RawStreamExtension {
    entry_type: u8,
    general_flags: u8,
    reserved_1: u8,
    name_length: u8,
    name_hash: u16,
    reserved_2: u16,
    valid_data_length: u64,
    reserved_3: u32,
    first_cluster: u32,
    data_length: u64,
}

impl Raw for RawStreamExtension {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::StreamExtension {
                general_flags,
                name_length,
                name_hash,
                first_cluster,
                data_length,
                file_name: _,
            } => {
                let general_flags: u8 = general_flags.to_byte();
                let reserved_1: u8 = 0;
                let name_length: u8 = *name_length;
                let name_hash: u16 = *name_hash;
                let reserved_2: u16 = 0;
                let reserved_3: u32 = 0;
                let data_length: u64 = *data_length as u64;
                let valid_data_length: u64 = data_length;
                let first_cluster: u32 = *first_cluster;
                Self {
                    entry_type,
                    general_flags,
                    reserved_1,
                    name_length,
                    name_hash,
                    reserved_2,
                    valid_data_length,
                    reserved_3,
                    first_cluster,
                    data_length,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawStreamExtension."),
        }
    }

    fn raw(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }

    fn read(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(packed)]
struct RawUpcaseTable {
    entry_type: u8,
    reserved_1: [u8; 0x3],
    table_checksum: u32,
    reserved_2: [u8; 0xc],
    first_cluster: u32,
    data_length: u64,
}

impl Raw for RawUpcaseTable {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::UpcaseTable {
                table_checksum,
                first_cluster,
                data_length,
                upcase_table: _,
            } => {
                let reserved_1: [u8; 0x3] = [0x0; 0x3];
                let table_checksum: u32 = *table_checksum;
                let reserved_2: [u8; 0xc] = [0x0; 0xc];
                let first_cluster: u32 = *first_cluster;
                let data_length: u64 = *data_length as u64;
                Self {
                    entry_type,
                    reserved_1,
                    table_checksum,
                    reserved_2,
                    first_cluster,
                    data_length,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawUpcaseTable."),
        }
    }

    fn raw(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }

    fn read(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(packed)]
struct RawVolumeGuid {
    entry_type: u8,
    secondary_count: u8,
    set_checksum: u16,
    general_flags: u16,
    volume_guid: u128,
    reserved: [u8; 0xa],
}

impl Raw for RawVolumeGuid {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::VolumeGuid {
                general_flags,
                volume_guid,
            } => {
                let secondary_count: u8 = 0;
                let set_checksum: u16 = 0;
                let general_flags: u16 = general_flags.to_byte() as u16;
                let volume_guid: u128 = *volume_guid;
                let reserved: [u8; 0xa] = [0; 0xa];
                let raw_volume_guid = Self {
                    entry_type,
                    secondary_count,
                    set_checksum,
                    general_flags,
                    volume_guid,
                    reserved,
                };
                let bytes: Vec<u8> = raw_volume_guid.raw().to_vec();
                let set_checksum: u16 = bytes
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| *i != 2 && *i != 3)
                    .map(|(_, byte)| byte)
                    .fold(0u16, |checksum, byte| (checksum << 15) + (checksum >> 1) + byte as u16);
                Self {
                    entry_type,
                    secondary_count,
                    set_checksum,
                    general_flags,
                    volume_guid,
                    reserved,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawVolumeGuid."),
        }
    }

    fn raw(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }

    fn read(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

const VOLUME_LABEL_MAX_LENGTH: usize = 11;

#[allow(dead_code)]
#[repr(packed)]
#[derive(Copy, Clone)]
struct RawVolumeLabel {
    entry_type: u8,
    character_count: u8,
    volume_label: [u16; VOLUME_LABEL_MAX_LENGTH],
    reserved: u64,
}

impl Raw for RawVolumeLabel {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::VolumeLabel {
                volume_label,
            } => {
                let mut volume_label: Vec<u16> = volume_label
                    .encode_utf16()
                    .collect();
                let character_count = volume_label.len() as u8;
                while volume_label.len() < VOLUME_LABEL_MAX_LENGTH {
                    volume_label.push(0x0000);
                }
                let (volume_label, _): (&[u16], &[u16]) = volume_label.split_at(VOLUME_LABEL_MAX_LENGTH);
                let volume_label: [u16; VOLUME_LABEL_MAX_LENGTH] = volume_label
                    .try_into()
                    .expect("Can't convert volume label into [u16; VOLUME_LABEL_MAX_LENGTH].");
                let reserved: u64 = 0;
                Self {
                    entry_type,
                    character_count,
                    volume_label,
                    reserved,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawVolumeLabel."),
        }
    }

    fn raw(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }

    fn read(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

