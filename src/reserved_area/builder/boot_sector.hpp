#ifndef _BOOT_SECTOR_HPP_
#define _BOOT_SECTOR_HPP_


class BootSector
{
private:
	static const unsigned int jump_instructions_offset = 0;
	static const unsigned int jump_instructions_size = 3;
	unsigned char jump_instructions[3];
public:
	BootSector(unsigned char const * const data);
};


#endif

