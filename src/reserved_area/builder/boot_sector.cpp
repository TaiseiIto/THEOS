#include <cstdlib>
#include <iomanip>
#include <iostream>
#include "boot_sector.hpp"

BootSector::BootSector(unsigned char const * const data)
{
	for(unsigned int i = 0; i < 0x200; i++)
	{
		if(i < jump_instructions_offset + jump_instructions_size)jump_instructions[i - jump_instructions_offset] = data[i];
	}
	std::cout << "jump instructions = ";
	for(unsigned int i = 0; i < sizeof(jump_instructions) / sizeof(jump_instructions[0]); i++)std::cout << std::hex << std::setfill('0') << std::setw(2) << (unsigned int)jump_instructions[i] << " ";
	std::cout << std::endl;
}

