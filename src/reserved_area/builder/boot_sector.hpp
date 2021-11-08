#ifndef _BOOT_SECTOR_HPP_
#define _BOOT_SECTOR_HPP_

#include <string>

class BootSector
{
private:
	static const unsigned int size = 0x200;

	unsigned char jump_instructions[3];
	static const unsigned int jump_instructions_offset = 0;
	static const unsigned int jump_instructions_size = sizeof(jump_instructions);

	std::string oem_identifier;
	static const unsigned int oem_identifier_offset = jump_instructions_offset + jump_instructions_size;
	static const unsigned int oem_identifier_size = 8;

	unsigned short bytes_per_sector;
	static const unsigned int bytes_per_sector_offset = oem_identifier_offset + oem_identifier_size;
	static const unsigned int bytes_per_sector_size = sizeof(bytes_per_sector);

	unsigned char sectors_per_cluster;
	static const unsigned int sectors_per_cluster_offset = bytes_per_sector_offset + bytes_per_sector_size;
	static const unsigned int sectors_per_cluster_size = sizeof(sectors_per_cluster);

	unsigned short reserved_sectors;
	static const unsigned int reserved_sectors_offset = sectors_per_cluster_offset + sectors_per_cluster_size;
	static const unsigned int reserved_sectors_size = sizeof(reserved_sectors);

	unsigned char fats;
	static const unsigned int fats_offset = reserved_sectors_offset + reserved_sectors_size;
	static const unsigned int fats_size = sizeof(fats);

	unsigned short root_directory_entries;
	static const unsigned int root_directory_entries_offset = fats_offset + fats_size;
	static const unsigned int root_directory_entries_size = sizeof(root_directory_entries);

	unsigned short short_sectors;
	static const unsigned int short_sectors_offset = root_directory_entries_offset + root_directory_entries_size;
	static const unsigned int short_sectors_size = sizeof(short_sectors);

	unsigned char media_type;
	static const unsigned int media_type_offset = short_sectors_offset + short_sectors_size;
	static const unsigned int media_type_size = sizeof(media_type);

	unsigned short short_sectors_per_fat;
	static const unsigned int short_sectors_per_fat_offset = media_type_offset + media_type_size;
	static const unsigned int short_sectors_per_fat_size = sizeof(short_sectors_per_fat);

	unsigned short sectors_per_track;
	static const unsigned int sectors_per_track_offset = short_sectors_per_fat_offset + short_sectors_per_fat_size;
	static const unsigned int sectors_per_track_size = sizeof(sectors_per_track);

	unsigned short heads;
	static const unsigned int heads_offset = sectors_per_track_offset + sectors_per_track_size;
	static const unsigned int heads_size = sizeof(heads);

	unsigned int hidden_sectors;
	static const unsigned int hidden_sectors_offset = heads_offset + heads_size;
	static const unsigned int hidden_sectors_size = sizeof(hidden_sectors);

	unsigned int long_sectors;
	static const unsigned int long_sectors_offset = hidden_sectors_offset + hidden_sectors_size;
	static const unsigned int long_sectors_size = sizeof(long_sectors);

	unsigned int long_sectors_per_fat;
	static const unsigned int long_sectors_per_fat_offset = long_sectors_offset + long_sectors_size;
	static const unsigned int long_sectors_per_fat_size = sizeof(long_sectors_per_fat);

	unsigned short flags;
	static const unsigned int flags_offset = long_sectors_per_fat_offset + long_sectors_per_fat_size;
	static const unsigned int flags_size = sizeof(flags);

	unsigned short fat_version;
	static const unsigned int fat_version_offset = flags_offset + flags_size;
	static const unsigned int fat_version_size = sizeof(fat_version);

	unsigned int root_dir_cluster;
	static const unsigned int root_dir_cluster_offset = fat_version_offset + fat_version_size;
	static const unsigned int root_dir_cluster_size = sizeof(root_dir_cluster);

	unsigned short fsinfo_sector;
	static const unsigned int fsinfo_sector_offset = root_dir_cluster_offset + root_dir_cluster_size;
	static const unsigned int fsinfo_sector_size = sizeof(fsinfo_sector);

	unsigned short backup_boot_sector;
	static const unsigned int backup_boot_sector_offset = fsinfo_sector_offset + fsinfo_sector_size;
	static const unsigned int backup_boot_sector_size = sizeof(backup_boot_sector);

	unsigned char reserved_1[0x0c];
	static const unsigned int reserved_1_offset = backup_boot_sector_offset + backup_boot_sector_size;
	static const unsigned int reserved_1_size = sizeof(reserved_1);

	unsigned char drive_number;
	static const unsigned int drive_number_offset = reserved_1_offset + reserved_1_size;
	static const unsigned int drive_number_size = sizeof(drive_number);

	unsigned char reserved_2;
	static const unsigned int reserved_2_offset = drive_number_offset + drive_number_size;
	static const unsigned int reserved_2_size = sizeof(reserved_2);
public:
	BootSector(unsigned char const * const data);
};


#endif

