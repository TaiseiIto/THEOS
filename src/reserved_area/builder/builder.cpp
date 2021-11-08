#include <fstream>
#include <iostream>

int main(int argc, char const * const * const argv)
{
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
	fsinfo_sector_file.open(argv[2], std::ifstream::binary | std::ifstream::in);
	if(!fsinfo_sector_file.is_open())
	{
		std::cerr << "Can't open" << argv[2] << std::endl;
		return EXIT_FAILURE;
	}
	return EXIT_SUCCESS;
}

