#include <fstream>
#include <iomanip>
#include <iostream>

void print_binary(unsigned char const * const data, unsigned int size);

int main(int argc, char const * const * const argv)
{
	char boot_sector[0x200];
	char fsinfo_sector[0x200];
	std::ifstream boot_sector_file;
	std::ifstream fsinfo_sector_file;
	if(argc != 4)
	{
		std::cerr << "Usage : $ ./builder ../boot_sector.bin ../fsinfo_sector.bin ../reserved_area.bin" << std::endl;
		return EXIT_FAILURE;
	}
	boot_sector_file.open(argv[1], std::ifstream::binary | std::ifstream::in);
	if(!boot_sector_file.is_open())
	{
		std::cerr << "Can't open" << argv[1] << std::endl;
		return EXIT_FAILURE;
	}
	boot_sector_file.read(boot_sector, sizeof(boot_sector));
	print_binary((unsigned char *)boot_sector, sizeof(boot_sector));
	boot_sector_file.close();
	fsinfo_sector_file.open(argv[2], std::ifstream::binary | std::ifstream::in);
	if(!fsinfo_sector_file.is_open())
	{
		std::cerr << "Can't open" << argv[2] << std::endl;
		return EXIT_FAILURE;
	}
	fsinfo_sector_file.read(fsinfo_sector, sizeof(fsinfo_sector));
	print_binary((unsigned char *)fsinfo_sector, sizeof(fsinfo_sector));
	fsinfo_sector_file.close();
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

