#include <iomanip>
#include <iostream>
#include "fsinfo_sector.hpp"

FSInfoSector::FSInfoSector(unsigned char const * const data)
{
	static const unsigned int hex_digits_per_byte = 2;
	for(unsigned int i = 0; i < size; i++)
	{
		if(i < signature_0_offset + signature_0_size)((unsigned char *)&signature_0)[i - signature_0_offset] = data[i];
		else if(i < reserved_0_offset + reserved_0_size)reserved_0[i - reserved_0_offset] = data[i];
		else if(i < signature_1_offset + signature_1_size)((unsigned char *)&signature_1)[i - signature_1_offset] = data[i];
		else if(i < free_clusters_offset + free_clusters_size)((unsigned char *)&free_clusters)[i - free_clusters_offset] = data[i];
		else if(i < last_cluster_offset + last_cluster_size)((unsigned char *)&last_cluster)[i - last_cluster_offset] = data[i];
		else if(i < reserved_1_offset + reserved_1_size)((unsigned char *)&reserved_1)[i - reserved_1_offset] = data[i];
		else if(i < signature_2_offset + signature_2_size)((unsigned char *)&signature_2)[i - signature_2_offset] = data[i];
	}
	std::cout << "signature_0 = 0x" << std::hex << std::setfill('0') << std::setw(sizeof(signature_0) * hex_digits_per_byte) << (unsigned int)signature_0 << std::endl;
	std::cout << "reserved_0 = ";
	for(unsigned int i = 0; i < sizeof(reserved_0) / sizeof(reserved_0[0]); i++)
	{
		if(i % 0x10 == 0)std::cout << std::endl;
		std::cout << std::hex << std::setfill('0') << std::setw(2) << (unsigned int)(reserved_0[i]) << " ";
	}
	std::cout << std::endl;
	std::cout << "signature_1 = 0x" << std::hex << std::setfill('1') << std::setw(sizeof(signature_1) * hex_digits_per_byte) << (unsigned int)signature_1 << std::endl;
	std::cout << "free_clusters = 0x" << std::hex << std::setfill('1') << std::setw(sizeof(free_clusters) * hex_digits_per_byte) << (unsigned int)free_clusters << std::endl;
	std::cout << "last_cluster = 0x" << std::hex << std::setfill('1') << std::setw(sizeof(last_cluster) * hex_digits_per_byte) << (unsigned int)last_cluster << std::endl;
	std::cout << "reserved_1 = ";
	for(unsigned int i = 0; i < sizeof(reserved_1) / sizeof(reserved_1[0]); i++)
	{
		if(i % 0x10 == 0)std::cout << std::endl;
		std::cout << std::hex << std::setfill('0') << std::setw(2) << (unsigned int)(reserved_1[i]) << " ";
	}
	std::cout << std::endl;
	std::cout << "signature_2 = 0x" << std::hex << std::setfill('1') << std::setw(sizeof(signature_2) * hex_digits_per_byte) << (unsigned int)signature_2 << std::endl;
}

