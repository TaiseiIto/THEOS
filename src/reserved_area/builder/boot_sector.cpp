#include <cstdlib>
#include <iomanip>
#include <iostream>
#include "boot_sector.hpp"

BootSector::BootSector(unsigned char const * const data)
{
	static const unsigned int hex_digits_per_byte = 2;
	oem_identifier = "";
	volume_label = "";
	for(unsigned int i = 0; i < size; i++)
	{
		if(i < jump_instructions_offset + jump_instructions_size)jump_instructions[i - jump_instructions_offset] = data[i];
		else if(i < oem_identifier_offset + oem_identifier_size)oem_identifier += data[i];
		else if(i < bytes_per_sector_offset + bytes_per_sector_size)((unsigned char *)&bytes_per_sector)[i - bytes_per_sector_offset] = data[i];
		else if(i < sectors_per_cluster_offset + sectors_per_cluster_size)((unsigned char *)&sectors_per_cluster)[i - sectors_per_cluster_offset] = data[i];
		else if(i < reserved_sectors_offset + reserved_sectors_size)((unsigned char *)&reserved_sectors)[i - reserved_sectors_offset] = data[i];
		else if(i < fats_offset + fats_size)((unsigned char *)&fats)[i - fats_offset] = data[i];
		else if(i < root_directory_entries_offset + root_directory_entries_size)((unsigned char *)&root_directory_entries)[i - root_directory_entries_offset] = data[i];
		else if(i < short_sectors_offset + short_sectors_size)((unsigned char *)&short_sectors)[i - short_sectors_offset] = data[i];
		else if(i < media_type_offset + media_type_size)((unsigned char *)&media_type)[i - media_type_offset] = data[i];
		else if(i < short_sectors_per_fat_offset + short_sectors_per_fat_size)((unsigned char *)&short_sectors_per_fat)[i - short_sectors_per_fat_offset] = data[i];
		else if(i < sectors_per_track_offset + sectors_per_track_size)((unsigned char *)&sectors_per_track)[i - sectors_per_track_offset] = data[i];
		else if(i < heads_offset + heads_size)((unsigned char *)&heads)[i - heads_offset] = data[i];
		else if(i < hidden_sectors_offset + hidden_sectors_size)((unsigned char *)&hidden_sectors)[i - hidden_sectors_offset] = data[i];
		else if(i < long_sectors_offset + long_sectors_size)((unsigned char *)&long_sectors)[i - long_sectors_offset] = data[i];
		else if(i < long_sectors_per_fat_offset + long_sectors_per_fat_size)((unsigned char *)&long_sectors_per_fat)[i - long_sectors_per_fat_offset] = data[i];
		else if(i < flags_offset + flags_size)((unsigned char *)&flags)[i - flags_offset] = data[i];
		else if(i < fat_version_offset + fat_version_size)((unsigned char *)&fat_version)[i - fat_version_offset] = data[i];
		else if(i < root_dir_cluster_offset + root_dir_cluster_size)((unsigned char *)&root_dir_cluster)[i - root_dir_cluster_offset] = data[i];
		else if(i < fsinfo_sector_offset + fsinfo_sector_size)((unsigned char *)&fsinfo_sector)[i - fsinfo_sector_offset] = data[i];
		else if(i < backup_boot_sector_offset + backup_boot_sector_size)((unsigned char *)&backup_boot_sector)[i - backup_boot_sector_offset] = data[i];
		else if(i < reserved_1_offset + reserved_1_size)((unsigned char *)&reserved_1)[i - reserved_1_offset] = data[i];
		else if(i < drive_number_offset + drive_number_size)((unsigned char *)&drive_number)[i - drive_number_offset] = data[i];
		else if(i < reserved_2_offset + reserved_2_size)((unsigned char *)&reserved_2)[i - reserved_2_offset] = data[i];
		else if(i < signature_offset + signature_size)((unsigned char *)&signature)[i - signature_offset] = data[i];
		else if(i < volume_ID_offset + volume_ID_size)((unsigned char *)&volume_ID)[i - volume_ID_offset] = data[i];
		else if(i < volume_label_offset + volume_label_size)volume_label += data[i];
		else if(i < system_identifier_offset + system_identifier_size)system_identifier += data[i];
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
	std::cout << "short_sectors = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(short_sectors) * hex_digits_per_byte) << (unsigned int)short_sectors << std::endl;
	std::cout << "media_type = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(media_type) * hex_digits_per_byte) << (unsigned int)media_type << std::endl;
	std::cout << "short_sectors_per_fat = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(short_sectors_per_fat) * hex_digits_per_byte) << (unsigned int)short_sectors_per_fat << std::endl;
	std::cout << "sectors_per_track = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(sectors_per_track) * hex_digits_per_byte) << (unsigned int)sectors_per_track << std::endl;
	std::cout << "heads = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(heads) * hex_digits_per_byte) << (unsigned int)heads << std::endl;
	std::cout << "hidden_sectors = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(hidden_sectors) * hex_digits_per_byte) << (unsigned int)hidden_sectors << std::endl;
	std::cout << "long_sectors = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(long_sectors) * hex_digits_per_byte) << (unsigned int)long_sectors << std::endl;
	std::cout << "long_sectors_per_fat = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(long_sectors_per_fat) * hex_digits_per_byte) << (unsigned int)long_sectors_per_fat << std::endl;
	std::cout << "flags = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(flags) * hex_digits_per_byte) << (unsigned int)flags << std::endl;
	std::cout << "fat_version = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(fat_version) * hex_digits_per_byte) << (unsigned int)fat_version << std::endl;
	std::cout << "root_dir_cluster = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(root_dir_cluster) * hex_digits_per_byte) << (unsigned int)root_dir_cluster << std::endl;
	std::cout << "fsinfo_sector = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(fsinfo_sector) * hex_digits_per_byte) << (unsigned int)fsinfo_sector << std::endl;
	std::cout << "backup_boot_sector = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(backup_boot_sector) * hex_digits_per_byte) << (unsigned int)backup_boot_sector << std::endl;
	std::cout << "reserved_1 = ";
	for(unsigned int i = 0; i < sizeof(reserved_1) / sizeof(reserved_1[0]); i++)std::cout << std::hex << std::setfill('0') << std::setw(2) << (unsigned int)reserved_1[i] << " ";
	std::cout << std::endl;
	std::cout << "drive_number = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(drive_number) * hex_digits_per_byte) << (unsigned int)drive_number << std::endl;
	std::cout << "reserved_2 = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(reserved_2) * hex_digits_per_byte) << (unsigned int)reserved_2 << std::endl;
	std::cout << "signature = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(signature) * hex_digits_per_byte) << (unsigned int)signature << std::endl;
	std::cout << "volume_ID = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(volume_ID) * hex_digits_per_byte) << (unsigned int)volume_ID << std::endl;
	std::cout << "volume_label = \"" << volume_label << "\"" << std::endl;
	std::cout << "system_identifier = \"" << system_identifier << "\"" << std::endl;
}

