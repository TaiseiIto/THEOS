#include <cstdlib>
#include <iomanip>
#include <iostream>
#include "boot_sector.hpp"

BootSector::BootSector(unsigned char const * const data)
{
	static const unsigned int hex_digits_per_byte = 2;
	oem_identifier = "";
	for(unsigned int i = 0; i < size; i++)
	{
		if(i < jump_instructions_offset + jump_instructions_size)jump_instructions[i - jump_instructions_offset] = data[i];
		else if(i < oem_identifier_offset + oem_identifier_size)oem_identifier += data[i];
		else if(i < bytes_per_sector_offset + bytes_per_sector_size)((unsigned char *)&bytes_per_sector)[i - bytes_per_sector_offset] = data[i];
		else if(i < sectors_per_cluster_offset + sectors_per_cluster_size)((unsigned char *)&sectors_per_cluster)[i - sectors_per_cluster_offset] = data[i];
		else if(i < reserved_sectors_offset + reserved_sectors_size)((unsigned char *)&reserved_sectors)[i - reserved_sectors_offset] = data[i];
		else if(i < fats_offset + fats_size)((unsigned char *)&fats)[i - fats_offset] = data[i];
		else if(i < root_directory_entries_offset + root_directory_entries_size)((unsigned char *)&root_directory_entries)[i - root_directory_entries_offset] = data[i];
		else if(i < sectors_offset + sectors_size)((unsigned char *)&sectors)[i - sectors_offset] = data[i];
		else if(i < media_type_offset + media_type_size)((unsigned char *)&media_type)[i - media_type_offset] = data[i];
		else if(i < sectors_per_fat_offset + sectors_per_fat_size)((unsigned char *)&sectors_per_fat)[i - sectors_per_fat_offset] = data[i];
		else if(i < sectors_per_track_offset + sectors_per_track_size)((unsigned char *)&sectors_per_track)[i - sectors_per_track_offset] = data[i];
		else if(i < heads_offset + heads_size)((unsigned char *)&heads)[i - heads_offset] = data[i];
	}

	std::cout << "jump_instructions = ";
	for(unsigned int i = 0; i < sizeof(jump_instructions) / sizeof(jump_instructions[0]); i++)std::cout << std::hex << std::setfill('0') << std::setw(2) << (unsigned int)jump_instructions[i] << " ";
	std::cout << std::endl;
	std::cout << "oem_identifier = \"" << oem_identifier << "\"" << std::endl;
	std::cout << "bytes_per_sector = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(bytes_per_sector) * hex_digits_per_byte) << (unsigned int)bytes_per_sector << std::endl;
	std::cout << "sectors_per_cluster = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(sectors_per_cluster) * hex_digits_per_byte) << (unsigned int)sectors_per_cluster << std::endl;
	std::cout << "reserved_sectors = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(reserved_sectors) * hex_digits_per_byte) << (unsigned int)reserved_sectors << std::endl;
	std::cout << "fats = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(fats) * hex_digits_per_byte) << (unsigned int)fats << std::endl;
	std::cout << "root_directory_entries = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(root_directory_entries) * hex_digits_per_byte) << (unsigned int)root_directory_entries << std::endl;
	std::cout << "sectors = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(sectors) * hex_digits_per_byte) << (unsigned int)sectors << std::endl;
	std::cout << "media_type = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(media_type) * hex_digits_per_byte) << (unsigned int)media_type << std::endl;
	std::cout << "sectors_per_fat = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(sectors_per_fat) * hex_digits_per_byte) << (unsigned int)sectors_per_fat << std::endl;
	std::cout << "sectors_per_track = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(sectors_per_track) * hex_digits_per_byte) << (unsigned int)sectors_per_track << std::endl;
	std::cout << "heads = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(heads) * hex_digits_per_byte) << (unsigned int)heads << std::endl;
}

