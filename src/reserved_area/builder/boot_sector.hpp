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
public:
	BootSector(unsigned char const * const data);
};


#endif

