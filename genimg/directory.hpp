#ifndef _DIRECTORY_HPP_
#define _DIRECTORY_HPP_

#include <string>

class Directory
{
private:
	std::string input_path; // directory path on the source repository
	std::string output_path; // directory path on the disk image
public:
	Directory(std::string path);
};

class DirectoryEntry
{
private:
	char name_prefix[8];
	char name_suffix[3];

	unsigned char attribute;
	static const unsigned char attribute_read_only = 0x01;
	static const unsigned char attribute_hidden = 0x02;
	static const unsigned char attribute_system = 0x04;
	static const unsigned char attribute_volume_id = 0x08;
	static const unsigned char attribute_long_file_name = 0x0f;
	static const unsigned char attribute_directory = 0x10;
	static const unsigned char attribute_archive = 0x20;

	unsigned char name_flag;
	static const unsigned char name_flag_prefix_lower_case = 0x08;
	static const unsigned char name_flag_suffix_lower_case = 0x08;

	unsigned char create_time_centi_second;
	unsigned short create_time;
	unsigned short create_date;
	unsigned short last_open_date;
	unsigned short last_write_time;
	unsigned short last_write_date;
	unsigned int cluster_number;
	unsigned int size;
public:
	DirectoryEntry(void);
};

#endif

