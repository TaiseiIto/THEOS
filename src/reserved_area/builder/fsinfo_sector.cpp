#include <iomanip>
#include <iostream>
#include "fsinfo_sector.hpp"

FSInfoSector::FSInfoSector(unsigned char const * const data)
{
	static const unsigned int hex_digits_per_byte = 2;
	for(unsigned int i = 0; i < size; i++)
	{
		if(i < signature_0_offset + signature_0_size)((unsigned char *)&signature_0)[i - signature_0_offset] = data[i];
	}
	std::cout << "signature_0 = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(signature_0) * hex_digits_per_byte) << (unsigned int)signature_0 << std::endl;
}

