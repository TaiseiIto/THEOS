#include <fstream>
#include <iomanip>
#include <iostream>
#include "boot_sector.hpp"
#include "fsinfo_sector.hpp"

void print_binary(unsigned char const * const data, unsigned int size);

int main(int argc, char const * const * const argv)
{
	char boot_sector_raw[0x200];
	char fsinfo_sector_raw[0x200];
	std::ifstream boot_sector_file;
	std::ifstream fsinfo_sector_file;
	if(argc != 5)
	{
		std::cerr << "Usage : $ ./genimg boot_sector/boot_sector.bin fsinfo_sector/fsinfo_sector.bin ../disk_root theos.img" << std::endl;
		return EXIT_FAILURE;
	}
	boot_sector_file.open(argv[1], std::ifstream::binary | std::ifstream::in);
	if(!boot_sector_file.is_open())
	{
		std::cerr << "Can't open" << argv[1] << std::endl;
		return EXIT_FAILURE;
	}
	boot_sector_file.read(boot_sector_raw, sizeof(boot_sector_raw));
	print_binary((unsigned char *)boot_sector_raw, sizeof(boot_sector_raw));
	boot_sector_file.close();
	BootSector boot_sector = BootSector((unsigned char *)boot_sector_raw);
	fsinfo_sector_file.open(argv[2], std::ifstream::binary | std::ifstream::in);
	if(!fsinfo_sector_file.is_open())
	{
		std::cerr << "Can't open" << argv[2] << std::endl;
		return EXIT_FAILURE;
	}
	fsinfo_sector_file.read(fsinfo_sector_raw, sizeof(fsinfo_sector_raw));
	print_binary((unsigned char *)fsinfo_sector_raw, sizeof(fsinfo_sector_raw));
	fsinfo_sector_file.close();
	FSInfoSector fsinfo_sector = FSInfoSector((unsigned char *)fsinfo_sector_raw);
	return EXIT_SUCCESS;
}

void print_binary(unsigned char const * const data, unsigned int size)
{
	for(unsigned int i = 0; i < size; i++)
	{
		if(i % 0x10 == 0)std::cout << std::endl;
		std::cout << std::hex << std::setfill('0') << std::setw(2) << (unsigned int)data[i] << " ";
	}
	std::cout << std::endl;
}

