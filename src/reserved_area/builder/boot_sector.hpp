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

	unsigned short sectors;
	static const unsigned int sectors_offset = root_directory_entries_offset + root_directory_entries_size;
	static const unsigned int sectors_size = sizeof(sectors);

	unsigned char media_type;
	static const unsigned int media_type_offset = sectors_offset + sectors_size;
	static const unsigned int media_type_size = sizeof(media_type);
public:
	BootSector(unsigned char const * const data);
};


#endif

