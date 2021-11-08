#include <iostream>

int main(int argc, char const * const * const argv)
{
	if(argc != 4)
	{
		std::cerr << "Usage : $ ./builder ../boot_sector.bin ../fsinfo_sector.bin ../reserved_area.bin" << std::endl;
	}
	return EXIT_SUCCESS;
}

