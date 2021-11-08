#ifndef _BOOT_SECTOR_HPP_
#define _BOOT_SECTOR_HPP_

#include <string>

class BootSector
{
private:
	static const unsigned int size = 0x200;

	static const unsigned int jump_instructions_offset = 0;
	static const unsigned int jump_instructions_size = 3;
	unsigned char jump_instructions[3];

	static const unsigned int oem_identifier_offset = jump_instructions_offset + jump_instructions_size;
	static const unsigned int oem_identifier_size = 8;
	std::string oem_identifier;

	static const unsigned int bytes_per_sector_offset = oem_identifier_offset + oem_identifier_size;
	static const unsigned int bytes_per_sector_size = 2;
	unsigned short bytes_per_sector;

	static const unsigned int sectors_per_cluster_offset = bytes_per_sector_offset + bytes_per_sector_size;
	static const unsigned int sectors_per_cluster_size = 1;
	unsigned char sectors_per_cluster;
public:
	BootSector(unsigned char const * const data);
};


#endif

