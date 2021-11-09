#ifndef _FSINFO_SECTOR_HPP_
#define _FSINFO_SECTOR_HPP_

class FSInfoSector
{
private:
	static const unsigned int size = 0x0200;

	unsigned int signature_0;
	static const unsigned int signature_0_offset = 0;
	static const unsigned int signature_0_size = sizeof(signature_0);
public:
	FSInfoSector(unsigned char const * const data);
};

#endif

