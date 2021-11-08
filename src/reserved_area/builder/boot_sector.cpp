#include <cstdlib>
#include <iomanip>
#include <iostream>
#include "boot_sector.hpp"

BootSector::BootSector(unsigned char const * const data)
{
	oem_identifier = "";
	for(unsigned int i = 0; i < size; i++)
	{
		if(i < jump_instructions_offset + jump_instructions_size)jump_instructions[i - jump_instructions_offset] = data[i];
		else if(i < oem_identifier_offset + oem_identifier_size)oem_identifier += data[i];
		else if(i < bytes_per_sector_offset + bytes_per_sector_size)((unsigned char *)&bytes_per_sector)[i - bytes_per_sector_offset] = data[i];
		else if(i < sectors_per_cluster_offset + sectors_per_cluster_size)((unsigned char *)&sectors_per_cluster)[i - sectors_per_cluster_offset] = data[i];
	}

	std::cout << "jump instructions = ";
	for(unsigned int i = 0; i < sizeof(jump_instructions) / sizeof(jump_instructions[0]); i++)std::cout << std::hex << std::setfill('0') << std::setw(2) << (unsigned int)jump_instructions[i] << " ";
	std::cout << std::endl;

	std::cout << "oem identifier = \"" << oem_identifier << "\"" << std::endl;
	std::cout << "bytes per sector = 0x" << std::hex << std::setfill('0') << std::setw(4) << (unsigned int)bytes_per_sector << std::endl;

	std::cout << "sectors per cluster = 0x" << std::hex << std::setfill('0') << std::setw(2) << (unsigned int)sectors_per_cluster << std::endl;
}

