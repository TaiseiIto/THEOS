#ifndef _FSINFO_SECTOR_HPP_
#define _FSINFO_SECTOR_HPP_

class FSInfoSector
{
private:
	static const unsigned int size = 0x0200;

	unsigned int signature_0;
	static const unsigned int signature_0_offset = 0;
	static const unsigned int signature_0_size = sizeof(signature_0);

	unsigned char reserved_0[0x01e0];
	static const unsigned int reserved_0_offset = signature_0_offset + signature_0_size;
	static const unsigned int reserved_0_size = sizeof(reserved_0);

	unsigned int signature_1;
	static const unsigned int signature_1_offset = reserved_0_offset + reserved_0_size;
	static const unsigned int signature_1_size = sizeof(signature_1);

	unsigned int free_clusters;
	static const unsigned int free_clusters_offset = signature_1_offset + signature_1_size;
	static const unsigned int free_clusters_size = sizeof(free_clusters);

	unsigned int last_cluster;
	static const unsigned int last_cluster_offset = free_clusters_offset + free_clusters_size;
	static const unsigned int last_cluster_size = sizeof(last_cluster);

	unsigned char reserved_1[0x0c];
	static const unsigned int reserved_1_offset = last_cluster_offset + last_cluster_size;
	static const unsigned int reserved_1_size = sizeof(reserved_1);

	unsigned int signature_2;
	static const unsigned int signature_2_offset = reserved_1_offset + reserved_1_size;
	static const unsigned int signature_2_size = sizeof(signature_2);
public:
	FSInfoSector(unsigned char const * const data);
};

#endif

